// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2C2 published pure-SU(3) Wilson-action plaquette benchmark logic.
//!
//! The primary source is R. C. Edgar, L. McCrossen and K. J. M. Moriarty,
//! J. Phys. G 7 (1981) L85, DOI 10.1088/0305-4616/7/5/001. Their four-
//! dimensional pure-SU(3) study reports the average action per plaquette on
//! 4^4 and 5^4 lattices, with 400 iterations through the 4^4 lattice. The
//! source presents the beta scan graphically, so this module freezes a
//! deliberately conservative graphical source-resolution envelope for the
//! equivalent normalized plaquette rather than inventing a quoted error bar.

use core::fmt;

use crate::su2_ensemble::diagnostics;
use crate::su3_ensemble::{
    derive_su3_chain_seed, Su3ChainReport, LINK_RESIDUAL_LIMIT, PROPOSAL_ID, RNG_ID, SAMPLER_ID,
    STREAM_ID,
};

pub const BENCHMARK_ID: &str = "YM-SU3-PLAQUETTE-BASELINE-V1";
pub const PRIMARY_DOI: &str = "10.1088/0305-4616/7/5/001";
pub const BETA: f64 = 6.0;
pub const EXTENTS: [usize; 4] = [4, 4, 4, 4];
pub const SOURCE_ENVELOPE_MIN: f64 = 0.56;
pub const SOURCE_ENVELOPE_MAX: f64 = 0.63;
pub const MASTER_SEED: u64 = 0x2c02_1981_0501_0001;
pub const CHAIN_IDS: [u64; 2] = [0, 1];
pub const FROZEN_CHAIN_DERIVED_SEEDS: [u64; 2] = [0x2ff3_bf48_550d_1457, 0xef76_36a5_6e8e_a29a];
pub const PROPOSAL_ANGLE: f64 = 0.35;
pub const THERMALIZATION_SWEEPS: usize = 192;
pub const MEASUREMENTS: usize = 128;
pub const SWEEPS_BETWEEN_MEASUREMENTS: usize = 2;
pub const AUTOCORRELATION_MAX_LAG: usize = 24;
pub const MIN_CHAIN_ESS: f64 = 8.0;
pub const MAX_CHAIN_CONSISTENCY_Z: f64 = 5.0;
pub const STATIONARITY_HALF_MEASUREMENTS: usize = MEASUREMENTS / 2;
pub const STATIONARITY_AUTOCORRELATION_MAX_LAG: usize = 12;
pub const MAX_STATIONARITY_Z: f64 = 5.0;
pub const MIN_METROPOLIS_ACCEPTANCE: f64 = 0.10;
pub const MAX_METROPOLIS_ACCEPTANCE: f64 = 0.95;
pub const MAX_LINK_UNITARITY_RESIDUAL: f64 = LINK_RESIDUAL_LIMIT;
pub const MAX_LINK_DETERMINANT_RESIDUAL: f64 = LINK_RESIDUAL_LIMIT;
pub const PLAQUETTE_SAMPLE_TOLERANCE: f64 = 1.0e-12;
pub const LINKS_PER_SWEEP: u64 = (EXTENTS[0] * EXTENTS[1] * EXTENTS[2] * EXTENTS[3] * 4) as u64;
pub const PROPOSALS_PER_LINK_PER_SWEEP: u64 = 8;
pub const EXPECTED_TOTAL_SWEEPS: usize =
    THERMALIZATION_SWEEPS + MEASUREMENTS * SWEEPS_BETWEEN_MEASUREMENTS;
pub const EXPECTED_PROPOSALS: u64 =
    LINKS_PER_SWEEP * PROPOSALS_PER_LINK_PER_SWEEP * EXPECTED_TOTAL_SWEEPS as u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchmarkError {
    WrongBenchmarkConfiguration,
    DuplicateChainIdentity,
    DuplicateMeasurementSeries,
    InvalidPlaquetteSample,
    InvalidMetropolisAccounting,
    InvalidAcceptanceEvidence,
    InvalidLinkResidual,
    StationarityDiagnosticFailed,
    NonFiniteStatistic,
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WrongBenchmarkConfiguration => {
                "chain report does not match the frozen Phase 2C2 benchmark configuration"
            }
            Self::DuplicateChainIdentity => "benchmark chains must have distinct stream identities",
            Self::DuplicateMeasurementSeries => {
                "benchmark chains must retain distinct raw measurement series"
            }
            Self::InvalidPlaquetteSample => {
                "retained plaquette evidence lies outside the physical SU(3) trace range"
            }
            Self::InvalidMetropolisAccounting => {
                "Metropolis proposal accounting does not match the frozen Phase 2C2 schedule"
            }
            Self::InvalidAcceptanceEvidence => {
                "Metropolis acceptance evidence is inconsistent or outside the frozen diagnostic window"
            }
            Self::InvalidLinkResidual => {
                "final SU(3) link residual evidence is invalid or exceeds the frozen gate"
            }
            Self::StationarityDiagnosticFailed => {
                "post-thermalisation stationarity diagnostic could not be evaluated"
            }
            Self::NonFiniteStatistic => "benchmark aggregation produced a non-finite statistic",
        };
        f.write_str(message)
    }
}

impl std::error::Error for BenchmarkError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StationarityDiagnostic {
    pub first_half_mean: f64,
    pub second_half_mean: f64,
    pub first_half_standard_error: f64,
    pub second_half_standard_error: f64,
    pub drift_z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BenchmarkSummary {
    pub chain_means: [f64; 2],
    pub chain_standard_errors: [f64; 2],
    pub chain_effective_sample_sizes: [f64; 2],
    pub chain_acceptances: [f64; 2],
    pub stationarity: [StationarityDiagnostic; 2],
    pub max_stationarity_z: f64,
    pub pooled_mean: f64,
    pub pooled_standard_error: f64,
    pub chain_consistency_z: f64,
    pub min_effective_sample_size: f64,
    pub max_link_unitarity_residual: f64,
    pub max_link_determinant_residual: f64,
    pub source_envelope_reproduced: bool,
    pub diagnostics_trustworthy: bool,
    pub published_su3_baseline_reproduced: bool,
}

fn frozen_derived_seed(chain_id: u64) -> Option<u64> {
    match chain_id {
        0 => Some(FROZEN_CHAIN_DERIVED_SEEDS[0]),
        1 => Some(FROZEN_CHAIN_DERIVED_SEEDS[1]),
        _ => None,
    }
}

fn report_matches_benchmark(report: &Su3ChainReport, chain_id: u64) -> bool {
    let Some(expected_derived_seed) = frozen_derived_seed(chain_id) else {
        return false;
    };
    let config = report.config;
    report.sampler_id == SAMPLER_ID
        && report.proposal_id == PROPOSAL_ID
        && report.rng_id == RNG_ID
        && report.stream_id == STREAM_ID
        && report.derived_seed == expected_derived_seed
        && config.extents == EXTENTS
        && config.beta == BETA
        && config.master_seed == MASTER_SEED
        && config.chain_id == chain_id
        && config.thermalization_sweeps == THERMALIZATION_SWEEPS
        && config.measurements == MEASUREMENTS
        && report.measurements.len() == MEASUREMENTS
        && config.sweeps_between_measurements == SWEEPS_BETWEEN_MEASUREMENTS
        && config.proposal_angle == PROPOSAL_ANGLE
        && config.autocorrelation_max_lag == AUTOCORRELATION_MAX_LAG
        && report.total_sweeps == EXPECTED_TOTAL_SWEEPS
}

fn measurements_are_physical(values: &[f64]) -> bool {
    let lower = -0.5 - PLAQUETTE_SAMPLE_TOLERANCE;
    let upper = 1.0 + PLAQUETTE_SAMPLE_TOLERANCE;
    values
        .iter()
        .all(|value| value.is_finite() && (lower..=upper).contains(value))
}

fn acceptance_from_counts(report: &Su3ChainReport) -> Result<f64, BenchmarkError> {
    if report.metropolis_proposals != EXPECTED_PROPOSALS
        || report.metropolis_accepts == 0
        || report.metropolis_accepts > report.metropolis_proposals
    {
        return Err(BenchmarkError::InvalidMetropolisAccounting);
    }
    let acceptance = report.metropolis_accepts as f64 / report.metropolis_proposals as f64;
    if !acceptance.is_finite()
        || report.mean_metropolis_acceptance != acceptance
        || !(MIN_METROPOLIS_ACCEPTANCE..=MAX_METROPOLIS_ACCEPTANCE).contains(&acceptance)
    {
        return Err(BenchmarkError::InvalidAcceptanceEvidence);
    }
    Ok(acceptance)
}

fn is_constant_series(values: &[f64]) -> bool {
    values
        .first()
        .is_some_and(|first| values[1..].iter().all(|value| value == first))
}

pub fn stationarity_diagnostic(values: &[f64]) -> Result<StationarityDiagnostic, BenchmarkError> {
    if values.len() != MEASUREMENTS
        || STATIONARITY_HALF_MEASUREMENTS * 2 != MEASUREMENTS
        || STATIONARITY_AUTOCORRELATION_MAX_LAG >= STATIONARITY_HALF_MEASUREMENTS
    {
        return Err(BenchmarkError::WrongBenchmarkConfiguration);
    }

    let split = STATIONARITY_HALF_MEASUREMENTS;
    let first_values = &values[..split];
    let second_values = &values[split..];
    if is_constant_series(first_values) || is_constant_series(second_values) {
        return Err(BenchmarkError::StationarityDiagnosticFailed);
    }

    let first = diagnostics(first_values, STATIONARITY_AUTOCORRELATION_MAX_LAG)
        .map_err(|_| BenchmarkError::StationarityDiagnosticFailed)?;
    let second = diagnostics(second_values, STATIONARITY_AUTOCORRELATION_MAX_LAG)
        .map_err(|_| BenchmarkError::StationarityDiagnosticFailed)?;
    if first.variance == 0.0 || second.variance == 0.0 {
        return Err(BenchmarkError::StationarityDiagnosticFailed);
    }

    let difference = (first.mean - second.mean).abs();
    let scale = (first.standard_error.powi(2) + second.standard_error.powi(2)).sqrt();
    if scale == 0.0 {
        return Err(BenchmarkError::StationarityDiagnosticFailed);
    }
    let drift_z = difference / scale;
    if !first.mean.is_finite()
        || !second.mean.is_finite()
        || !first.standard_error.is_finite()
        || !second.standard_error.is_finite()
        || !drift_z.is_finite()
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

    Ok(StationarityDiagnostic {
        first_half_mean: first.mean,
        second_half_mean: second.mean,
        first_half_standard_error: first.standard_error,
        second_half_standard_error: second.standard_error,
        drift_z,
    })
}

pub fn summarize_two_chains(
    first: &Su3ChainReport,
    second: &Su3ChainReport,
) -> Result<BenchmarkSummary, BenchmarkError> {
    if first.derived_seed == second.derived_seed {
        return Err(BenchmarkError::DuplicateChainIdentity);
    }
    if !report_matches_benchmark(first, CHAIN_IDS[0])
        || !report_matches_benchmark(second, CHAIN_IDS[1])
    {
        return Err(BenchmarkError::WrongBenchmarkConfiguration);
    }
    if first.measurements == second.measurements {
        return Err(BenchmarkError::DuplicateMeasurementSeries);
    }
    if !measurements_are_physical(&first.measurements)
        || !measurements_are_physical(&second.measurements)
    {
        return Err(BenchmarkError::InvalidPlaquetteSample);
    }

    let chain_acceptances = [
        acceptance_from_counts(first)?,
        acceptance_from_counts(second)?,
    ];
    if !first.final_link_unitarity_residual.is_finite()
        || !second.final_link_unitarity_residual.is_finite()
        || !first.final_link_determinant_residual.is_finite()
        || !second.final_link_determinant_residual.is_finite()
        || first.final_link_unitarity_residual < 0.0
        || second.final_link_unitarity_residual < 0.0
        || first.final_link_determinant_residual < 0.0
        || second.final_link_determinant_residual < 0.0
    {
        return Err(BenchmarkError::InvalidLinkResidual);
    }

    let first_diagnostics = diagnostics(&first.measurements, AUTOCORRELATION_MAX_LAG)
        .map_err(|_| BenchmarkError::NonFiniteStatistic)?;
    let second_diagnostics = diagnostics(&second.measurements, AUTOCORRELATION_MAX_LAG)
        .map_err(|_| BenchmarkError::NonFiniteStatistic)?;
    let chain_means = [first_diagnostics.mean, second_diagnostics.mean];
    let chain_standard_errors = [
        first_diagnostics.standard_error,
        second_diagnostics.standard_error,
    ];
    let chain_effective_sample_sizes = [
        first_diagnostics.effective_sample_size,
        second_diagnostics.effective_sample_size,
    ];
    let stationarity = [
        stationarity_diagnostic(&first.measurements)?,
        stationarity_diagnostic(&second.measurements)?,
    ];
    let max_stationarity_z = stationarity[0].drift_z.max(stationarity[1].drift_z);

    let pooled_mean = 0.5 * (chain_means[0] + chain_means[1]);
    let pooled_standard_error =
        0.5 * (chain_standard_errors[0].powi(2) + chain_standard_errors[1].powi(2)).sqrt();
    let chain_difference = (chain_means[0] - chain_means[1]).abs();
    let consistency_scale =
        (chain_standard_errors[0].powi(2) + chain_standard_errors[1].powi(2)).sqrt();
    let chain_consistency_z = if consistency_scale == 0.0 {
        if chain_difference == 0.0 {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        chain_difference / consistency_scale
    };
    let min_effective_sample_size =
        chain_effective_sample_sizes[0].min(chain_effective_sample_sizes[1]);
    let max_link_unitarity_residual = first
        .final_link_unitarity_residual
        .max(second.final_link_unitarity_residual);
    let max_link_determinant_residual = first
        .final_link_determinant_residual
        .max(second.final_link_determinant_residual);

    if !chain_means.iter().all(|value| value.is_finite())
        || !chain_standard_errors.iter().all(|value| value.is_finite())
        || !chain_effective_sample_sizes
            .iter()
            .all(|value| value.is_finite())
        || !chain_acceptances.iter().all(|value| value.is_finite())
        || !max_stationarity_z.is_finite()
        || !pooled_mean.is_finite()
        || !pooled_standard_error.is_finite()
        || !chain_consistency_z.is_finite()
        || !min_effective_sample_size.is_finite()
        || !max_link_unitarity_residual.is_finite()
        || !max_link_determinant_residual.is_finite()
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

    let source_envelope_reproduced =
        (SOURCE_ENVELOPE_MIN..=SOURCE_ENVELOPE_MAX).contains(&pooled_mean);
    let diagnostics_trustworthy = min_effective_sample_size >= MIN_CHAIN_ESS
        && chain_consistency_z <= MAX_CHAIN_CONSISTENCY_Z
        && max_stationarity_z <= MAX_STATIONARITY_Z
        && max_link_unitarity_residual <= MAX_LINK_UNITARITY_RESIDUAL
        && max_link_determinant_residual <= MAX_LINK_DETERMINANT_RESIDUAL;
    let published_su3_baseline_reproduced = source_envelope_reproduced && diagnostics_trustworthy;

    Ok(BenchmarkSummary {
        chain_means,
        chain_standard_errors,
        chain_effective_sample_sizes,
        chain_acceptances,
        stationarity,
        max_stationarity_z,
        pooled_mean,
        pooled_standard_error,
        chain_consistency_z,
        min_effective_sample_size,
        max_link_unitarity_residual,
        max_link_determinant_residual,
        source_envelope_reproduced,
        diagnostics_trustworthy,
        published_su3_baseline_reproduced,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::su3_ensemble::{Su3ChainReport, Su3EnsembleConfig};

    fn config(chain_id: u64) -> Su3EnsembleConfig {
        Su3EnsembleConfig {
            extents: EXTENTS,
            beta: BETA,
            master_seed: MASTER_SEED,
            chain_id,
            thermalization_sweeps: THERMALIZATION_SWEEPS,
            measurements: MEASUREMENTS,
            sweeps_between_measurements: SWEEPS_BETWEEN_MEASUREMENTS,
            proposal_angle: PROPOSAL_ANGLE,
            autocorrelation_max_lag: AUTOCORRELATION_MAX_LAG,
        }
    }

    fn stationary_measurements(phase: usize) -> Vec<f64> {
        (0..MEASUREMENTS)
            .map(|index| match (index + phase) % 4 {
                0 => 0.590,
                1 => 0.594,
                2 => 0.596,
                _ => 0.592,
            })
            .collect()
    }

    fn synthetic_report(chain_id: u64, measurements: Vec<f64>) -> Su3ChainReport {
        let diagnostics = diagnostics(&measurements, AUTOCORRELATION_MAX_LAG)
            .expect("synthetic benchmark diagnostics must be valid");
        let acceptance = 0.5;
        let accepts = (EXPECTED_PROPOSALS as f64 * acceptance) as u64;
        Su3ChainReport {
            sampler_id: SAMPLER_ID,
            proposal_id: PROPOSAL_ID,
            rng_id: RNG_ID,
            stream_id: STREAM_ID,
            config: config(chain_id),
            derived_seed: frozen_derived_seed(chain_id).expect("frozen chain ID"),
            total_sweeps: EXPECTED_TOTAL_SWEEPS,
            metropolis_proposals: EXPECTED_PROPOSALS,
            metropolis_accepts: accepts,
            mean_metropolis_acceptance: accepts as f64 / EXPECTED_PROPOSALS as f64,
            measurements,
            diagnostics,
            final_link_unitarity_residual: 0.0,
            final_link_determinant_residual: 0.0,
        }
    }

    #[test]
    fn executable_identity_and_source_are_frozen() {
        assert_eq!(BENCHMARK_ID, "YM-SU3-PLAQUETTE-BASELINE-V1");
        assert_eq!(PRIMARY_DOI, "10.1088/0305-4616/7/5/001");
        assert_eq!(EXTENTS, [4, 4, 4, 4]);
        assert_eq!(BETA, 6.0);
        assert_eq!([SOURCE_ENVELOPE_MIN, SOURCE_ENVELOPE_MAX], [0.56, 0.63]);
    }

    #[test]
    fn frozen_stream_seeds_match_phase2c1_derivation() {
        assert_eq!(
            derive_su3_chain_seed(MASTER_SEED, 0),
            FROZEN_CHAIN_DERIVED_SEEDS[0]
        );
        assert_eq!(
            derive_su3_chain_seed(MASTER_SEED, 1),
            FROZEN_CHAIN_DERIVED_SEEDS[1]
        );
        assert_ne!(FROZEN_CHAIN_DERIVED_SEEDS[0], FROZEN_CHAIN_DERIVED_SEEDS[1]);
    }

    #[test]
    fn schedule_stays_inside_frozen_phase2c1_surface() {
        assert_eq!(EXPECTED_TOTAL_SWEEPS, 448);
        assert_eq!(LINKS_PER_SWEEP, 1024);
        assert_eq!(EXPECTED_PROPOSALS, 3_670_016);
        assert!(EXPECTED_TOTAL_SWEEPS <= crate::su3_ensemble::MAX_REFERENCE_SWEEPS);
        assert!(MEASUREMENTS <= crate::su3_ensemble::MAX_REFERENCE_MEASUREMENTS);
    }

    #[test]
    fn stationary_independent_synthetic_chains_pass() {
        let first = synthetic_report(0, stationary_measurements(0));
        let second = synthetic_report(1, stationary_measurements(1));
        let summary = summarize_two_chains(&first, &second).expect("valid synthetic benchmark");
        assert!(summary.source_envelope_reproduced);
        assert!(summary.diagnostics_trustworthy);
        assert!(summary.published_su3_baseline_reproduced);
    }

    #[test]
    fn duplicate_stream_or_series_fails_closed() {
        let first = synthetic_report(0, stationary_measurements(0));
        let mut second = synthetic_report(1, stationary_measurements(1));
        second.derived_seed = first.derived_seed;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::DuplicateChainIdentity)
        );

        let mut second = synthetic_report(1, first.measurements.clone());
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::DuplicateMeasurementSeries)
        );
        second.measurements[0] = f64::NAN;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::InvalidPlaquetteSample)
        );
    }

    #[test]
    fn accounting_acceptance_and_residuals_fail_closed() {
        let first = synthetic_report(0, stationary_measurements(0));
        let second = synthetic_report(1, stationary_measurements(1));

        let mut broken = first.clone();
        broken.metropolis_proposals -= 1;
        assert_eq!(
            summarize_two_chains(&broken, &second),
            Err(BenchmarkError::InvalidMetropolisAccounting)
        );

        let mut broken = first.clone();
        broken.metropolis_accepts = EXPECTED_PROPOSALS;
        broken.mean_metropolis_acceptance = 1.0;
        assert_eq!(
            summarize_two_chains(&broken, &second),
            Err(BenchmarkError::InvalidAcceptanceEvidence)
        );

        let mut broken = first;
        broken.final_link_unitarity_residual = MAX_LINK_UNITARITY_RESIDUAL * 2.0;
        let summary =
            summarize_two_chains(&broken, &second).expect("finite residual is summarized");
        assert!(!summary.diagnostics_trustworthy);
        assert!(!summary.published_su3_baseline_reproduced);
    }
}
