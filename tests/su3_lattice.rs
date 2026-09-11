// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cell::Cell;

#[path = "../reference/rust/src/su3.rs"]
mod su3;
#[path = "../reference/rust/src/su3_lattice.rs"]
mod su3_lattice;

use su3::Su3;
use su3_lattice::{
    deterministic_su3_site_gauge, reference_acceptance_tolerance,
    single_traceless_su3_link_lattice, traceless_cycle_link, Su3Lattice4, Su3LatticeError,
    MAX_SU3_REFERENCE_LINKS,
};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let residual = (actual - expected).abs();
    assert!(
        residual <= tolerance,
        "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

#[test]
fn identity_lattice_has_exact_zero_action() {
    let lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    assert_eq!(lattice.wilson_action(2.0).unwrap(), 0.0);
    assert_eq!(
        lattice.plaquette([0, 0, 0, 0], 0, 1).unwrap(),
        Su3::IDENTITY
    );
}

#[test]
fn traceless_cycle_link_is_exact_su3_fixture() {
    let link = traceless_cycle_link().unwrap();
    assert_eq!(link.matrix().trace().re, 0.0);
    assert_eq!(link.matrix().trace().im, 0.0);
    assert_eq!(link.determinant().re, 1.0);
    assert_eq!(link.determinant().im, 0.0);
    assert_eq!(link.unitarity_residual(), 0.0);
    assert_eq!(link.determinant_residual(), 0.0);
}

#[test]
fn single_traceless_cycle_link_has_analytic_action() {
    let beta = 2.0;
    let lattice = single_traceless_su3_link_lattice().unwrap();
    assert_close(lattice.wilson_action(beta).unwrap(), 6.0 * beta, 1.0e-12);
}

#[test]
fn local_gauge_transform_preserves_wilson_action() {
    let beta = 1.7;
    let lattice = single_traceless_su3_link_lattice().unwrap();
    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge).unwrap();
    let residual =
        (lattice.wilson_action(beta).unwrap() - transformed.wilson_action(beta).unwrap()).abs();
    assert!(residual <= 1.0e-10, "gauge-action residual={residual:.17e}");
    assert!(transformed.max_link_unitarity_residual() <= reference_acceptance_tolerance());
    assert!(transformed.max_link_determinant_residual() <= reference_acceptance_tolerance());
}

#[test]
fn gauge_transform_samples_each_site_field_exactly_once() {
    let lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let calls = Cell::new(0usize);
    let transformed = lattice
        .gauge_transform(|x| {
            let call = calls.get();
            calls.set(call + 1);
            let code = x[0] + 2 * x[1] + 3 * x[2] + 5 * x[3];
            let mut coefficients = [0.0; 8];
            coefficients[2] = 0.017 * (call as f64 + 1.0) + 0.011 * code as f64;
            Su3::from_algebra_coefficients(coefficients).unwrap()
        })
        .unwrap();
    assert_eq!(calls.get(), lattice.site_count());
    assert_close(transformed.wilson_action(2.0).unwrap(), 0.0, 1.0e-10);
}

#[test]
fn lattice_bounds_and_invalid_inputs_fail_closed() {
    assert_eq!(
        Su3Lattice4::new_identity([1, 2, 2, 2]).unwrap_err(),
        Su3LatticeError::InvalidLatticeExtent
    );
    assert_eq!(
        Su3Lattice4::new_identity([usize::MAX, 2, 2, 2]).unwrap_err(),
        Su3LatticeError::LatticeSizeOverflow
    );

    let too_many_sites = MAX_SU3_REFERENCE_LINKS / 4 + 1;
    assert_eq!(
        Su3Lattice4::new_identity([too_many_sites, 2, 2, 2]).unwrap_err(),
        Su3LatticeError::LatticeTooLarge
    );

    let lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    assert_eq!(
        lattice.link([2, 0, 0, 0], 0).unwrap_err(),
        Su3LatticeError::CoordinateOutOfRange
    );
    assert_eq!(
        lattice.plaquette([0, 0, 0, 0], 1, 1).unwrap_err(),
        Su3LatticeError::DirectionsMustDiffer
    );
    assert_eq!(
        lattice.wilson_action(f64::NAN).unwrap_err(),
        Su3LatticeError::NonFiniteBeta
    );
    assert_eq!(
        lattice.wilson_action(-1.0).unwrap_err(),
        Su3LatticeError::NegativeBeta
    );
}

#[test]
fn non_finite_action_result_fails_closed() {
    let lattice = single_traceless_su3_link_lattice().unwrap();
    assert_eq!(
        lattice.wilson_action(f64::MAX).unwrap_err(),
        Su3LatticeError::NonFiniteAction
    );
}
