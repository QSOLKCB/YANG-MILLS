// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3C connected Euclidean correlator mechanics for the frozen scalar
//! glueball operator basis.
//!
//! This surface defines a bounded finite-sample periodic two-point estimator
//! and vacuum subtraction only. It does not estimate uncertainty, extract an
//! effective mass, select a fit window, or establish a glueball spectrum.

use core::fmt;

use crate::glueball_operators::{scalar_a1pp_operator_basis, GlueballOperatorError};
use crate::su3_lattice::{
    traceless_cycle_link, Su3Lattice4, Su3LatticeError,
};

pub const CORRELATOR_CONTRACT_ID: &str = "YM-GLUEBALL-CONNECTED-CORRELATOR-V1";
pub const CORRELATOR_STATUS: &str = "phase-3c-candidate";
pub const MAX_CONFIGURATIONS: usize = 64;
pub const MAX_TIME_EXTENT: usize = 64;
pub const GAUGE_INVARIANCE_TOLERANCE: f64 = 5.0e-11;
pub const ANALYTIC_REPLAY_TOLERANCE: f64 = 2.0e-12;

pub const CANONICAL_PLAQUETTE_MEAN: f64 = 80.0 / 81.0;
pub const CANONICAL_RECTANGLE_MEAN: f64 = 53.0 / 54.0;
pub const CANONICAL_PLAQUETTE_RAW: [f64; 4] = [
    6403.0 / 6561.0,
    79.0 / 81.0,
    79.0 / 81.0,
    79.0 / 81.0,
];
pub const CANONICAL_RECTANGLE_RAW: [f64; 4] = [
    703.0 / 729.0,
    26.0 / 27.0,
    26.0 / 27.0,
    26.0 / 27.0,
];
pub const CANONICAL_PLAQUETTE_CONNECTED: [f64; 4] = [
    1.0 / 2187.0,
    -1.0 / 6561.0,
    -1.0 / 6561.0,
    -1.0 / 6561.0,
];
pub const CANONICAL_RECTANGLE_CONNECTED: [f64; 4] = [
    1.0 / 972.0,
    -1.0 / 2916.0,
    -1.0 / 2916.0,
    -1.0 / 2916.0,
];

#[derive(Clone, Debug, PartialEq)]
pub struct ScalarConnectedCorrelators {
    pub configuration_count: usize,
    pub time_extent: usize,
    pub plaquette_vacuum_mean: f64,
    pub rectangle_vacuum_mean: f64,
    pub plaquette_raw: Vec<f64>,
    pub rectangle_raw: Vec<f64>,
    pub plaquette_connected: Vec<f64>,
    pub rectangle_connected: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlueballCorrelatorError {
    EmptyConfigurationSet,
    TooManyConfigurations,
    TimeExtentTooLarge,
    LatticeExtentsMismatch,
    SampleCountOverflow,
    AllocationFailed,
    NonFiniteObservable,
    NonFiniteAccumulator,
    CorrelatorShapeMismatch,
    Operator(GlueballOperatorError),
    Lattice(Su3LatticeError),
}

impl fmt::Display for GlueballCorrelatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyConfigurationSet => "Phase 3C correlators require at least one configuration",
            Self::TooManyConfigurations => "Phase 3C configuration count exceeds the bounded reference limit",
            Self::TimeExtentTooLarge => "Phase 3C Euclidean time extent exceeds the bounded reference limit",
            Self::LatticeExtentsMismatch => "Phase 3C configurations must have identical lattice extents",
            Self::SampleCountOverflow => "Phase 3C sample-count arithmetic overflowed",
            Self::AllocationFailed => "Phase 3C correlator allocation failed",
            Self::NonFiniteObservable => "Phase 3C operator input became non-finite",
            Self::NonFiniteAccumulator => "Phase 3C correlator accumulator became non-finite",
            Self::CorrelatorShapeMismatch => "Phase 3C correlator outputs have incompatible shapes",
            Self::Operator(_) => "Phase 3C scalar operator evaluation failed",
            Self::Lattice(_) => "Phase 3C lattice fixture construction failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for GlueballCorrelatorError {}

impl From<GlueballOperatorError> for GlueballCorrelatorError {
    fn from(value: GlueballOperatorError) -> Self {
        Self::Operator(value)
    }
}

impl From<Su3LatticeError> for GlueballCorrelatorError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

fn reserve_f64(len: usize) -> Result<Vec<f64>, GlueballCorrelatorError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| GlueballCorrelatorError::AllocationFailed)?;
    Ok(values)
}

fn periodic_connected_estimator(
    samples: &[f64],
    configuration_count: usize,
    time_extent: usize,
) -> Result<(f64, Vec<f64>, Vec<f64>), GlueballCorrelatorError> {
    let expected_len = configuration_count
        .checked_mul(time_extent)
        .ok_or(GlueballCorrelatorError::SampleCountOverflow)?;
    if samples.len() != expected_len || expected_len == 0 {
        return Err(GlueballCorrelatorError::CorrelatorShapeMismatch);
    }

    let mut mean_sum = 0.0_f64;
    for &value in samples {
        if !value.is_finite() {
            return Err(GlueballCorrelatorError::NonFiniteObservable);
        }
        mean_sum += value;
        if !mean_sum.is_finite() {
            return Err(GlueballCorrelatorError::NonFiniteAccumulator);
        }
    }
    let vacuum_mean = mean_sum / expected_len as f64;
    if !vacuum_mean.is_finite() {
        return Err(GlueballCorrelatorError::NonFiniteAccumulator);
    }

    let mut raw = reserve_f64(time_extent)?;
    let mut connected = reserve_f64(time_extent)?;
    let vacuum_square = vacuum_mean * vacuum_mean;
    if !vacuum_square.is_finite() {
        return Err(GlueballCorrelatorError::NonFiniteAccumulator);
    }

    for tau in 0..time_extent {
        let mut sum = 0.0_f64;
        for configuration in 0..configuration_count {
            let offset = configuration * time_extent;
            for time in 0..time_extent {
                let first = samples[offset + time];
                let second = samples[offset + (time + tau) % time_extent];
                let product = first * second;
                if !product.is_finite() {
                    return Err(GlueballCorrelatorError::NonFiniteAccumulator);
                }
                sum += product;
                if !sum.is_finite() {
                    return Err(GlueballCorrelatorError::NonFiniteAccumulator);
                }
            }
        }
        let raw_value = sum / expected_len as f64;
        let connected_value = raw_value - vacuum_square;
        if !raw_value.is_finite() || !connected_value.is_finite() {
            return Err(GlueballCorrelatorError::NonFiniteAccumulator);
        }
        raw.push(raw_value);
        connected.push(connected_value);
    }

    Ok((vacuum_mean, raw, connected))
}

/// Build diagonal connected correlators for both frozen Phase 3A scalar basis
/// elements. Every configuration and every periodic Euclidean time origin has
/// equal arithmetic weight. The finite-sample vacuum subtraction is
/// `C(tau)=<O(t)O(t+tau)>-<O>^2`, with both averages evaluated on exactly the
/// supplied configurations and all time origins.
pub fn scalar_connected_correlators(
    configurations: &[Su3Lattice4],
) -> Result<ScalarConnectedCorrelators, GlueballCorrelatorError> {
    if configurations.is_empty() {
        return Err(GlueballCorrelatorError::EmptyConfigurationSet);
    }
    if configurations.len() > MAX_CONFIGURATIONS {
        return Err(GlueballCorrelatorError::TooManyConfigurations);
    }

    let extents = configurations[0].extents();
    let time_extent = extents[3];
    if time_extent > MAX_TIME_EXTENT {
        return Err(GlueballCorrelatorError::TimeExtentTooLarge);
    }
    if configurations
        .iter()
        .any(|configuration| configuration.extents() != extents)
    {
        return Err(GlueballCorrelatorError::LatticeExtentsMismatch);
    }

    let sample_count = configurations
        .len()
        .checked_mul(time_extent)
        .ok_or(GlueballCorrelatorError::SampleCountOverflow)?;
    let mut plaquette_samples = reserve_f64(sample_count)?;
    let mut rectangle_samples = reserve_f64(sample_count)?;

    for configuration in configurations {
        for time_slice in 0..time_extent {
            let basis = scalar_a1pp_operator_basis(configuration, time_slice)?;
            if !basis.spatial_plaquette_1x1.is_finite()
                || !basis.spatial_rectangle_2x1.is_finite()
            {
                return Err(GlueballCorrelatorError::NonFiniteObservable);
            }
            plaquette_samples.push(basis.spatial_plaquette_1x1);
            rectangle_samples.push(basis.spatial_rectangle_2x1);
        }
    }

    let (plaquette_vacuum_mean, plaquette_raw, plaquette_connected) =
        periodic_connected_estimator(&plaquette_samples, configurations.len(), time_extent)?;
    let (rectangle_vacuum_mean, rectangle_raw, rectangle_connected) =
        periodic_connected_estimator(&rectangle_samples, configurations.len(), time_extent)?;

    Ok(ScalarConnectedCorrelators {
        configuration_count: configurations.len(),
        time_extent,
        plaquette_vacuum_mean,
        rectangle_vacuum_mean,
        plaquette_raw,
        rectangle_raw,
        plaquette_connected,
        rectangle_connected,
    })
}

pub fn max_correlator_residual(
    first: &ScalarConnectedCorrelators,
    second: &ScalarConnectedCorrelators,
) -> Result<f64, GlueballCorrelatorError> {
    if first.configuration_count != second.configuration_count
        || first.time_extent != second.time_extent
        || first.plaquette_raw.len() != second.plaquette_raw.len()
        || first.rectangle_raw.len() != second.rectangle_raw.len()
        || first.plaquette_connected.len() != second.plaquette_connected.len()
        || first.rectangle_connected.len() != second.rectangle_connected.len()
    {
        return Err(GlueballCorrelatorError::CorrelatorShapeMismatch);
    }

    let mut maximum = (first.plaquette_vacuum_mean - second.plaquette_vacuum_mean)
        .abs()
        .max((first.rectangle_vacuum_mean - second.rectangle_vacuum_mean).abs());
    for (left, right) in first
        .plaquette_raw
        .iter()
        .zip(&second.plaquette_raw)
        .chain(first.rectangle_raw.iter().zip(&second.rectangle_raw))
        .chain(
            first
                .plaquette_connected
                .iter()
                .zip(&second.plaquette_connected),
        )
        .chain(
            first
                .rectangle_connected
                .iter()
                .zip(&second.rectangle_connected),
        )
    {
        let residual = (left - right).abs();
        if !residual.is_finite() {
            return Err(GlueballCorrelatorError::NonFiniteAccumulator);
        }
        maximum = maximum.max(residual);
    }
    Ok(maximum)
}

/// Four deterministic `3^3 x 4` configurations. Configuration `c` carries one
/// exact traceless-cycle spatial link at Euclidean time `c`. This is a mechanics
/// fixture, not a Monte Carlo ensemble or a physical correlator measurement.
pub fn canonical_shifted_defect_configurations(
) -> Result<Vec<Su3Lattice4>, GlueballCorrelatorError> {
    let mut configurations = Vec::new();
    configurations
        .try_reserve_exact(4)
        .map_err(|_| GlueballCorrelatorError::AllocationFailed)?;
    let defect = traceless_cycle_link()?;
    for time_slice in 0..4 {
        let mut lattice = Su3Lattice4::new_identity([3, 3, 3, 4])?;
        lattice.set_link([0, 0, 0, time_slice], 0, defect)?;
        configurations.push(lattice);
    }
    Ok(configurations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::su3_lattice::deterministic_su3_site_gauge;

    fn assert_close(actual: f64, expected: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= ANALYTIC_REPLAY_TOLERANCE,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e}"
        );
    }

    fn assert_series_close(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for (&left, &right) in actual.iter().zip(expected) {
            assert_close(left, right);
        }
    }

    #[test]
    fn phase3c_identity_is_frozen() {
        assert_eq!(
            CORRELATOR_CONTRACT_ID,
            "YM-GLUEBALL-CONNECTED-CORRELATOR-V1"
        );
        assert_eq!(CORRELATOR_STATUS, "phase-3c-candidate");
        assert_eq!(MAX_CONFIGURATIONS, 64);
        assert_eq!(MAX_TIME_EXTENT, 64);
        assert_eq!(GAUGE_INVARIANCE_TOLERANCE, 5.0e-11);
        assert_eq!(ANALYTIC_REPLAY_TOLERANCE, 2.0e-12);
    }

    #[test]
    fn identity_configurations_have_zero_connected_correlator() {
        let first = Su3Lattice4::new_identity([3, 3, 3, 4]).unwrap();
        let second = Su3Lattice4::new_identity([3, 3, 3, 4]).unwrap();
        let result = scalar_connected_correlators(&[first, second]).unwrap();
        assert_eq!(result.plaquette_vacuum_mean, 1.0);
        assert_eq!(result.rectangle_vacuum_mean, 1.0);
        assert!(result.plaquette_raw.iter().all(|&value| value == 1.0));
        assert!(result.rectangle_raw.iter().all(|&value| value == 1.0));
        assert!(result.plaquette_connected.iter().all(|&value| value == 0.0));
        assert!(result.rectangle_connected.iter().all(|&value| value == 0.0));
    }

    #[test]
    fn shifted_defect_fixture_matches_analytic_correlators() {
        let configurations = canonical_shifted_defect_configurations().unwrap();
        let result = scalar_connected_correlators(&configurations).unwrap();
        assert_eq!(result.configuration_count, 4);
        assert_eq!(result.time_extent, 4);
        assert_close(result.plaquette_vacuum_mean, CANONICAL_PLAQUETTE_MEAN);
        assert_close(result.rectangle_vacuum_mean, CANONICAL_RECTANGLE_MEAN);
        assert_series_close(&result.plaquette_raw, &CANONICAL_PLAQUETTE_RAW);
        assert_series_close(&result.rectangle_raw, &CANONICAL_RECTANGLE_RAW);
        assert_series_close(
            &result.plaquette_connected,
            &CANONICAL_PLAQUETTE_CONNECTED,
        );
        assert_series_close(
            &result.rectangle_connected,
            &CANONICAL_RECTANGLE_CONNECTED,
        );
    }

    #[test]
    fn connected_correlators_are_gauge_invariant() {
        let configurations = canonical_shifted_defect_configurations().unwrap();
        let before = scalar_connected_correlators(&configurations).unwrap();
        let mut transformed = Vec::new();
        transformed.try_reserve_exact(configurations.len()).unwrap();
        for configuration in &configurations {
            transformed.push(
                configuration
                    .gauge_transform(deterministic_su3_site_gauge)
                    .unwrap(),
            );
        }
        let after = scalar_connected_correlators(&transformed).unwrap();
        let residual = max_correlator_residual(&before, &after).unwrap();
        assert!(
            residual <= GAUGE_INVARIANCE_TOLERANCE,
            "residual={residual:.17e}"
        );
    }

    #[test]
    fn empty_and_mismatched_configuration_sets_fail_closed() {
        assert_eq!(
            scalar_connected_correlators(&[]).unwrap_err(),
            GlueballCorrelatorError::EmptyConfigurationSet
        );
        let first = Su3Lattice4::new_identity([3, 3, 3, 4]).unwrap();
        let second = Su3Lattice4::new_identity([4, 4, 4, 4]).unwrap();
        assert_eq!(
            scalar_connected_correlators(&[first, second]).unwrap_err(),
            GlueballCorrelatorError::LatticeExtentsMismatch
        );
    }

    #[test]
    fn bounded_configuration_and_time_limits_fail_closed() {
        let template = Su3Lattice4::new_identity([3, 3, 3, 2]).unwrap();
        let mut too_many = Vec::new();
        too_many.try_reserve_exact(MAX_CONFIGURATIONS + 1).unwrap();
        for _ in 0..=MAX_CONFIGURATIONS {
            too_many.push(template.clone());
        }
        assert_eq!(
            scalar_connected_correlators(&too_many).unwrap_err(),
            GlueballCorrelatorError::TooManyConfigurations
        );

        let long_time = Su3Lattice4::new_identity([3, 3, 3, MAX_TIME_EXTENT + 1]).unwrap();
        assert_eq!(
            scalar_connected_correlators(&[long_time]).unwrap_err(),
            GlueballCorrelatorError::TimeExtentTooLarge
        );
    }
}
