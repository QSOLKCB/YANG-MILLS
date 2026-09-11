// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use su3_lattice::{
    deterministic_su3_site_gauge, reference_acceptance_tolerance,
    single_traceless_su3_link_lattice, Su3Lattice4,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let beta = 2.0;
    let identity = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_action = identity.wilson_action(beta)?;

    let single = single_traceless_su3_link_lattice()?;
    let single_action = single.wilson_action(beta)?;
    let analytic = 6.0 * beta;

    let transformed = single.gauge_transform(deterministic_su3_site_gauge)?;
    let transformed_action = transformed.wilson_action(beta)?;
    let gauge_action_residual = (single_action - transformed_action).abs();
    let tolerance = reference_acceptance_tolerance();

    if identity_action != 0.0
        || (single_action - analytic).abs() > 1.0e-12
        || gauge_action_residual > 1.0e-10
        || transformed.max_link_unitarity_residual() > tolerance
        || transformed.max_link_determinant_residual() > tolerance
    {
        return Err("Phase 1B SU(3) lattice reference regression mismatch".into());
    }

    println!("contract=YM-SU3-LATTICE-V1");
    println!("gauge_group=SU(3)");
    println!("lattice=3x3x3x3");
    println!("beta={beta:.17e}");
    println!("identity_action={identity_action:.17e}");
    println!("single_traceless_cycle_link_action={single_action:.17e}");
    println!("analytic_single_link_action={analytic:.17e}");
    println!("gauge_action_residual={gauge_action_residual:.17e}");
    println!(
        "transformed_max_unitarity_residual={:.17e}",
        transformed.max_link_unitarity_residual()
    );
    println!(
        "transformed_max_determinant_residual={:.17e}",
        transformed.max_link_determinant_residual()
    );
    println!("status=passed");
    Ok(())
}
