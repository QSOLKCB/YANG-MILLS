// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3I scalar temporal-extent recovery.
//!
//! This phase freezes the negative Phase 3H result and changes only the
//! fine/coarse temporal extent plus the fresh master stream. Spatial geometry,
//! independent-chain count, operators, transformations, fit windows, fit
//! semantics, and stability thresholds remain fixed.

use core::fmt;

use crate::glueball_blocking::{
    spatial_stride2_block, GlueballBlockingError, BLOCKING_CONTRACT_ID, BLOCKING_STATUS,
    BLOCK_FACTOR,
};
use crate::glueball_correlators::{scalar_connected_correlators, GlueballCorrelatorError};
use crate::glueball_effective_mass::EffectiveMassError;
use crate::glueball_operators::{scalar_a1pp_operator_basis, GlueballOperatorError};
use crate::glueball_scalar_fit::{correlated_scalar_fit, ScalarFitError, ScalarFitResult};
use crate::glueball_scalar_production::{
    ChainEvidence, CorrelatorEvidence, OperatorKind, ProductionError, ProductionSummary,
    SurfaceOutcome, SurfaceRecord, BETA as PHASE3F_BETA, CANDIDATE_WINDOWS,
    CORRELATOR_REPLAY_TOLERANCE, MAX_ACCEPTANCE, MIN_ACCEPTANCE, OBSERVED_REJECTION_CODE,
    POST_THERMALIZATION_SWEEPS as PHASE3F_POST_THERMALIZATION_SWEEPS,
    PROPOSAL_ANGLE as PHASE3F_PROPOSAL_ANGLE, STABILITY_MAX_Z, STOUT_LEVELS,
    THERMALIZATION_SWEEPS as PHASE3F_THERMALIZATION_SWEEPS,
    TOTAL_SWEEPS_PER_CHAIN as PHASE3F_TOTAL_SWEEPS_PER_CHAIN,
};
use crate::glueball_scalar_geometry_recovery::{
    CHAIN_COUNT as PHASE3H_CHAIN_COUNT, TEMPORAL_RECOVERY_STATUS as PHASE3H_STATUS,
    MASTER_SEED as PHASE3H_MASTER_SEED,
};
use crate::glueball_smearing::{spatial_stout_smear, CANONICAL_RHO};
use crate::su2_ensemble::SplitMix64;
use crate::su3_ensemble::{
    average_plaquette, derive_su3_chain_seed, metropolis_sweep, MetropolisAccounting,
    LINK_RESIDUAL_LIMIT, PROPOSAL_ID, RNG_ID, SAMPLER_ID, STREAM_ID,
};
use crate::su3_lattice::{Su3Lattice4, DIM};

pub const TEMPORAL_RECOVERY_CONTRACT_ID: &str = "YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1";
pub const TEMPORAL_RECOVERY_STATUS: &str = "phase-3i-candidate";
pub const EXTENTS: [usize; DIM] = [6, 6, 6, 16];
pub const BLOCKED_EXTENTS: [usize; DIM] = [3, 3, 3, 16];
pub const BETA: f64 = PHASE3F_BETA;
pub const MASTER_SEED: u64 = 0x3f03_2026_0927_0001;
pub const CHAIN_COUNT: usize = PHASE3H_CHAIN_COUNT;
pub const THERMALIZATION_SWEEPS: usize = PHASE3F_THERMALIZATION_SWEEPS;
pub const POST_THERMALIZATION_SWEEPS: usize = PHASE3F_POST_THERMALIZATION_SWEEPS;
pub const TOTAL_SWEEPS_PER_CHAIN: usize = PHASE3F_TOTAL_SWEEPS_PER_CHAIN;
pub const PROPOSAL_ANGLE: f64 = PHASE3F_PROPOSAL_ANGLE;
pub const RETAINED_CONFIGURATIONS_PER_CHAIN: usize = 1;
pub const LINKS_PER_SWEEP: u64 = (EXTENTS[0] * EXTENTS[1] * EXTENTS[2] * EXTENTS[3] * DIM) as u64;
pub const PROPOSALS_PER_LINK: u64 = 8;
pub const EXPECTED_PROPOSALS_PER_CHAIN: u64 =
    LINKS_PER_SWEEP * PROPOSALS_PER_LINK * TOTAL_SWEEPS_PER_CHAIN as u64;
pub const EXPECTED_TOTAL_PROPOSALS: u64 = EXPECTED_PROPOSALS_PER_CHAIN * CHAIN_COUNT as u64;
pub const TRANSFORMATION_LABELS: [&str; 4] = ["raw", "stout-1", "stout-2", "block-1"];
pub const PRIMARY_TRANSFORMATION: &str = "block-1";
pub const PRIMARY_OPERATOR: OperatorKind = OperatorKind::Plaquette1x1;

#[derive(Debug)]
pub enum TemporalRecoveryError {
    Analysis(ProductionError),
    Blocking(GlueballBlockingError),
}

impl fmt::Display for TemporalRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Analysis(error) => {
                write!(
                    f,
                    "Phase 3I temporal recovery implementation failure: {error:?}"
                )
            }
            Self::Blocking(error) => {
                write!(f, "Phase 3I frozen blocking transformation failed: {error}")
            }
        }
    }
}

impl std::error::Error for TemporalRecoveryError {}

impl From<ProductionError> for TemporalRecoveryError {
    fn from(value: ProductionError) -> Self {
        Self::Analysis(value)
    }
}

impl From<GlueballBlockingError> for TemporalRecoveryError {
    fn from(value: GlueballBlockingError) -> Self {
        Self::Blocking(value)
    }
}

fn validate_chain_evidence(evidence: &ChainEvidence) -> Result<(), ProductionError> {
    if evidence.proposals == 0 || evidence.accepts > evidence.proposals {
        return Err(ProductionError::AccountingOverflow);
    }
    if !evidence.acceptance.is_finite()
        || !evidence.plaquette.is_finite()
        || !evidence.unitarity_residual.is_finite()
        || !evidence.determinant_residual.is_finite()
    {
        return Err(ProductionError::NonFiniteStatistic);
    }
    if evidence.unitarity_residual < 0.0
        || evidence.determinant_residual < 0.0
        || evidence.unitarity_residual > LINK_RESIDUAL_LIMIT
        || evidence.determinant_residual > LINK_RESIDUAL_LIMIT
    {
        return Err(ProductionError::LinkResidualExceeded);
    }
    Ok(())
}

fn generate_fresh_configurations() -> Result<(Vec<ChainEvidence>, Vec<Su3Lattice4>), ProductionError>
{
    let mut evidence = Vec::new();
    let mut lattices = Vec::new();
    evidence
        .try_reserve_exact(CHAIN_COUNT)
        .map_err(|_| ProductionError::AllocationFailed)?;
    lattices
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
        let chain_evidence = ChainEvidence {
            chain_id,
            derived_seed,
            proposals: accounting.proposals,
            accepts: accounting.accepts,
            acceptance: accounting.accepts as f64 / accounting.proposals as f64,
            plaquette: average_plaquette(&lattice)?,
            unitarity_residual: lattice.max_link_unitarity_residual(),
            determinant_residual: lattice.max_link_determinant_residual(),
        };
        validate_chain_evidence(&chain_evidence)?;
        evidence.push(chain_evidence);
        lattices.push(lattice);
    }

    Ok((evidence, lattices))
}

fn zero_vec(len: usize) -> Result<Vec<f64>, ProductionError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| ProductionError::AllocationFailed)?;
    values.resize(len, 0.0);
    Ok(values)
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

fn block_transform(
    configurations: &[Su3Lattice4],
) -> Result<Vec<Su3Lattice4>, TemporalRecoveryError> {
    let mut out = Vec::new();
    out.try_reserve_exact(configurations.len())
        .map_err(|_| ProductionError::AllocationFailed)?;
    for configuration in configurations {
        let blocked = spatial_stride2_block(configuration)?;
        if blocked.extents() != BLOCKED_EXTENTS {
            return Err(ProductionError::ConfigurationShapeMismatch.into());
        }
        out.push(blocked);
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

pub fn run_scalar_temporal_recovery_analysis() -> Result<ProductionSummary, TemporalRecoveryError> {
    let (chain_evidence, raw) = generate_fresh_configurations()?;
    let sampler_diagnostics_passed = chain_evidence.iter().all(|evidence| {
        (MIN_ACCEPTANCE..=MAX_ACCEPTANCE).contains(&evidence.acceptance)
            && evidence.acceptance == evidence.accepts as f64 / evidence.proposals as f64
    });

    let stout1 = stout_transform(&raw, STOUT_LEVELS[0])?;
    let stout2 = stout_transform(&raw, STOUT_LEVELS[1])?;
    let blocked = block_transform(&raw)?;

    let mut surfaces = Vec::new();
    surfaces
        .try_reserve_exact(8)
        .map_err(|_| ProductionError::AllocationFailed)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[0], &raw)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[1], &stout1)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[2], &stout2)?;
    add_transformation_surfaces(&mut surfaces, TRANSFORMATION_LABELS[3], &blocked)?;

    let primary = surface(&surfaces, PRIMARY_TRANSFORMATION, PRIMARY_OPERATOR)
        .ok_or(ProductionError::ConfigurationShapeMismatch)?;
    let (primary_window_successes, primary_window_max_z) = window_stability(primary)?;

    let operator_stability_z = match (
        surface(
            &surfaces,
            PRIMARY_TRANSFORMATION,
            OperatorKind::Plaquette1x1,
        )
        .and_then(|record| record.outcome.fit.as_ref()),
        surface(
            &surfaces,
            PRIMARY_TRANSFORMATION,
            OperatorKind::Rectangle2x1,
        )
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

    let scalar_pipeline_validated = sampler_diagnostics_passed
        && primary.outcome.succeeded()
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

    #[test]
    fn phase3i_identity_is_predeclared_and_fresh() {
        assert_eq!(
            TEMPORAL_RECOVERY_CONTRACT_ID,
            "YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1"
        );
        assert_eq!(TEMPORAL_RECOVERY_STATUS, "phase-3i-candidate");
        assert_eq!(PHASE3H_STATUS, "phase-3g-frozen");
        assert_eq!(SAMPLER_ID, "YM-SU3-ENSEMBLE-V1");
        assert_eq!(PROPOSAL_ID, "YM-SU3-GENERATOR-METROPOLIS-V1");
        assert_eq!(RNG_ID, "SPLITMIX64-V1");
        assert_eq!(STREAM_ID, "YM-SU3-STREAM-V1");
        assert_eq!(MASTER_SEED, 0x3f03_2026_0927_0001);
        assert_ne!(MASTER_SEED, PHASE3H_MASTER_SEED);
        assert_eq!(CHAIN_COUNT, 32);
        assert_eq!(CHAIN_COUNT, PHASE3H_CHAIN_COUNT);
    }

    #[test]
    fn phase3i_temporal_extent_and_accounting_are_predeclared() {
        assert_eq!(EXTENTS, [6, 6, 6, 16]);
        assert_eq!(BLOCKED_EXTENTS, [3, 3, 3, 16]);
        assert_eq!(BETA, 5.7);
        assert_eq!(THERMALIZATION_SWEEPS, 64);
        assert_eq!(POST_THERMALIZATION_SWEEPS, 4);
        assert_eq!(TOTAL_SWEEPS_PER_CHAIN, 68);
        assert_eq!(PROPOSAL_ANGLE, 0.35);
        assert_eq!(RETAINED_CONFIGURATIONS_PER_CHAIN, 1);
        assert_eq!(LINKS_PER_SWEEP, 8_640);
        assert_eq!(PROPOSALS_PER_LINK, 8);
        assert_eq!(EXPECTED_PROPOSALS_PER_CHAIN, 4_700_160);
        assert_eq!(EXPECTED_TOTAL_PROPOSALS, 150_405_120);
    }

    #[test]
    fn phase3i_analysis_surface_is_unchanged() {
        assert_eq!(BLOCKING_CONTRACT_ID, "YM-GLUEBALL-SPATIAL-BLOCK-V1");
        assert_eq!(BLOCKING_STATUS, "phase-3b2-frozen");
        assert_eq!(BLOCK_FACTOR, 2);
        assert_eq!(CANONICAL_RHO, 0.1);
        assert_eq!(STOUT_LEVELS, [1, 2]);
        assert_eq!(
            TRANSFORMATION_LABELS,
            ["raw", "stout-1", "stout-2", "block-1"]
        );
        assert_eq!(PRIMARY_TRANSFORMATION, "block-1");
        assert_eq!(PRIMARY_OPERATOR, OperatorKind::Plaquette1x1);
        assert_eq!(STABILITY_MAX_Z, 5.0);
        assert_eq!(CORRELATOR_REPLAY_TOLERANCE, 2.0e-12);
        assert_eq!(LINK_RESIDUAL_LIMIT, 5.0e-12);
        assert_eq!(CANDIDATE_WINDOWS.len(), 3);
    }

    #[test]
    fn impossible_acceptance_accounting_is_an_implementation_failure() {
        let evidence = ChainEvidence {
            chain_id: 0,
            derived_seed: 0,
            proposals: 10,
            accepts: 11,
            acceptance: 1.1,
            plaquette: 0.5,
            unitarity_residual: 0.0,
            determinant_residual: 0.0,
        };
        assert_eq!(
            validate_chain_evidence(&evidence),
            Err(ProductionError::AccountingOverflow)
        );
    }

    #[test]
    fn computational_failures_remain_implementation_failures() {
        assert_eq!(
            scientific_fit_error_code(&ScalarFitError::AllocationFailed),
            None
        );
        assert_eq!(
            scientific_fit_error_code(&ScalarFitError::NonFiniteStatistic),
            None
        );
        assert_eq!(
            production_fit_error(ScalarFitError::AllocationFailed),
            ProductionError::AllocationFailed
        );
    }
}
