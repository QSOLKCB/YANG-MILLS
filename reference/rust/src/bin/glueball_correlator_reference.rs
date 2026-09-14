// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[path = "../glueball_correlators.rs"]
mod glueball_correlators;
#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_correlators::{
    canonical_shifted_defect_configurations, max_correlator_residual,
    scalar_connected_correlators, ANALYTIC_REPLAY_TOLERANCE, CANONICAL_PLAQUETTE_CONNECTED,
    CANONICAL_PLAQUETTE_MEAN, CANONICAL_PLAQUETTE_RAW, CANONICAL_RECTANGLE_CONNECTED,
    CANONICAL_RECTANGLE_MEAN, CANONICAL_RECTANGLE_RAW, CORRELATOR_CONTRACT_ID,
    CORRELATOR_STATUS, GAUGE_INVARIANCE_TOLERANCE,
};
use su3_lattice::deterministic_su3_site_gauge;

fn max_series_residual(actual: &[f64], expected: &[f64]) -> Result<f64, &'static str> {
    if actual.len() != expected.len() {
        return Err("Phase 3C analytic vector shape mismatch");
    }
    let mut maximum = 0.0_f64;
    for (&left, &right) in actual.iter().zip(expected) {
        let residual = (left - right).abs();
        if !residual.is_finite() {
            return Err("Phase 3C analytic vector residual became non-finite");
        }
        maximum = maximum.max(residual);
    }
    Ok(maximum)
}

fn print_series(label: &str, values: &[f64]) {
    print!("{label}=");
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            print!(",");
        }
        print!("{value:.17e}");
    }
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configurations = canonical_shifted_defect_configurations()?;
    let result = scalar_connected_correlators(&configurations)?;

    let analytic_residual = (result.plaquette_vacuum_mean - CANONICAL_PLAQUETTE_MEAN)
        .abs()
        .max((result.rectangle_vacuum_mean - CANONICAL_RECTANGLE_MEAN).abs())
        .max(max_series_residual(
            &result.plaquette_raw,
            &CANONICAL_PLAQUETTE_RAW,
        )?)
        .max(max_series_residual(
            &result.rectangle_raw,
            &CANONICAL_RECTANGLE_RAW,
        )?)
        .max(max_series_residual(
            &result.plaquette_connected,
            &CANONICAL_PLAQUETTE_CONNECTED,
        )?)
        .max(max_series_residual(
            &result.rectangle_connected,
            &CANONICAL_RECTANGLE_CONNECTED,
        )?);
    if analytic_residual > ANALYTIC_REPLAY_TOLERANCE {
        return Err("Phase 3C analytic correlator regression mismatch".into());
    }

    let mut transformed = Vec::new();
    transformed.try_reserve_exact(configurations.len())?;
    for configuration in &configurations {
        transformed.push(configuration.gauge_transform(deterministic_su3_site_gauge)?);
    }
    let transformed_result = scalar_connected_correlators(&transformed)?;
    let gauge_invariance_residual = max_correlator_residual(&result, &transformed_result)?;
    if gauge_invariance_residual > GAUGE_INVARIANCE_TOLERANCE {
        return Err("Phase 3C gauge-invariance regression mismatch".into());
    }

    println!("contract={CORRELATOR_CONTRACT_ID}");
    println!("status={CORRELATOR_STATUS}");
    println!("configuration_count={}", result.configuration_count);
    println!("time_extent={}", result.time_extent);
    println!("canonical_fixture_is_physical_ensemble=false");
    println!("vacuum_subtraction_defined=true");
    println!("connected_correlator_constructed=true");
    println!(
        "plaquette_vacuum_mean={:.17e}",
        result.plaquette_vacuum_mean
    );
    println!(
        "rectangle_vacuum_mean={:.17e}",
        result.rectangle_vacuum_mean
    );
    print_series("plaquette_raw", &result.plaquette_raw);
    print_series("rectangle_raw", &result.rectangle_raw);
    print_series("plaquette_connected", &result.plaquette_connected);
    print_series("rectangle_connected", &result.rectangle_connected);
    println!("analytic_replay_residual={analytic_residual:.17e}");
    println!("gauge_invariance_residual={gauge_invariance_residual:.17e}");
    println!("uncertainty_estimated=false");
    println!("effective_mass_extracted=false");
    println!("fit_window_selected=false");
    println!("glueball_spectrum_extracted=false");
    println!("finite_volume_mass_gap_established=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("result=passed");
    Ok(())
}
