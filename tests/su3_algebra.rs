// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../reference/rust/src/su3.rs"]
mod su3;

use su3::{
    cubic_casimir_fundamental, gell_mann, quadratic_casimir_fundamental, structure_d,
    structure_f, Mat3, Su3, SU3_ACCEPTANCE_TOLERANCE, SU3_GENERATORS,
};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let residual = (actual - expected).abs();
    assert!(
        residual <= tolerance,
        "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

#[test]
fn public_canonical_su3_generator_surface() {
    for index in 0..SU3_GENERATORS {
        let lambda = gell_mann(index).unwrap();
        assert!(lambda.hermiticity_residual() <= 1.0e-15);
        assert!(lambda.trace().abs() <= 1.0e-15);
    }
    assert_close(structure_f(0, 1, 2).unwrap(), 1.0, 2.0e-15);
    assert_close(structure_d(0, 0, 7).unwrap(), 0.5773502691896258, 2.0e-15);
}

#[test]
fn public_su3_casimir_surface() {
    assert!(
        quadratic_casimir_fundamental()
            .max_residual(Mat3::IDENTITY.scale_real(4.0 / 3.0))
            <= 3.0e-15
    );
    assert!(
        cubic_casimir_fundamental()
            .max_residual(Mat3::IDENTITY.scale_real(10.0 / 9.0))
            <= 8.0e-15
    );
}

#[test]
fn public_bounded_exponential_surface_returns_accepted_su3() {
    let mut coefficients = [0.0; SU3_GENERATORS];
    coefficients[2] = 0.7;
    let u = Su3::from_algebra_coefficients(coefficients).unwrap();
    assert!(u.unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    assert!(u.determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
}
