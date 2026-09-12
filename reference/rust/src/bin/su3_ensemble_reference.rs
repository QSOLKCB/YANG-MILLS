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

use su3_ensemble::{run_chain, Su3EnsembleConfig, LINK_RESIDUAL_LIMIT};

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
    println!("derived_seed={:016x}", report.derived_seed);
    println!("lattice=2,2,2,2");
    println!("beta={:.17e}", report.config.beta);
    println!("proposal_angle={:.17e}", report.config.proposal_angle);
    println!("total_sweeps={}", report.total_sweeps);
    println!("metropolis_proposals={}", report.metropolis_proposals);
    println!("metropolis_accepts={}", report.metropolis_accepts);
    println!(
        "metropolis_acceptance={:.17e}",
        report.mean_metropolis_acceptance
    );
    println!("mean_plaquette={:.17e}", report.diagnostics.mean);
    println!(
        "tau_int={:.17e}",
        report.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "effective_sample_size={:.17e}",
        report.diagnostics.effective_sample_size
    );
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
