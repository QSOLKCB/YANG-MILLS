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
#[path = "../su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use su3_benchmark::{
    summarize_two_chains, AUTOCORRELATION_MAX_LAG, BENCHMARK_ID, BETA, CHAIN_IDS, EXTENTS,
    MASTER_SEED, MAX_CHAIN_CONSISTENCY_Z, MAX_METROPOLIS_ACCEPTANCE, MAX_STATIONARITY_Z,
    MEASUREMENTS, MIN_CHAIN_ESS, MIN_METROPOLIS_ACCEPTANCE, PRIMARY_DOI, PROPOSAL_ANGLE,
    SOURCE_ENVELOPE_MAX, SOURCE_ENVELOPE_MIN, STATIONARITY_AUTOCORRELATION_MAX_LAG,
    STATIONARITY_HALF_MEASUREMENTS, SWEEPS_BETWEEN_MEASUREMENTS, THERMALIZATION_SWEEPS,
};
use su3_ensemble::{run_chain, Su3EnsembleConfig};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first = run_chain(config(CHAIN_IDS[0]))?;
    let second = run_chain(config(CHAIN_IDS[1]))?;
    let summary = summarize_two_chains(&first, &second)?;

    // V1 is an immutable failed attempt. This executable now verifies the
    // archived failure boundary rather than silently turning that failure into
    // a success after the V2 protocol was designed.
    let sole_predeclared_failure_reproduced = summary.source_envelope_reproduced
        && !summary.published_su3_baseline_reproduced
        && summary.min_effective_sample_size < MIN_CHAIN_ESS
        && summary.chain_consistency_z <= MAX_CHAIN_CONSISTENCY_Z
        && summary.max_stationarity_z <= MAX_STATIONARITY_Z;
    if !sole_predeclared_failure_reproduced {
        return Err(format!(
            "Phase 2C2 V1 archived boundary drift: pooled_mean={:.17e}, envelope=[{:.17e},{:.17e}], min_ess={:.17e}, required_ess={:.17e}, chain_z={:.17e}, stationarity_z={:.17e}",
            summary.pooled_mean,
            SOURCE_ENVELOPE_MIN,
            SOURCE_ENVELOPE_MAX,
            summary.min_effective_sample_size,
            MIN_CHAIN_ESS,
            summary.chain_consistency_z,
            summary.max_stationarity_z,
        )
        .into());
    }

    println!("contract={BENCHMARK_ID}");
    println!("primary_doi={PRIMARY_DOI}");
    println!("result_role=archived_v1_failed_attempt");
    println!("lattice=4,4,4,4");
    println!("beta={BETA:.17e}");
    println!("source_envelope_min={SOURCE_ENVELOPE_MIN:.17e}");
    println!("source_envelope_max={SOURCE_ENVELOPE_MAX:.17e}");
    println!("source_envelope_is_paper_error_bar=false");
    println!("master_seed={MASTER_SEED:016x}");
    println!("chain0_id={}", CHAIN_IDS[0]);
    println!("chain1_id={}", CHAIN_IDS[1]);
    println!("chain0_derived_seed={:016x}", first.derived_seed);
    println!("chain1_derived_seed={:016x}", second.derived_seed);
    println!("proposal_angle={PROPOSAL_ANGLE:.17e}");
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("measurements_per_chain={MEASUREMENTS}");
    println!("sweeps_between_measurements={SWEEPS_BETWEEN_MEASUREMENTS}");
    println!("autocorrelation_max_lag={AUTOCORRELATION_MAX_LAG}");
    println!("stationarity_half_measurements={STATIONARITY_HALF_MEASUREMENTS}");
    println!("stationarity_autocorrelation_max_lag={STATIONARITY_AUTOCORRELATION_MAX_LAG}");
    println!("minimum_effective_sample_size_required={MIN_CHAIN_ESS:.17e}");
    println!("maximum_chain_consistency_z={MAX_CHAIN_CONSISTENCY_Z:.17e}");
    println!("maximum_stationarity_z={MAX_STATIONARITY_Z:.17e}");
    println!("minimum_metropolis_acceptance={MIN_METROPOLIS_ACCEPTANCE:.17e}");
    println!("maximum_metropolis_acceptance={MAX_METROPOLIS_ACCEPTANCE:.17e}");
    println!("chain0_measurements={}", series_text(&first.measurements));
    println!("chain1_measurements={}", series_text(&second.measurements));
    println!("chain0_mean={:.17e}", summary.chain_means[0]);
    println!("chain1_mean={:.17e}", summary.chain_means[1]);
    println!(
        "chain0_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[0]
    );
    println!(
        "chain1_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[1]
    );
    println!(
        "minimum_effective_sample_size_observed={:.17e}",
        summary.min_effective_sample_size
    );
    println!("chain_consistency_z={:.17e}", summary.chain_consistency_z);
    println!(
        "maximum_stationarity_z_observed={:.17e}",
        summary.max_stationarity_z
    );
    println!(
        "chain0_metropolis_acceptance={:.17e}",
        summary.chain_acceptances[0]
    );
    println!(
        "chain1_metropolis_acceptance={:.17e}",
        summary.chain_acceptances[1]
    );
    println!("pooled_mean={:.17e}", summary.pooled_mean);
    println!("source_envelope_reproduced=true");
    println!("published_su3_baseline_reproduced=false");
    println!("failed_gate=minimum_effective_sample_size_per_chain");
    println!("phase2c_complete=false");
    println!("glueball_analysis_enabled=false");
    println!("confinement_claimed=false");
    println!("continuum_qft_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=archived_failed_gate_reproduced");
    Ok(())
}
