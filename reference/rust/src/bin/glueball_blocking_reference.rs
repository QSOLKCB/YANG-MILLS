// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_blocking.rs"]
mod glueball_blocking;
#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_blocking::{
    deterministic_blocking_fixture, fine_site_from_coarse, max_link_residual,
    spatial_stride2_block, BLOCKING_CONTRACT_ID, BLOCKING_STATUS, BLOCK_FACTOR,
    FROZEN_COARSE_PLAQUETTE_1X1, FROZEN_COARSE_RECTANGLE_2X1, FROZEN_OUTPUT_ABS_TOLERANCE,
    GAUGE_COVARIANCE_TOLERANCE, TRANSFORMATION_ID,
};
use glueball_operators::scalar_a1pp_operator_basis;
use su3::SU3_ACCEPTANCE_TOLERANCE;
use su3_lattice::{deterministic_su3_site_gauge, traceless_cycle_link, Su3Lattice4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = Su3Lattice4::new_identity([6, 6, 6, 3])?;
    let identity_blocked = spatial_stride2_block(&identity)?;
    let coarse_identity = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_residual = max_link_residual(&identity_blocked, &coarse_identity)?;
    if identity_residual != 0.0 {
        return Err("Phase 3B2 identity blocking regression mismatch".into());
    }

    let fine = deterministic_blocking_fixture()?;
    let coarse = spatial_stride2_block(&fine)?;
    let basis = scalar_a1pp_operator_basis(&coarse, 0)?;
    if (basis.spatial_plaquette_1x1 - FROZEN_COARSE_PLAQUETTE_1X1).abs()
        > FROZEN_OUTPUT_ABS_TOLERANCE
        || (basis.spatial_rectangle_2x1 - FROZEN_COARSE_RECTANGLE_2X1).abs()
            > FROZEN_OUTPUT_ABS_TOLERANCE
    {
        return Err("Phase 3B2 frozen coarse-operator regression mismatch".into());
    }

    let transformed = fine.gauge_transform(deterministic_su3_site_gauge)?;
    let blocked_transformed = spatial_stride2_block(&transformed)?;
    let blocked_then_transformed =
        coarse.gauge_transform(|y| deterministic_su3_site_gauge(fine_site_from_coarse(y)))?;
    let gauge_covariance_residual =
        max_link_residual(&blocked_transformed, &blocked_then_transformed)?;
    if !gauge_covariance_residual.is_finite()
        || gauge_covariance_residual > GAUGE_COVARIANCE_TOLERANCE
    {
        return Err("Phase 3B2 gauge covariance regression mismatch".into());
    }

    let mut temporal_fixture = Su3Lattice4::new_identity([6, 6, 6, 3])?;
    let temporal = traceless_cycle_link()?;
    temporal_fixture.set_link([0, 0, 0, 0], 3, temporal)?;
    let temporal_blocked = spatial_stride2_block(&temporal_fixture)?;
    let temporal_sampling_residual = temporal_blocked
        .link([0, 0, 0, 0], 3)?
        .matrix()
        .max_residual(temporal.matrix());
    if temporal_sampling_residual != 0.0 {
        return Err("Phase 3B2 temporal-link sampling regression mismatch".into());
    }

    let max_unitarity_residual = coarse.max_link_unitarity_residual();
    let max_determinant_residual = coarse.max_link_determinant_residual();
    if max_unitarity_residual > SU3_ACCEPTANCE_TOLERANCE
        || max_determinant_residual > SU3_ACCEPTANCE_TOLERANCE
    {
        return Err("Phase 3B2 group residual regression mismatch".into());
    }

    println!("contract={BLOCKING_CONTRACT_ID}");
    println!("transformation={TRANSFORMATION_ID}");
    println!("status={BLOCKING_STATUS}");
    println!("block_factor={BLOCK_FACTOR}");
    println!("fine_extents=6x6x6x3");
    println!("coarse_extents=3x3x3x3");
    println!("identity_blocking_residual={identity_residual:.17e}");
    println!("gauge_covariance_residual={gauge_covariance_residual:.17e}");
    println!("temporal_sampling_residual={temporal_sampling_residual:.17e}");
    println!("max_unitarity_residual={max_unitarity_residual:.17e}");
    println!("max_determinant_residual={max_determinant_residual:.17e}");
    println!(
        "coarse_operator_plaquette_1x1={:.17e}",
        basis.spatial_plaquette_1x1
    );
    println!(
        "coarse_operator_rectangle_2x1={:.17e}",
        basis.spatial_rectangle_2x1
    );
    println!("smearing_required=false");
    println!("connected_correlator_constructed=false");
    println!("effective_mass_extracted=false");
    println!("glueball_spectrum_extracted=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("result=passed");
    Ok(())
}
