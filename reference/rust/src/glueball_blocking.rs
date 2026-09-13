// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3B2 deterministic stride-2 spatial blocking.
//!
//! This surface defines a finite-lattice coarse-link transformation only. It
//! does not construct connected correlators, extract a glueball mass, take a
//! continuum limit, or establish the Yang--Mills mass gap.

use core::fmt;

use crate::glueball_operators::MINIMUM_SPATIAL_EXTENT;
use crate::su3::{Su3Error, SU3_ACCEPTANCE_TOLERANCE};
use crate::su3_lattice::{traceless_cycle_link, Su3Lattice4, Su3LatticeError, DIM};

pub const BLOCKING_CONTRACT_ID: &str = "YM-GLUEBALL-SPATIAL-BLOCK-V1";
pub const TRANSFORMATION_ID: &str = "YM-SPATIAL-STRIDE2-BLOCK-V1";
pub const BLOCKING_STATUS: &str = "phase-3b2-candidate";
pub const BLOCK_FACTOR: usize = 2;
pub const MINIMUM_FINE_SPATIAL_EXTENT: usize = BLOCK_FACTOR * MINIMUM_SPATIAL_EXTENT;
pub const GAUGE_COVARIANCE_TOLERANCE: f64 = 5.0e-11;
pub const FROZEN_OUTPUT_ABS_TOLERANCE: f64 = 1.0e-12;
pub const FROZEN_COARSE_PLAQUETTE_1X1: f64 = 0.9506172839506173;
pub const FROZEN_COARSE_RECTANGLE_2X1: f64 = 0.9259259259259259;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlueballBlockingError {
    SpatialExtentTooSmall,
    SpatialExtentsNotCubic,
    SpatialExtentNotEven,
    NonFiniteResidual,
    Group(Su3Error),
    Lattice(Su3LatticeError),
}

impl fmt::Display for GlueballBlockingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::SpatialExtentTooSmall => {
                "Phase 3B2 blocking requires every fine spatial extent to be at least 6"
            }
            Self::SpatialExtentsNotCubic => {
                "Phase 3B2 blocking requires equal fine spatial extents"
            }
            Self::SpatialExtentNotEven => {
                "Phase 3B2 stride-2 blocking requires even fine spatial extents"
            }
            Self::NonFiniteResidual => "Phase 3B2 blocking residual became non-finite",
            Self::Group(_) => "Phase 3B2 blocking group operation failed",
            Self::Lattice(_) => "Phase 3B2 blocking lattice operation failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for GlueballBlockingError {}

impl From<Su3Error> for GlueballBlockingError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

impl From<Su3LatticeError> for GlueballBlockingError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

fn validate_geometry(lattice: &Su3Lattice4) -> Result<(), GlueballBlockingError> {
    let extents = lattice.extents();
    if extents[0] != extents[1] || extents[1] != extents[2] {
        return Err(GlueballBlockingError::SpatialExtentsNotCubic);
    }
    if extents[0] < MINIMUM_FINE_SPATIAL_EXTENT {
        return Err(GlueballBlockingError::SpatialExtentTooSmall);
    }
    if extents[0] % BLOCK_FACTOR != 0 {
        return Err(GlueballBlockingError::SpatialExtentNotEven);
    }
    Ok(())
}

pub fn blocked_extents(lattice: &Su3Lattice4) -> Result<[usize; DIM], GlueballBlockingError> {
    validate_geometry(lattice)?;
    let extents = lattice.extents();
    Ok([
        extents[0] / BLOCK_FACTOR,
        extents[1] / BLOCK_FACTOR,
        extents[2] / BLOCK_FACTOR,
        extents[3],
    ])
}

pub const fn fine_site_from_coarse(y: [usize; DIM]) -> [usize; DIM] {
    [
        BLOCK_FACTOR * y[0],
        BLOCK_FACTOR * y[1],
        BLOCK_FACTOR * y[2],
        y[3],
    ]
}

/// Deterministic stride-2 spatial coarse-link map.
///
/// For spatial `mu`, `V_mu(y)=U_mu(x) U_mu(x+mu)` with
/// `x=(2y0,2y1,2y2,t)`. The temporal link is sampled exactly as
/// `V_3(y)=U_3(x)`. No projection, smearing, or averaging is performed.
pub fn spatial_stride2_block(lattice: &Su3Lattice4) -> Result<Su3Lattice4, GlueballBlockingError> {
    let coarse_extents = blocked_extents(lattice)?;
    let mut coarse = Su3Lattice4::new_identity(coarse_extents)?;

    for y0 in 0..coarse_extents[0] {
        for y1 in 0..coarse_extents[1] {
            for y2 in 0..coarse_extents[2] {
                for y3 in 0..coarse_extents[3] {
                    let y = [y0, y1, y2, y3];
                    let x = fine_site_from_coarse(y);
                    for mu in 0..3 {
                        let x_plus_mu = lattice.neighbor_plus(x, mu)?;
                        let blocked = lattice.link(x, mu)?.mul(lattice.link(x_plus_mu, mu)?)?;
                        if blocked.unitarity_residual() > SU3_ACCEPTANCE_TOLERANCE
                            || blocked.determinant_residual() > SU3_ACCEPTANCE_TOLERANCE
                        {
                            return Err(GlueballBlockingError::Group(Su3Error::NonUnitaryMatrix));
                        }
                        coarse.set_link(y, mu, blocked)?;
                    }
                    coarse.set_link(y, 3, lattice.link(x, 3)?)?;
                }
            }
        }
    }
    Ok(coarse)
}

pub fn max_link_residual(
    first: &Su3Lattice4,
    second: &Su3Lattice4,
) -> Result<f64, GlueballBlockingError> {
    if first.extents() != second.extents() {
        return Err(GlueballBlockingError::Lattice(
            Su3LatticeError::InvalidLatticeExtent,
        ));
    }
    let extents = first.extents();
    let mut maximum = 0.0_f64;
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        let residual = first
                            .link(x, mu)?
                            .matrix()
                            .max_residual(second.link(x, mu)?.matrix());
                        if !residual.is_finite() {
                            return Err(GlueballBlockingError::NonFiniteResidual);
                        }
                        maximum = maximum.max(residual);
                    }
                }
            }
        }
    }
    Ok(maximum)
}

pub fn deterministic_blocking_fixture() -> Result<Su3Lattice4, GlueballBlockingError> {
    let mut lattice = Su3Lattice4::new_identity([6, 6, 6, 3])?;
    lattice.set_link([0, 0, 0, 0], 0, traceless_cycle_link()?)?;
    Ok(lattice)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glueball_operators::scalar_a1pp_operator_basis;
    use crate::su3_lattice::{deterministic_su3_site_gauge, traceless_cycle_link};

    #[test]
    fn phase3b2_identity_and_geometry_are_frozen() {
        assert_eq!(BLOCKING_CONTRACT_ID, "YM-GLUEBALL-SPATIAL-BLOCK-V1");
        assert_eq!(TRANSFORMATION_ID, "YM-SPATIAL-STRIDE2-BLOCK-V1");
        assert_eq!(BLOCKING_STATUS, "phase-3b2-candidate");
        assert_eq!(BLOCK_FACTOR, 2);
        assert_eq!(MINIMUM_FINE_SPATIAL_EXTENT, 6);
        assert_eq!(GAUGE_COVARIANCE_TOLERANCE, 5.0e-11);
        assert_eq!(FROZEN_OUTPUT_ABS_TOLERANCE, 1.0e-12);
    }

    #[test]
    fn identity_blocks_exactly_to_identity() {
        let fine = Su3Lattice4::new_identity([6, 6, 6, 3]).unwrap();
        let coarse = spatial_stride2_block(&fine).unwrap();
        assert_eq!(coarse.extents(), [3, 3, 3, 3]);
        let identity = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        assert_eq!(max_link_residual(&coarse, &identity).unwrap(), 0.0);
    }

    #[test]
    fn canonical_nontrivial_output_is_frozen() {
        let fine = deterministic_blocking_fixture().unwrap();
        let coarse = spatial_stride2_block(&fine).unwrap();
        let basis = scalar_a1pp_operator_basis(&coarse, 0).unwrap();
        assert!(
            (basis.spatial_plaquette_1x1 - FROZEN_COARSE_PLAQUETTE_1X1).abs()
                <= FROZEN_OUTPUT_ABS_TOLERANCE
        );
        assert!(
            (basis.spatial_rectangle_2x1 - FROZEN_COARSE_RECTANGLE_2X1).abs()
                <= FROZEN_OUTPUT_ABS_TOLERANCE
        );
    }

    #[test]
    fn temporal_link_is_sampled_exactly() {
        let mut fine = Su3Lattice4::new_identity([6, 6, 6, 3]).unwrap();
        let temporal = traceless_cycle_link().unwrap();
        fine.set_link([0, 0, 0, 0], 3, temporal).unwrap();
        let coarse = spatial_stride2_block(&fine).unwrap();
        assert_eq!(coarse.link([0, 0, 0, 0], 3).unwrap(), temporal);
    }

    #[test]
    fn blocking_is_gauge_covariant_under_coarse_site_restriction() {
        let fine = deterministic_blocking_fixture().unwrap();
        let transformed = fine.gauge_transform(deterministic_su3_site_gauge).unwrap();
        let blocked_transformed = spatial_stride2_block(&transformed).unwrap();
        let blocked = spatial_stride2_block(&fine).unwrap();
        let blocked_then_transformed = blocked
            .gauge_transform(|y| deterministic_su3_site_gauge(fine_site_from_coarse(y)))
            .unwrap();
        let residual = max_link_residual(&blocked_transformed, &blocked_then_transformed).unwrap();
        assert!(
            residual <= GAUGE_COVARIANCE_TOLERANCE,
            "residual={residual:.17e}"
        );
    }

    #[test]
    fn blocked_links_remain_in_su3() {
        let fine = deterministic_blocking_fixture().unwrap();
        let coarse = spatial_stride2_block(&fine).unwrap();
        assert!(coarse.max_link_unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
        assert!(coarse.max_link_determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    }

    #[test]
    fn invalid_geometry_fails_closed() {
        let too_small = Su3Lattice4::new_identity([4, 4, 4, 3]).unwrap();
        assert_eq!(
            spatial_stride2_block(&too_small).unwrap_err(),
            GlueballBlockingError::SpatialExtentTooSmall
        );

        let odd = Su3Lattice4::new_identity([7, 7, 7, 3]).unwrap();
        assert_eq!(
            spatial_stride2_block(&odd).unwrap_err(),
            GlueballBlockingError::SpatialExtentNotEven
        );

        let anisotropic = Su3Lattice4::new_identity([6, 8, 6, 3]).unwrap();
        assert_eq!(
            spatial_stride2_block(&anisotropic).unwrap_err(),
            GlueballBlockingError::SpatialExtentsNotCubic
        );
    }
}
