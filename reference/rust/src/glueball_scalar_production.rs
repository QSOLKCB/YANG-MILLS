// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3F production scalar spectroscopy and stability evidence.
//!
//! This surface generates one retained configuration from each independently
//! seeded frozen SU(3) Markov chain, evaluates the frozen scalar operator and
//! correlator authorities on raw and stout-transformed configurations, and
//! applies the frozen Phase 3E fit mechanics. Scientific stability gates are
//! reported as evidence; they are not silently converted into implementation
//! failures and they are not continuum or mass-gap claims.

use core::fmt;

use crate::glueball_correlators::{scalar_connected_correlators, GlueballCorrelatorError};
use crate::glueball_effective_mass::EffectiveMassError;
use crate::glueball_operators::{scalar_a1pp_operator_basis, GlueballOperatorError};
use crate::glueball_scalar_fit::{
    correlated_scalar_fit, FitWindow, ScalarFitError, ScalarFitResult,
};
use crate::glueball_smearing::{spatial_stout_smear, GlueballSmearingError, CANONICAL_RHO};
use crate::su2_ensemble::SplitMix64;
use crate::su3_ensemble::{
    average_plaquette, derive_su3_chain_seed, metropolis_sweep, MetropolisAccounting,
    Su3EnsembleError, LINK_RESIDUAL_LIMIT, PROPOSAL_ID, RNG_ID, SAMPLER_ID, STREAM_ID,
};
use crate::su3_lattice::{Su3Lattice4, Su3LatticeError};

pub const PRODUCTION_CONTRACT_ID: &str = "YM-GLUEBALL-SCALAR-PRODUCTION-V1";
pub const PRODUCTION_STATUS: &str = "phase-3f-candidate";
pub const EXTENTS: [usize; 4] = [4, 4, 4, 10];
pub const BETA: f64 = 5.7;
pub const MASTER_SEED: u64 = 0x3f00_2026_0915_0001;
pub const CHAIN_COUNT: usize = 16;
pub const THERMALIZATION_SWEEPS: usize = 64;
pub const POST_THERMALIZATION_SWEEPS: usize = 4;
pub const TOTAL_SWEEPS_PER_CHAIN: usize = THERMALIZATION_SWEEPS + POST_THERMALIZATION_SWEEPS;
pub const PROPOSAL_ANGLE: f64 = 0.35;
pub const RETAINED_CONFIGURATIONS_PER_CHAIN: usize = 1;
pub const LINKS_PER_SWEEP: u64 = (EXTENTS[0] * EXTENTS[1] * EXTENTS[2] * EXTENTS[3] * 4) as u64;
pub const PROPOSALS_PER_LINK: u64 = 8;
pub const EXPECTED_PROPOSALS_PER_CHAIN: u64 =
    LINKS_PER_SWEEP * PROPOSALS_PER_LINK * TOTAL_SWEEPS_PER_CHAIN as u64;
pub const MIN_ACCEPTANCE: f64 = 0.10;
pub const MAX_ACCEPTANCE: f64 = 0.95;
pub const STOUT_LEVELS: [usize; 2] = [1, 2];
pub const TRANSFORMATION_LABELS: [&str; 3] = ["raw", "stout-1", "stout-2"];
pub const PRIMARY_TRANSFORMATION: &str = "stout-2";
pub const STABILITY_MAX_Z: f64 = 5.0;
pub const CORRELATOR_REPLAY_TOLERANCE: f64 = 2.0e-12;
pub const CANDIDATE_WINDOWS: [FitWindow; 3] = [
    FitWindow { start: 1, end: 3 },
    FitWindow { start: 2, end: 4 },
    FitWindow { start: 1, end: 4 },
];
pub const OBSERVED_RECEIPT_HEAD: &str = "88b7f450701884e6351878e7890410e4095e3d6d";
pub const OBSERVED_RECEIPT_WORKFLOW_RUN_ID: u64 = 34_862_755_015;
pub const OBSERVED_REJECTION_CODE: &str = "effective-mass-rejected";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChainEvidence {
    pub chain_id: u64,
    pub derived_seed: u64,
    pub proposals: u64,
    pub accepts: u64,
    pub acceptance: f64,
    pub plaquette: f64,
    pub unitarity_residual: f64,
    pub determinant_residual: f64,
}

struct RetainedConfiguration {
    evidence: ChainEvidence,
    lattice: Su3Lattice4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorKind {
    Plaquette1x1,
    Rectangle2x1,
}

impl OperatorKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Plaquette1x1 => "plaquette-1x1",
            Self::Rectangle2x1 => "rectangle-2x1",
        }
    }
}

pub const PRIMARY_OPERATOR: OperatorKind = OperatorKind::Plaquette1x1;
pub const OBSERVED_SURFACE_OUTCOMES: [(&str, OperatorKind, &str); 6] = [
    ("raw", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    (
        "raw",
        OperatorKind::Rectangle2x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-1",
        OperatorKind::Plaquette1x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-1",
        OperatorKind::Rectangle2x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-2",
        OperatorKind::Plaquette1x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-2",
        OperatorKind::Rectangle2x1,
        OBSERVED_REJECTION_CODE,
    ),
];

#[derive(Clone, Debug)]
pub struct CorrelatorEvidence {
    pub full: Vec<f64>,
    pub jackknife: Vec<Vec<f64>>,
}

#[derive(Clone, Debug)]
pub struct SurfaceOutcome {
    pub transformation: &'static str,
    pub operator: OperatorKind,
    pub fit: Option<ScalarFitResult>,
    pub error_code: Option<&'static str>,
}

impl SurfaceOutcome {
    pub fn succeeded(&self) -> bool {
        self.fit.is_some()
    }
}

#[derive(Clone, Debug)]
pub struct SurfaceRecord {
    pub outcome: SurfaceOutcome,
    pub correlator: CorrelatorEvidence,
}

#[derive(Clone, Debug)]
pub struct ProductionSummary {
    pub chain_evidence: Vec<ChainEvidence>,
    pub surfaces: Vec<SurfaceRecord>,
    pub sampler_diagnostics_passed: bool,
    pub primary_window_successes: usize,
    pub primary_window_max_z: Option<f64>,
    pub operator_stability_z: Option<f64>,
    pub transformation_stability_successes: usize,
    pub transformation_stability_max_z: Option<f64>,
    pub scalar_pipeline_validated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionError {
    AllocationFailed,
    AccountingOverflow,
    EmptyConfigurationSet,
    ConfigurationShapeMismatch,
    NonFiniteStatistic,
    CorrelatorReplayMismatch,
    LinkResidualExceeded,
    Sampler(Su3EnsembleError),
    Lattice(Su3LatticeError),
    Smearing(GlueballSmearingError),
    Operator(GlueballOperatorError),
    Correlator(GlueballCorrelatorError),
    Fit(ScalarFitError),
}

impl fmt::Display for ProductionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::AllocationFailed => "Phase 3F bounded allocation failed",
            Self::AccountingOverflow => "Phase 3F sampler accounting overflowed",
            Self::EmptyConfigurationSet => "Phase 3F requires retained configurations",
            Self::ConfigurationShapeMismatch => "Phase 3F configuration shapes do not match",
            Self::NonFiniteStatistic => "Phase 3F produced a non-finite statistic",
            Self::CorrelatorReplayMismatch => {
                "Phase 3F operator-sample correlator disagrees with frozen Phase 3C"
            }
            Self::LinkResidualExceeded => {
                "Phase 3F generated link violated the frozen SU(3) residual bound"
            }
            Self::Sampler(_) => "Phase 3F frozen SU(3) sampler failed",
            Self::Lattice(_) => "Phase 3F lattice operation failed",
            Self::Smearing(_) => "Phase 3F stout transformation failed",
            Self::Operator(_) => "Phase 3F scalar operator evaluation failed",
            Self::Correlator(_) => "Phase 3F frozen correlator evaluation failed",
            Self::Fit(_) => "Phase 3F frozen scalar fit failed computationally",
        };
        f.write_str(message)
    }
}

impl std::error::Error for ProductionError {}

impl From<Su3EnsembleError> for ProductionError {
    fn from(value: Su3EnsembleError) -> Self {
        Self::Sampler(value)
    }
}

impl From<Su3LatticeError> for ProductionError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

impl From<GlueballSmearingError> for ProductionError {
    fn from(value: GlueballSmearingError) -> Self {
        Self::Smearing(value)
    }
}

impl From<GlueballOperatorError> for ProductionError {
    fn from(value: GlueballOperatorError) -> Self {
        Self::Operator(value)
    }
}

impl From<GlueballCorrelatorError> for ProductionError {
    fn from(value: GlueballCorrelatorError) -> Self {
        Self::Correlator(value)
    }
}

fn zero_vec(len: usize) -> Result<Vec<f64>, ProductionError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| ProductionError::AllocationFailed)?;
    values.resize(len, 0.0);
    Ok(values)
}

fn validate_chain_evidence(evidence: &ChainEvidence) -> Result<(), ProductionError> {
    if !evidence.acceptance.is_finite()
        || !evidence.plaquette.is_finite()
        || !evidence.unitarity_residual.is_finite()
        || !evidence.determinant_residual.is_finite()
    {
        return Err(ProductionError::NonFiniteStatistic);
    }
    if evidence.unitarity_residual > LINK_RESIDUAL_LIMIT
        || evidence.determinant_residual > LINK_RESIDUAL_LIMIT
    {
        return Err(ProductionError::LinkResidualExceeded);
    }
    Ok(())
}

fn generate_independent_configurations() -> Result<Vec<RetainedConfiguration>, ProductionError> {
    let mut retained = Vec::new();
    retained
        .try_reserve_exact(CHAIN_COUNT)
        .map_err(|_| ProductionError::AllocationFailed)?;

    for chain_index in 0..CHAIN_COUNT {
        let chain_id = chain_index as u64;
        let derived_seed = derive_su3_chain_seed(MASTER_SEED, chain_id);
        let mut rng = SplitMix64::new(derived_seed);
        let mut lattice = Su3Lattice4::new_identity(EXTENTS)?;
        let mut accounting = MetropolisAccounting::default();
        for _ in 0..TOTAL_SWEEPS_PER_CHAIN {
            let step = metropolis_sweep(&mut lattice, BETA, PROPOSAL_ANGLE, &mut rng)?;
            accounting.proposals = accounting
                .proposals
                .checked_add(step.proposals)
                .ok_or(ProductionError::AccountingOverflow)?;
            accounting.accepts = accounting
                .accepts
                .checked_add(step.accepts)
                .ok_or(ProductionError::AccountingOverflow)?;
        }
        if accounting.proposals != EXPECTED_PROPOSALS_PER_CHAIN {
            return Err(ProductionError::AccountingOverflow);
        }
        let evidence = ChainEvidence {
            chain_id,
            derived_seed,
            proposals: accounting.proposals,
            accepts: accounting.accepts,
            acceptance: accounting.accepts as f64 / accounting.proposals as f64,
            plaquette: average_plaquette(&lattice)?,
            unitarity_residual: lattice.max_link_unitarity_residual(),
            determinant_residual: lattice.max_link_determinant_residual(),
        };
        validate_chain_evidence(&evidence)?;
        retained.push(RetainedConfiguration { evidence, lattice });
    }
    Ok(retained)
}

fn extract_lattices(
    configurations: &[RetainedConfiguration],
) -> Result<Vec<Su3Lattice4>, ProductionError> {
    let mut out = Vec::new();
    out.try_reserve_exact(configurations.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for configuration in configurations {
        let extents = configuration.lattice.extents();
        let mut copy = Su3Lattice4::new_identity(extents)?;
        for x0 in 0..extents[0] {
            for x1 in 0..extents[1] {
                for x2 in 0..extents[2] {
                    for x3 in 0..extents[3] {
                        let x = [x0, x1, x2, x3];
                        for mu in 0..4 {
                            copy.set_link(x, mu, configuration.lattice.link(x, mu)?)?;
                        }
                    }
                }
            }
        }
        out.push(copy);
    }
    Ok(out)
}

fn stout_transform(
    configurations: &[Su3Lattice4],
    levels: usize,
) -> Result<Vec<Su3Lattice4>, ProductionError> {
    let mut out = Vec::new();
    out.try_reserve_exact(configurations.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for configuration in configurations {
        out.push(spatial_stout_smear(configuration, CANONICAL_RHO, levels)?);
    }
    Ok(out)
}

fn operator_samples(
    configurations: &[Su3Lattice4],
) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), ProductionError> {
    let first = configurations
        .first()
        .ok_or(ProductionError::EmptyConfigurationSet)?;
    let extents = first.extents();
    let time_extent = extents[3];
    if configurations
        .iter()
        .any(|configuration| configuration.extents() != extents)
    {
        return Err(ProductionError::ConfigurationShapeMismatch);
    }

    let mut plaquette = Vec::new();
    let mut rectangle = Vec::new();
    plaquette
        .try_reserve_exact(configurations.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    rectangle
        .try_reserve_exact(configurations.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for configuration in configurations {
        let mut p = Vec::new();
        let mut r = Vec::new();
        p.try_reserve_exact(time_extent)
            .map_err(|_| ProductionError::AllocationFailed)?;
        r.try_reserve_exact(time_extent)
            .map_err(|_| ProductionError::AllocationFailed)?;
        for time in 0..time_extent {
            let basis = scalar_a1pp_operator_basis(configuration, time)?;
            if !basis.spatial_plaquette_1x1.is_finite() || !basis.spatial_rectangle_2x1.is_finite()
            {
                return Err(ProductionError::NonFiniteStatistic);
            }
            p.push(basis.spatial_plaquette_1x1);
            r.push(basis.spatial_rectangle_2x1);
        }
        plaquette.push(p);
        rectangle.push(r);
    }
    Ok((plaquette, rectangle))
}

fn connected_from_samples(
    samples: &[Vec<f64>],
    omitted: Option<usize>,
) -> Result<Vec<f64>, ProductionError> {
    let first = samples
        .first()
        .ok_or(ProductionError::EmptyConfigurationSet)?;
    let time_extent = first.len();
    if time_extent == 0 || samples.iter().any(|row| row.len() != time_extent) {
        return Err(ProductionError::ConfigurationShapeMismatch);
    }
    let kept = match omitted {
        Some(index) if index < samples.len() => samples
            .len()
            .checked_sub(1)
            .ok_or(ProductionError::EmptyConfigurationSet)?,
        Some(_) => return Err(ProductionError::ConfigurationShapeMismatch),
        None => samples.len(),
    };
    if kept == 0 {
        return Err(ProductionError::EmptyConfigurationSet);
    }
    let sample_count = kept
        .checked_mul(time_extent)
        .ok_or(ProductionError::AccountingOverflow)?;
    let mut mean_sum = 0.0_f64;
    for (index, row) in samples.iter().enumerate() {
        if omitted == Some(index) {
            continue;
        }
        for &value in row {
            if !value.is_finite() {
                return Err(ProductionError::NonFiniteStatistic);
            }
            mean_sum += value;
        }
    }
    let mean = mean_sum / sample_count as f64;
    let mean_square = mean * mean;
    if !mean.is_finite() || !mean_square.is_finite() {
        return Err(ProductionError::NonFiniteStatistic);
    }
    let mut connected = zero_vec(time_extent)?;
    for tau in 0..time_extent {
        let mut sum = 0.0_f64;
        for (index, row) in samples.iter().enumerate() {
            if omitted == Some(index) {
                continue;
            }
            for time in 0..time_extent {
                sum += row[time] * row[(time + tau) % time_extent];
            }
        }
        let value = sum / sample_count as f64 - mean_square;
        if !value.is_finite() {
            return Err(ProductionError::NonFiniteStatistic);
        }
        connected[tau] = value;
    }
    Ok(connected)
}

fn correlator_evidence(
    configurations: &[Su3Lattice4],
    samples: &[Vec<f64>],
    operator: OperatorKind,
) -> Result<CorrelatorEvidence, ProductionError> {
    let full = connected_from_samples(samples, None)?;
    let frozen = scalar_connected_correlators(configurations)?;
    let frozen_series = match operator {
        OperatorKind::Plaquette1x1 => &frozen.plaquette_connected,
        OperatorKind::Rectangle2x1 => &frozen.rectangle_connected,
    };
    if frozen_series.len() != full.len()
        || frozen_series
            .iter()
            .zip(&full)
            .any(|(&left, &right)| (left - right).abs() > CORRELATOR_REPLAY_TOLERANCE)
    {
        return Err(ProductionError::CorrelatorReplayMismatch);
    }
    let mut jackknife = Vec::new();
    jackknife
        .try_reserve_exact(samples.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for omitted in 0..samples.len() {
        jackknife.push(connected_from_samples(samples, Some(omitted))?);
    }
    Ok(CorrelatorEvidence { full, jackknife })
}

fn scientific_fit_error_code(error: &ScalarFitError) -> Option<&'static str> {
    match error {
        ScalarFitError::DegeneratePlateauUncertainty => Some("degenerate-plateau-uncertainty"),
        ScalarFitError::NoAcceptedPlateauWindow => Some("no-accepted-plateau-window"),
        ScalarFitError::CovarianceNotPositiveDefinite => Some("covariance-not-positive-definite"),
        ScalarFitError::SingularCovariance => Some("singular-covariance"),
        ScalarFitError::NoFiniteGridFit => Some("no-finite-grid-fit"),
        ScalarFitError::EffectiveMass(EffectiveMassError::NonPositiveCentralCorrelator)
        | ScalarFitError::EffectiveMass(EffectiveMassError::AcoshDomainViolation) => {
            Some(OBSERVED_REJECTION_CODE)
        }
        _ => None,
    }
}

fn production_fit_error(error: ScalarFitError) -> ProductionError {
    match error {
        ScalarFitError::AllocationFailed
        | ScalarFitError::EffectiveMass(EffectiveMassError::AllocationFailed) => {
            ProductionError::AllocationFailed
        }
        ScalarFitError::NonFiniteStatistic
        | ScalarFitError::EffectiveMass(EffectiveMassError::NonFiniteCorrelator)
        | ScalarFitError::EffectiveMass(EffectiveMassError::NonFiniteStatistic) => {
            ProductionError::NonFiniteStatistic
        }
        other => ProductionError::Fit(other),
    }
}

fn analyze_surface(
    transformation: &'static str,
    operator: OperatorKind,
    correlator: CorrelatorEvidence,
) -> Result<SurfaceRecord, ProductionError> {
    match correlated_scalar_fit(&correlator.full, &correlator.jackknife, &CANDIDATE_WINDOWS) {
        Ok(fit) => Ok(SurfaceRecord {
            outcome: SurfaceOutcome {
                transformation,
                operator,
                fit: Some(fit),
                error_code: None,
            },
            correlator,
        }),
        Err(error) => match scientific_fit_error_code(&error) {
            Some(error_code) => Ok(SurfaceRecord {
                outcome: SurfaceOutcome {
                    transformation,
                    operator,
                    fit: None,
                    error_code: Some(error_code),
                },
                correlator,
            }),
            None => Err(production_fit_error(error)),
        },
    }
}

fn add_transformation_surfaces(
    surfaces: &mut Vec<SurfaceRecord>,
    transformation: &'static str,
    configurations: &[Su3Lattice4],
) -> Result<(), ProductionError> {
    let (plaquette_samples, rectangle_samples) = operator_samples(configurations)?;
    let p = correlator_evidence(
        configurations,
        &plaquette_samples,
        OperatorKind::Plaquette1x1,
    )?;
    let r = correlator_evidence(
        configurations,
        &rectangle_samples,
        OperatorKind::Rectangle2x1,
    )?;
    surfaces.push(analyze_surface(
        transformation,
        OperatorKind::Plaquette1x1,
        p,
    )?);
    surfaces.push(analyze_surface(
        transformation,
        OperatorKind::Rectangle2x1,
        r,
    )?);
    Ok(())
}

fn stable_z(first: &ScalarFitResult, second: &ScalarFitResult) -> Option<f64> {
    let scale = first.mass_standard_error.hypot(second.mass_standard_error);
    if !scale.is_finite() || scale <= 0.0 {
        return None;
    }
    let z = (first.fitted_mass - second.fitted_mass).abs() / scale;
    z.is_finite().then_some(z)
}

fn maximum_pairwise_z(fits: &[ScalarFitResult]) -> (usize, Option<f64>) {
    if fits.len() < 2 {
        return (fits.len(), None);
    }
    let mut maximum = 0.0_f64;
    for first in 0..fits.len() {
        for second in (first + 1)..fits.len() {
            let Some(z) = stable_z(&fits[first], &fits[second]) else {
                return (fits.len(), None);
            };
            maximum = maximum.max(z);
        }
    }
    (fits.len(), Some(maximum))
}

fn window_stability(primary: &SurfaceRecord) -> Result<(usize, Option<f64>), ProductionError> {
    let mut successful = Vec::new();
    successful
        .try_reserve_exact(CANDIDATE_WINDOWS.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for &window in &CANDIDATE_WINDOWS {
        match correlated_scalar_fit(
            &primary.correlator.full,
            &primary.correlator.jackknife,
            &[window],
        ) {
            Ok(fit) => successful.push(fit),
            Err(error) if scientific_fit_error_code(&error).is_some() => {}
            Err(error) => return Err(production_fit_error(error)),
        }
    }
    Ok(maximum_pairwise_z(&successful))
}

fn surface<'a>(
    surfaces: &'a [SurfaceRecord],
    transformation: &str,
    operator: OperatorKind,
) -> Option<&'a SurfaceRecord> {
    surfaces.iter().find(|record| {
        record.outcome.transformation == transformation && record.outcome.operator == operator
    })
}

pub fn observed_phase3f_receipt_matches(summary: &ProductionSummary) -> bool {
    let surfaces_match = summary.surfaces.len() == OBSERVED_SURFACE_OUTCOMES.len()
        && summary
            .surfaces
            .iter()
            .zip(OBSERVED_SURFACE_OUTCOMES.iter())
            .all(|(record, expected)| {
                let (transformation, operator, rejection_code) = *expected;
                record.outcome.transformation == transformation
                    && record.outcome.operator == operator
                    && record.outcome.fit.is_none()
                    && record.outcome.error_code == Some(rejection_code)
            });
    summary.sampler_diagnostics_passed
        && surfaces_match
        && summary.primary_window_successes == 0
        && summary.primary_window_max_z.is_none()
        && summary.operator_stability_z.is_none()
        && summary.transformation_stability_successes == 0
        && summary.transformation_stability_max_z.is_none()
        && !summary.scalar_pipeline_validated
}

pub fn run_production_scalar_analysis() -> Result<ProductionSummary, ProductionError> {
    let retained = generate_independent_configurations()?;
    let mut chain_evidence = Vec::new();
    chain_evidence
        .try_reserve_exact(retained.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for configuration in &retained {
        chain_evidence.push(configuration.evidence);
    }
    let sampler_diagnostics_passed = chain_evidence
        .iter()
        .all(|evidence| (MIN_ACCEPTANCE..=MAX_ACCEPTANCE).contains(&evidence.acceptance));

    let raw = extract_lattices(&retained)?;
    let stout1 = stout_transform(&raw, STOUT_LEVELS[0])?;
    let stout2 = stout_transform(&raw, STOUT_LEVELS[1])?;
    let mut surfaces = Vec::new();
    surfaces
        .try_reserve_exact(6)
        .map_err(|_| ProductionError::AllocationFailed)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[0], &raw)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[1], &stout1)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[2], &stout2)?;

    let primary = surface(&surfaces, PRIMARY_TRANSFORMATION, PRIMARY_OPERATOR)
        .ok_or(ProductionError::ConfigurationShapeMismatch)?;
    let (primary_window_successes, primary_window_max_z) = window_stability(primary)?;

    let operator_stability_z = match (
        surface(&surfaces, PRIMARY_TRANSFORMATION, OperatorKind::Plaquette1x1)
            .and_then(|record| record.outcome.fit.as_ref()),
        surface(&surfaces, PRIMARY_TRANSFORMATION, OperatorKind::Rectangle2x1)
            .and_then(|record| record.outcome.fit.as_ref()),
    ) {
        (Some(first), Some(second)) => stable_z(first, second),
        _ => None,
    };

    let mut transformation_fits = Vec::new();
    transformation_fits
        .try_reserve_exact(TRANSFORMATION_LABELS.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for label in TRANSFORMATION_LABELS {
        if let Some(fit) = surface(&surfaces, label, PRIMARY_OPERATOR)
            .and_then(|record| record.outcome.fit.as_ref())
        {
            transformation_fits.push(fit.clone());
        }
    }
    let (transformation_stability_successes, transformation_stability_max_z) =
        maximum_pairwise_z(&transformation_fits);

    let primary_succeeded = primary.outcome.succeeded();
    let scalar_pipeline_validated = sampler_diagnostics_passed
        && primary_succeeded
        && primary_window_successes >= 2
        && primary_window_max_z.is_some_and(|z| z <= STABILITY_MAX_Z)
        && operator_stability_z.is_some_and(|z| z <= STABILITY_MAX_Z)
        && transformation_stability_successes >= 2
        && transformation_stability_max_z.is_some_and(|z| z <= STABILITY_MAX_Z);

    Ok(ProductionSummary {
        chain_evidence,
        surfaces,
        sampler_diagnostics_passed,
        primary_window_successes,
        primary_window_max_z,
        operator_stability_z,
        transformation_stability_successes,
        transformation_stability_max_z,
        scalar_pipeline_validated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fit_result(mass: f64, standard_error: f64) -> ScalarFitResult {
        ScalarFitResult {
            time_extent: 10,
            jackknife_replicates: 16,
            selected_window: FitWindow { start: 1, end: 3 },
            plateau_weighted_effective_mass: mass,
            plateau_maximum_z: 1.0,
            fitted_mass: mass,
            fitted_amplitude: 1.0,
            chi_square: 1.0,
            degrees_of_freedom: 1,
            chi_square_per_dof: 1.0,
            mass_jackknife_mean: mass,
            mass_standard_error: standard_error,
            amplitude_jackknife_mean: 1.0,
            amplitude_standard_error: 0.1,
            maximum_off_diagonal_correlation: 0.2,
        }
    }

    #[test]
    fn phase3f_identity_is_predeclared() {
        assert_eq!(PRODUCTION_CONTRACT_ID, "YM-GLUEBALL-SCALAR-PRODUCTION-V1");
        assert_eq!(PRODUCTION_STATUS, "phase-3f-candidate");
        assert_eq!(SAMPLER_ID, "YM-SU3-ENSEMBLE-V1");
        assert_eq!(PROPOSAL_ID, "YM-SU3-GENERATOR-METROPOLIS-V1");
        assert_eq!(RNG_ID, "SPLITMIX64-V1");
        assert_eq!(STREAM_ID, "YM-SU3-STREAM-V1");
        assert_eq!(EXTENTS, [4, 4, 4, 10]);
        assert_eq!(BETA, 5.7);
        assert_eq!(MASTER_SEED, 0x3f00_2026_0915_0001);
        assert_eq!(CHAIN_COUNT, 16);
        assert_eq!(THERMALIZATION_SWEEPS, 64);
        assert_eq!(POST_THERMALIZATION_SWEEPS, 4);
        assert_eq!(TOTAL_SWEEPS_PER_CHAIN, 68);
        assert_eq!(PROPOSAL_ANGLE, 0.35);
        assert_eq!(RETAINED_CONFIGURATIONS_PER_CHAIN, 1);
        assert_eq!(LINKS_PER_SWEEP, 2560);
        assert_eq!(PROPOSALS_PER_LINK, 8);
        assert_eq!(EXPECTED_PROPOSALS_PER_CHAIN, 1_392_640);
        assert_eq!(MIN_ACCEPTANCE, 0.1);
        assert_eq!(MAX_ACCEPTANCE, 0.95);
        assert_eq!(LINK_RESIDUAL_LIMIT, 5.0e-12);
        assert_eq!(CANONICAL_RHO, 0.1);
        assert_eq!(STOUT_LEVELS, [1, 2]);
        assert_eq!(TRANSFORMATION_LABELS, ["raw", "stout-1", "stout-2"]);
        assert_eq!(PRIMARY_TRANSFORMATION, "stout-2");
        assert_eq!(PRIMARY_OPERATOR, OperatorKind::Plaquette1x1);
        assert_eq!(STABILITY_MAX_Z, 5.0);
        assert_eq!(CORRELATOR_REPLAY_TOLERANCE, 2.0e-12);
        assert_eq!(OBSERVED_RECEIPT_HEAD, "88b7f450701884e6351878e7890410e4095e3d6d");
        assert_eq!(OBSERVED_RECEIPT_WORKFLOW_RUN_ID, 34_862_755_015);
        assert_eq!(
            CANDIDATE_WINDOWS,
            [
                FitWindow { start: 1, end: 3 },
                FitWindow { start: 2, end: 4 },
                FitWindow { start: 1, end: 4 },
            ]
        );
    }

    #[test]
    fn link_residual_violation_is_an_implementation_failure() {
        let mut evidence = ChainEvidence {
            chain_id: 0,
            derived_seed: 1,
            proposals: EXPECTED_PROPOSALS_PER_CHAIN,
            accepts: EXPECTED_PROPOSALS_PER_CHAIN / 2,
            acceptance: 0.5,
            plaquette: 0.6,
            unitarity_residual: LINK_RESIDUAL_LIMIT,
            determinant_residual: LINK_RESIDUAL_LIMIT,
        };
        assert_eq!(validate_chain_evidence(&evidence), Ok(()));
        evidence.unitarity_residual = LINK_RESIDUAL_LIMIT * 2.0;
        assert_eq!(
            validate_chain_evidence(&evidence),
            Err(ProductionError::LinkResidualExceeded)
        );
        evidence.unitarity_residual = 0.0;
        evidence.determinant_residual = LINK_RESIDUAL_LIMIT * 2.0;
        assert_eq!(
            validate_chain_evidence(&evidence),
            Err(ProductionError::LinkResidualExceeded)
        );
    }

    #[test]
    fn computational_fit_failures_are_not_scientific_rejections() {
        assert_eq!(scientific_fit_error_code(&ScalarFitError::AllocationFailed), None);
        assert_eq!(
            scientific_fit_error_code(&ScalarFitError::NonFiniteStatistic),
            None
        );
        assert_eq!(
            scientific_fit_error_code(&ScalarFitError::EffectiveMass(
                EffectiveMassError::AllocationFailed
            )),
            None
        );
        assert_eq!(
            scientific_fit_error_code(&ScalarFitError::EffectiveMass(
                EffectiveMassError::NonPositiveCentralCorrelator
            )),
            Some(OBSERVED_REJECTION_CODE)
        );
        assert_eq!(
            production_fit_error(ScalarFitError::AllocationFailed),
            ProductionError::AllocationFailed
        );
        assert_eq!(
            production_fit_error(ScalarFitError::NonFiniteStatistic),
            ProductionError::NonFiniteStatistic
        );
    }

    #[test]
    fn stability_z_uses_combined_uncertainty() {
        let mut first = fit_result(0.50, 0.03);
        let mut second = fit_result(0.55, 0.04);
        assert!((stable_z(&first, &second).unwrap() - 1.0).abs() <= 1.0e-12);
        first.mass_standard_error = 0.0;
        second.mass_standard_error = 0.0;
        assert_eq!(stable_z(&first, &second), None);
    }

    #[test]
    fn pairwise_stability_requires_every_successful_pair_to_be_scorable() {
        let fits = [
            fit_result(0.50, 0.0),
            fit_result(0.55, 0.0),
            fit_result(0.525, 0.05),
        ];
        assert_eq!(maximum_pairwise_z(&fits), (3, None));
    }

    #[test]
    fn pinned_observed_receipt_matches_only_the_recorded_negative_outcome() {
        let surfaces = OBSERVED_SURFACE_OUTCOMES
            .iter()
            .map(|&(transformation, operator, rejection_code)| SurfaceRecord {
                outcome: SurfaceOutcome {
                    transformation,
                    operator,
                    fit: None,
                    error_code: Some(rejection_code),
                },
                correlator: CorrelatorEvidence {
                    full: Vec::new(),
                    jackknife: Vec::new(),
                },
            })
            .collect();
        let mut summary = ProductionSummary {
            chain_evidence: Vec::new(),
            surfaces,
            sampler_diagnostics_passed: true,
            primary_window_successes: 0,
            primary_window_max_z: None,
            operator_stability_z: None,
            transformation_stability_successes: 0,
            transformation_stability_max_z: None,
            scalar_pipeline_validated: false,
        };
        assert!(observed_phase3f_receipt_matches(&summary));
        summary.scalar_pipeline_validated = true;
        assert!(!observed_phase3f_receipt_matches(&summary));
    }
}
