// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3A unsmeared spatial Wilson-loop operator basis.
//!
//! This surface defines bounded finite-lattice A1++ operator candidates only.
//! It does not construct connected correlators, extract a glueball mass, take
//! a continuum limit, or establish the Yang--Mills mass gap.

use core::fmt;

use crate::path_holonomy::{rectangular_path, wilson_loop, PathHolonomyError};
use crate::su3_lattice::{Su3Lattice4, DIM};

pub const OPERATOR_CONTRACT_ID: &str = "YM-GLUEBALL-OPERATORS-V1";
pub const OPERATOR_BASIS_ID: &str = "YM-GLUEBALL-SPATIAL-A1PP-V1";
pub const EUCLIDEAN_TIME_DIRECTION: usize = 3;
pub const SPATIAL_DIRECTIONS: [usize; 3] = [0, 1, 2];
pub const MINIMUM_SPATIAL_EXTENT: usize = 3;
pub const GAUGE_INVARIANCE_TOLERANCE: f64 = 2.0e-11;

const PLAQUETTE_ORIENTATIONS: [(usize, usize); 3] = [(0, 1), (0, 2), (1, 2)];
const RECTANGLE_ORIENTATIONS: [(usize, usize); 6] = [
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 2),
    (2, 0),
    (2, 1),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalarOperatorBasis {
    pub spatial_plaquette_1x1: f64,
    pub spatial_rectangle_2x1: f64,
    pub plaquette_loop_count: usize,
    pub rectangle_loop_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlueballOperatorError {
    TimeSliceOutOfRange,
    SpatialExtentTooSmall,
    CountOverflow,
    NonFiniteObservable,
    Path(PathHolonomyError),
}

impl fmt::Display for GlueballOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TimeSliceOutOfRange => "Phase 3A Euclidean time slice is outside the lattice",
            Self::SpatialExtentTooSmall => {
                "Phase 3A 2x1 spatial operator basis requires every spatial extent to be at least 3"
            }
            Self::CountOverflow => "Phase 3A spatial loop count overflowed",
            Self::NonFiniteObservable => "Phase 3A spatial loop observable became non-finite",
            Self::Path(_) => "Phase 3A spatial loop construction failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for GlueballOperatorError {}

impl From<PathHolonomyError> for GlueballOperatorError {
    fn from(value: PathHolonomyError) -> Self {
        Self::Path(value)
    }
}

fn validate_geometry(lattice: &Su3Lattice4, time_slice: usize) -> Result<(), GlueballOperatorError> {
    let extents = lattice.extents();
    debug_assert_eq!(DIM, 4);
    if time_slice >= extents[EUCLIDEAN_TIME_DIRECTION] {
        return Err(GlueballOperatorError::TimeSliceOutOfRange);
    }
    if SPATIAL_DIRECTIONS
        .iter()
        .any(|&direction| extents[direction] < MINIMUM_SPATIAL_EXTENT)
    {
        return Err(GlueballOperatorError::SpatialExtentTooSmall);
    }
    Ok(())
}

fn average_orbit(
    lattice: &Su3Lattice4,
    time_slice: usize,
    orientations: &[(usize, usize)],
    first_extent: usize,
    second_extent: usize,
) -> Result<(f64, usize), GlueballOperatorError> {
    let extents = lattice.extents();
    let mut sum = 0.0_f64;
    let mut count = 0usize;

    for &(first_direction, second_direction) in orientations {
        let steps = rectangular_path(
            first_direction,
            second_direction,
            first_extent,
            second_extent,
        )?;
        for x0 in 0..extents[0] {
            for x1 in 0..extents[1] {
                for x2 in 0..extents[2] {
                    let start = [x0, x1, x2, time_slice];
                    let value = wilson_loop(lattice, start, &steps)?.re;
                    if !value.is_finite() {
                        return Err(GlueballOperatorError::NonFiniteObservable);
                    }
                    sum += value;
                    if !sum.is_finite() {
                        return Err(GlueballOperatorError::NonFiniteObservable);
                    }
                    count = count
                        .checked_add(1)
                        .ok_or(GlueballOperatorError::CountOverflow)?;
                }
            }
        }
    }

    if count == 0 {
        return Err(GlueballOperatorError::CountOverflow);
    }
    let mean = sum / count as f64;
    if !mean.is_finite() {
        return Err(GlueballOperatorError::NonFiniteObservable);
    }
    Ok((mean, count))
}

/// Translation- and cubic-orientation-averaged unsmeared A1++ operator basis
/// on a fixed Euclidean time slice.
pub fn scalar_a1pp_operator_basis(
    lattice: &Su3Lattice4,
    time_slice: usize,
) -> Result<ScalarOperatorBasis, GlueballOperatorError> {
    validate_geometry(lattice, time_slice)?;

    let (spatial_plaquette_1x1, plaquette_loop_count) =
        average_orbit(lattice, time_slice, &PLAQUETTE_ORIENTATIONS, 1, 1)?;
    let (spatial_rectangle_2x1, rectangle_loop_count) =
        average_orbit(lattice, time_slice, &RECTANGLE_ORIENTATIONS, 2, 1)?;

    Ok(ScalarOperatorBasis {
        spatial_plaquette_1x1,
        spatial_rectangle_2x1,
        plaquette_loop_count,
        rectangle_loop_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::su3_lattice::{
        deterministic_su3_site_gauge, single_traceless_su3_link_lattice, Su3Lattice4,
    };

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
        );
    }

    #[test]
    fn phase3a_identity_and_geometry_are_frozen() {
        assert_eq!(OPERATOR_CONTRACT_ID, "YM-GLUEBALL-OPERATORS-V1");
        assert_eq!(OPERATOR_BASIS_ID, "YM-GLUEBALL-SPATIAL-A1PP-V1");
        assert_eq!(EUCLIDEAN_TIME_DIRECTION, 3);
        assert_eq!(SPATIAL_DIRECTIONS, [0, 1, 2]);
        assert_eq!(MINIMUM_SPATIAL_EXTENT, 3);
        assert_eq!(GAUGE_INVARIANCE_TOLERANCE, 2.0e-11);
    }

    #[test]
    fn identity_configuration_gives_exact_unity_on_every_time_slice() {
        let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        for time_slice in 0..3 {
            let basis = scalar_a1pp_operator_basis(&lattice, time_slice).unwrap();
            assert_eq!(basis.spatial_plaquette_1x1, 1.0);
            assert_eq!(basis.spatial_rectangle_2x1, 1.0);
            assert_eq!(basis.plaquette_loop_count, 81);
            assert_eq!(basis.rectangle_loop_count, 162);
        }
    }

    #[test]
    fn basis_is_invariant_under_deterministic_local_gauge_transform() {
        let lattice = single_traceless_su3_link_lattice().unwrap();
        let transformed = lattice
            .gauge_transform(deterministic_su3_site_gauge)
            .unwrap();
        let before = scalar_a1pp_operator_basis(&lattice, 0).unwrap();
        let after = scalar_a1pp_operator_basis(&transformed, 0).unwrap();
        assert_close(
            before.spatial_plaquette_1x1,
            after.spatial_plaquette_1x1,
            GAUGE_INVARIANCE_TOLERANCE,
        );
        assert_close(
            before.spatial_rectangle_2x1,
            after.spatial_rectangle_2x1,
            GAUGE_INVARIANCE_TOLERANCE,
        );
        assert_eq!(before.plaquette_loop_count, after.plaquette_loop_count);
        assert_eq!(before.rectangle_loop_count, after.rectangle_loop_count);
    }

    #[test]
    fn invalid_time_slice_fails_closed() {
        let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        assert_eq!(
            scalar_a1pp_operator_basis(&lattice, 3),
            Err(GlueballOperatorError::TimeSliceOutOfRange)
        );
    }

    #[test]
    fn spatial_extent_two_is_not_admitted_for_the_2x1_basis() {
        let lattice = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();
        assert_eq!(
            scalar_a1pp_operator_basis(&lattice, 0),
            Err(GlueballOperatorError::SpatialExtentTooSmall)
        );
    }
}
