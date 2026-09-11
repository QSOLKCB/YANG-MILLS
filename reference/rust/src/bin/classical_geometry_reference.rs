// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../classical_geometry.rs"]
mod classical_geometry;

use classical_geometry::TwoForm4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let alpha = TwoForm4::from_components([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;
    let beta = TwoForm4::from_components([6.0, -5.0, 4.0, -3.0, 2.0, -1.0])?;

    let star_square_residual = alpha.hodge_star().hodge_star().max_residual(alpha);
    let hodge_inner_wedge_residual = (alpha
        .wedge_volume_coefficient(beta.hodge_star())
        - alpha.inner_product(beta))
    .abs();

    let self_dual = alpha.self_dual_part()?;
    let anti_self_dual = alpha.anti_self_dual_part()?;
    let self_dual_residual = self_dual.hodge_star().max_residual(self_dual);
    let anti_self_dual_residual = anti_self_dual
        .hodge_star()
        .max_residual(anti_self_dual.scale(-1.0)?);
    let decomposition_residual = self_dual.add(anti_self_dual).max_residual(alpha);
    let orthogonality_residual = self_dual.inner_product(anti_self_dual).abs();

    if star_square_residual != 0.0
        || hodge_inner_wedge_residual != 0.0
        || self_dual_residual != 0.0
        || anti_self_dual_residual != 0.0
        || decomposition_residual != 0.0
        || orthogonality_residual != 0.0
    {
        return Err("Phase 1E classical geometry regression mismatch".into());
    }

    println!("contract=YM-CLASSICAL-GEOMETRY-V1");
    println!("star_square_residual={star_square_residual:.17e}");
    println!("hodge_inner_wedge_residual={hodge_inner_wedge_residual:.17e}");
    println!("self_dual_residual={self_dual_residual:.17e}");
    println!("anti_self_dual_residual={anti_self_dual_residual:.17e}");
    println!("decomposition_residual={decomposition_residual:.17e}");
    println!("self_anti_orthogonality_residual={orthogonality_residual:.17e}");
    println!("continuum_quantum_theory_constructed=false");
    println!("positive_mass_gap_proved=false");
    println!("status=passed");
    Ok(())
}
