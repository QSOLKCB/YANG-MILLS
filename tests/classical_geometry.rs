// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../reference/rust/src/classical_geometry.rs"]
mod classical_geometry;

use classical_geometry::{ClassicalGeometryError, TwoForm4};

#[test]
fn hodge_star_basis_signs_match_the_frozen_orientation() {
    let basis = [
        TwoForm4::from_components([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap(),
        TwoForm4::from_components([0.0, 1.0, 0.0, 0.0, 0.0, 0.0]).unwrap(),
        TwoForm4::from_components([0.0, 0.0, 1.0, 0.0, 0.0, 0.0]).unwrap(),
        TwoForm4::from_components([0.0, 0.0, 0.0, 1.0, 0.0, 0.0]).unwrap(),
        TwoForm4::from_components([0.0, 0.0, 0.0, 0.0, 1.0, 0.0]).unwrap(),
        TwoForm4::from_components([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]).unwrap(),
    ];
    let expected = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 0.0, -1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];

    for (form, expected_components) in basis.into_iter().zip(expected) {
        assert_eq!(form.hodge_star().components(), expected_components);
    }
}

#[test]
fn hodge_star_squares_to_identity_on_two_forms() {
    let form = TwoForm4::from_components([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    assert_eq!(form.hodge_star().hodge_star(), form);
}

#[test]
fn wedge_star_matches_the_euclidean_two_form_inner_product() {
    let alpha = TwoForm4::from_components([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    let beta = TwoForm4::from_components([6.0, -5.0, 4.0, -3.0, 2.0, -1.0]).unwrap();
    assert_eq!(
        alpha.wedge_volume_coefficient(beta.hodge_star()),
        alpha.inner_product(beta)
    );
}

#[test]
fn self_dual_and_anti_self_dual_parts_match_the_frozen_vectors() {
    let form = TwoForm4::from_components([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    let plus = form.self_dual_part().unwrap();
    let minus = form.anti_self_dual_part().unwrap();

    assert_eq!(plus.components(), [3.5, -1.5, 3.5, 3.5, 1.5, 3.5]);
    assert_eq!(minus.components(), [-2.5, 3.5, -0.5, 0.5, 3.5, 2.5]);
    assert_eq!(plus.hodge_star(), plus);
    assert_eq!(minus.hodge_star(), minus.scale(-1.0).unwrap());
    assert_eq!(plus.add(minus).unwrap(), form);
    assert_eq!(plus.inner_product(minus), 0.0);
}

#[test]
fn non_finite_two_form_inputs_fail_closed() {
    assert_eq!(
        TwoForm4::from_components([f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap_err(),
        ClassicalGeometryError::NonFiniteComponent
    );
    assert_eq!(
        TwoForm4::ZERO.scale(f64::INFINITY).unwrap_err(),
        ClassicalGeometryError::NonFiniteComponent
    );
}

#[test]
fn two_form_addition_and_subtraction_overflow_fail_closed() {
    let positive = TwoForm4::from_components([f64::MAX, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let negative = TwoForm4::from_components([-f64::MAX, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();

    assert_eq!(
        positive.add(positive).unwrap_err(),
        ClassicalGeometryError::NonFiniteComponent
    );
    assert_eq!(
        positive.sub(negative).unwrap_err(),
        ClassicalGeometryError::NonFiniteComponent
    );
}
