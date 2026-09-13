// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2C2 V2 fresh-stream confirmation after the predeclared V1 run
//! reproduced the published plaquette envelope but failed its ESS gate.
//!
//! V2 does not reclassify V1 and does not change the source target, sampler,
//! proposal scale, or per-chain schedule. It preserves the V1 design's
//! intended total information target (2 chains * ESS 8 = aggregate ESS 16)
//! while collecting four fresh independent chains under a new master seed.

use crate::su2_ensemble::diagnostics;
use crate::su3_benchmark::{
    acceptance_from_counts, stationarity_diagnostic, BenchmarkError, StationarityDiagnostic,
    AUTOCORRELATION_MAX_LAG, BETA, EXPECTED_PROPOSALS, EXPECTED_TOTAL_SWEEPS, EXTENTS,
    MAX_LINK_DETERMINANT_RESIDUAL, MAX_LINK_UNITARITY_RESIDUAL, MAX_STATIONARITY_Z, MEASUREMENTS,
    PLAQUETTE_SAMPLE_TOLERANCE, PROPOSAL_ANGLE, SOURCE_ENVELOPE_MAX, SOURCE_ENVELOPE_MIN,
    SWEEPS_BETWEEN_MEASUREMENTS, THERMALIZATION_SWEEPS,
};
use crate::su3_ensemble::{
    derive_su3_chain_seed, Su3ChainReport, PROPOSAL_ID, RNG_ID, SAMPLER_ID, STREAM_ID,
};

pub const CONFIRMATION_ID: &str = "YM-SU3-PLAQUETTE-BASELINE-V2";
pub const MASTER_SEED: u64 = 0x2c02_1981_0501_0002;
pub const CHAIN_IDS: [u64; 4] = [0, 1, 2, 3];
pub const FROZEN_CHAIN_DERIVED_SEEDS: [u64; 4] = [
    0xdfcd_ab17_9476_617a,
    0x151d_d841_ccef_81c6,
    0x9c28_eded_ca3d_1ae2,
    0x17db_bf4a_0021_4611,
];
pub const MIN_AGGREGATE_ESS: f64 = 16.0;
pub const MAX_PAIRWISE_CHAIN_CONSISTENCY_Z: f64 = 5.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfirmationSummary {
    pub chain_means: [f64; 4],
    pub chain_standard_errors: [f64; 4],
    pub chain_effective_sample_sizes: [f64; 4],
    pub chain_acceptances: [f64; 4],
    pub stationarity: [StationarityDiagnostic; 4],
    pub max_stationarity_z: f64,
    pub pooled_mean: f64,
    pub pooled_standard_error: f64,
    pub aggregate_effective_sample_size: f64,
    pub max_pairwise_chain_consistency_z: f64,
    pub max_link_unitarity_residual: f64,
    pub max_link_determinant_residual: f64,
    pub source_envelope_reproduced: bool,
    pub diagnostics_trustworthy: bool,
    pub published_su3_baseline_reproduced: bool,
}

fn frozen_derived_seed(chain_id: u64) -> Option<u64> {
    CHAIN_IDS
        .iter()
        .position(|candidate| *candidate == chain_id)
        .map(|index| FROZEN_CHAIN_DERIVED_SEEDS[index])
}

fn report_matches_confirmation(report: &Su3ChainReport, chain_id: u64) -> bool {
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

fn validate_residuals(report: &Su3ChainReport) -> Result<(), BenchmarkError> {
    if !report.final_link_unitarity_residual.is_finite()
        || !report.final_link_determinant_residual.is_finite()
        || report.final_link_unitarity_residual < 0.0
        || report.final_link_determinant_residual < 0.0
    {
        return Err(BenchmarkError::InvalidLinkResidual);
    }
    Ok(())
}

fn max_pairwise_consistency_z(
    means: &[f64; 4],
    standard_errors: &[f64; 4],
) -> Result<f64, BenchmarkError> {
    let mut maximum = 0.0_f64;
    for first in 0..means.len() {
        for second in (first + 1)..means.len() {
            let difference = (means[first] - means[second]).abs();
            let scale = (standard_errors[first].powi(2) + standard_errors[second].powi(2)).sqrt();
            if !difference.is_finite() || !scale.is_finite() || scale <= 0.0 {
                return Err(BenchmarkError::NonFiniteStatistic);
            }
            let z = difference / scale;
            if !z.is_finite() {
                return Err(BenchmarkError::NonFiniteStatistic);
            }
            maximum = maximum.max(z);
        }
    }
    Ok(maximum)
}

pub fn summarize_four_chains(
    reports: [&Su3ChainReport; 4],
) -> Result<ConfirmationSummary, BenchmarkError> {
    for first in 0..reports.len() {
        for second in (first + 1)..reports.len() {
            if reports[first].derived_seed == reports[second].derived_seed {
                return Err(BenchmarkError::DuplicateChainIdentity);
            }
            if reports[first].measurements == reports[second].measurements {
                return Err(BenchmarkError::DuplicateMeasurementSeries);
            }
        }
    }

    for (index, report) in reports.iter().enumerate() {
        if !report_matches_confirmation(report, CHAIN_IDS[index]) {
            return Err(BenchmarkError::WrongBenchmarkConfiguration);
        }
        if !measurements_are_physical(&report.measurements) {
            return Err(BenchmarkError::InvalidPlaquetteSample);
        }
        validate_residuals(report)?;
    }

    let chain_acceptances = [
        acceptance_from_counts(reports[0])?,
        acceptance_from_counts(reports[1])?,
        acceptance_from_counts(reports[2])?,
        acceptance_from_counts(reports[3])?,
    ];
    let chain_diagnostics = [
        diagnostics(&reports[0].measurements, AUTOCORRELATION_MAX_LAG)
            .map_err(|_| BenchmarkError::NonFiniteStatistic)?,
        diagnostics(&reports[1].measurements, AUTOCORRELATION_MAX_LAG)
            .map_err(|_| BenchmarkError::NonFiniteStatistic)?,
        diagnostics(&reports[2].measurements, AUTOCORRELATION_MAX_LAG)
            .map_err(|_| BenchmarkError::NonFiniteStatistic)?,
        diagnostics(&reports[3].measurements, AUTOCORRELATION_MAX_LAG)
            .map_err(|_| BenchmarkError::NonFiniteStatistic)?,
    ];
    let chain_means = [
        chain_diagnostics[0].mean,
        chain_diagnostics[1].mean,
        chain_diagnostics[2].mean,
        chain_diagnostics[3].mean,
    ];
    let chain_standard_errors = [
        chain_diagnostics[0].standard_error,
        chain_diagnostics[1].standard_error,
        chain_diagnostics[2].standard_error,
        chain_diagnostics[3].standard_error,
    ];
    let chain_effective_sample_sizes = [
        chain_diagnostics[0].effective_sample_size,
        chain_diagnostics[1].effective_sample_size,
        chain_diagnostics[2].effective_sample_size,
        chain_diagnostics[3].effective_sample_size,
    ];
    if !chain_means.iter().all(|value| value.is_finite())
        || !chain_standard_errors
            .iter()
            .all(|value| value.is_finite() && *value > 0.0)
        || !chain_effective_sample_sizes
            .iter()
            .all(|value| value.is_finite() && *value > 0.0)
        || !chain_acceptances.iter().all(|value| value.is_finite())
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

    let stationarity = [
        stationarity_diagnostic(&reports[0].measurements)?,
        stationarity_diagnostic(&reports[1].measurements)?,
        stationarity_diagnostic(&reports[2].measurements)?,
        stationarity_diagnostic(&reports[3].measurements)?,
    ];
    let max_stationarity_z = stationarity
        .iter()
        .map(|diagnostic| diagnostic.drift_z)
        .fold(0.0_f64, f64::max);
    let pooled_mean = chain_means.iter().sum::<f64>() / chain_means.len() as f64;
    let pooled_standard_error = chain_standard_errors
        .iter()
        .map(|value| value.powi(2))
        .sum::<f64>()
        .sqrt()
        / chain_standard_errors.len() as f64;
    let aggregate_effective_sample_size = chain_effective_sample_sizes.iter().sum::<f64>();
    let max_pairwise_chain_consistency_z =
        max_pairwise_consistency_z(&chain_means, &chain_standard_errors)?;
    let max_link_unitarity_residual = reports
        .iter()
        .map(|report| report.final_link_unitarity_residual)
        .fold(0.0_f64, f64::max);
    let max_link_determinant_residual = reports
        .iter()
        .map(|report| report.final_link_determinant_residual)
        .fold(0.0_f64, f64::max);

    if !max_stationarity_z.is_finite()
        || !pooled_mean.is_finite()
        || !pooled_standard_error.is_finite()
        || !aggregate_effective_sample_size.is_finite()
        || !max_pairwise_chain_consistency_z.is_finite()
        || !max_link_unitarity_residual.is_finite()
        || !max_link_determinant_residual.is_finite()
    {
        return Err(BenchmarkError::NonFiniteStatistic);
    }

    let source_envelope_reproduced =
        (SOURCE_ENVELOPE_MIN..=SOURCE_ENVELOPE_MAX).contains(&pooled_mean);
    let diagnostics_trustworthy = aggregate_effective_sample_size >= MIN_AGGREGATE_ESS
        && max_pairwise_chain_consistency_z <= MAX_PAIRWISE_CHAIN_CONSISTENCY_Z
        && max_stationarity_z <= MAX_STATIONARITY_Z
        && max_link_unitarity_residual <= MAX_LINK_UNITARITY_RESIDUAL
        && max_link_determinant_residual <= MAX_LINK_DETERMINANT_RESIDUAL;
    let published_su3_baseline_reproduced = source_envelope_reproduced && diagnostics_trustworthy;

    Ok(ConfirmationSummary {
        chain_means,
        chain_standard_errors,
        chain_effective_sample_sizes,
        chain_acceptances,
        stationarity,
        max_stationarity_z,
        pooled_mean,
        pooled_standard_error,
        aggregate_effective_sample_size,
        max_pairwise_chain_consistency_z,
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
            .map(|index| {
                let lane = (index * 17 + phase * 11) % 23;
                0.592 + (lane as f64 - 11.0) * 0.00035
            })
            .collect()
    }

    fn synthetic_report(chain_id: u64, phase: usize) -> Su3ChainReport {
        let measurements = stationary_measurements(phase);
        let diagnostics = diagnostics(&measurements, AUTOCORRELATION_MAX_LAG)
            .expect("synthetic confirmation diagnostics must be valid");
        let acceptance = 0.5;
        let accepts = (EXPECTED_PROPOSALS as f64 * acceptance) as u64;
        Su3ChainReport {
            sampler_id: SAMPLER_ID,
            proposal_id: PROPOSAL_ID,
            rng_id: RNG_ID,
            stream_id: STREAM_ID,
            config: config(chain_id),
            derived_seed: frozen_derived_seed(chain_id).expect("frozen confirmation chain ID"),
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
    fn confirmation_identity_and_fresh_streams_are_frozen() {
        assert_eq!(CONFIRMATION_ID, "YM-SU3-PLAQUETTE-BASELINE-V2");
        for (index, chain_id) in CHAIN_IDS.iter().enumerate() {
            assert_eq!(
                derive_su3_chain_seed(MASTER_SEED, *chain_id),
                FROZEN_CHAIN_DERIVED_SEEDS[index]
            );
        }
        for first in 0..FROZEN_CHAIN_DERIVED_SEEDS.len() {
            for second in (first + 1)..FROZEN_CHAIN_DERIVED_SEEDS.len() {
                assert_ne!(
                    FROZEN_CHAIN_DERIVED_SEEDS[first],
                    FROZEN_CHAIN_DERIVED_SEEDS[second]
                );
            }
        }
    }

    #[test]
    fn v2_preserves_v1_target_and_sampler_schedule() {
        assert_eq!(MIN_AGGREGATE_ESS, 16.0);
        assert_eq!(MIN_AGGREGATE_ESS, 2.0 * crate::su3_benchmark::MIN_CHAIN_ESS);
        assert_eq!(MAX_PAIRWISE_CHAIN_CONSISTENCY_Z, 5.0);
        assert_eq!(EXPECTED_TOTAL_SWEEPS, 448);
        assert!(EXPECTED_TOTAL_SWEEPS <= crate::su3_ensemble::MAX_REFERENCE_SWEEPS);
        assert_eq!(PROPOSAL_ANGLE, crate::su3_benchmark::PROPOSAL_ANGLE);
        assert_eq!([SOURCE_ENVELOPE_MIN, SOURCE_ENVELOPE_MAX], [0.56, 0.63]);
    }

    #[test]
    fn four_independent_stationary_synthetic_chains_pass() {
        let reports = [
            synthetic_report(0, 0),
            synthetic_report(1, 1),
            synthetic_report(2, 2),
            synthetic_report(3, 3),
        ];
        let summary = summarize_four_chains([&reports[0], &reports[1], &reports[2], &reports[3]])
            .expect("valid synthetic confirmation");
        assert!(summary.aggregate_effective_sample_size >= MIN_AGGREGATE_ESS);
        assert!(summary.source_envelope_reproduced);
        assert!(summary.diagnostics_trustworthy);
        assert!(summary.published_su3_baseline_reproduced);
    }

    #[test]
    fn duplicate_confirmation_stream_or_series_fails_closed() {
        let first = synthetic_report(0, 0);
        let mut second = synthetic_report(1, 1);
        let third = synthetic_report(2, 2);
        let fourth = synthetic_report(3, 3);
        second.derived_seed = first.derived_seed;
        assert_eq!(
            summarize_four_chains([&first, &second, &third, &fourth]),
            Err(BenchmarkError::DuplicateChainIdentity)
        );

        let mut second = synthetic_report(1, 1);
        second.measurements = first.measurements.clone();
        assert_eq!(
            summarize_four_chains([&first, &second, &third, &fourth]),
            Err(BenchmarkError::DuplicateMeasurementSeries)
        );
    }
}
