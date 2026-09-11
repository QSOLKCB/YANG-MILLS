// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../su3.rs"]
mod su3;

use su3::{
    cubic_casimir_fundamental, quadratic_casimir_fundamental, structure_d, structure_f, Mat3, Su3,
    SU3_ACCEPTANCE_TOLERANCE, SU3_GENERATORS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f123 = structure_f(0, 1, 2)?;
    let d118 = structure_d(0, 0, 7)?;

    let quadratic = quadratic_casimir_fundamental();
    let quadratic_residual = quadratic.max_residual(Mat3::IDENTITY.scale_real(4.0 / 3.0));

    let cubic = cubic_casimir_fundamental();
    let cubic_residual = cubic.max_residual(Mat3::IDENTITY.scale_real(10.0 / 9.0));

    let mut theta3 = [0.0; SU3_GENERATORS];
    theta3[2] = 0.7;
    let u = Su3::from_algebra_coefficients(theta3)?;

    if (f123 - 1.0).abs() > 2.0e-15
        || (d118 - 0.5773502691896258).abs() > 2.0e-15
        || quadratic_residual > 3.0e-15
        || cubic_residual > 8.0e-15
        || u.unitarity_residual() > SU3_ACCEPTANCE_TOLERANCE
        || u.determinant_residual() > SU3_ACCEPTANCE_TOLERANCE
    {
        return Err("Phase 1B SU(3) reference regression mismatch".into());
    }

    println!("contract=YM-SU3-ALGEBRA-V1");
    println!("generator_count={SU3_GENERATORS}");
    println!("f123={f123:.17e}");
    println!("d118={d118:.17e}");
    println!("quadratic_casimir_residual={quadratic_residual:.17e}");
    println!("cubic_casimir_residual={cubic_residual:.17e}");
    println!(
        "theta3_0p7_unitarity_residual={:.17e}",
        u.unitarity_residual()
    );
    println!(
        "theta3_0p7_determinant_residual={:.17e}",
        u.determinant_residual()
    );
    println!("status=passed");
    Ok(())
}
