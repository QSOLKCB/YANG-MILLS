// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;
#[path = "../su3_center.rs"]
mod su3_center;

use su3::{Complex64, Mat3, Su3};
use su3_center::{
    apply_center_transform, center_acceptance_tolerance, center_commutator_residual,
    center_element, center_phase, cycle_generator, cycle_noncentral_residual, C3,
};
use su3_lattice::Su3Lattice4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let g = C3::GENERATOR;
    let c3_order_ok = g.pow(3) == C3::IDENTITY;

    let omega = cycle_generator()?;
    let omega3 = omega.mul(omega)?.mul(omega)?;
    let omega3_residual = omega3.matrix().max_residual(Mat3::IDENTITY);
    let noncentral_residual = cycle_noncentral_residual()?;

    let z = center_element(g)?;
    let sample = Su3::from_algebra_coefficients([
        0.3, -0.2, 0.1, 0.05, -0.07, 0.11, -0.09, 0.13,
    ])?;
    let center_commutator = center_commutator_residual(g, sample)?;

    let lattice = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let start = [2, 1, 2, 1];
    let before = lattice.polyakov_loop(start, 0)?;
    let transformed = apply_center_transform(&lattice, 0, 0, g)?;
    let after = transformed.polyakov_loop(start, 0)?;
    let expected_after = center_phase(g).mul(before);
    let polyakov_residual = after.sub(expected_after).abs();
    let action_residual = (transformed.wilson_action(2.0)? - lattice.wilson_action(2.0)?).abs();

    let tolerance = center_acceptance_tolerance();
    if !c3_order_ok
        || omega3_residual != 0.0
        || noncentral_residual != 2.0
        || z.unitarity_residual() > tolerance
        || z.determinant_residual() > tolerance
        || center_commutator > 2.0e-15
        || before != Complex64::ONE
        || polyakov_residual > 2.0e-15
        || action_residual > 2.0e-12
    {
        return Err("Phase 1C SU(3) center regression mismatch".into());
    }

    println!("contract=YM-SU3-CENTER-V1");
    println!("abstract_c3_order_three={c3_order_ok}");
    println!("omega_cubed_residual={omega3_residual:.17e}");
    println!("omega_noncentral_residual={noncentral_residual:.17e}");
    println!("center_commutator_residual={center_commutator:.17e}");
    println!("center_unitarity_residual={:.17e}", z.unitarity_residual());
    println!("center_determinant_residual={:.17e}", z.determinant_residual());
    println!("polyakov_center_rotation_residual={polyakov_residual:.17e}");
    println!("center_transform_action_residual={action_residual:.17e}");
    println!("confinement_claimed=false");
    println!("status=passed");
    Ok(())
}
