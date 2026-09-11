// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{
    deterministic_nontrivial_field, deterministic_site_gauge, Lattice4, Su2,
};

#[test]
fn phase0_public_reference_surface() {
    let beta = 2.0;
    let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
    lattice
        .set_link(
            [0, 0, 0, 0],
            0,
            Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0]).unwrap(),
        )
        .unwrap();
    assert!((lattice.wilson_action(beta).unwrap() - 12.0).abs() <= 1.0e-12);
}

#[test]
fn public_gauge_transform_surface_preserves_action() {
    let beta = 1.7;
    let lattice = deterministic_nontrivial_field();
    let transformed = lattice.gauge_transform(deterministic_site_gauge).unwrap();
    let residual =
        (lattice.wilson_action(beta).unwrap() - transformed.wilson_action(beta).unwrap()).abs();
    assert!(residual <= 2.0e-11, "gauge-action residual={residual:.17e}");
}
