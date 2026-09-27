// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_correlators.rs"]
mod glueball_correlators;
#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../glueball_scalar_variational.rs"]
mod glueball_scalar_variational;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_correlators::canonical_shifted_defect_configurations;
use glueball_scalar_variational::{
    canonical_two_state_matrices, max_matrix_series_residual, principal_generalized_projection,
    scalar_variational_connected_correlator, CANONICAL_GROUND_MASS, CANONICAL_TIME_EXTENT,
    DIAGONALIZATION_TIME, GAUGE_INVARIANCE_TOLERANCE, REFERENCE_TIME, VARIATIONAL_CONTRACT_ID,
    VARIATIONAL_STATUS,
};
use su3_lattice::deterministic_su3_site_gauge;

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
    let fixture = canonical_two_state_matrices();
    let projection =
        principal_generalized_projection(&fixture, REFERENCE_TIME, DIAGONALIZATION_TIME)?;

    let configurations = canonical_shifted_defect_configurations()?;
    let before = scalar_variational_connected_correlator(&configurations)?;
    let transformed: Vec<_> = configurations
        .iter()
        .map(|configuration| configuration.gauge_transform(deterministic_su3_site_gauge))
        .collect::<Result<_, _>>()?;
    let after = scalar_variational_connected_correlator(&transformed)?;
    let gauge_residual = max_matrix_series_residual(&before.matrices, &after.matrices)?;
    if !gauge_residual.is_finite() || gauge_residual > GAUGE_INVARIANCE_TOLERANCE {
        return Err("Phase 3J gauge-invariance regression mismatch".into());
    }

    println!("contract={VARIATIONAL_CONTRACT_ID}");
    println!("status={VARIATIONAL_STATUS}");
    println!("evidence_class=deterministic-variational-mechanics");
    println!("basis_dimension=2");
    println!("basis=plaquette-1x1,rectangle-2x1");
    println!("connected_cross_correlator_matrix_constructed=true");
    println!("matrix_symmetrization=frozen-forward-reverse-average");
    println!("reference_time={REFERENCE_TIME}");
    println!("diagonalization_time={DIAGONALIZATION_TIME}");
    println!("reference_matrix_positive_definite_required=true");
    println!("generalized_eigenproblem_solved=true");
    println!(
        "principal_eigenvalue={:.17e}",
        projection.principal_eigenvalue
    );
    println!(
        "principal_coefficients={:.17e},{:.17e}",
        projection.coefficients[0], projection.coefficients[1]
    );
    print_series("projected_correlator", &projection.projected_correlator);
    println!("canonical_time_extent={CANONICAL_TIME_EXTENT}");
    println!("canonical_ground_mass={CANONICAL_GROUND_MASS:.17e}");
    println!("gauge_invariance_residual={gauge_residual:.17e}");
    println!("phase3i_outcome_assumed=false");
    println!("production_recovery_activated=false");
    println!("physical_glueball_mass_claimed=false");
    println!("finite_volume_mass_gap_established=false");
    println!("continuum_qft_constructed=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("clay_solution_claimed=false");
    println!("result=passed");
    Ok(())
}
