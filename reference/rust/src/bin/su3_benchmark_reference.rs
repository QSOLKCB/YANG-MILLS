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

    if !summary.published_su3_baseline_reproduced {
        return Err(format!(
            "Phase 2C2 benchmark failed: pooled_mean={:.17e}, envelope=[{:.17e},{:.17e}], min_ess={:.17e}, chain_z={:.17e}, stationarity_z={:.17e}, acceptance=[{:.17e},{:.17e}]",
            summary.pooled_mean,
            SOURCE_ENVELOPE_MIN,
            SOURCE_ENVELOPE_MAX,
            summary.min_effective_sample_size,
            summary.chain_consistency_z,
            summary.max_stationarity_z,
            summary.chain_acceptances[0],
            summary.chain_acceptances[1]
        )
        .into());
    }

    println!("contract={BENCHMARK_ID}");
    println!("primary_doi={PRIMARY_DOI}");
    println!("source_surface=Edgar-McCrossen-Moriarty-1981-4^4-graphical-plaquette");
    println!("lattice=4,4,4,4");
    println!("beta={BETA:.17e}");
    println!("observable=normalized_average_plaquette");
    println!("source_envelope_min={SOURCE_ENVELOPE_MIN:.17e}");
    println!("source_envelope_max={SOURCE_ENVELOPE_MAX:.17e}");
    println!("source_envelope_is_paper_error_bar=false");
    println!("master_seed={MASTER_SEED:016x}");
    println!("chain0_id={}", CHAIN_IDS[0]);
    println!("chain1_id={}", CHAIN_IDS[1]);
    println!("chain0_sampler_id={}", first.sampler_id);
    println!("chain1_sampler_id={}", second.sampler_id);
    println!("chain0_proposal_id={}", first.proposal_id);
    println!("chain1_proposal_id={}", second.proposal_id);
    println!("chain0_rng_id={}", first.rng_id);
    println!("chain1_rng_id={}", second.rng_id);
    println!("chain0_stream_id={}", first.stream_id);
    println!("chain1_stream_id={}", second.stream_id);
    println!("chain0_derived_seed={:016x}", first.derived_seed);
    println!("chain1_derived_seed={:016x}", second.derived_seed);
    println!("proposal_angle={PROPOSAL_ANGLE:.17e}");
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("measurements_per_chain={MEASUREMENTS}");
    println!("sweeps_between_measurements={SWEEPS_BETWEEN_MEASUREMENTS}");
    println!("autocorrelation_max_lag={AUTOCORRELATION_MAX_LAG}");
    println!("stationarity_half_measurements={STATIONARITY_HALF_MEASUREMENTS}");
    println!("stationarity_autocorrelation_max_lag={STATIONARITY_AUTOCORRELATION_MAX_LAG}");
    println!("minimum_effective_sample_size={MIN_CHAIN_ESS:.17e}");
    println!("maximum_chain_consistency_z={MAX_CHAIN_CONSISTENCY_Z:.17e}");
    println!("maximum_stationarity_z={MAX_STATIONARITY_Z:.17e}");
    println!("minimum_metropolis_acceptance={MIN_METROPOLIS_ACCEPTANCE:.17e}");
    println!("maximum_metropolis_acceptance={MAX_METROPOLIS_ACCEPTANCE:.17e}");
    println!("chain0_total_sweeps={}", first.total_sweeps);
    println!("chain1_total_sweeps={}", second.total_sweeps);
    println!("chain0_metropolis_proposals={}", first.metropolis_proposals);
    println!(
        "chain1_metropolis_proposals={}",
        second.metropolis_proposals
    );
    println!("chain0_metropolis_accepts={}", first.metropolis_accepts);
    println!("chain1_metropolis_accepts={}", second.metropolis_accepts);
    println!(
        "chain0_metropolis_acceptance={:.17e}",
        summary.chain_acceptances[0]
    );
    println!(
        "chain1_metropolis_acceptance={:.17e}",
        summary.chain_acceptances[1]
    );
    println!("chain0_measurements={}", series_text(&first.measurements));
    println!("chain1_measurements={}", series_text(&second.measurements));
    println!("chain0_mean={:.17e}", summary.chain_means[0]);
    println!("chain1_mean={:.17e}", summary.chain_means[1]);
    println!(
        "chain0_standard_error={:.17e}",
        summary.chain_standard_errors[0]
    );
    println!(
        "chain1_standard_error={:.17e}",
        summary.chain_standard_errors[1]
    );
    println!(
        "chain0_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[0]
    );
    println!(
        "chain1_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[1]
    );
    println!(
        "chain0_tau_int={:.17e}",
        first.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "chain1_tau_int={:.17e}",
        second.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "chain0_stationarity_first_half_mean={:.17e}",
        summary.stationarity[0].first_half_mean
    );
    println!(
        "chain0_stationarity_second_half_mean={:.17e}",
        summary.stationarity[0].second_half_mean
    );
    println!(
        "chain0_stationarity_z={:.17e}",
        summary.stationarity[0].drift_z
    );
    println!(
        "chain1_stationarity_first_half_mean={:.17e}",
        summary.stationarity[1].first_half_mean
    );
    println!(
        "chain1_stationarity_second_half_mean={:.17e}",
        summary.stationarity[1].second_half_mean
    );
    println!(
        "chain1_stationarity_z={:.17e}",
        summary.stationarity[1].drift_z
    );
    println!("max_stationarity_z={:.17e}", summary.max_stationarity_z);
    println!("pooled_mean={:.17e}", summary.pooled_mean);
    println!(
        "pooled_standard_error={:.17e}",
        summary.pooled_standard_error
    );
    println!("chain_consistency_z={:.17e}", summary.chain_consistency_z);
    println!(
        "min_effective_sample_size={:.17e}",
        summary.min_effective_sample_size
    );
    println!(
        "max_link_unitarity_residual={:.17e}",
        summary.max_link_unitarity_residual
    );
    println!(
        "max_link_determinant_residual={:.17e}",
        summary.max_link_determinant_residual
    );
    println!("published_su3_baseline_reproduced=true");
    println!("phase2c_complete=true");
    println!("glueball_analysis_enabled=true");
    println!("confinement_claimed=false");
    println!("continuum_qft_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=passed");
    Ok(())
}
