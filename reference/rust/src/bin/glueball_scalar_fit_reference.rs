// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_effective_mass.rs"]
mod glueball_effective_mass;
#[path = "../glueball_scalar_fit.rs"]
mod glueball_scalar_fit;

use glueball_scalar_fit::{
    canonical_scalar_fit_fixture, correlated_scalar_fit, ANALYTIC_REPLAY_TOLERANCE,
    SCALAR_FIT_CONTRACT_ID, SCALAR_FIT_STATUS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (full, jackknife, candidates) = canonical_scalar_fit_fixture()?;
    let result = correlated_scalar_fit(&full, &jackknife, &candidates)?;

    if result.selected_window.start != 3 || result.selected_window.end != 6 {
        return Err("Phase 3E canonical window-selection regression mismatch".into());
    }
    let maximum_replay_residual = (result.plateau_weighted_effective_mass - 0.5)
        .abs()
        .max((result.fitted_mass - 0.5).abs())
        .max((result.fitted_amplitude - 1.0).abs())
        .max(result.chi_square.abs())
        .max((result.mass_jackknife_mean - 0.5).abs())
        .max((result.mass_standard_error - 0.005916079783099616).abs())
        .max((result.maximum_off_diagonal_correlation - 1.0 / 17.0).abs());
    if !maximum_replay_residual.is_finite()
        || maximum_replay_residual > ANALYTIC_REPLAY_TOLERANCE
    {
        return Err("Phase 3E analytic correlated-fit regression mismatch".into());
    }

    println!("contract={SCALAR_FIT_CONTRACT_ID}");
    println!("status={SCALAR_FIT_STATUS}");
    println!("canonical_fixture_is_physical_ensemble=false");
    println!("time_extent={}", result.time_extent);
    println!("jackknife_replicates={}", result.jackknife_replicates);
    println!("candidate_window_count={}", candidates.len());
    println!("selected_window_start={}", result.selected_window.start);
    println!("selected_window_end={}", result.selected_window.end);
    println!(
        "plateau_weighted_effective_mass={:.17e}",
        result.plateau_weighted_effective_mass
    );
    println!("plateau_maximum_z={:.17e}", result.plateau_maximum_z);
    println!("fitted_mass={:.17e}", result.fitted_mass);
    println!("fitted_amplitude={:.17e}", result.fitted_amplitude);
    println!("chi_square={:.17e}", result.chi_square);
    println!("degrees_of_freedom={}", result.degrees_of_freedom);
    println!("chi_square_per_dof={:.17e}", result.chi_square_per_dof);
    println!(
        "mass_jackknife_mean={:.17e}",
        result.mass_jackknife_mean
    );
    println!("mass_standard_error={:.17e}", result.mass_standard_error);
    println!(
        "amplitude_jackknife_mean={:.17e}",
        result.amplitude_jackknife_mean
    );
    println!(
        "amplitude_standard_error={:.17e}",
        result.amplitude_standard_error
    );
    println!(
        "maximum_off_diagonal_correlation={:.17e}",
        result.maximum_off_diagonal_correlation
    );
    println!("analytic_replay_residual={maximum_replay_residual:.17e}");
    println!("fit_window_selected=true");
    println!("full_jackknife_covariance_used=true");
    println!("diagonal_covariance_fallback=false");
    println!("correlated_single_state_fit_performed=true");
    println!("production_ensemble_analyzed=false");
    println!("physical_glueball_mass_reported=false");
    println!("scalar_pipeline_fully_validated=false");
    println!("variational_gevp_solved=false");
    println!("finite_volume_mass_gap_established=false");
    println!("confinement_established=false");
    println!("continuum_qft_constructed=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("clay_solution_claimed=false");
    println!("result=passed");
    Ok(())
}
