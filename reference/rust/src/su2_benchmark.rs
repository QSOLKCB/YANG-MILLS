// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2B published SU(2) average-plaquette benchmark logic.
//!
//! The primary source is Langfeld, Lucini & Rago, Phys. Rev. Lett. 109,
//! 111601 (2012), DOI 10.1103/PhysRevLett.109.111601. Their Fig. 5 shows
//! the average plaquette for four-dimensional SU(2) Wilson gauge theory on
//! a 10^4 lattice and compares density-of-states and local-hybrid Monte Carlo
//! results. The paper presents this comparison graphically rather than as a
//! table of point estimates, so this module freezes a deliberately conservative
//! source-resolution envelope rather than inventing a paper uncertainty.

use core::fmt;

use crate::su2_ensemble::{diagnostics, ChainReport};

pub const BENCHMARK_ID: &str = "YM-SU2-PLAQUETTE-BASELINE-V1";
pub const PRIMARY_DOI: &str = "10.1103/PhysRevLett.109.111601";
pub const BETA: f64 = 2.3;
pub const EXTENTS: [usize; 4] = [10, 10, 10, 10];
pub const SOURCE_ENVELOPE_MIN: f64 = 0.59;
pub const SOURCE_ENVELOPE_MAX: f64 = 0.62;
pub const MASTER_SEED: u64 = 0x2b00_2012_1116_0101;
pub const CHAIN_IDS: [u64; 2] = [0, 1];
pub const FROZEN_PHASE2A_SAMPLER_ID: &str = "YM-SU2-ENSEMBLE-V1";
pub const FROZEN_PHASE2A_RNG_ID: &str = "SPLITMIX64-V1";
pub const FROZEN_CHAIN_DERIVED_SEEDS: [u64; 2] = [0x044d_f781_f86a_2ade, 0x4ebd_9478_3d50_8d2e];
pub const THERMALIZATION_SWEEPS: usize = 40;
pub const MEASUREMENTS: usize = 48;
pub const SWEEPS_BETWEEN_MEASUREMENTS: usize = 1;
pub const OVERRELAXATION_SWEEPS_PER_HEATBATH: usize = 1;
pub const MAX_HEATBATH_ATTEMPTS_PER_LINK: usize = 10_000;
pub const AUTOCORRELATION_MAX_LAG: usize = 12;
pub const MIN_CHAIN_ESS: f64 = 6.0;
pub const MAX_CHAIN_CONSISTENCY_Z: f64 = 5.0;
pub const STATIONARITY_HALF_MEASUREMENTS: usize = MEASUREMENTS / 2;
pub const STATIONARITY_AUTOCORRELATION_MAX_LAG: usize = 6;
pub const MAX_STATIONARITY_Z: f64 = 5.0;
pub const MAX_LINK_UNIT_RESIDUAL: f64 = 1.0e-12;
pub const PLAQUETTE_SAMPLE_TOLERANCE: f64 = 1.0e-12;
pub const LINKS_PER_HEATBATH_SWEEP: u64 =
    (EXTENTS[0] * EXTENTS[1] * EXTENTS[2] * EXTENTS[3] * 4) as u64;
pub const EXPECTED_HEATBATH_SWEEPS: usize =
    THERMALIZATION_SWEEPS + MEASUREMENTS * SWEEPS_BETWEEN_MEASUREMENTS;
pub const EXPECTED_OVERRELAXATION_SWEEPS: usize =
    EXPECTED_HEATBATH_SWEEPS * OVERRELAXATION_SWEEPS_PER_HEATBATH;
pub const EXPECTED_HEATBATH_UPDATES: u64 =
    LINKS_PER_HEATBATH_SWEEP * EXPECTED_HEATBATH_SWEEPS as u64;
pub const MAX_EXPECTED_HEATBATH_PROPOSALS: u64 =
    EXPECTED_HEATBATH_UPDATES * MAX_HEATBATH_ATTEMPTS_PER_LINK as u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchmarkError {
    WrongBenchmarkConfiguration,
    DuplicateChainIdentity,
    DuplicateMeasurementSeries,
    InvalidPlaquetteSample,
    InvalidHeatbathAccounting,
    InvalidAcceptanceEvidence,
    InvalidLinkUnitResidual,
    StationarityDiagnosticFailed,
    NonFiniteStatistic,
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WrongBenchmarkConfiguration => {
                "chain report does not match the frozen Phase 2B benchmark configuration"
            }
            Self::DuplicateChainIdentity => "benchmark chains must have distinct stream identities",
            Self::DuplicateMeasurementSeries => {
                "benchmark chains must retain distinct raw measurement series"
            }
            Self::InvalidPlaquetteSample => {
                "retained plaquette evidence lies outside the physical SU(2) range"
            }
            Self::InvalidHeatbathAccounting => {
                "heat-bath sweep/update accounting does not match the frozen schedule"
            }
            Self::InvalidAcceptanceEvidence => {
                "heat-bath acceptance evidence is inconsistent with proposal/update counts"
            }
            Self::InvalidLinkUnitResidual => {
                "final link-unit residual must be finite and nonnegative"
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
    pub stationarity: [StationarityDiagnostic; 2],
    pub max_stationarity_z: f64,
    pub pooled_mean: f64,
    pub pooled_standard_error: f64,
    pub chain_consistency_z: f64,
    pub min_effective_sample_size: f64,
    pub max_link_unit_residual: f64,
    pub source_envelope_reproduced: bool,
    pub diagnostics_trustworthy: bool,
    pub phase2_graduation_gate_satisfied: bool,
}

fn frozen_derived_seed(chain_id: u64) -> Option<u64> {
    match chain_id {
        0 => Some(FROZEN_CHAIN_DERIVED_SEEDS[0]),
        1 => Some(FROZEN_CHAIN_DERIVED_SEEDS[1]),
        _ => None,
    }
}

fn report_matches_benchmark(report: &ChainReport, chain_id: u64) -> bool {
    let Some(expected_derived_seed) = frozen_derived_seed(chain_id) else {
        return false;
    };
    let config = report.config;
    report.sampler_id == FROZEN_PHASE2A_SAMPLER_ID
        && report.rng_id == FROZEN_PHASE2A_RNG_ID
        && report.derived_seed == expected_derived_seed
        && config.extents == EXTENTS
        && config.beta == BETA
        && config.master_seed == MASTER_SEED
        && config.chain_id == chain_id
        && config.thermalization_sweeps == THERMALIZATION_SWEEPS
        && config.measurements == MEASUREMENTS
        && report.measurements.len() == MEASUREMENTS
        && config.sweeps_between_measurements == SWEEPS_BETWEEN_MEASUREMENTS
        && config.overrelaxation_sweeps_per_heatbath == OVERRELAXATION_SWEEPS_PER_HEATBATH
        && config.max_heatbath_attempts_per_link == MAX_HEATBATH_ATTEMPTS_PER_LINK
        && config.autocorrelation_max_lag == AUTOCORRELATION_MAX_LAG
}

fn measurements_are_physical(values: &[f64]) -> bool {
    let lower = -1.0 - PLAQUETTE_SAMPLE_TOLERANCE;
    let upper = 1.0 + PLAQUETTE_SAMPLE_TOLERANCE;
    values
        .iter()
        .all(|value| value.is_finite() && (lower..=upper).contains(value))
}

fn validate_heatbath_accounting(report: &ChainReport) -> Result<(), BenchmarkError> {
    if report.total_heatbath_sweeps != EXPECTED_HEATBATH_SWEEPS
        || report.total_overrelaxation_sweeps != EXPECTED_OVERRELAXATION_SWEEPS
        || report.heatbath_updates != EXPECTED_HEATBATH_UPDATES
        || report.heatbath_proposals < EXPECTED_HEATBATH_UPDATES
        || report.heatbath_proposals > MAX_EXPECTED_HEATBATH_PROPOSALS
    {
        return Err(BenchmarkError::InvalidHeatbathAccounting);
    }
    Ok(())
}

fn acceptance_from_counts(report: &ChainReport) -> Result<f64, BenchmarkError> {
    validate_heatbath_accounting(report)?;
    let acceptance = report.heatbath_updates as f64 / report.heatbath_proposals as f64;
    if !acceptance.is_finite()
        || acceptance <= 0.0
        || acceptance > 1.0
        || report.mean_heatbath_acceptance != acceptance
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

    // Reject a stuck observable directly from the raw samples. Checking only
    // the variance is insufficient because summing a repeated non-binary f64
    // can shift the computed mean by roundoff and create a tiny fake variance.
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
    first: &ChainReport,
    second: &ChainReport,
) -> Result<BenchmarkSummary, BenchmarkError> {
    if !report_matches_benchmark(first, CHAIN_IDS[0])
        || !report_matches_benchmark(second, CHAIN_IDS[1])
    {
        return Err(BenchmarkError::WrongBenchmarkConfiguration);
    }
    if first.derived_seed == second.derived_seed {
        return Err(BenchmarkError::DuplicateChainIdentity);
    }
    if first.measurements == second.measurements {
        return Err(BenchmarkError::DuplicateMeasurementSeries);
    }
    if !measurements_are_physical(&first.measurements)
        || !measurements_are_physical(&second.measurements)
    {
        return Err(BenchmarkError::InvalidPlaquetteSample);
    }
    let _first_acceptance = acceptance_from_counts(first)?;
    let _second_acceptance = acceptance_from_counts(second)?;
    if !first.final_link_unit_residual.is_finite()
        || !second.final_link_unit_residual.is_finite()
        || first.final_link_unit_residual < 0.0
        || second.final_link_unit_residual < 0.0
    {
        return Err(BenchmarkError::InvalidLinkUnitResidual);
    }

    // The retained measurement series are the evidence authority. Recompute
    // the frozen Phase 2A diagnostics here rather than trusting cached fields
    // carried by a constructed or stale ChainReport.
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

    if !chain_means.iter().all(|value| value.is_finite())
        || !chain_standard_errors.iter().all(|value| value.is_finite())
        || !chain_effective_sample_sizes
            .iter()
            .all(|value| value.is_finite())
        || !max_stationarity_z.is_finite()
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

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
    let max_link_unit_residual = first
        .final_link_unit_residual
        .max(second.final_link_unit_residual);

    if !pooled_mean.is_finite()
        || !pooled_standard_error.is_finite()
        || !chain_consistency_z.is_finite()
        || !min_effective_sample_size.is_finite()
        || !max_link_unit_residual.is_finite()
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

    let source_envelope_reproduced =
        (SOURCE_ENVELOPE_MIN..=SOURCE_ENVELOPE_MAX).contains(&pooled_mean);
    let diagnostics_trustworthy = min_effective_sample_size >= MIN_CHAIN_ESS
        && chain_consistency_z <= MAX_CHAIN_CONSISTENCY_Z
        && max_stationarity_z <= MAX_STATIONARITY_Z
        && max_link_unit_residual <= MAX_LINK_UNIT_RESIDUAL;

    Ok(BenchmarkSummary {
        chain_means,
        chain_standard_errors,
        chain_effective_sample_sizes,
        stationarity,
        max_stationarity_z,
        pooled_mean,
        pooled_standard_error,
        chain_consistency_z,
        min_effective_sample_size,
        max_link_unit_residual,
        source_envelope_reproduced,
        diagnostics_trustworthy,
        phase2_graduation_gate_satisfied: source_envelope_reproduced && diagnostics_trustworthy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::su2_ensemble::{derive_chain_seed, EnsembleConfig, RNG_ID, SAMPLER_ID};

    fn benchmark_config(chain_id: u64) -> EnsembleConfig {
        EnsembleConfig {
            extents: EXTENTS,
            beta: BETA,
            master_seed: MASTER_SEED,
            chain_id,
            thermalization_sweeps: THERMALIZATION_SWEEPS,
            measurements: MEASUREMENTS,
            sweeps_between_measurements: SWEEPS_BETWEEN_MEASUREMENTS,
            overrelaxation_sweeps_per_heatbath: OVERRELAXATION_SWEEPS_PER_HEATBATH,
            max_heatbath_attempts_per_link: MAX_HEATBATH_ATTEMPTS_PER_LINK,
            autocorrelation_max_lag: AUTOCORRELATION_MAX_LAG,
        }
    }

    fn synthetic_report(chain_id: u64, measurements: Vec<f64>) -> ChainReport {
        let config = benchmark_config(chain_id);
        let chain_diagnostics = diagnostics(&measurements, AUTOCORRELATION_MAX_LAG)
            .expect("synthetic benchmark diagnostics must be valid");
        let heatbath_proposals = EXPECTED_HEATBATH_UPDATES * 2;
        let mean_heatbath_acceptance = EXPECTED_HEATBATH_UPDATES as f64 / heatbath_proposals as f64;

        ChainReport {
            sampler_id: FROZEN_PHASE2A_SAMPLER_ID,
            rng_id: FROZEN_PHASE2A_RNG_ID,
            config,
            derived_seed: frozen_derived_seed(chain_id)
                .expect("synthetic benchmark chain ID must have a frozen golden seed"),
            total_heatbath_sweeps: EXPECTED_HEATBATH_SWEEPS,
            total_overrelaxation_sweeps: EXPECTED_OVERRELAXATION_SWEEPS,
            heatbath_proposals,
            heatbath_updates: EXPECTED_HEATBATH_UPDATES,
            mean_heatbath_acceptance,
            measurements,
            diagnostics: chain_diagnostics,
            final_link_unit_residual: 0.0,
        }
    }

    fn stationary_measurements(phase: usize) -> Vec<f64> {
        (0..MEASUREMENTS)
            .map(|index| {
                if (index + phase) % 2 == 0 {
                    0.601
                } else {
                    0.603
                }
            })
            .collect()
    }

    fn drifting_measurements(phase: usize) -> Vec<f64> {
        let mut measurements = Vec::with_capacity(MEASUREMENTS);
        for index in 0..STATIONARITY_HALF_MEASUREMENTS {
            let alternating = if (index + phase) % 2 == 0 {
                0.001
            } else {
                -0.001
            };
            measurements.push(0.601 + alternating);
        }
        for index in 0..STATIONARITY_HALF_MEASUREMENTS {
            let alternating = if (index + phase) % 2 == 0 {
                0.001
            } else {
                -0.001
            };
            measurements.push(0.603 + alternating);
        }
        measurements
    }

    #[test]
    fn executable_benchmark_identity_matches_frozen_contract() {
        assert_eq!(BENCHMARK_ID, "YM-SU2-PLAQUETTE-BASELINE-V1");
        assert_eq!(PRIMARY_DOI, "10.1103/PhysRevLett.109.111601");
    }

    #[test]
    fn phase2a_stream_identity_and_derivation_match_golden_values() {
        assert_eq!(FROZEN_PHASE2A_SAMPLER_ID, "YM-SU2-ENSEMBLE-V1");
        assert_eq!(FROZEN_PHASE2A_RNG_ID, "SPLITMIX64-V1");
        assert_eq!(
            FROZEN_CHAIN_DERIVED_SEEDS,
            [0x044d_f781_f86a_2ade, 0x4ebd_9478_3d50_8d2e]
        );

        assert_eq!(SAMPLER_ID, "YM-SU2-ENSEMBLE-V1");
        assert_eq!(RNG_ID, "SPLITMIX64-V1");
        assert_eq!(
            derive_chain_seed(0x2b00_2012_1116_0101, 0),
            0x044d_f781_f86a_2ade
        );
        assert_eq!(
            derive_chain_seed(0x2b00_2012_1116_0101, 1),
            0x4ebd_9478_3d50_8d2e
        );
    }

    #[test]
    fn executable_source_envelope_matches_frozen_contract() {
        assert_eq!(SOURCE_ENVELOPE_MIN, 0.59);
        assert_eq!(SOURCE_ENVELOPE_MAX, 0.62);
    }

    #[test]
    fn executable_diagnostic_thresholds_match_frozen_contract() {
        assert_eq!(BETA, 2.3);
        assert_eq!(EXTENTS, [10, 10, 10, 10]);
        assert_eq!(MASTER_SEED, 0x2b00_2012_1116_0101);
        assert_eq!(CHAIN_IDS, [0, 1]);
        assert_eq!(THERMALIZATION_SWEEPS, 40);
        assert_eq!(MEASUREMENTS, 48);
        assert_eq!(SWEEPS_BETWEEN_MEASUREMENTS, 1);
        assert_eq!(OVERRELAXATION_SWEEPS_PER_HEATBATH, 1);
        assert_eq!(MAX_HEATBATH_ATTEMPTS_PER_LINK, 10_000);
        assert_eq!(AUTOCORRELATION_MAX_LAG, 12);
        assert_eq!(MIN_CHAIN_ESS, 6.0);
        assert_eq!(MAX_CHAIN_CONSISTENCY_Z, 5.0);
        assert_eq!(STATIONARITY_HALF_MEASUREMENTS, 24);
        assert_eq!(STATIONARITY_AUTOCORRELATION_MAX_LAG, 6);
        assert_eq!(MAX_STATIONARITY_Z, 5.0);
        assert_eq!(MAX_LINK_UNIT_RESIDUAL, 1.0e-12);
        assert_eq!(PLAQUETTE_SAMPLE_TOLERANCE, 1.0e-12);
        assert_eq!(LINKS_PER_HEATBATH_SWEEP, 40_000);
        assert_eq!(EXPECTED_HEATBATH_SWEEPS, 88);
        assert_eq!(EXPECTED_OVERRELAXATION_SWEEPS, 88);
        assert_eq!(EXPECTED_HEATBATH_UPDATES, 3_520_000);
        assert_eq!(MAX_EXPECTED_HEATBATH_PROPOSALS, 35_200_000_000);
    }

    #[test]
    fn frozen_chain_report_identity_is_required() {
        let first = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        let second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));
        assert!(summarize_two_chains(&first, &second).is_ok());

        let mut wrong_sampler = first.clone();
        wrong_sampler.sampler_id = "YM-SU2-ENSEMBLE-OTHER";
        assert_eq!(
            summarize_two_chains(&wrong_sampler, &second),
            Err(BenchmarkError::WrongBenchmarkConfiguration)
        );

        let mut wrong_rng = first.clone();
        wrong_rng.rng_id = "SPLITMIX64-OTHER";
        assert_eq!(
            summarize_two_chains(&wrong_rng, &second),
            Err(BenchmarkError::WrongBenchmarkConfiguration)
        );

        let mut wrong_seed = first;
        wrong_seed.derived_seed ^= 1;
        assert_eq!(
            summarize_two_chains(&wrong_seed, &second),
            Err(BenchmarkError::WrongBenchmarkConfiguration)
        );
    }

    #[test]
    fn complete_heatbath_accounting_is_required() {
        let second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));

        let mut wrong_sweeps = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        wrong_sweeps.total_heatbath_sweeps -= 1;
        assert_eq!(
            summarize_two_chains(&wrong_sweeps, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );

        let mut wrong_overrelaxation = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        wrong_overrelaxation.total_overrelaxation_sweeps -= 1;
        assert_eq!(
            summarize_two_chains(&wrong_overrelaxation, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );

        let mut partial_updates = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        partial_updates.heatbath_updates -= 1;
        assert_eq!(
            summarize_two_chains(&partial_updates, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );

        let mut too_many_proposals = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        too_many_proposals.heatbath_proposals = MAX_EXPECTED_HEATBATH_PROPOSALS + 1;
        too_many_proposals.mean_heatbath_acceptance = too_many_proposals.heatbath_updates as f64
            / too_many_proposals.heatbath_proposals as f64;
        assert_eq!(
            summarize_two_chains(&too_many_proposals, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );
    }

    #[test]
    fn negative_link_unit_residuals_fail_closed() {
        let second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));
        let mut first = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        first.final_link_unit_residual = -1.0e-16;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::InvalidLinkUnitResidual)
        );

        let first = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        let mut second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));
        second.final_link_unit_residual = -1.0;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::InvalidLinkUnitResidual)
        );
    }

    #[test]
    fn duplicate_measurement_series_fail_closed() {
        let measurements = stationary_measurements(0);
        let first = synthetic_report(CHAIN_IDS[0], measurements.clone());
        let second = synthetic_report(CHAIN_IDS[1], measurements);
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::DuplicateMeasurementSeries)
        );
    }

    #[test]
    fn out_of_range_plaquette_samples_fail_closed() {
        let mut first = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        let second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));
        first.measurements[0] = 1.0 + 2.0 * PLAQUETTE_SAMPLE_TOLERANCE;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::InvalidPlaquetteSample)
        );

        let mut first = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        first.measurements[0] = -1.0 - 2.0 * PLAQUETTE_SAMPLE_TOLERANCE;
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::InvalidPlaquetteSample)
        );
    }

    #[test]
    fn acceptance_evidence_is_derived_from_counts_and_verified() {
        let second = synthetic_report(CHAIN_IDS[1], stationary_measurements(1));

        let mut impossible_cached = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        impossible_cached.mean_heatbath_acceptance = 2.0;
        assert_eq!(
            summarize_two_chains(&impossible_cached, &second),
            Err(BenchmarkError::InvalidAcceptanceEvidence)
        );

        let mut zero_counts = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        zero_counts.heatbath_proposals = 0;
        zero_counts.heatbath_updates = 0;
        zero_counts.mean_heatbath_acceptance = 0.0;
        assert_eq!(
            summarize_two_chains(&zero_counts, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );

        let mut too_many_updates = synthetic_report(CHAIN_IDS[0], stationary_measurements(0));
        too_many_updates.heatbath_updates = EXPECTED_HEATBATH_UPDATES + 1;
        too_many_updates.mean_heatbath_acceptance =
            too_many_updates.heatbath_updates as f64 / too_many_updates.heatbath_proposals as f64;
        assert_eq!(
            summarize_two_chains(&too_many_updates, &second),
            Err(BenchmarkError::InvalidHeatbathAccounting)
        );
    }

    #[test]
    fn gate_diagnostics_are_derived_from_retained_measurements() {
        let raw_first: Vec<f64> = (0..MEASUREMENTS)
            .map(|index| if index % 2 == 0 { 0.899 } else { 0.901 })
            .collect();
        let raw_second: Vec<f64> = (0..MEASUREMENTS)
            .map(|index| if index % 2 == 0 { 0.8985 } else { 0.9015 })
            .collect();
        let cached = diagnostics(&stationary_measurements(0), AUTOCORRELATION_MAX_LAG)
            .expect("cached synthetic diagnostics must be valid");

        let mut first = synthetic_report(CHAIN_IDS[0], raw_first);
        let mut second = synthetic_report(CHAIN_IDS[1], raw_second);
        first.diagnostics = cached.clone();
        second.diagnostics = cached;

        let summary = summarize_two_chains(&first, &second)
            .expect("retained samples should be sufficient to recompute diagnostics");

        assert!(summary.chain_means[0] > 0.89);
        assert!(summary.chain_means[1] > 0.89);
        assert!(!summary.source_envelope_reproduced);
        assert!(!summary.phase2_graduation_gate_satisfied);
    }

    #[test]
    fn zero_variance_stationarity_fails_closed() {
        let first_measurements = vec![0.602; MEASUREMENTS];
        let second_measurements = vec![0.603; MEASUREMENTS];
        assert_eq!(
            stationarity_diagnostic(&first_measurements),
            Err(BenchmarkError::StationarityDiagnosticFailed)
        );

        let first = synthetic_report(CHAIN_IDS[0], first_measurements);
        let second = synthetic_report(CHAIN_IDS[1], second_measurements);
        assert_eq!(
            summarize_two_chains(&first, &second),
            Err(BenchmarkError::StationarityDiagnosticFailed)
        );
    }

    #[test]
    fn executable_stationarity_gate_rejects_drift_above_frozen_threshold() {
        let first = synthetic_report(CHAIN_IDS[0], drifting_measurements(0));
        let second = synthetic_report(CHAIN_IDS[1], drifting_measurements(1));
        let summary = summarize_two_chains(&first, &second)
            .expect("synthetic drifting reports must remain structurally valid");

        assert_eq!(MAX_STATIONARITY_Z, 5.0);
        assert!(summary.source_envelope_reproduced);
        assert!(summary.min_effective_sample_size >= MIN_CHAIN_ESS);
        assert_eq!(summary.chain_consistency_z, 0.0);
        assert!(summary.max_stationarity_z > MAX_STATIONARITY_Z);
        assert!(summary.max_stationarity_z < 50.0);
        assert!(!summary.diagnostics_trustworthy);
        assert!(!summary.phase2_graduation_gate_satisfied);
    }
}
