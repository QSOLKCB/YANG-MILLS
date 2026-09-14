// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_effective_mass.rs"]
mod glueball_effective_mass;

use glueball_effective_mass::{
    analytic_periodic_single_state, cosh_effective_mass_with_jackknife, ANALYTIC_REPLAY_TOLERANCE,
    EFFECTIVE_MASS_CONTRACT_ID, EFFECTIVE_MASS_STATUS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time_extent = 16usize;
    let full_mass = 0.5_f64;
    let replica_masses = [0.48_f64, 0.49, 0.50, 0.51, 0.52];
    let lags = [1usize, 2, 3, 4, 5, 6, 7];

    let full = analytic_periodic_single_state(time_extent, full_mass, 1.0)?;
    let mut jackknife = Vec::new();
    jackknife.try_reserve_exact(replica_masses.len())?;
    for mass in replica_masses {
        jackknife.push(analytic_periodic_single_state(time_extent, mass, 1.0)?);
    }

    let result = cosh_effective_mass_with_jackknife(&full, &jackknife, &lags)?;
    let expected_standard_error = 0.0008_f64.sqrt();
    let mut maximum_replay_residual = 0.0_f64;
    for point in &result.points {
        maximum_replay_residual = maximum_replay_residual
            .max((point.full_estimate - full_mass).abs())
            .max((point.jackknife_mean - full_mass).abs())
            .max((point.standard_error - expected_standard_error).abs());
    }
    if !maximum_replay_residual.is_finite() || maximum_replay_residual > ANALYTIC_REPLAY_TOLERANCE {
        return Err("Phase 3D analytic effective-mass regression mismatch".into());
    }

    println!("contract={EFFECTIVE_MASS_CONTRACT_ID}");
    println!("status={EFFECTIVE_MASS_STATUS}");
    println!("canonical_fixture_is_physical_ensemble=false");
    println!("time_extent={}", result.time_extent);
    println!("jackknife_replicates={}", result.jackknife_replicates);
    println!("estimator=periodic-nearest-neighbour-cosh");
    println!("jackknife_scheme=delete-one-configuration-recompute-correlator");
    println!("full_fixture_mass={full_mass:.17e}");
    println!("expected_standard_error={expected_standard_error:.17e}");
    for point in &result.points {
        println!(
            "tau{}_effective_mass={:.17e}",
            point.tau, point.full_estimate
        );
        println!(
            "tau{}_jackknife_mean={:.17e}",
            point.tau, point.jackknife_mean
        );
        println!(
            "tau{}_standard_error={:.17e}",
            point.tau, point.standard_error
        );
    }
    println!("analytic_replay_residual={maximum_replay_residual:.17e}");
    println!("uncertainty_propagation_defined=true");
    println!("effective_mass_estimator_evaluated=true");
    println!("fit_window_selected=false");
    println!("spectral_fit_performed=false");
    println!("variational_gevp_solved=false");
    println!("glueball_mass_measured=false");
    println!("finite_volume_mass_gap_established=false");
    println!("confinement_established=false");
    println!("continuum_qft_constructed=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("clay_solution_claimed=false");
    println!("result=passed");
    Ok(())
}
