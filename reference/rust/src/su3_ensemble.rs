// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2C1 bounded SU(3) statistical-ensemble reference surface.
//!
//! This correctness-first reference uses symmetric left-multiplicative
//! proposals along the eight frozen Gell-Mann generator directions followed
//! by the Metropolis acceptance rule. It deliberately does not claim to be a
//! Cabibbo-Marinari quasi-heatbath implementation; those algorithms are
//! literature context for later performance work.

use core::fmt;

use crate::su2_ensemble::{diagnostics, ChainDiagnostics, SplitMix64};
use crate::su3::{Su3, Su3Error, SU3_GENERATORS};
use crate::su3_lattice::{Su3Lattice4, Su3LatticeError, DIM, MAX_SU3_REFERENCE_LINKS};

pub const SAMPLER_ID: &str = "YM-SU3-ENSEMBLE-V1";
pub const PROPOSAL_ID: &str = "YM-SU3-GENERATOR-METROPOLIS-V1";
pub const RNG_ID: &str = "SPLITMIX64-V1";
pub const STREAM_ID: &str = "YM-SU3-STREAM-V1";
pub const STREAM_DOMAIN: u64 = 0x5355_3345_4e53_5631;
pub const MAX_REFERENCE_BETA: f64 = 12.0;
pub const MAX_REFERENCE_SWEEPS: usize = 512;
pub const MAX_REFERENCE_MEASUREMENTS: usize = 512;
pub const MIN_PROPOSAL_ANGLE: f64 = 1.0e-8;
pub const MAX_PROPOSAL_ANGLE: f64 = 1.0;
pub const PLAQUETTE_TOLERANCE: f64 = 1.0e-12;
pub const LINK_RESIDUAL_LIMIT: f64 = 5.0e-12;
pub const ANGLE_GRID_MAGNITUDE_BITS: u32 = 52;
const ANGLE_GRID_MAGNITUDE_MASK: u64 = (1_u64 << ANGLE_GRID_MAGNITUDE_BITS) - 1;
const ANGLE_GRID_SCALE: f64 = 1.0 / 9_007_199_254_740_992.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Su3EnsembleConfig {
    pub extents: [usize; DIM],
    pub beta: f64,
    pub master_seed: u64,
    pub chain_id: u64,
    pub thermalization_sweeps: usize,
    pub measurements: usize,
    pub sweeps_between_measurements: usize,
    pub proposal_angle: f64,
    pub autocorrelation_max_lag: usize,
}

fn validate_proposal_angle(proposal_angle: f64) -> Result<(), Su3EnsembleError> {
    if !proposal_angle.is_finite()
        || proposal_angle < MIN_PROPOSAL_ANGLE
        || proposal_angle > MAX_PROPOSAL_ANGLE
    {
        return Err(Su3EnsembleError::ProposalAngleOutsideReferenceRange);
    }
    Ok(())
}

impl Su3EnsembleConfig {
    pub fn validate(self) -> Result<Self, Su3EnsembleError> {
        if !self.beta.is_finite() || self.beta < 0.0 || self.beta > MAX_REFERENCE_BETA {
            return Err(Su3EnsembleError::BetaOutsideReferenceRange);
        }
        validate_proposal_angle(self.proposal_angle)?;
        if self.measurements < 2 || self.measurements > MAX_REFERENCE_MEASUREMENTS {
            return Err(Su3EnsembleError::MeasurementCountOutsideReferenceRange);
        }
        if self.sweeps_between_measurements == 0 {
            return Err(Su3EnsembleError::ZeroMeasurementStride);
        }
        if self.autocorrelation_max_lag >= self.measurements {
            return Err(Su3EnsembleError::AutocorrelationLagOutOfRange);
        }
        let measurement_sweeps = self
            .measurements
            .checked_mul(self.sweeps_between_measurements)
            .ok_or(Su3EnsembleError::ScheduleOutsideReferenceRange)?;
        let total_sweeps = self
            .thermalization_sweeps
            .checked_add(measurement_sweeps)
            .ok_or(Su3EnsembleError::ScheduleOutsideReferenceRange)?;
        if total_sweeps > MAX_REFERENCE_SWEEPS {
            return Err(Su3EnsembleError::ScheduleOutsideReferenceRange);
        }
        let sites = self
            .extents
            .iter()
            .try_fold(1usize, |acc, extent| acc.checked_mul(*extent))
            .ok_or(Su3EnsembleError::LatticeOutsideReferenceRange)?;
        let links = sites
            .checked_mul(DIM)
            .ok_or(Su3EnsembleError::LatticeOutsideReferenceRange)?;
        if self.extents.iter().any(|extent| *extent < 2) || links > MAX_SU3_REFERENCE_LINKS {
            return Err(Su3EnsembleError::LatticeOutsideReferenceRange);
        }
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Su3ChainReport {
    pub sampler_id: &'static str,
    pub proposal_id: &'static str,
    pub rng_id: &'static str,
    pub stream_id: &'static str,
    pub config: Su3EnsembleConfig,
    pub derived_seed: u64,
    pub total_sweeps: usize,
    pub metropolis_proposals: u64,
    pub metropolis_accepts: u64,
    pub mean_metropolis_acceptance: f64,
    pub measurements: Vec<f64>,
    pub diagnostics: ChainDiagnostics,
    pub final_link_unitarity_residual: f64,
    pub final_link_determinant_residual: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MetropolisAccounting {
    pub proposals: u64,
    pub accepts: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Su3EnsembleError {
    BetaOutsideReferenceRange,
    ProposalAngleOutsideReferenceRange,
    MeasurementCountOutsideReferenceRange,
    ZeroMeasurementStride,
    AutocorrelationLagOutOfRange,
    ScheduleOutsideReferenceRange,
    LatticeOutsideReferenceRange,
    NonFiniteStatistic,
    AccountingOverflow,
    Group(Su3Error),
    Lattice(Su3LatticeError),
    Diagnostic,
}

impl fmt::Display for Su3EnsembleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::BetaOutsideReferenceRange => "SU(3) beta is outside the bounded reference range",
            Self::ProposalAngleOutsideReferenceRange => {
                "SU(3) Metropolis proposal angle is outside the bounded reference range"
            }
            Self::MeasurementCountOutsideReferenceRange => {
                "SU(3) measurement count is outside the bounded reference range"
            }
            Self::ZeroMeasurementStride => "SU(3) measurement stride must be positive",
            Self::AutocorrelationLagOutOfRange => {
                "SU(3) autocorrelation lag must be smaller than the measurement count"
            }
            Self::ScheduleOutsideReferenceRange => {
                "SU(3) schedule exceeds the bounded reference range"
            }
            Self::LatticeOutsideReferenceRange => {
                "SU(3) lattice exceeds the bounded reference range"
            }
            Self::NonFiniteStatistic => "SU(3) ensemble calculation produced a non-finite value",
            Self::AccountingOverflow => "SU(3) Metropolis accounting overflowed",
            Self::Group(_) => "SU(3) group operation failed during ensemble generation",
            Self::Lattice(_) => "SU(3) lattice operation failed during ensemble generation",
            Self::Diagnostic => "SU(3) ensemble diagnostic failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for Su3EnsembleError {}

impl From<Su3Error> for Su3EnsembleError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

impl From<Su3LatticeError> for Su3EnsembleError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

const fn mix64(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

pub fn derive_su3_chain_seed(master_seed: u64, chain_id: u64) -> u64 {
    let lane = chain_id.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    mix64(master_seed ^ STREAM_DOMAIN ^ lane)
}

/// Map one raw RNG output to an exactly paired finite proposal-angle grid.
///
/// The high bit selects the sign. The low 52 bits select an odd multiple of
/// 2^-53 in (0,1), so toggling only the sign bit produces the exact floating
/// inverse angle while neither endpoint +/-proposal_angle is present.
pub fn proposal_angle_from_u64(raw: u64, proposal_angle: f64) -> Result<f64, Su3EnsembleError> {
    validate_proposal_angle(proposal_angle)?;
    let magnitude_index = raw & ANGLE_GRID_MAGNITUDE_MASK;
    let odd_numerator = magnitude_index
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(Su3EnsembleError::AccountingOverflow)?;
    let magnitude = proposal_angle * (odd_numerator as f64) * ANGLE_GRID_SCALE;
    let sign = if raw >> 63 == 0 { -1.0 } else { 1.0 };
    let angle = sign * magnitude;
    if !angle.is_finite() || angle == 0.0 || angle.abs() >= proposal_angle {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }
    Ok(angle)
}

fn x_minus(mut x: [usize; DIM], direction: usize, extents: [usize; DIM]) -> [usize; DIM] {
    x[direction] = if x[direction] == 0 {
        extents[direction] - 1
    } else {
        x[direction] - 1
    };
    x
}

fn normalized_real_trace(value: Su3) -> f64 {
    value.matrix().trace().re / 3.0
}

pub fn average_plaquette(lattice: &Su3Lattice4) -> Result<f64, Su3EnsembleError> {
    let extents = lattice.extents();
    let mut sum = 0.0_f64;
    let mut count = 0usize;
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        for nu in (mu + 1)..DIM {
                            let value = normalized_real_trace(lattice.plaquette(x, mu, nu)?);
                            if !value.is_finite() {
                                return Err(Su3EnsembleError::NonFiniteStatistic);
                            }
                            sum += value;
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    if count == 0 {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }
    let mean = sum / count as f64;
    if !mean.is_finite() {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }
    Ok(mean)
}

fn local_link_action_with_value(
    lattice: &Su3Lattice4,
    x: [usize; DIM],
    mu: usize,
    link_value: Su3,
    beta: f64,
) -> Result<f64, Su3EnsembleError> {
    if !beta.is_finite() || beta < 0.0 || beta > MAX_REFERENCE_BETA {
        return Err(Su3EnsembleError::BetaOutsideReferenceRange);
    }

    lattice.link(x, mu)?;
    let extents = lattice.extents();
    let mut sum = 0.0_f64;
    for nu in 0..DIM {
        if nu == mu {
            continue;
        }

        let x_mu = lattice.neighbor_plus(x, mu)?;
        let x_nu = lattice.neighbor_plus(x, nu)?;
        let forward = link_value
            .mul(lattice.link(x_mu, nu)?)?
            .mul(lattice.link(x_nu, mu)?.dagger())?
            .mul(lattice.link(x, nu)?.dagger())?;

        let backward_x = x_minus(x, nu, extents);
        let backward_x_mu = lattice.neighbor_plus(backward_x, mu)?;
        let backward = lattice
            .link(backward_x, mu)?
            .mul(lattice.link(backward_x_mu, nu)?)?
            .mul(link_value.dagger())?
            .mul(lattice.link(backward_x, nu)?.dagger())?;

        sum += 1.0 - normalized_real_trace(forward);
        sum += 1.0 - normalized_real_trace(backward);
        if !sum.is_finite() {
            return Err(Su3EnsembleError::NonFiniteStatistic);
        }
    }

    let action = beta * sum;
    if !action.is_finite() {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }
    Ok(action)
}

pub fn local_link_action(
    lattice: &Su3Lattice4,
    x: [usize; DIM],
    mu: usize,
    beta: f64,
) -> Result<f64, Su3EnsembleError> {
    let old = lattice.link(x, mu)?;
    local_link_action_with_value(lattice, x, mu, old, beta)
}

pub fn local_action_difference(
    lattice: &Su3Lattice4,
    x: [usize; DIM],
    mu: usize,
    candidate: Su3,
    beta: f64,
) -> Result<f64, Su3EnsembleError> {
    let old = lattice.link(x, mu)?;
    let old_action = local_link_action_with_value(lattice, x, mu, old, beta)?;
    let new_action = local_link_action_with_value(lattice, x, mu, candidate, beta)?;
    let difference = new_action - old_action;
    if !difference.is_finite() {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }
    Ok(difference)
}

fn generator_proposal(generator: usize, angle: f64) -> Result<Su3, Su3EnsembleError> {
    let mut coefficients = [0.0; SU3_GENERATORS];
    coefficients[generator] = angle;
    Ok(Su3::from_algebra_coefficients(coefficients)?)
}

pub fn metropolis_sweep(
    lattice: &mut Su3Lattice4,
    beta: f64,
    proposal_angle: f64,
    rng: &mut SplitMix64,
) -> Result<MetropolisAccounting, Su3EnsembleError> {
    if !beta.is_finite() || beta < 0.0 || beta > MAX_REFERENCE_BETA {
        return Err(Su3EnsembleError::BetaOutsideReferenceRange);
    }
    validate_proposal_angle(proposal_angle)?;

    let extents = lattice.extents();
    let mut accounting = MetropolisAccounting::default();
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        for generator in 0..SU3_GENERATORS {
                            let angle_raw = rng.next_u64();
                            let angle = proposal_angle_from_u64(angle_raw, proposal_angle)?;
                            let rotation = generator_proposal(generator, angle)?;
                            let old = lattice.link(x, mu)?;
                            let candidate = rotation.mul(old)?;
                            let delta = local_action_difference(lattice, x, mu, candidate, beta)?;
                            let acceptance = (-delta.max(0.0)).exp();
                            if !acceptance.is_finite() || !(0.0..=1.0).contains(&acceptance) {
                                return Err(Su3EnsembleError::NonFiniteStatistic);
                            }
                            let accept_draw = rng.unit_f64();
                            accounting.proposals = accounting
                                .proposals
                                .checked_add(1)
                                .ok_or(Su3EnsembleError::AccountingOverflow)?;
                            if accept_draw < acceptance {
                                lattice.set_link(x, mu, candidate)?;
                                accounting.accepts = accounting
                                    .accepts
                                    .checked_add(1)
                                    .ok_or(Su3EnsembleError::AccountingOverflow)?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(accounting)
}

fn accumulate_accounting(
    total: &mut MetropolisAccounting,
    step: MetropolisAccounting,
) -> Result<(), Su3EnsembleError> {
    total.proposals = total
        .proposals
        .checked_add(step.proposals)
        .ok_or(Su3EnsembleError::AccountingOverflow)?;
    total.accepts = total
        .accepts
        .checked_add(step.accepts)
        .ok_or(Su3EnsembleError::AccountingOverflow)?;
    Ok(())
}

pub fn run_chain(config: Su3EnsembleConfig) -> Result<Su3ChainReport, Su3EnsembleError> {
    let config = config.validate()?;
    let derived_seed = derive_su3_chain_seed(config.master_seed, config.chain_id);
    let mut rng = SplitMix64::new(derived_seed);
    let mut lattice = Su3Lattice4::new_identity(config.extents)?;
    let mut accounting = MetropolisAccounting::default();

    for _ in 0..config.thermalization_sweeps {
        let step = metropolis_sweep(&mut lattice, config.beta, config.proposal_angle, &mut rng)?;
        accumulate_accounting(&mut accounting, step)?;
    }

    let mut measurements = Vec::new();
    measurements
        .try_reserve_exact(config.measurements)
        .map_err(|_| Su3EnsembleError::LatticeOutsideReferenceRange)?;
    for _ in 0..config.measurements {
        for _ in 0..config.sweeps_between_measurements {
            let step =
                metropolis_sweep(&mut lattice, config.beta, config.proposal_angle, &mut rng)?;
            accumulate_accounting(&mut accounting, step)?;
        }
        let measurement = average_plaquette(&lattice)?;
        if measurement < -0.5 - PLAQUETTE_TOLERANCE || measurement > 1.0 + PLAQUETTE_TOLERANCE {
            return Err(Su3EnsembleError::NonFiniteStatistic);
        }
        measurements.push(measurement);
    }

    let chain_diagnostics = diagnostics(&measurements, config.autocorrelation_max_lag)
        .map_err(|_| Su3EnsembleError::Diagnostic)?;
    let total_sweeps = config
        .thermalization_sweeps
        .checked_add(
            config
                .measurements
                .checked_mul(config.sweeps_between_measurements)
                .ok_or(Su3EnsembleError::ScheduleOutsideReferenceRange)?,
        )
        .ok_or(Su3EnsembleError::ScheduleOutsideReferenceRange)?;
    let mean_metropolis_acceptance = if accounting.proposals == 0 {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    } else {
        accounting.accepts as f64 / accounting.proposals as f64
    };
    let final_link_unitarity_residual = lattice.max_link_unitarity_residual();
    let final_link_determinant_residual = lattice.max_link_determinant_residual();
    if !mean_metropolis_acceptance.is_finite()
        || !final_link_unitarity_residual.is_finite()
        || !final_link_determinant_residual.is_finite()
        || final_link_unitarity_residual > LINK_RESIDUAL_LIMIT
        || final_link_determinant_residual > LINK_RESIDUAL_LIMIT
    {
        return Err(Su3EnsembleError::NonFiniteStatistic);
    }

    Ok(Su3ChainReport {
        sampler_id: SAMPLER_ID,
        proposal_id: PROPOSAL_ID,
        rng_id: RNG_ID,
        stream_id: STREAM_ID,
        config,
        derived_seed,
        total_sweeps,
        metropolis_proposals: accounting.proposals,
        metropolis_accepts: accounting.accepts,
        mean_metropolis_acceptance,
        measurements,
        diagnostics: chain_diagnostics,
        final_link_unitarity_residual,
        final_link_determinant_residual,
    })
}
