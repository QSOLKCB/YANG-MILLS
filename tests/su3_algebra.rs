// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../reference/rust/src/su3.rs"]
mod su3;

use su3::{
    cubic_casimir_fundamental, gell_mann, generator, quadratic_casimir_fundamental, structure_d,
    structure_f, Complex64, Mat3, Su3, Su3Error, SU3_ACCEPTANCE_TOLERANCE, SU3_EXP_MAX_COEFFICIENT,
    SU3_GENERATORS,
};

const INV_SQRT_3: f64 = 0.5773502691896258;

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let residual = (actual - expected).abs();
    assert!(
        residual <= tolerance,
        "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

fn assert_matrix_close(actual: Mat3, expected: Mat3, tolerance: f64) {
    let residual = actual.max_residual(expected);
    assert!(
        residual <= tolerance,
        "matrix residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

#[test]
fn canonical_gell_mann_basis_is_hermitian_traceless_and_orthogonal() {
    for a in 0..SU3_GENERATORS {
        let lambda_a = gell_mann(a).unwrap();
        assert!(lambda_a.hermiticity_residual() <= 1.0e-15);
        assert!(lambda_a.trace().abs() <= 1.0e-15);
        for b in 0..SU3_GENERATORS {
            let trace = lambda_a.mul(gell_mann(b).unwrap()).trace();
            let expected = if a == b { 2.0 } else { 0.0 };
            assert_close(trace.re, expected, 2.0e-15);
            assert_close(trace.im, 0.0, 2.0e-15);
        }
    }
}

#[test]
fn canonical_f_structure_constants_match_golden_values() {
    let golden = [
        ((0, 1, 2), 1.0),
        ((0, 3, 6), 0.5),
        ((0, 4, 5), -0.5),
        ((1, 3, 5), 0.5),
        ((1, 4, 6), 0.5),
        ((2, 3, 4), 0.5),
        ((2, 5, 6), -0.5),
        ((3, 4, 7), 0.8660254037844386),
        ((5, 6, 7), 0.8660254037844386),
    ];
    for ((a, b, c), expected) in golden {
        assert_close(structure_f(a, b, c).unwrap(), expected, 2.0e-15);
    }
}

#[test]
fn canonical_d_structure_constants_match_golden_values() {
    let golden = [
        ((0, 0, 7), INV_SQRT_3),
        ((7, 7, 7), -INV_SQRT_3),
        ((0, 3, 5), 0.5),
        ((0, 4, 6), 0.5),
        ((1, 3, 6), -0.5),
        ((2, 3, 3), 0.5),
        ((3, 3, 7), -0.5 * INV_SQRT_3),
    ];
    for ((a, b, c), expected) in golden {
        assert_close(structure_d(a, b, c).unwrap(), expected, 2.0e-15);
    }
}

#[test]
fn commutator_and_anticommutator_reconstruct_from_f_and_d() {
    for a in 0..SU3_GENERATORS {
        let ta = generator(a).unwrap();
        for b in 0..SU3_GENERATORS {
            let tb = generator(b).unwrap();

            let commutator = ta.mul(tb).sub(tb.mul(ta));
            let mut expected_commutator = Mat3::ZERO;
            for c in 0..SU3_GENERATORS {
                let f = structure_f(a, b, c).unwrap();
                expected_commutator = expected_commutator
                    .add(generator(c).unwrap().scale_complex(Complex64::new(0.0, f)));
            }
            assert_matrix_close(commutator, expected_commutator, 3.0e-15);

            let anticommutator = ta.mul(tb).add(tb.mul(ta));
            let mut expected_anticommutator = if a == b {
                Mat3::IDENTITY.scale_real(1.0 / 3.0)
            } else {
                Mat3::ZERO
            };
            for c in 0..SU3_GENERATORS {
                let d = structure_d(a, b, c).unwrap();
                expected_anticommutator =
                    expected_anticommutator.add(generator(c).unwrap().scale_real(d));
            }
            assert_matrix_close(anticommutator, expected_anticommutator, 3.0e-15);
        }
    }
}

#[test]
fn fundamental_quadratic_and_cubic_casimirs_match_regressions() {
    assert_matrix_close(
        quadratic_casimir_fundamental(),
        Mat3::IDENTITY.scale_real(4.0 / 3.0),
        3.0e-15,
    );
    assert_matrix_close(
        cubic_casimir_fundamental(),
        Mat3::IDENTITY.scale_real(10.0 / 9.0),
        8.0e-15,
    );
}

#[test]
fn bounded_exponentiation_maps_traceless_hermitian_algebra_to_su3() {
    let theta = 0.7_f64;
    let mut coefficients = [0.0; SU3_GENERATORS];
    coefficients[2] = theta;
    let u = Su3::from_algebra_coefficients(coefficients).unwrap();
    assert!(u.unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    assert!(u.determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);

    let half = 0.5 * theta;
    let expected = Mat3::from_rows([
        [
            Complex64::new(half.cos(), half.sin()),
            Complex64::ZERO,
            Complex64::ZERO,
        ],
        [
            Complex64::ZERO,
            Complex64::new(half.cos(), -half.sin()),
            Complex64::ZERO,
        ],
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
    ]);
    assert_matrix_close(u.matrix(), expected, 2.0e-14);
}

#[test]
fn dagger_preserves_the_checked_su3_invariant() {
    let u =
        Su3::from_algebra_coefficients([10.0, 20.0, -5.0, 3.0, 8.0, -9.0, 2.0, 7.0]).unwrap();
    let dagger = u.dagger();
    assert!(dagger.unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    assert!(dagger.determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    assert_matrix_close(
        dagger.dagger().matrix(),
        u.matrix(),
        SU3_ACCEPTANCE_TOLERANCE,
    );
}

#[test]
fn general_bounded_exponentiation_and_group_product_pass_acceptance() {
    let a = Su3::from_algebra_coefficients([10.0, 20.0, -5.0, 3.0, 8.0, -9.0, 2.0, 7.0]).unwrap();
    let b = Su3::from_algebra_coefficients([-4.0, 1.0, 6.0, 2.0, -3.0, 5.0, 8.0, -7.0]).unwrap();
    let product = a.mul(b).unwrap();
    assert!(product.unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    assert!(product.determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    let identity = product.mul(product.dagger()).unwrap();
    assert_matrix_close(identity.matrix(), Mat3::IDENTITY, SU3_ACCEPTANCE_TOLERANCE);
}

#[test]
fn invalid_generator_and_exponentiation_inputs_fail_closed() {
    assert_eq!(
        gell_mann(8).unwrap_err(),
        Su3Error::GeneratorIndexOutOfRange
    );

    let mut non_finite = [0.0; SU3_GENERATORS];
    non_finite[0] = f64::NAN;
    assert_eq!(
        Su3::from_algebra_coefficients(non_finite).unwrap_err(),
        Su3Error::NonFiniteCoefficient
    );

    let mut too_large = [0.0; SU3_GENERATORS];
    too_large[0] = SU3_EXP_MAX_COEFFICIENT + 1.0;
    assert_eq!(
        Su3::from_algebra_coefficients(too_large).unwrap_err(),
        Su3Error::AlgebraCoefficientTooLarge
    );

    let bad = Mat3::from_rows([
        [Complex64::new(2.0, 0.0), Complex64::ZERO, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ONE, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
    ]);
    assert_eq!(
        Su3::from_checked_matrix(bad).unwrap_err(),
        Su3Error::NonUnitaryMatrix
    );
}
