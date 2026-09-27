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
    scalar_variational_connected_correlator, PrincipalProjection, CANONICAL_EXCITED_AMPLITUDE,
    CANONICAL_EXCITED_COUPLING, CANONICAL_EXCITED_MASS, CANONICAL_GROUND_AMPLITUDE,
    CANONICAL_GROUND_COUPLING, CANONICAL_GROUND_MASS, CANONICAL_TIME_EXTENT, DIAGONALIZATION_TIME,
    GAUGE_INVARIANCE_TOLERANCE, GOLDEN_CANONICAL_PRINCIPAL_EIGENVALUE, REFERENCE_TIME,
    VARIATIONAL_CONTRACT_ID, VARIATIONAL_STATUS,
};
use su3_lattice::deterministic_su3_site_gauge;

const FROZEN_CANONICAL_TIME_EXTENT: usize = 16;
const FROZEN_CANONICAL_GROUND_MASS: f64 = 0.4;
const FROZEN_CANONICAL_EXCITED_MASS: f64 = 0.9;
const FROZEN_CANONICAL_GROUND_AMPLITUDE: f64 = 1.2;
const FROZEN_CANONICAL_EXCITED_AMPLITUDE: f64 = 0.8;
const FROZEN_CANONICAL_GROUND_COUPLING: [f64; 2] = [1.0, 0.5];
const FROZEN_CANONICAL_EXCITED_COUPLING: [f64; 2] = [0.25, 1.0];
const FROZEN_REFERENCE_TIME: usize = 1;
const FROZEN_DIAGONALIZATION_TIME: usize = 2;
const FROZEN_PRINCIPAL_EIGENVALUE: f64 = 0.673_346_666_250_657;
const FROZEN_PRINCIPAL_EIGENVALUE_TOLERANCE: f64 = 2.0e-15;

#[derive(Clone, Copy)]
struct FrozenFixtureSnapshot {
    time_extent: usize,
    ground_mass: f64,
    excited_mass: f64,
    ground_amplitude: f64,
    excited_amplitude: f64,
    ground_coupling: [f64; 2],
    excited_coupling: [f64; 2],
    reference_time: usize,
    diagonalization_time: usize,
    principal_eigenvalue: f64,
}

fn validate_frozen_fixture_snapshot(snapshot: FrozenFixtureSnapshot) -> Result<(), &'static str> {
    if snapshot.time_extent != FROZEN_CANONICAL_TIME_EXTENT
        || snapshot.ground_mass.to_bits() != FROZEN_CANONICAL_GROUND_MASS.to_bits()
        || snapshot.excited_mass.to_bits() != FROZEN_CANONICAL_EXCITED_MASS.to_bits()
        || snapshot.ground_amplitude.to_bits() != FROZEN_CANONICAL_GROUND_AMPLITUDE.to_bits()
        || snapshot.excited_amplitude.to_bits() != FROZEN_CANONICAL_EXCITED_AMPLITUDE.to_bits()
        || snapshot.ground_coupling != FROZEN_CANONICAL_GROUND_COUPLING
        || snapshot.excited_coupling != FROZEN_CANONICAL_EXCITED_COUPLING
        || snapshot.reference_time != FROZEN_REFERENCE_TIME
        || snapshot.diagonalization_time != FROZEN_DIAGONALIZATION_TIME
    {
        return Err("Phase 3J frozen canonical fixture identity drifted");
    }

    let residual = (snapshot.principal_eigenvalue - FROZEN_PRINCIPAL_EIGENVALUE).abs();
    if !residual.is_finite() || residual > FROZEN_PRINCIPAL_EIGENVALUE_TOLERANCE {
        return Err("Phase 3J frozen canonical principal eigenvalue drifted");
    }
    Ok(())
}

fn validate_frozen_canonical_fixture(projection: &PrincipalProjection) -> Result<(), &'static str> {
    if GOLDEN_CANONICAL_PRINCIPAL_EIGENVALUE.to_bits() != FROZEN_PRINCIPAL_EIGENVALUE.to_bits() {
        return Err("Phase 3J source golden eigenvalue drifted");
    }

    validate_frozen_fixture_snapshot(FrozenFixtureSnapshot {
        time_extent: CANONICAL_TIME_EXTENT,
        ground_mass: CANONICAL_GROUND_MASS,
        excited_mass: CANONICAL_EXCITED_MASS,
        ground_amplitude: CANONICAL_GROUND_AMPLITUDE,
        excited_amplitude: CANONICAL_EXCITED_AMPLITUDE,
        ground_coupling: CANONICAL_GROUND_COUPLING,
        excited_coupling: CANONICAL_EXCITED_COUPLING,
        reference_time: REFERENCE_TIME,
        diagonalization_time: DIAGONALIZATION_TIME,
        principal_eigenvalue: projection.principal_eigenvalue,
    })
}

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
    validate_frozen_canonical_fixture(&projection)?;

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
    println!("canonical_excited_mass={CANONICAL_EXCITED_MASS:.17e}");
    println!("canonical_ground_amplitude={CANONICAL_GROUND_AMPLITUDE:.17e}");
    println!("canonical_excited_amplitude={CANONICAL_EXCITED_AMPLITUDE:.17e}");
    println!(
        "canonical_ground_coupling={:.17e},{:.17e}",
        CANONICAL_GROUND_COUPLING[0], CANONICAL_GROUND_COUPLING[1]
    );
    println!(
        "canonical_excited_coupling={:.17e},{:.17e}",
        CANONICAL_EXCITED_COUPLING[0], CANONICAL_EXCITED_COUPLING[1]
    );
    println!("canonical_fixture_golden_matches=true");
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

#[cfg(test)]
mod receipt_tests {
    use super::*;

    fn golden_snapshot() -> FrozenFixtureSnapshot {
        FrozenFixtureSnapshot {
            time_extent: 16,
            ground_mass: 0.4,
            excited_mass: 0.9,
            ground_amplitude: 1.2,
            excited_amplitude: 0.8,
            ground_coupling: [1.0, 0.5],
            excited_coupling: [0.25, 1.0],
            reference_time: 1,
            diagonalization_time: 2,
            principal_eigenvalue: 0.673_346_666_250_657,
        }
    }

    #[test]
    fn frozen_receipt_accepts_literal_canonical_fixture() {
        assert_eq!(validate_frozen_fixture_snapshot(golden_snapshot()), Ok(()));
    }

    #[test]
    fn frozen_receipt_rejects_ground_mass_mutation() {
        let mut snapshot = golden_snapshot();
        snapshot.ground_mass = 0.5;
        assert_eq!(
            validate_frozen_fixture_snapshot(snapshot),
            Err("Phase 3J frozen canonical fixture identity drifted")
        );
    }

    #[test]
    fn frozen_receipt_rejects_principal_eigenvalue_mutation() {
        let mut snapshot = golden_snapshot();
        snapshot.principal_eigenvalue = 0.607_480_148_713_566_9;
        assert_eq!(
            validate_frozen_fixture_snapshot(snapshot),
            Err("Phase 3J frozen canonical principal eigenvalue drifted")
        );
    }

    #[test]
    fn executable_gate_matches_contract_literals() {
        let fixture = canonical_two_state_matrices();
        let projection =
            principal_generalized_projection(&fixture, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_eq!(validate_frozen_canonical_fixture(&projection), Ok(()));
    }
}
