// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../su2_ensemble.rs"]
mod su2_ensemble;

use su2_ensemble::{
    heatbath_sweep, overrelaxation_sweep, run_chain, EnsembleConfig, SplitMix64,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = SplitMix64::new(0);
    let expected_rng = [
        0xe220_a839_7b1d_cdafu64,
        0x6e78_9e6a_a1b9_65f4,
        0x06c4_5d18_8009_454f,
        0xf88b_b8a8_724c_81ec,
        0x1b39_896a_51a8_749b,
    ];
    for expected in expected_rng {
        if rng.next_u64() != expected {
            return Err("SplitMix64 golden stream mismatch".into());
        }
    }

    let config = EnsembleConfig {
        extents: [2, 2, 2, 2],
        beta: 0.0,
        master_seed: 0x0123_4567_89ab_cdef,
        chain_id: 0,
        thermalization_sweeps: 1,
        measurements: 128,
        sweeps_between_measurements: 1,
        overrelaxation_sweeps_per_heatbath: 1,
        max_heatbath_attempts_per_link: 10_000,
        autocorrelation_max_lag: 16,
    };

    let report = run_chain(config)?;
    let replay = run_chain(config)?;
    if report.measurements != replay.measurements || report.derived_seed != replay.derived_seed {
        return Err("fixed Phase 2A chain did not replay deterministically".into());
    }
    if report.diagnostics.mean.abs() > 0.08 {
        return Err("beta=0 average plaquette is outside the analytic Haar sanity band".into());
    }
    if report.diagnostics.effective_sample_size < 16.0 {
        return Err("beta=0 effective sample size is unexpectedly small".into());
    }
    if !(0.5..=1.0).contains(&report.mean_heatbath_acceptance) {
        return Err("heat-bath scalar rejection acceptance is outside the sanity band".into());
    }
    if report.final_link_unit_residual > 1.0e-12 {
        return Err("ensemble generation violated the SU(2) unit-link invariant".into());
    }

    let mut independent_config = config;
    independent_config.chain_id = 1;
    let independent = run_chain(independent_config)?;
    if independent.derived_seed == report.derived_seed
        || independent.measurements == report.measurements
    {
        return Err("independent chain identity did not produce an independent stream".into());
    }

    let mut action_lattice = Lattice4::new_identity([2, 2, 2, 2])?;
    let mut action_rng = SplitMix64::new(0xfeed_face_cafe_beef);
    heatbath_sweep(&mut action_lattice, 2.0, &mut action_rng, 100_000)?;
    let action_before = action_lattice.wilson_action(2.0)?;
    overrelaxation_sweep(&mut action_lattice)?;
    let action_after = action_lattice.wilson_action(2.0)?;
    let overrelaxation_action_residual = (action_after - action_before).abs();
    if overrelaxation_action_residual > 2.0e-10 {
        return Err("over-relaxation failed the microcanonical action check".into());
    }

    println!("contract=YM-SU2-ENSEMBLE-V1");
    println!("rng=SPLITMIX64-V1");
    println!("derived_seed={:016x}", report.derived_seed);
    println!("beta0_mean_plaquette={:.17e}", report.diagnostics.mean);
    println!(
        "beta0_tau_int={:.17e}",
        report.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "beta0_effective_sample_size={:.17e}",
        report.diagnostics.effective_sample_size
    );
    println!(
        "heatbath_acceptance={:.17e}",
        report.mean_heatbath_acceptance
    );
    println!(
        "overrelaxation_action_residual={overrelaxation_action_residual:.17e}"
    );
    println!("final_link_unit_residual={:.17e}", report.final_link_unit_residual);
    println!("published_plaquette_baseline_reproduced=false");
    println!("su3_ensemble_enabled=false");
    println!("confinement_claimed=false");
    println!("continuum_mass_gap_claimed=false");
    println!("status=passed");
    Ok(())
}
