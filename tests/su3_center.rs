// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../reference/rust/src/su3.rs"]
mod su3;
#[path = "../reference/rust/src/su3_lattice.rs"]
mod su3_lattice;
#[path = "../reference/rust/src/su3_center.rs"]
mod su3_center;

use su3::{Complex64, Mat3, Su3, SU3_ACCEPTANCE_TOLERANCE};
use su3_center::{
    apply_center_transform, center_commutator_residual, center_element, center_phase,
    cycle_generator, cycle_noncentral_residual, C3, Su3CenterError,
};
use su3_lattice::{single_traceless_su3_link_lattice, Su3Lattice4};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let residual = (actual - expected).abs();
    assert!(
        residual <= tolerance,
        "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

fn assert_complex_close(actual: Complex64, expected: Complex64, tolerance: f64) {
    assert_close(actual.re, expected.re, tolerance);
    assert_close(actual.im, expected.im, tolerance);
}

#[test]
fn abstract_c3_has_order_three_and_modular_composition() {
    let g = C3::GENERATOR;
    assert_eq!(g.pow(0), C3::IDENTITY);
    assert_eq!(g.pow(1), C3::new(1));
    assert_eq!(g.pow(2), C3::new(2));
    assert_eq!(g.pow(3), C3::IDENTITY);
    assert_eq!(g.compose(g), C3::new(2));
    assert_eq!(g.compose(g).compose(g), C3::IDENTITY);
    assert_eq!(g.compose(g.inverse()), C3::IDENTITY);
    assert_eq!(C3::new(255), C3::new(0));
}

#[test]
fn cycle_embedding_has_order_three_but_is_not_central() {
    let omega = cycle_generator().unwrap();
    let omega2 = omega.mul(omega).unwrap();
    let omega3 = omega2.mul(omega).unwrap();
    assert_eq!(omega3.matrix(), Mat3::IDENTITY);
    assert_eq!(omega.unitarity_residual(), 0.0);
    assert_eq!(omega.determinant_residual(), 0.0);
    assert_eq!(cycle_noncentral_residual().unwrap(), 2.0);
}

#[test]
fn genuine_center_is_c3_and_commutes_with_tested_su3_elements() {
    let test_values = [
        Su3::IDENTITY,
        Su3::from_algebra_coefficients([0.3, -0.2, 0.1, 0.05, -0.07, 0.11, -0.09, 0.13])
            .unwrap(),
        Su3::from_algebra_coefficients([1.0, 0.5, -0.25, 0.125, 0.0, -0.2, 0.3, -0.4])
            .unwrap(),
    ];

    for k in 0..3 {
        let element = C3::new(k);
        let z = center_element(element).unwrap();
        assert!(z.unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
        assert!(z.determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
        for value in test_values {
            assert!(center_commutator_residual(element, value).unwrap() <= 2.0e-15);
        }

        for l in 0..3 {
            let lhs = z.mul(center_element(C3::new(l)).unwrap()).unwrap();
            let rhs = center_element(element.compose(C3::new(l))).unwrap();
            assert!(lhs.matrix().max_residual(rhs.matrix()) <= 5.0e-15);
        }
    }
}

#[test]
fn noncentral_cycle_embedding_is_distinct_from_the_center_embedding() {
    let omega = cycle_generator().unwrap().matrix();
    for k in 0..3 {
        let center = center_element(C3::new(k)).unwrap().matrix();
        assert!(omega.max_residual(center) >= 1.0);
    }
    assert_eq!(cycle_noncentral_residual().unwrap(), 2.0);
    assert!(center_commutator_residual(C3::GENERATOR, su3_center::noncentral_witness().unwrap())
        .unwrap()
        <= 2.0e-15);
}

#[test]
fn center_slice_transform_rotates_polyakov_loop_and_preserves_action() {
    let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    let start = [2, 1, 2, 1];
    let direction = 0;
    let before = lattice.polyakov_loop(start, direction).unwrap();
    assert_complex_close(before, Complex64::ONE, 0.0);

    let element = C3::GENERATOR;
    let transformed = apply_center_transform(&lattice, direction, 0, element).unwrap();
    let after = transformed.polyakov_loop(start, direction).unwrap();
    assert_complex_close(after, center_phase(element).mul(before), 2.0e-15);
    assert_close(lattice.wilson_action(2.0).unwrap(), 0.0, 0.0);
    assert_close(transformed.wilson_action(2.0).unwrap(), 0.0, 2.0e-12);
}

#[test]
fn center_transform_preserves_nontrivial_finite_lattice_action() {
    let lattice = single_traceless_su3_link_lattice().unwrap();
    let transformed = apply_center_transform(&lattice, 0, 1, C3::new(2)).unwrap();
    let before = lattice.wilson_action(2.0).unwrap();
    let after = transformed.wilson_action(2.0).unwrap();
    assert_close(before, 12.0, 1.0e-12);
    assert_close(after, before, 1.0e-10);
}

#[test]
fn center_transform_and_polyakov_inputs_fail_closed() {
    let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    assert_eq!(
        apply_center_transform(&lattice, 4, 0, C3::GENERATOR).unwrap_err(),
        Su3CenterError::DirectionOutOfRange
    );
    assert_eq!(
        apply_center_transform(&lattice, 0, 3, C3::GENERATOR).unwrap_err(),
        Su3CenterError::SliceOutOfRange
    );
    assert!(lattice.polyakov_loop([0, 0, 0, 0], 4).is_err());
    assert!(lattice.polyakov_loop([3, 0, 0, 0], 0).is_err());
}
