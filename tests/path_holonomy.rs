// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../reference/rust/src/su3.rs"]
mod su3;
#[path = "../reference/rust/src/su3_lattice.rs"]
mod su3_lattice;
#[path = "../reference/rust/src/path_holonomy.rs"]
mod path_holonomy;

use path_holonomy::{
    closed_path_holonomy, concatenate_paths, endpoint_covariance_prediction, path_holonomy,
    rectangular_path, reverse_path, wilson_loop, OrientedStep, PathHolonomyError,
};
use su3::{Complex64, Mat3, Su3};
use su3_lattice::{deterministic_su3_site_gauge, Su3Lattice4, Su3LatticeError};
use yang_mills_reference::{Lattice4 as Su2Lattice4, PathStep as Su2PathStep, Su2};

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

fn noncommuting_test_lattice() -> Su3Lattice4 {
    let mut lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    let a = Su3::from_algebra_coefficients([0.4, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.1]).unwrap();
    let b = Su3::from_algebra_coefficients([0.0, -0.3, 0.0, 0.25, 0.0, 0.0, 0.1, -0.05]).unwrap();
    lattice.set_link([0, 0, 0, 0], 0, a).unwrap();
    lattice.set_link([1, 0, 0, 0], 1, b).unwrap();
    lattice
}

#[test]
fn path_reversal_returns_inverse_transport() {
    let lattice = noncommuting_test_lattice();
    let start = [0, 0, 0, 0];
    let steps = [OrientedStep::Forward(0), OrientedStep::Forward(1)];
    let (forward, end) = path_holonomy(&lattice, start, &steps).unwrap();
    let reversed = reverse_path(&steps).unwrap();
    let (backward, returned) = path_holonomy(&lattice, end, &reversed).unwrap();
    assert_eq!(returned, start);
    assert!(backward.matrix().max_residual(forward.dagger().matrix()) <= 5.0e-12);
    assert!(forward.mul(backward).unwrap().matrix().max_residual(Mat3::IDENTITY) <= 5.0e-12);
}

#[test]
fn concatenation_uses_frozen_left_to_right_traversal_order() {
    let lattice = noncommuting_test_lattice();
    let start = [0, 0, 0, 0];
    let first = [OrientedStep::Forward(0)];
    let second = [OrientedStep::Forward(1)];
    let (u1, midpoint) = path_holonomy(&lattice, start, &first).unwrap();
    let (u2, end) = path_holonomy(&lattice, midpoint, &second).unwrap();
    let combined = concatenate_paths(&first, &second).unwrap();
    let (total, combined_end) = path_holonomy(&lattice, start, &combined).unwrap();
    assert_eq!(combined_end, end);
    let expected = u1.mul(u2).unwrap();
    assert!(total.matrix().max_residual(expected.matrix()) <= 5.0e-12);
}

#[test]
fn open_transport_obeys_endpoint_gauge_covariance() {
    let lattice = noncommuting_test_lattice();
    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge).unwrap();
    let start = [0, 0, 0, 0];
    let steps = [
        OrientedStep::Forward(0),
        OrientedStep::Forward(1),
        OrientedStep::Forward(2),
    ];
    let (transport, end) = path_holonomy(&lattice, start, &steps).unwrap();
    let (transformed_transport, transformed_end) =
        path_holonomy(&transformed, start, &steps).unwrap();
    assert_eq!(transformed_end, end);
    let predicted = endpoint_covariance_prediction(
        deterministic_su3_site_gauge(start),
        transport,
        deterministic_su3_site_gauge(end),
    )
    .unwrap();
    assert!(
        transformed_transport
            .matrix()
            .max_residual(predicted.matrix())
            <= 2.0e-11
    );
}

#[test]
fn arbitrary_closed_wilson_loop_is_gauge_invariant_and_cyclic() {
    let lattice = noncommuting_test_lattice();
    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge).unwrap();
    let start = [0, 0, 0, 0];
    let steps = [
        OrientedStep::Forward(0),
        OrientedStep::Forward(1),
        OrientedStep::Backward(0),
        OrientedStep::Backward(1),
    ];
    let original = wilson_loop(&lattice, start, &steps).unwrap();
    let gauged = wilson_loop(&transformed, start, &steps).unwrap();
    assert_complex_close(gauged, original, 2.0e-11);

    let shifted_start = [1, 0, 0, 0];
    let rotated = [
        OrientedStep::Forward(1),
        OrientedStep::Backward(0),
        OrientedStep::Backward(1),
        OrientedStep::Forward(0),
    ];
    let cyclic = wilson_loop(&lattice, shifted_start, &rotated).unwrap();
    assert_complex_close(cyclic, original, 2.0e-14);
}

#[test]
fn nonplanar_closed_path_has_deterministic_ordered_product() {
    let mut lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    let cycle = Su3::from_checked_matrix(Mat3::from_rows([
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
        [Complex64::ONE, Complex64::ZERO, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ONE, Complex64::ZERO],
    ]))
    .unwrap();
    lattice.set_link([0, 0, 0, 0], 0, cycle).unwrap();
    let chair = [
        OrientedStep::Forward(0),
        OrientedStep::Forward(1),
        OrientedStep::Forward(2),
        OrientedStep::Backward(0),
        OrientedStep::Backward(1),
        OrientedStep::Backward(2),
    ];
    let holonomy = closed_path_holonomy(&lattice, [0, 0, 0, 0], &chair).unwrap();
    assert!(holonomy.matrix().max_residual(cycle.matrix()) <= 5.0e-12);
    assert_complex_close(wilson_loop(&lattice, [0, 0, 0, 0], &chair).unwrap(), Complex64::ZERO, 0.0);
}

#[test]
fn polyakov_trace_is_invariant_under_periodic_local_gauge_transform() {
    let lattice = noncommuting_test_lattice();
    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge).unwrap();
    let start = [0, 0, 0, 0];
    let before = lattice.polyakov_loop(start, 0).unwrap();
    let after = transformed.polyakov_loop(start, 0).unwrap();
    assert_complex_close(after, before, 2.0e-11);
}

#[test]
fn rectangle_builder_prepares_static_potential_path_surface_without_inference() {
    let lattice = Su3Lattice4::new_identity([4, 4, 4, 4]).unwrap();
    let rectangle = rectangular_path(0, 3, 2, 3).unwrap();
    assert_eq!(rectangle.len(), 10);
    assert_complex_close(
        wilson_loop(&lattice, [0, 0, 0, 0], &rectangle).unwrap(),
        Complex64::ONE,
        0.0,
    );
}

#[test]
fn phase1d_step_orientation_matches_frozen_su2_semantics() {
    let mut lattice = Su2Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    lattice
        .set_link(
            [0, 0, 0, 0],
            0,
            Su2::from_axis_angle([1.0, 0.0, 0.0], 0.7).unwrap(),
        )
        .unwrap();
    let phase1d = [OrientedStep::Forward(0), OrientedStep::Backward(0)];
    let su2 = [Su2PathStep::Forward(0), Su2PathStep::Backward(0)];
    let (su2_transport, su2_end) = lattice.path_holonomy([0, 0, 0, 0], &su2).unwrap();
    assert_eq!(su2_end, [0, 0, 0, 0]);
    assert!(su2_transport.is_unit(1.0e-12));
    assert_eq!(phase1d[0], OrientedStep::Forward(0));
    assert_eq!(phase1d[1], OrientedStep::Backward(0));
}

#[test]
fn invalid_paths_fail_closed() {
    let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    assert_eq!(
        path_holonomy(&lattice, [0, 0, 0, 0], &[]).unwrap_err(),
        PathHolonomyError::EmptyPath
    );
    assert_eq!(
        closed_path_holonomy(&lattice, [0, 0, 0, 0], &[OrientedStep::Forward(0)])
            .unwrap_err(),
        PathHolonomyError::PathNotClosed
    );
    assert_eq!(
        path_holonomy(&lattice, [0, 0, 0, 0], &[OrientedStep::Forward(4)]).unwrap_err(),
        PathHolonomyError::Lattice(Su3LatticeError::DirectionOutOfRange)
    );
    assert_eq!(
        rectangular_path(0, 0, 1, 1).unwrap_err(),
        PathHolonomyError::DirectionsMustDiffer
    );
    assert_eq!(
        rectangular_path(0, 1, 0, 1).unwrap_err(),
        PathHolonomyError::ZeroRectangleExtent
    );
}
