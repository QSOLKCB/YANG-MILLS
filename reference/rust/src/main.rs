// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, Su2, DEFAULT_TOLERANCE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let beta = 2.0;
    let identity = Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_action = identity.wilson_action(beta)?;

    let mut single_link = Lattice4::new_identity([3, 3, 3, 3])?;
    let traceless = Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0])?;
    single_link.set_link([0, 0, 0, 0], 0, traceless)?;
    let single_link_action = single_link.wilson_action(beta)?;
    let analytic = 6.0 * beta;

    let identity_residual = identity_action.abs();
    let single_link_residual = (single_link_action - analytic).abs();
    if identity_residual > DEFAULT_TOLERANCE || single_link_residual > DEFAULT_TOLERANCE {
        return Err("Phase 0 reference vector mismatch".into());
    }

    println!("contract=YM-LATTICE-V1");
    println!("gauge_group=SU(2)");
    println!("lattice=3x3x3x3");
    println!("beta={beta:.17e}");
    println!("tolerance={DEFAULT_TOLERANCE:.17e}");
    println!("identity_action={identity_action:.17e}");
    println!("identity_residual={identity_residual:.17e}");
    println!("single_traceless_link_action={single_link_action:.17e}");
    println!("analytic_single_traceless_link_action={analytic:.17e}");
    println!("single_traceless_link_residual={single_link_residual:.17e}");
    println!("status=passed");
    Ok(())
}
