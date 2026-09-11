// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{
    deterministic_nontrivial_field, deterministic_site_gauge, Lattice4, PathStep, Su2,
    DEFAULT_TOLERANCE,
};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let residual = (actual - expected).abs();
    assert!(
        residual <= tolerance,
        "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
    );
}

#[test]
fn public_phase1a_wilson_loop_surface() {
    let lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    let path = Lattice4::rectangular_path(0, 1, 2, 1).unwrap();
    assert_eq!(
        lattice.wilson_loop([0, 0, 0, 0], &path).unwrap(),
        1.0
    );

    let reverse_cancel = [PathStep::Forward(2), PathStep::Backward(2)];
    assert_close(
        lattice
            .wilson_loop([0, 0, 0, 0], &reverse_cancel)
            .unwrap(),
        1.0,
        DEFAULT_TOLERANCE,
    );
}

#[test]
fn public_phase1a_gauge_invariant_observables() {
    let lattice = deterministic_nontrivial_field();
    let transformed = lattice.gauge_transform(deterministic_site_gauge).unwrap();
    let path = Lattice4::rectangular_path(0, 1, 2, 1).unwrap();
    assert_close(
        lattice.wilson_loop([0, 0, 0, 0], &path).unwrap(),
        transformed
            .wilson_loop([0, 0, 0, 0], &path)
            .unwrap(),
        2.0e-11,
    );
    assert_close(
        lattice.polyakov_loop([0, 0, 0, 0], 0).unwrap(),
        transformed.polyakov_loop([0, 0, 0, 0], 0).unwrap(),
        2.0e-11,
    );
}

#[test]
fn public_phase1a_local_update_and_serialization_surface() {
    let beta = 2.0;
    let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    let traceless = Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0]).unwrap();
    lattice.set_link([0, 0, 0, 0], 0, traceless).unwrap();

    let local_delta = lattice
        .local_action_difference([0, 0, 0, 0], 0, Su2::IDENTITY, beta)
        .unwrap();
    assert_close(local_delta, -12.0, DEFAULT_TOLERANCE);

    let encoded = lattice.to_canonical_config().unwrap();
    let decoded = Lattice4::from_canonical_config(&encoded).unwrap();
    assert_eq!(decoded.to_canonical_config().unwrap(), encoded);
}
