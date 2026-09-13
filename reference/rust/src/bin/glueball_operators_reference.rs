// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_operators::{
    scalar_a1pp_operator_basis, GAUGE_INVARIANCE_TOLERANCE, OPERATOR_BASIS_ID,
    OPERATOR_CONTRACT_ID, OPERATOR_STATUS,
};
use su3_lattice::{deterministic_su3_site_gauge, single_traceless_su3_link_lattice, Su3Lattice4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_basis = scalar_a1pp_operator_basis(&identity, 0)?;
    if identity_basis.spatial_plaquette_1x1 != 1.0
        || identity_basis.spatial_rectangle_2x1 != 1.0
        || identity_basis.plaquette_loop_count != 81
        || identity_basis.rectangle_loop_count != 162
    {
        return Err("Phase 3A identity operator regression mismatch".into());
    }

    let lattice = single_traceless_su3_link_lattice()?;
    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge)?;
    let before = scalar_a1pp_operator_basis(&lattice, 0)?;
    let after = scalar_a1pp_operator_basis(&transformed, 0)?;
    let gauge_residual = (before.spatial_plaquette_1x1 - after.spatial_plaquette_1x1)
        .abs()
        .max((before.spatial_rectangle_2x1 - after.spatial_rectangle_2x1).abs());
    if !gauge_residual.is_finite() || gauge_residual > GAUGE_INVARIANCE_TOLERANCE {
        return Err("Phase 3A gauge-invariance regression mismatch".into());
    }

    println!("contract={OPERATOR_CONTRACT_ID}");
    println!("basis={OPERATOR_BASIS_ID}");
    println!("status={OPERATOR_STATUS}");
    println!(
        "identity_spatial_plaquette_1x1={:.17e}",
        identity_basis.spatial_plaquette_1x1
    );
    println!(
        "identity_spatial_rectangle_2x1={:.17e}",
        identity_basis.spatial_rectangle_2x1
    );
    println!(
        "identity_plaquette_loop_count={}",
        identity_basis.plaquette_loop_count
    );
    println!(
        "identity_rectangle_loop_count={}",
        identity_basis.rectangle_loop_count
    );
    println!("deterministic_gauge_residual={gauge_residual:.17e}");
    println!("finite_lattice_irrep=A1++");
    println!("continuum_channel_intent=0++");
    println!("smearing_or_blocking_applied=false");
    println!("connected_correlator_constructed=false");
    println!("effective_mass_extracted=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("result=passed");
    Ok(())
}
