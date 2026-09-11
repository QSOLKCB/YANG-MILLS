// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../su2_benchmark.rs"]
mod su2_benchmark;
#[path = "../su2_ensemble.rs"]
mod su2_ensemble;

use su2_benchmark::{
    summarize_two_chains, AUTOCORRELATION_MAX_LAG, BENCHMARK_ID, BETA, CHAIN_IDS, EXTENTS,
    MASTER_SEED, MAX_HEATBATH_ATTEMPTS_PER_LINK, MEASUREMENTS,
    OVERRELAXATION_SWEEPS_PER_HEATBATH, PRIMARY_DOI, SOURCE_ENVELOPE_MAX, SOURCE_ENVELOPE_MIN,
    SWEEPS_BETWEEN_MEASUREMENTS, THERMALIZATION_SWEEPS,
};
use su2_ensemble::{run_chain, EnsembleConfig};

fn config(chain_id: u64) -> EnsembleConfig {
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

    if !summary.phase2_graduation_gate_satisfied {
        return Err(format!(
            "Phase 2B benchmark failed: pooled_mean={:.17e}, envelope=[{:.17e},{:.17e}], min_ess={:.17e}, chain_z={:.17e}",
            summary.pooled_mean,
            SOURCE_ENVELOPE_MIN,
            SOURCE_ENVELOPE_MAX,
            summary.min_effective_sample_size,
            summary.chain_consistency_z
        )
        .into());
    }

    println!("contract={BENCHMARK_ID}");
    println!("primary_doi={PRIMARY_DOI}");
    println!("lattice=10,10,10,10");
    println!("beta={BETA:.17e}");
    println!("master_seed={MASTER_SEED:016x}");
    println!("chain0_id={}", CHAIN_IDS[0]);
    println!("chain1_id={}", CHAIN_IDS[1]);
    println!("chain0_derived_seed={:016x}", first.derived_seed);
    println!("chain1_derived_seed={:016x}", second.derived_seed);
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("measurements_per_chain={MEASUREMENTS}");
    println!("sweeps_between_measurements={SWEEPS_BETWEEN_MEASUREMENTS}");
    println!(
        "overrelaxation_sweeps_per_heatbath={OVERRELAXATION_SWEEPS_PER_HEATBATH}"
    );
    println!("autocorrelation_max_lag={AUTOCORRELATION_MAX_LAG}");
    println!("source_envelope_min={SOURCE_ENVELOPE_MIN:.17e}");
    println!("source_envelope_max={SOURCE_ENVELOPE_MAX:.17e}");
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
        "chain0_heatbath_acceptance={:.17e}",
        first.mean_heatbath_acceptance
    );
    println!(
        "chain1_heatbath_acceptance={:.17e}",
        second.mean_heatbath_acceptance
    );
    println!("pooled_mean={:.17e}", summary.pooled_mean);
    println!(
        "pooled_standard_error={:.17e}",
        summary.pooled_standard_error
    );
    println!("chain_consistency_z={:.17e}", summary.chain_consistency_z);
    println!(
        "minimum_effective_sample_size={:.17e}",
        summary.min_effective_sample_size
    );
    println!(
        "max_link_unit_residual={:.17e}",
        summary.max_link_unit_residual
    );
    println!("published_plaquette_baseline_reproduced=true");
    println!("phase2_graduation_gate_satisfied=true");
    println!("su3_ensemble_enabled=false");
    println!("continuum_qft_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=passed");
    Ok(())
}
