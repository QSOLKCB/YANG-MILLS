// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../glueball_smearing.rs"]
mod glueball_smearing;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_operators::scalar_a1pp_operator_basis;
use glueball_smearing::{
    max_link_residual, spatial_stout_smear, CANONICAL_RHO, GAUGE_COVARIANCE_TOLERANCE, MAX_LEVELS,
    SMEARING_CONTRACT_ID, TRANSFORMATION_ID,
};
use su3::SU3_ACCEPTANCE_TOLERANCE;
use su3_lattice::{deterministic_su3_site_gauge, single_traceless_su3_link_lattice, Su3Lattice4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_smeared = spatial_stout_smear(&identity, CANONICAL_RHO, 3)?;
    let identity_residual = max_link_residual(&identity, &identity_smeared)?;
    if identity_residual != 0.0 {
        return Err("Phase 3B1 identity fixed-point regression mismatch".into());
    }

    let lattice = single_traceless_su3_link_lattice()?;
    let zero_level = spatial_stout_smear(&lattice, CANONICAL_RHO, 0)?;
    let zero_level_residual = max_link_residual(&lattice, &zero_level)?;
    if zero_level_residual != 0.0 {
        return Err("Phase 3B1 zero-level no-op regression mismatch".into());
    }

    let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, 1)?;
    let basis_before = scalar_a1pp_operator_basis(&lattice, 0)?;
    let basis_after = scalar_a1pp_operator_basis(&smeared, 0)?;
    let spatial_change = max_link_residual(&lattice, &smeared)?;
    if !spatial_change.is_finite() || spatial_change <= 0.0 {
        return Err("Phase 3B1 deterministic nontrivial field was not changed".into());
    }

    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge)?;
    let smeared_then_transformed = smeared.gauge_transform(deterministic_su3_site_gauge)?;
    let transformed_then_smeared = spatial_stout_smear(&transformed, CANONICAL_RHO, 1)?;
    let gauge_covariance_residual =
        max_link_residual(&smeared_then_transformed, &transformed_then_smeared)?;
    if !gauge_covariance_residual.is_finite()
        || gauge_covariance_residual > GAUGE_COVARIANCE_TOLERANCE
    {
        return Err("Phase 3B1 gauge covariance regression mismatch".into());
    }

    let stress = spatial_stout_smear(&lattice, CANONICAL_RHO, MAX_LEVELS)?;
    let max_unitarity_residual = stress.max_link_unitarity_residual();
    let max_determinant_residual = stress.max_link_determinant_residual();
    if max_unitarity_residual > SU3_ACCEPTANCE_TOLERANCE
        || max_determinant_residual > SU3_ACCEPTANCE_TOLERANCE
    {
        return Err("Phase 3B1 group residual regression mismatch".into());
    }

    println!("contract={SMEARING_CONTRACT_ID}");
    println!("transformation={TRANSFORMATION_ID}");
    println!("status=phase-3b1-candidate");
    println!("rho={CANONICAL_RHO:.17e}");
    println!("canonical_levels=1");
    println!("identity_fixed_point_residual={identity_residual:.17e}");
    println!("zero_level_noop_residual={zero_level_residual:.17e}");
    println!("deterministic_spatial_change={spatial_change:.17e}");
    println!("gauge_covariance_residual={gauge_covariance_residual:.17e}");
    println!("max_unitarity_residual_after_{MAX_LEVELS}_levels={max_unitarity_residual:.17e}");
    println!("max_determinant_residual_after_{MAX_LEVELS}_levels={max_determinant_residual:.17e}");
    println!(
        "operator_before_plaquette_1x1={:.17e}",
        basis_before.spatial_plaquette_1x1
    );
    println!(
        "operator_after_plaquette_1x1={:.17e}",
        basis_after.spatial_plaquette_1x1
    );
    println!(
        "operator_before_rectangle_2x1={:.17e}",
        basis_before.spatial_rectangle_2x1
    );
    println!(
        "operator_after_rectangle_2x1={:.17e}",
        basis_after.spatial_rectangle_2x1
    );
    println!("temporal_links_modified=false");
    println!("blocking_applied=false");
    println!("connected_correlator_constructed=false");
    println!("effective_mass_extracted=false");
    println!("glueball_spectrum_extracted=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("result=passed");
    Ok(())
}
