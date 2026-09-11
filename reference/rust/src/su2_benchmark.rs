// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 2B published SU(2) average-plaquette benchmark logic.
//!
//! The primary source is Langfeld, Lucini & Rago, Phys. Rev. Lett. 109,
//! 111601 (2012), DOI 10.1103/PhysRevLett.109.111601.  Their Fig. 5 shows
//! the average plaquette for four-dimensional SU(2) Wilson gauge theory on
//! a 10^4 lattice and compares density-of-states and local-hybrid Monte Carlo
//! results.  The paper presents this comparison graphically rather than as a
//! table of point estimates, so this module freezes a deliberately conservative
//! source-resolution envelope rather than inventing a paper uncertainty.

use core::fmt;

use crate::su2_ensemble::ChainReport;

pub const BENCHMARK_ID: &str = "YM-SU2-PLAQUETTE-BASELINE-V1";
pub const PRIMARY_DOI: &str = "10.1103/PhysRevLett.109.111601";
pub const BETA: f64 = 2.3;
pub const EXTENTS: [usize; 4] = [10, 10, 10, 10];
pub const SOURCE_ENVELOPE_MIN: f64 = 0.59;
pub const SOURCE_ENVELOPE_MAX: f64 = 0.62;
pub const MASTER_SEED: u64 = 0x2b00_2012_1116_0101;
pub const CHAIN_IDS: [u64; 2] = [0, 1];
pub const THERMALIZATION_SWEEPS: usize = 40;
pub const MEASUREMENTS: usize = 48;
pub const SWEEPS_BETWEEN_MEASUREMENTS: usize = 1;
pub const OVERRELAXATION_SWEEPS_PER_HEATBATH: usize = 1;
pub const MAX_HEATBATH_ATTEMPTS_PER_LINK: usize = 10_000;
pub const AUTOCORRELATION_MAX_LAG: usize = 12;
pub const MIN_CHAIN_ESS: f64 = 6.0;
pub const MAX_CHAIN_CONSISTENCY_Z: f64 = 5.0;
pub const MAX_LINK_UNIT_RESIDUAL: f64 = 1.0e-12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchmarkError {
    WrongBenchmarkConfiguration,
    DuplicateChainIdentity,
    NonFiniteStatistic,
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WrongBenchmarkConfiguration => {
                "chain report does not match the frozen Phase 2B benchmark configuration"
            }
            Self::DuplicateChainIdentity => "benchmark chains must have distinct stream identities",
            Self::NonFiniteStatistic => "benchmark aggregation produced a non-finite statistic",
        };
        f.write_str(message)
    }
}

impl std::error::Error for BenchmarkError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BenchmarkSummary {
    pub chain_means: [f64; 2],
    pub chain_standard_errors: [f64; 2],
    pub chain_effective_sample_sizes: [f64; 2],
    pub pooled_mean: f64,
    pub pooled_standard_error: f64,
    pub chain_consistency_z: f64,
    pub min_effective_sample_size: f64,
    pub max_link_unit_residual: f64,
    pub source_envelope_reproduced: bool,
    pub diagnostics_trustworthy: bool,
    pub phase2_graduation_gate_satisfied: bool,
}

fn report_matches_benchmark(report: &ChainReport, chain_id: u64) -> bool {
    let config = report.config;
    config.extents == EXTENTS
        && config.beta == BETA
        && config.master_seed == MASTER_SEED
        && config.chain_id == chain_id
        && config.thermalization_sweeps == THERMALIZATION_SWEEPS
        && config.measurements == MEASUREMENTS
        && config.sweeps_between_measurements == SWEEPS_BETWEEN_MEASUREMENTS
        && config.overrelaxation_sweeps_per_heatbath == OVERRELAXATION_SWEEPS_PER_HEATBATH
        && config.max_heatbath_attempts_per_link == MAX_HEATBATH_ATTEMPTS_PER_LINK
        && config.autocorrelation_max_lag == AUTOCORRELATION_MAX_LAG
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

    let chain_means = [first.diagnostics.mean, second.diagnostics.mean];
    let chain_standard_errors = [
        first.diagnostics.standard_error,
        second.diagnostics.standard_error,
    ];
    let chain_effective_sample_sizes = [
        first.diagnostics.effective_sample_size,
        second.diagnostics.effective_sample_size,
    ];

    if !chain_means.iter().all(|value| value.is_finite())
        || !chain_standard_errors.iter().all(|value| value.is_finite())
        || !chain_effective_sample_sizes
            .iter()
            .all(|value| value.is_finite())
        || !first.final_link_unit_residual.is_finite()
        || !second.final_link_unit_residual.is_finite()
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
        && max_link_unit_residual <= MAX_LINK_UNIT_RESIDUAL
        && first.mean_heatbath_acceptance.is_finite()
        && second.mean_heatbath_acceptance.is_finite()
        && first.mean_heatbath_acceptance > 0.0
        && second.mean_heatbath_acceptance > 0.0;

    Ok(BenchmarkSummary {
        chain_means,
        chain_standard_errors,
        chain_effective_sample_sizes,
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
