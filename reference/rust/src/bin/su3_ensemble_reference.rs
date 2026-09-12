// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../su2_ensemble.rs"]
mod su2_ensemble;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use su3_ensemble::{
    run_chain, Su3EnsembleConfig, ANGLE_GRID_MAGNITUDE_BITS, LINK_RESIDUAL_LIMIT,
    MAX_REFERENCE_SWEEPS,
};

fn print_series(label: &str, values: &[f64]) {
    print!("{label}=");
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            print!(",");
        }
        print!("{value:.17e}");
    }
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Su3EnsembleConfig {
        extents: [2, 2, 2, 2],
        beta: 0.0,
        master_seed: 0x3c00_1982_1223_0101,
        chain_id: 0,
        thermalization_sweeps: 4,
        measurements: 16,
        sweeps_between_measurements: 1,
        proposal_angle: 0.35,
        autocorrelation_max_lag: 4,
    };
    let report = run_chain(config)?;

    if report.mean_metropolis_acceptance != 1.0 {
        return Err("beta=0 SU(3) Metropolis sweep must accept every proposal".into());
    }
    if report.final_link_unitarity_residual > LINK_RESIDUAL_LIMIT
        || report.final_link_determinant_residual > LINK_RESIDUAL_LIMIT
    {
        return Err("SU(3) link residual exceeded the frozen Phase 2C1 limit".into());
    }
    if report
        .measurements
        .iter()
        .any(|value| !(-0.5 - 1.0e-12..=1.0 + 1.0e-12).contains(value))
    {
        return Err("SU(3) normalized plaquette left its physical trace range".into());
    }

    println!("contract=YM-SU3-ENSEMBLE-V1");
    println!("sampler_id={}", report.sampler_id);
    println!("proposal_id={}", report.proposal_id);
    println!("rng_id={}", report.rng_id);
    println!("stream_id={}", report.stream_id);
    println!("master_seed={:016x}", report.config.master_seed);
    println!("chain_id={}", report.config.chain_id);
    println!("derived_seed={:016x}", report.derived_seed);
    println!(
        "lattice={},{},{},{}",
        report.config.extents[0],
        report.config.extents[1],
        report.config.extents[2],
        report.config.extents[3]
    );
    println!("beta={:.17e}", report.config.beta);
    println!("proposal_angle={:.17e}", report.config.proposal_angle);
    println!("angle_grid_magnitude_bits={ANGLE_GRID_MAGNITUDE_BITS}");
    println!("maximum_reference_sweeps={MAX_REFERENCE_SWEEPS}");
    println!(
        "thermalization_sweeps={}",
        report.config.thermalization_sweeps
    );
    println!("measurements={}", report.config.measurements);
    println!(
        "sweeps_between_measurements={}",
        report.config.sweeps_between_measurements
    );
    println!(
        "autocorrelation_max_lag={}",
        report.config.autocorrelation_max_lag
    );
    println!("total_sweeps={}", report.total_sweeps);
    println!("metropolis_proposals={}", report.metropolis_proposals);
    println!("metropolis_accepts={}", report.metropolis_accepts);
    println!(
        "metropolis_acceptance={:.17e}",
        report.mean_metropolis_acceptance
    );
    print_series("measurements", &report.measurements);
    println!("mean_plaquette={:.17e}", report.diagnostics.mean);
    println!("variance={:.17e}", report.diagnostics.variance);
    print_series("autocorrelation", &report.diagnostics.autocorrelation);
    println!(
        "positive_window_lag={}",
        report.diagnostics.positive_window_lag
    );
    println!(
        "tau_int={:.17e}",
        report.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "effective_sample_size={:.17e}",
        report.diagnostics.effective_sample_size
    );
    println!("standard_error={:.17e}", report.diagnostics.standard_error);
    println!(
        "final_link_unitarity_residual={:.17e}",
        report.final_link_unitarity_residual
    );
    println!(
        "final_link_determinant_residual={:.17e}",
        report.final_link_determinant_residual
    );
    println!("published_su3_baseline_reproduced=false");
    println!("glueball_analysis_enabled=false");
    println!("confinement_claimed=false");
    println!("continuum_qft_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=passed");
    Ok(())
}
