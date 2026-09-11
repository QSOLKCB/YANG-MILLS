// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, Su2, DEFAULT_TOLERANCE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let beta = 2.0;
    let identity = Lattice4::new_identity([3, 3, 3, 3])?;
    let identity_action = identity.wilson_action(beta)?;
    let identity_rectangle = identity.rectangular_wilson_loop([0, 0, 0, 0], 0, 1, 2, 1)?;

    let mut single_link = Lattice4::new_identity([3, 3, 3, 3])?;
    let traceless = Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0])?;
    single_link.set_link([0, 0, 0, 0], 0, traceless)?;
    let single_link_action = single_link.wilson_action(beta)?;
    let analytic_action = 6.0 * beta;
    let single_link_rectangle =
        single_link.rectangular_wilson_loop([0, 0, 0, 0], 0, 1, 2, 1)?;
    let single_link_polyakov = single_link.polyakov_loop([0, 0, 0, 0], 0)?;
    let local_delta =
        single_link.local_action_difference([0, 0, 0, 0], 0, Su2::IDENTITY, beta)?;

    let encoded = single_link.to_canonical_config()?;
    let decoded = Lattice4::from_canonical_config(&encoded)?;
    let round_trip_exact = decoded.to_canonical_config()? == encoded;

    let identity_residual = identity_action.abs();
    let single_link_residual = (single_link_action - analytic_action).abs();
    if identity_residual > DEFAULT_TOLERANCE
        || single_link_residual > DEFAULT_TOLERANCE
        || (identity_rectangle - 1.0).abs() > DEFAULT_TOLERANCE
        || single_link_rectangle.abs() > DEFAULT_TOLERANCE
        || single_link_polyakov.abs() > DEFAULT_TOLERANCE
        || (local_delta + analytic_action).abs() > DEFAULT_TOLERANCE
        || !round_trip_exact
    {
        return Err("Phase 1A reference vector mismatch".into());
    }

    println!("lattice_contract=YM-SU2-LATTICE-V1");
    println!("loop_contract=YM-WILSON-LOOP-V1");
    println!("gauge_group=SU(2)");
    println!("lattice=3x3x3x3");
    println!("beta={beta:.17e}");
    println!("tolerance={DEFAULT_TOLERANCE:.17e}");
    println!("identity_action={identity_action:.17e}");
    println!("single_traceless_link_action={single_link_action:.17e}");
    println!("identity_rectangle_2x1={identity_rectangle:.17e}");
    println!("single_traceless_rectangle_2x1={single_link_rectangle:.17e}");
    println!("single_traceless_polyakov_dir0={single_link_polyakov:.17e}");
    println!("local_delta_to_identity={local_delta:.17e}");
    println!("canonical_round_trip_exact={round_trip_exact}");
    println!("status=passed");
    Ok(())
}
