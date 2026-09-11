// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use path_holonomy::{
    concatenate_paths, endpoint_covariance_prediction, path_holonomy, reverse_path, wilson_loop,
    OrientedStep,
};
use su3::Su3;
use su3_lattice::{deterministic_su3_site_gauge, Su3Lattice4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lattice = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    let a = Su3::from_algebra_coefficients([0.4, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.1])?;
    let b = Su3::from_algebra_coefficients([0.0, -0.3, 0.0, 0.25, 0.0, 0.0, 0.1, -0.05])?;
    lattice.set_link([0, 0, 0, 0], 0, a)?;
    lattice.set_link([1, 0, 0, 0], 1, b)?;

    let start = [0, 0, 0, 0];
    let first = [OrientedStep::Forward(0)];
    let second = [OrientedStep::Forward(1)];
    let (u1, midpoint) = path_holonomy(&lattice, start, &first)?;
    let (u2, end) = path_holonomy(&lattice, midpoint, &second)?;
    let combined = concatenate_paths(&first, &second)?;
    let (total, total_end) = path_holonomy(&lattice, start, &combined)?;
    let composition_residual = total.matrix().max_residual(u1.mul(u2)?.matrix());

    let reversed = reverse_path(&combined)?;
    let (inverse, returned) = path_holonomy(&lattice, end, &reversed)?;
    let reversal_residual = inverse.matrix().max_residual(total.dagger().matrix());

    let closed = [
        OrientedStep::Forward(0),
        OrientedStep::Forward(1),
        OrientedStep::Backward(0),
        OrientedStep::Backward(1),
    ];
    let loop_before = wilson_loop(&lattice, start, &closed)?;

    let transformed = lattice.gauge_transform(deterministic_su3_site_gauge)?;
    let loop_after = wilson_loop(&transformed, start, &closed)?;
    let loop_gauge_residual = loop_after.sub(loop_before).abs();

    let open_steps = [
        OrientedStep::Forward(0),
        OrientedStep::Forward(1),
        OrientedStep::Forward(2),
    ];
    let (open_before, open_end) = path_holonomy(&lattice, start, &open_steps)?;
    let (open_after, transformed_end) = path_holonomy(&transformed, start, &open_steps)?;
    let predicted = endpoint_covariance_prediction(
        deterministic_su3_site_gauge(start),
        open_before,
        deterministic_su3_site_gauge(open_end),
    )?;
    let endpoint_covariance_residual = open_after.matrix().max_residual(predicted.matrix());

    let polyakov_before = lattice.polyakov_loop(start, 0)?;
    let polyakov_after = transformed.polyakov_loop(start, 0)?;
    let polyakov_gauge_residual = polyakov_after.sub(polyakov_before).abs();

    if total_end != end
        || returned != start
        || transformed_end != open_end
        || composition_residual > 5.0e-12
        || reversal_residual > 5.0e-12
        || endpoint_covariance_residual > 2.0e-11
        || loop_gauge_residual > 2.0e-11
        || polyakov_gauge_residual > 2.0e-11
    {
        return Err("Phase 1D path/holonomy regression mismatch".into());
    }

    println!("contract=YM-PATH-HOLONOMY-V1");
    println!("composition_residual={composition_residual:.17e}");
    println!("reversal_residual={reversal_residual:.17e}");
    println!("endpoint_covariance_residual={endpoint_covariance_residual:.17e}");
    println!("closed_loop_gauge_residual={loop_gauge_residual:.17e}");
    println!("polyakov_gauge_residual={polyakov_gauge_residual:.17e}");
    println!("continuum_parallel_transport_numerically_solved=false");
    println!("confinement_claimed=false");
    println!("static_potential_extracted=false");
    println!("status=passed");
    Ok(())
}
