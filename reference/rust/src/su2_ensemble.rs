// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2A bounded SU(2) statistical-ensemble reference surface.
//!
//! This module introduces the first stochastic authority in YANG-MILLS.  It
//! deliberately keeps pseudorandom-stream identity, Markov-chain mechanics,
//! statistical diagnostics, and physical interpretation separate.

use core::f64::consts::TAU;
use core::fmt;

use crate::{Lattice4, ReferenceError, Su2, DIM};

pub const RNG_ID: &str = "SPLITMIX64-V1";
pub const SAMPLER_ID: &str = "YM-SU2-ENSEMBLE-V1";
pub const MAX_REFERENCE_BETA: f64 = 8.0;
pub const MAX_REFERENCE_SWEEPS: usize = 1 << 20;
pub const MAX_REFERENCE_MEASUREMENTS: usize = 1 << 20;
pub const MAX_OVERRELAXATION_SWEEPS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix64(self.state)
    }

    /// Uniform binary64 value in [0,1), using the high 53 random bits.
    pub fn unit_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / 9_007_199_254_740_992.0;
        ((self.next_u64() >> 11) as f64) * SCALE
    }
}

const fn mix64(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

pub fn derive_chain_seed(master_seed: u64, chain_id: u64) -> u64 {
    let lane = chain_id
        .wrapping_add(1)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    mix64(master_seed ^ lane)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnsembleConfig {
    pub extents: [usize; DIM],
    pub beta: f64,
    pub master_seed: u64,
    pub chain_id: u64,
    pub thermalization_sweeps: usize,
    pub measurements: usize,
    pub sweeps_between_measurements: usize,
    pub overrelaxation_sweeps_per_heatbath: usize,
    pub max_heatbath_attempts_per_link: usize,
    pub autocorrelation_max_lag: usize,
}

impl EnsembleConfig {
    pub fn validate(self) -> Result<Self, EnsembleError> {
        if !self.beta.is_finite() || self.beta < 0.0 || self.beta > MAX_REFERENCE_BETA {
            return Err(EnsembleError::BetaOutsideReferenceRange);
        }
        if self.thermalization_sweeps > MAX_REFERENCE_SWEEPS
            || self.measurements > MAX_REFERENCE_MEASUREMENTS
            || self
                .measurements
                .checked_mul(self.sweeps_between_measurements)
                .is_none_or(|sweeps| sweeps > MAX_REFERENCE_SWEEPS)
        {
            return Err(EnsembleError::ScheduleOutsideReferenceRange);
        }
        if self.measurements < 2 {
            return Err(EnsembleError::TooFewMeasurements);
        }
        if self.sweeps_between_measurements == 0 {
            return Err(EnsembleError::ZeroMeasurementStride);
        }
        if self.overrelaxation_sweeps_per_heatbath > MAX_OVERRELAXATION_SWEEPS {
            return Err(EnsembleError::TooManyOverrelaxationSweeps);
        }
        if self.max_heatbath_attempts_per_link == 0 {
            return Err(EnsembleError::ZeroHeatbathAttemptLimit);
        }
        if self.autocorrelation_max_lag >= self.measurements {
            return Err(EnsembleError::AutocorrelationLagOutOfRange);
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnsembleError {
    BetaOutsideReferenceRange,
    ScheduleOutsideReferenceRange,
    TooFewMeasurements,
    ZeroMeasurementStride,
    TooManyOverrelaxationSweeps,
    ZeroHeatbathAttemptLimit,
    AutocorrelationLagOutOfRange,
    HeatbathAttemptLimitExceeded,
    NonFiniteStatistic,
    Reference(ReferenceError),
}

impl fmt::Display for EnsembleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::BetaOutsideReferenceRange => "beta is outside the bounded Phase 2A reference range",
            Self::ScheduleOutsideReferenceRange => "ensemble schedule exceeds the bounded Phase 2A reference range",
            Self::TooFewMeasurements => "at least two measurements are required",
            Self::ZeroMeasurementStride => "measurement stride must be positive",
            Self::TooManyOverrelaxationSweeps => "over-relaxation sweep count exceeds the Phase 2A bound",
            Self::ZeroHeatbathAttemptLimit => "heat-bath rejection attempt limit must be positive",
            Self::AutocorrelationLagOutOfRange => "autocorrelation maximum lag must be smaller than the measurement count",
            Self::HeatbathAttemptLimitExceeded => "SU(2) heat-bath rejection sampler exhausted its bounded attempt limit",
            Self::NonFiniteStatistic => "ensemble diagnostic produced a non-finite statistic",
            Self::Reference(_) => "frozen SU(2) lattice reference operation failed during ensemble generation",
        };
        f.write_str(message)
    }
}

impl std::error::Error for EnsembleError {}

impl From<ReferenceError> for EnsembleError {
    fn from(value: ReferenceError) -> Self {
        Self::Reference(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChainDiagnostics {
    pub mean: f64,
    pub variance: f64,
    pub autocorrelation: Vec<f64>,
    pub positive_window_lag: usize,
    pub integrated_autocorrelation_time: f64,
    pub effective_sample_size: f64,
    pub standard_error: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChainReport {
    pub sampler_id: &'static str,
    pub rng_id: &'static str,
    pub config: EnsembleConfig,
    pub derived_seed: u64,
    pub total_heatbath_sweeps: usize,
    pub total_overrelaxation_sweeps: usize,
    pub heatbath_proposals: u64,
    pub heatbath_updates: u64,
    pub mean_heatbath_acceptance: f64,
    pub measurements: Vec<f64>,
    pub diagnostics: ChainDiagnostics,
    pub final_link_unit_residual: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SweepAccounting {
    proposals: u64,
    updates: u64,
}

fn staple_norm(components: [f64; 4]) -> Result<f64, EnsembleError> {
    let norm_sq = components.iter().map(|value| value * value).sum::<f64>();
    let norm = norm_sq.sqrt();
    if norm.is_finite() {
        Ok(norm)
    } else {
        Err(EnsembleError::NonFiniteStatistic)
    }
}

fn uniform_sphere_direction(rng: &mut SplitMix64) -> [f64; 3] {
    let z = 2.0 * rng.unit_f64() - 1.0;
    let phi = TAU * rng.unit_f64();
    let planar = (1.0 - z * z).max(0.0).sqrt();
    [planar * phi.cos(), planar * phi.sin(), z]
}

/// Draw R in SU(2) from the local conditional density
///
/// dP(R) proportional to exp(alpha * r0) d_mu_Haar(R), alpha >= 0.
///
/// The scalar component proposal is the exact truncated exponential on
/// [-1,1].  The Haar Jacobian factor sqrt(1-r0^2) is then imposed by rejection,
/// and the vector direction is uniform on S^2.
fn sample_local_heatbath(
    alpha: f64,
    rng: &mut SplitMix64,
    max_attempts: usize,
) -> Result<(Su2, u64), EnsembleError> {
    if !alpha.is_finite() || alpha < 0.0 {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    for attempt in 1..=max_attempts {
        let proposal_uniform = rng.unit_f64();
        let r0 = if alpha == 0.0 {
            2.0 * proposal_uniform - 1.0
        } else {
            let truncated_mass = -(-2.0 * alpha).exp_m1();
            let y = -(-proposal_uniform * truncated_mass).ln_1p() / alpha;
            (1.0 - y).clamp(-1.0, 1.0)
        };

        let radial_sq = (1.0 - r0 * r0).max(0.0);
        let radial = radial_sq.sqrt();
        if rng.unit_f64() <= radial {
            let direction = uniform_sphere_direction(rng);
            let sample = Su2::from_normalized_components([
                r0,
                radial * direction[0],
                radial * direction[1],
                radial * direction[2],
            ])?;
            return Ok((sample, attempt as u64));
        }
    }

    Err(EnsembleError::HeatbathAttemptLimitExceeded)
}

fn heatbath_link(
    lattice: &mut Lattice4,
    x: [usize; DIM],
    mu: usize,
    beta: f64,
    rng: &mut SplitMix64,
    max_attempts: usize,
) -> Result<u64, EnsembleError> {
    let staple = lattice.staple(x, mu)?;
    let components = staple.components();
    let norm = staple_norm(components)?;
    let alpha = beta * norm;
    if !alpha.is_finite() {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    let (r, attempts) = sample_local_heatbath(alpha, rng, max_attempts)?;
    let next = if norm == 0.0 {
        r
    } else {
        let normalized_staple = Su2::from_normalized_components(components)?;
        r.mul(normalized_staple.dagger())
    };
    lattice.set_link(x, mu, next)?;
    Ok(attempts)
}

fn overrelax_link(
    lattice: &mut Lattice4,
    x: [usize; DIM],
    mu: usize,
) -> Result<(), EnsembleError> {
    let staple = lattice.staple(x, mu)?;
    let components = staple.components();
    let norm = staple_norm(components)?;
    if norm == 0.0 {
        return Ok(());
    }

    let normalized_staple = Su2::from_normalized_components(components)?;
    let old = lattice.link(x, mu)?;
    let reflected = normalized_staple
        .dagger()
        .mul(old.dagger())
        .mul(normalized_staple.dagger());
    lattice.set_link(x, mu, reflected)?;
    Ok(())
}

fn for_each_link<F>(lattice: &mut Lattice4, mut callback: F) -> Result<(), EnsembleError>
where
    F: FnMut(&mut Lattice4, [usize; DIM], usize) -> Result<(), EnsembleError>,
{
    let extents = lattice.extents();
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        callback(lattice, x, mu)?;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn heatbath_sweep(
    lattice: &mut Lattice4,
    beta: f64,
    rng: &mut SplitMix64,
    max_attempts_per_link: usize,
) -> Result<(u64, u64), EnsembleError> {
    if !beta.is_finite() || beta < 0.0 || beta > MAX_REFERENCE_BETA {
        return Err(EnsembleError::BetaOutsideReferenceRange);
    }
    if max_attempts_per_link == 0 {
        return Err(EnsembleError::ZeroHeatbathAttemptLimit);
    }

    let mut accounting = SweepAccounting::default();
    for_each_link(lattice, |lattice, x, mu| {
        let attempts = heatbath_link(
            lattice,
            x,
            mu,
            beta,
            rng,
            max_attempts_per_link,
        )?;
        accounting.proposals = accounting.proposals.saturating_add(attempts);
        accounting.updates = accounting.updates.saturating_add(1);
        Ok(())
    })?;
    Ok((accounting.proposals, accounting.updates))
}

pub fn overrelaxation_sweep(lattice: &mut Lattice4) -> Result<(), EnsembleError> {
    for_each_link(lattice, overrelax_link)
}

pub fn average_plaquette(lattice: &Lattice4) -> Result<f64, EnsembleError> {
    let extents = lattice.extents();
    let mut sum = 0.0;
    let mut count = 0usize;
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        for nu in (mu + 1)..DIM {
                            sum += lattice.plaquette(x, mu, nu)?.half_real_trace();
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let value = sum / count as f64;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EnsembleError::NonFiniteStatistic)
    }
}

pub fn diagnostics(values: &[f64], max_lag: usize) -> Result<ChainDiagnostics, EnsembleError> {
    if values.len() < 2 {
        return Err(EnsembleError::TooFewMeasurements);
    }
    if max_lag >= values.len() {
        return Err(EnsembleError::AutocorrelationLagOutOfRange);
    }
    if !values.iter().all(|value| value.is_finite()) {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let variance = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / n;

    if !mean.is_finite() || !variance.is_finite() {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    let mut autocorrelation = Vec::new();
    autocorrelation
        .try_reserve_exact(max_lag + 1)
        .map_err(|_| EnsembleError::Reference(ReferenceError::AllocationFailed))?;
    autocorrelation.push(1.0);

    if variance == 0.0 {
        autocorrelation.resize(max_lag + 1, 0.0);
        autocorrelation[0] = 1.0;
        return Ok(ChainDiagnostics {
            mean,
            variance,
            autocorrelation,
            positive_window_lag: 0,
            integrated_autocorrelation_time: 0.5,
            effective_sample_size: n,
            standard_error: 0.0,
        });
    }

    let mut tau = 0.5;
    let mut positive_window_lag = 0usize;
    let mut window_open = true;
    for lag in 1..=max_lag {
        let pairs = values.len() - lag;
        let covariance = values[..pairs]
            .iter()
            .zip(&values[lag..])
            .map(|(left, right)| (left - mean) * (right - mean))
            .sum::<f64>()
            / pairs as f64;
        let rho = covariance / variance;
        if !rho.is_finite() {
            return Err(EnsembleError::NonFiniteStatistic);
        }
        autocorrelation.push(rho);
        if window_open && rho > 0.0 {
            tau += rho;
            positive_window_lag = lag;
        } else {
            window_open = false;
        }
    }

    if !tau.is_finite() || tau < 0.5 {
        return Err(EnsembleError::NonFiniteStatistic);
    }
    let effective_sample_size = (n / (2.0 * tau)).min(n);
    let standard_error = (variance / effective_sample_size).sqrt();
    if !effective_sample_size.is_finite() || !standard_error.is_finite() {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    Ok(ChainDiagnostics {
        mean,
        variance,
        autocorrelation,
        positive_window_lag,
        integrated_autocorrelation_time: tau,
        effective_sample_size,
        standard_error,
    })
}

fn markov_sweep(
    lattice: &mut Lattice4,
    config: EnsembleConfig,
    rng: &mut SplitMix64,
    heatbath_proposals: &mut u64,
    heatbath_updates: &mut u64,
    overrelaxation_sweeps: &mut usize,
) -> Result<(), EnsembleError> {
    let (proposals, updates) = heatbath_sweep(
        lattice,
        config.beta,
        rng,
        config.max_heatbath_attempts_per_link,
    )?;
    *heatbath_proposals = heatbath_proposals.saturating_add(proposals);
    *heatbath_updates = heatbath_updates.saturating_add(updates);
    for _ in 0..config.overrelaxation_sweeps_per_heatbath {
        overrelaxation_sweep(lattice)?;
        *overrelaxation_sweeps += 1;
    }
    Ok(())
}

pub fn run_chain(config: EnsembleConfig) -> Result<ChainReport, EnsembleError> {
    let config = config.validate()?;
    let mut lattice = Lattice4::new_identity(config.extents)?;
    let derived_seed = derive_chain_seed(config.master_seed, config.chain_id);
    let mut rng = SplitMix64::new(derived_seed);
    let mut heatbath_proposals = 0u64;
    let mut heatbath_updates = 0u64;
    let mut overrelaxation_sweeps = 0usize;

    for _ in 0..config.thermalization_sweeps {
        markov_sweep(
            &mut lattice,
            config,
            &mut rng,
            &mut heatbath_proposals,
            &mut heatbath_updates,
            &mut overrelaxation_sweeps,
        )?;
    }

    let mut measurements = Vec::new();
    measurements
        .try_reserve_exact(config.measurements)
        .map_err(|_| EnsembleError::Reference(ReferenceError::AllocationFailed))?;
    for _ in 0..config.measurements {
        for _ in 0..config.sweeps_between_measurements {
            markov_sweep(
                &mut lattice,
                config,
                &mut rng,
                &mut heatbath_proposals,
                &mut heatbath_updates,
                &mut overrelaxation_sweeps,
            )?;
        }
        measurements.push(average_plaquette(&lattice)?);
    }

    let diagnostics = diagnostics(&measurements, config.autocorrelation_max_lag)?;
    let mean_heatbath_acceptance = heatbath_updates as f64 / heatbath_proposals as f64;
    let final_link_unit_residual = lattice.max_link_unit_residual();
    if !mean_heatbath_acceptance.is_finite() || !final_link_unit_residual.is_finite() {
        return Err(EnsembleError::NonFiniteStatistic);
    }

    Ok(ChainReport {
        sampler_id: SAMPLER_ID,
        rng_id: RNG_ID,
        config,
        derived_seed,
        total_heatbath_sweeps: config.thermalization_sweeps
            + config.measurements * config.sweeps_between_measurements,
        total_overrelaxation_sweeps: overrelaxation_sweeps,
        heatbath_proposals,
        heatbath_updates,
        mean_heatbath_acceptance,
        measurements,
        diagnostics,
        final_link_unit_residual,
    })
}
