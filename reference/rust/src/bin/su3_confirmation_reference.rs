// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../su2_ensemble.rs"]
mod su2_ensemble;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_benchmark.rs"]
mod su3_benchmark;
#[path = "../su3_confirmation.rs"]
mod su3_confirmation;
#[path = "../su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use su3_benchmark::{
    AUTOCORRELATION_MAX_LAG, BETA, EXTENTS, MAX_METROPOLIS_ACCEPTANCE, MAX_STATIONARITY_Z,
    MEASUREMENTS, MIN_METROPOLIS_ACCEPTANCE, PROPOSAL_ANGLE, SOURCE_ENVELOPE_MAX,
    SOURCE_ENVELOPE_MIN, STATIONARITY_AUTOCORRELATION_MAX_LAG, STATIONARITY_HALF_MEASUREMENTS,
    SWEEPS_BETWEEN_MEASUREMENTS, THERMALIZATION_SWEEPS,
};
use su3_confirmation::{
    summarize_four_chains, CHAIN_IDS, CONFIRMATION_ID, MASTER_SEED,
    MAX_PAIRWISE_CHAIN_CONSISTENCY_Z, MIN_AGGREGATE_ESS,
};
use su3_ensemble::{run_chain, Su3ChainReport, Su3EnsembleConfig};

const FROZEN_SUMMARY_ABS_TOLERANCE: f64 = 1.0e-12;
const FROZEN_SUMMARY_REL_TOLERANCE: f64 = 1.0e-10;
const FROZEN_RESIDUAL_ABS_TOLERANCE: f64 = 1.0e-13;
const FROZEN_CHAIN_MEANS: [f64; 4] = [
    0.5951228942049566,
    0.6010578916054883,
    0.6006811934765629,
    0.6003060377873822,
];
const FROZEN_CHAIN_STANDARD_ERRORS: [f64; 4] = [
    0.00073731922680896,
    0.0017334647942562633,
    0.0013810304643338124,
    0.0023415778524011042,
];
const FROZEN_CHAIN_EFFECTIVE_SAMPLE_SIZES: [f64; 4] = [
    37.108298427827265,
    12.12458490228564,
    19.780522239783117,
    9.930599191453949,
];
const FROZEN_CHAIN_ACCEPTANCES: [f64; 4] = [
    0.8675000326974052,
    0.8676090240478516,
    0.867549078805106,
    0.867567607334682,
];
const FROZEN_STATIONARITY_Z: [f64; 4] = [
    1.4650733209908853,
    1.2503134066297152,
    0.6586970772179463,
    0.6522302609275793,
];
const FROZEN_POOLED_MEAN: f64 = 0.5992920042685975;
const FROZEN_POOLED_STANDARD_ERROR: f64 = 0.0008268453534905086;
const FROZEN_AGGREGATE_ESS: f64 = 78.94400476134997;
const FROZEN_MAX_PAIRWISE_CHAIN_Z: f64 = 3.5504261086461475;
const FROZEN_MAX_STATIONARITY_Z: f64 = 1.4650733209908853;
const FROZEN_MAX_UNITARITY_RESIDUAL: f64 = 3.1308289294429414e-14;
const FROZEN_MAX_DETERMINANT_RESIDUAL: f64 = 3.553115420753491e-14;

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

fn series_text(values: &[f64]) -> String {
    values
        .iter()
        .map(|value| format!("{value:.17e}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn evidence_close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs()
        <= FROZEN_SUMMARY_ABS_TOLERANCE + FROZEN_SUMMARY_REL_TOLERANCE * expected.abs()
}

fn evidence_array_close<const N: usize>(actual: &[f64; N], expected: &[f64; N]) -> bool {
    actual
        .iter()
        .zip(expected)
        .all(|(actual, expected)| evidence_close(*actual, *expected))
}

fn validate_frozen_v2_replay(
    summary: &su3_confirmation::ConfirmationSummary,
) -> Result<(), String> {
    let stationarity_z = summary.stationarity.map(|entry| entry.drift_z);
    let summaries_match = evidence_array_close(&summary.chain_means, &FROZEN_CHAIN_MEANS)
        && evidence_array_close(
            &summary.chain_standard_errors,
            &FROZEN_CHAIN_STANDARD_ERRORS,
        )
        && evidence_array_close(
            &summary.chain_effective_sample_sizes,
            &FROZEN_CHAIN_EFFECTIVE_SAMPLE_SIZES,
        )
        && evidence_array_close(&summary.chain_acceptances, &FROZEN_CHAIN_ACCEPTANCES)
        && evidence_array_close(&stationarity_z, &FROZEN_STATIONARITY_Z)
        && evidence_close(summary.pooled_mean, FROZEN_POOLED_MEAN)
        && evidence_close(summary.pooled_standard_error, FROZEN_POOLED_STANDARD_ERROR)
        && evidence_close(
            summary.aggregate_effective_sample_size,
            FROZEN_AGGREGATE_ESS,
        )
        && evidence_close(
            summary.max_pairwise_chain_consistency_z,
            FROZEN_MAX_PAIRWISE_CHAIN_Z,
        )
        && evidence_close(summary.max_stationarity_z, FROZEN_MAX_STATIONARITY_Z)
        && (summary.max_link_unitarity_residual - FROZEN_MAX_UNITARITY_RESIDUAL).abs()
            <= FROZEN_RESIDUAL_ABS_TOLERANCE
        && (summary.max_link_determinant_residual - FROZEN_MAX_DETERMINANT_RESIDUAL).abs()
            <= FROZEN_RESIDUAL_ABS_TOLERANCE;

    if !summaries_match {
        return Err(format!(
            "frozen Phase 2C2 V2 evidence drift: summary={summary:?}, stationarity_z={stationarity_z:?}"
        ));
    }
    Ok(())
}

fn print_chain(
    index: usize,
    report: &Su3ChainReport,
    summary: &su3_confirmation::ConfirmationSummary,
) {
    println!("chain{index}_id={}", report.config.chain_id);
    println!("chain{index}_derived_seed={:016x}", report.derived_seed);
    println!("chain{index}_sampler_id={}", report.sampler_id);
    println!("chain{index}_proposal_id={}", report.proposal_id);
    println!("chain{index}_rng_id={}", report.rng_id);
    println!("chain{index}_stream_id={}", report.stream_id);
    println!("chain{index}_total_sweeps={}", report.total_sweeps);
    println!(
        "chain{index}_metropolis_proposals={}",
        report.metropolis_proposals
    );
    println!(
        "chain{index}_metropolis_accepts={}",
        report.metropolis_accepts
    );
    println!(
        "chain{index}_metropolis_acceptance={:.17e}",
        summary.chain_acceptances[index]
    );
    println!(
        "chain{index}_measurements={}",
        series_text(&report.measurements)
    );
    println!("chain{index}_mean={:.17e}", summary.chain_means[index]);
    println!("chain{index}_variance={:.17e}", report.diagnostics.variance);
    println!(
        "chain{index}_autocorrelation={}",
        series_text(&report.diagnostics.autocorrelation)
    );
    println!(
        "chain{index}_positive_window_lag={}",
        report.diagnostics.positive_window_lag
    );
    println!(
        "chain{index}_standard_error={:.17e}",
        summary.chain_standard_errors[index]
    );
    println!(
        "chain{index}_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[index]
    );
    println!(
        "chain{index}_tau_int={:.17e}",
        report.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "chain{index}_stationarity_first_half_mean={:.17e}",
        summary.stationarity[index].first_half_mean
    );
    println!(
        "chain{index}_stationarity_second_half_mean={:.17e}",
        summary.stationarity[index].second_half_mean
    );
    println!(
        "chain{index}_stationarity_first_half_standard_error={:.17e}",
        summary.stationarity[index].first_half_standard_error
    );
    println!(
        "chain{index}_stationarity_second_half_standard_error={:.17e}",
        summary.stationarity[index].second_half_standard_error
    );
    println!(
        "chain{index}_stationarity_z={:.17e}",
        summary.stationarity[index].drift_z
    );
    println!(
        "chain{index}_final_link_unitarity_residual={:.17e}",
        report.final_link_unitarity_residual
    );
    println!(
        "chain{index}_final_link_determinant_residual={:.17e}",
        report.final_link_determinant_residual
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chain0 = run_chain(config(CHAIN_IDS[0]))?;
    let chain1 = run_chain(config(CHAIN_IDS[1]))?;
    let chain2 = run_chain(config(CHAIN_IDS[2]))?;
    let chain3 = run_chain(config(CHAIN_IDS[3]))?;
    let reports = [&chain0, &chain1, &chain2, &chain3];
    let summary = summarize_four_chains(reports)?;

    if !summary.published_su3_baseline_reproduced {
        return Err(format!(
            "Phase 2C2 V2 confirmation failed: pooled_mean={:.17e}, envelope=[{:.17e},{:.17e}], aggregate_ess={:.17e}, required_aggregate_ess={:.17e}, max_pairwise_chain_z={:.17e}, max_stationarity_z={:.17e}",
            summary.pooled_mean,
            SOURCE_ENVELOPE_MIN,
            SOURCE_ENVELOPE_MAX,
            summary.aggregate_effective_sample_size,
            MIN_AGGREGATE_ESS,
            summary.max_pairwise_chain_consistency_z,
            summary.max_stationarity_z,
        )
        .into());
    }
    validate_frozen_v2_replay(&summary)?;

    println!("contract={CONFIRMATION_ID}");
    println!("primary_doi=10.1088/0305-4616/7/5/001");
    println!("confirmation_role=fresh_four_chain_v2");
    println!("frozen_evidence_reproduced=true");
    println!("frozen_summary_abs_tolerance={FROZEN_SUMMARY_ABS_TOLERANCE:.17e}");
    println!("frozen_summary_rel_tolerance={FROZEN_SUMMARY_REL_TOLERANCE:.17e}");
    println!("frozen_residual_abs_tolerance={FROZEN_RESIDUAL_ABS_TOLERANCE:.17e}");
    println!("lattice=4,4,4,4");
    println!("beta={BETA:.17e}");
    println!("source_envelope_min={SOURCE_ENVELOPE_MIN:.17e}");
    println!("source_envelope_max={SOURCE_ENVELOPE_MAX:.17e}");
    println!("source_envelope_is_paper_error_bar=false");
    println!("master_seed={MASTER_SEED:016x}");
    println!("proposal_angle={PROPOSAL_ANGLE:.17e}");
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("measurements_per_chain={MEASUREMENTS}");
    println!("sweeps_between_measurements={SWEEPS_BETWEEN_MEASUREMENTS}");
    println!("autocorrelation_max_lag={AUTOCORRELATION_MAX_LAG}");
    println!("stationarity_half_measurements={STATIONARITY_HALF_MEASUREMENTS}");
    println!("stationarity_autocorrelation_max_lag={STATIONARITY_AUTOCORRELATION_MAX_LAG}");
    println!("minimum_aggregate_effective_sample_size={MIN_AGGREGATE_ESS:.17e}");
    println!("maximum_pairwise_chain_consistency_z={MAX_PAIRWISE_CHAIN_CONSISTENCY_Z:.17e}");
    println!("maximum_stationarity_z={MAX_STATIONARITY_Z:.17e}");
    println!("minimum_metropolis_acceptance={MIN_METROPOLIS_ACCEPTANCE:.17e}");
    println!("maximum_metropolis_acceptance={MAX_METROPOLIS_ACCEPTANCE:.17e}");
    print_chain(0, &chain0, &summary);
    print_chain(1, &chain1, &summary);
    print_chain(2, &chain2, &summary);
    print_chain(3, &chain3, &summary);
    println!("pooled_mean={:.17e}", summary.pooled_mean);
    println!(
        "pooled_standard_error={:.17e}",
        summary.pooled_standard_error
    );
    println!(
        "aggregate_effective_sample_size={:.17e}",
        summary.aggregate_effective_sample_size
    );
    println!(
        "max_pairwise_chain_consistency_z={:.17e}",
        summary.max_pairwise_chain_consistency_z
    );
    println!("max_stationarity_z={:.17e}", summary.max_stationarity_z);
    println!(
        "max_link_unitarity_residual={:.17e}",
        summary.max_link_unitarity_residual
    );
    println!(
        "max_link_determinant_residual={:.17e}",
        summary.max_link_determinant_residual
    );
    println!("source_envelope_reproduced=true");
    println!("published_su3_baseline_reproduced=true");
    println!("phase2c_complete=true");
    println!("glueball_analysis_enabled=true");
    println!("v1_reclassified_as_pass=false");
    println!("confinement_claimed=false");
    println!("continuum_qft_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=passed");
    Ok(())
}

#[cfg(test)]
mod replay_tolerance_tests {
    use super::*;

    #[test]
    fn frozen_v2_replay_tolerances_are_frozen() {
        assert_eq!(FROZEN_SUMMARY_ABS_TOLERANCE, 1.0e-12);
        assert_eq!(FROZEN_SUMMARY_REL_TOLERANCE, 1.0e-10);
        assert_eq!(FROZEN_RESIDUAL_ABS_TOLERANCE, 1.0e-13);
    }
}
