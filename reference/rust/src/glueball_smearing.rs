// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3B1 spatial stout-link smearing for the Phase 3A glueball operators.
//!
//! This is a deterministic transformation authority only. It does not define
//! connected correlators, fit windows, effective masses, a glueball spectrum,
//! confinement, or a continuum mass-gap claim.

use core::fmt;

use crate::glueball_operators::{
    EUCLIDEAN_TIME_DIRECTION, MINIMUM_SPATIAL_EXTENT, SPATIAL_DIRECTIONS,
};
use crate::su3::{gell_mann, Complex64, Mat3, Su3, Su3Error, SU3_ACCEPTANCE_TOLERANCE, SU3_GENERATORS};
use crate::su3_lattice::{Su3Lattice4, Su3LatticeError, DIM};

pub const SMEARING_CONTRACT_ID: &str = "YM-GLUEBALL-SPATIAL-STOUT-V1";
pub const TRANSFORMATION_ID: &str = "YM-SPATIAL-STOUT-SMEAR-V1";
pub const CANONICAL_RHO: f64 = 0.10;
pub const MAX_RHO: f64 = 0.25;
pub const MAX_LEVELS: usize = 8;
pub const ALGEBRA_RESIDUAL_TOLERANCE: f64 = 2.0e-12;
pub const GAUGE_COVARIANCE_TOLERANCE: f64 = 5.0e-11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlueballSmearingError {
    NonFiniteRho,
    RhoOutOfRange,
    TooManyLevels,
    SpatialExtentTooSmall,
    SpatialExtentsNotCubic,
    NonFiniteStaple,
    AlgebraProjectionResidual,
    Group(Su3Error),
    Lattice(Su3LatticeError),
}

impl fmt::Display for GlueballSmearingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NonFiniteRho => "Phase 3B1 stout rho must be finite",
            Self::RhoOutOfRange => "Phase 3B1 stout rho must satisfy 0 < rho <= 0.25",
            Self::TooManyLevels => "Phase 3B1 stout level count exceeds the bounded reference limit",
            Self::SpatialExtentTooSmall => {
                "Phase 3B1 spatial stout smearing requires every spatial extent to be at least 3"
            }
            Self::SpatialExtentsNotCubic => {
                "Phase 3B1 spatial stout smearing requires equal spatial extents"
            }
            Self::NonFiniteStaple => "Phase 3B1 stout staple or algebra element became non-finite",
            Self::AlgebraProjectionResidual => {
                "Phase 3B1 stout algebra projection exceeded its Hermitian/traceless residual tolerance"
            }
            Self::Group(_) => "Phase 3B1 stout group operation failed",
            Self::Lattice(_) => "Phase 3B1 stout lattice operation failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for GlueballSmearingError {}

impl From<Su3Error> for GlueballSmearingError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

impl From<Su3LatticeError> for GlueballSmearingError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

fn validate_parameters(lattice: &Su3Lattice4, rho: f64, levels: usize) -> Result<(), GlueballSmearingError> {
    if !rho.is_finite() {
        return Err(GlueballSmearingError::NonFiniteRho);
    }
    if rho <= 0.0 || rho > MAX_RHO {
        return Err(GlueballSmearingError::RhoOutOfRange);
    }
    if levels > MAX_LEVELS {
        return Err(GlueballSmearingError::TooManyLevels);
    }
    let extents = lattice.extents();
    if SPATIAL_DIRECTIONS
        .iter()
        .any(|&direction| extents[direction] < MINIMUM_SPATIAL_EXTENT)
    {
        return Err(GlueballSmearingError::SpatialExtentTooSmall);
    }
    if extents[0] != extents[1] || extents[1] != extents[2] {
        return Err(GlueballSmearingError::SpatialExtentsNotCubic);
    }
    Ok(())
}

fn neighbor_minus(
    lattice: &Su3Lattice4,
    mut x: [usize; DIM],
    direction: usize,
) -> Result<[usize; DIM], GlueballSmearingError> {
    if direction >= DIM {
        return Err(GlueballSmearingError::Lattice(
            Su3LatticeError::DirectionOutOfRange,
        ));
    }
    let extents = lattice.extents();
    if x.iter().zip(extents).any(|(&value, extent)| value >= extent) {
        return Err(GlueballSmearingError::Lattice(
            Su3LatticeError::CoordinateOutOfRange,
        ));
    }
    x[direction] = if x[direction] == 0 {
        extents[direction] - 1
    } else {
        x[direction] - 1
    };
    Ok(x)
}

fn spatial_staple_sum(
    lattice: &Su3Lattice4,
    x: [usize; DIM],
    mu: usize,
    rho: f64,
) -> Result<Mat3, GlueballSmearingError> {
    let mut sum = Mat3::ZERO;
    for &nu in &SPATIAL_DIRECTIONS {
        if nu == mu {
            continue;
        }

        let x_plus_mu = lattice.neighbor_plus(x, mu)?;
        let x_plus_nu = lattice.neighbor_plus(x, nu)?;
        let forward = lattice
            .link(x, nu)?
            .mul(lattice.link(x_plus_nu, mu)?)?
            .mul(lattice.link(x_plus_mu, nu)?.dagger())?;
        sum = sum.add(forward.matrix());

        let x_minus_nu = neighbor_minus(lattice, x, nu)?;
        let x_minus_nu_plus_mu = lattice.neighbor_plus(x_minus_nu, mu)?;
        let backward = lattice
            .link(x_minus_nu, nu)?
            .dagger()
            .mul(lattice.link(x_minus_nu, mu)?)?
            .mul(lattice.link(x_minus_nu_plus_mu, nu)?)?;
        sum = sum.add(backward.matrix());
    }

    let weighted = sum.scale_real(rho);
    if !weighted.is_finite() {
        return Err(GlueballSmearingError::NonFiniteStaple);
    }
    Ok(weighted)
}

fn stout_q(staple: Mat3, link: Su3) -> Result<Mat3, GlueballSmearingError> {
    let omega = staple.mul(link.dagger().matrix());
    if !omega.is_finite() {
        return Err(GlueballSmearingError::NonFiniteStaple);
    }

    let anti = omega.dagger().sub(omega);
    let first = anti.scale_complex(Complex64::new(0.0, 0.5));
    let trace_correction = anti.trace().mul(Complex64::I).scale(1.0 / 6.0);
    let q = first.sub(Mat3::IDENTITY.scale_complex(trace_correction));
    if !q.is_finite() {
        return Err(GlueballSmearingError::NonFiniteStaple);
    }
    if q.hermiticity_residual() > ALGEBRA_RESIDUAL_TOLERANCE
        || q.trace().abs() > ALGEBRA_RESIDUAL_TOLERANCE
    {
        return Err(GlueballSmearingError::AlgebraProjectionResidual);
    }
    Ok(q)
}

fn q_coefficients(q: Mat3) -> Result<[f64; SU3_GENERATORS], GlueballSmearingError> {
    let mut coefficients = [0.0; SU3_GENERATORS];
    for (index, coefficient) in coefficients.iter_mut().enumerate() {
        let projection = q.mul(gell_mann(index)?).trace();
        if !projection.is_finite() || projection.im.abs() > ALGEBRA_RESIDUAL_TOLERANCE {
            return Err(GlueballSmearingError::AlgebraProjectionResidual);
        }
        *coefficient = projection.re;
    }
    Ok(coefficients)
}

fn smear_one_level(lattice: &Su3Lattice4, rho: f64) -> Result<Su3Lattice4, GlueballSmearingError> {
    let extents = lattice.extents();
    let mut out = lattice.clone();
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for &mu in &SPATIAL_DIRECTIONS {
                        let old = lattice.link(x, mu)?;
                        let staple = spatial_staple_sum(lattice, x, mu, rho)?;
                        let q = stout_q(staple, old)?;
                        let rotation = Su3::from_algebra_coefficients(q_coefficients(q)?)?;
                        let updated = rotation.mul(old)?;
                        if updated.unitarity_residual() > SU3_ACCEPTANCE_TOLERANCE
                            || updated.determinant_residual() > SU3_ACCEPTANCE_TOLERANCE
                        {
                            return Err(GlueballSmearingError::Group(Su3Error::NonUnitaryMatrix));
                        }
                        out.set_link(x, mu, updated)?;
                    }
                }
            }
        }
    }
    Ok(out)
}

/// Apply bounded synchronous spatial stout-smearing levels.
///
/// Each level is computed entirely from the previous level. Temporal links are
/// copied unchanged at every level.
pub fn spatial_stout_smear(
    lattice: &Su3Lattice4,
    rho: f64,
    levels: usize,
) -> Result<Su3Lattice4, GlueballSmearingError> {
    validate_parameters(lattice, rho, levels)?;
    let mut current = lattice.clone();
    for _ in 0..levels {
        current = smear_one_level(&current, rho)?;
    }
    Ok(current)
}

pub fn max_link_residual(
    first: &Su3Lattice4,
    second: &Su3Lattice4,
) -> Result<f64, GlueballSmearingError> {
    if first.extents() != second.extents() {
        return Err(GlueballSmearingError::Lattice(
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
                            return Err(GlueballSmearingError::NonFiniteStaple);
                        }
                        maximum = maximum.max(residual);
                    }
                }
            }
        }
    }
    Ok(maximum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glueball_operators::scalar_a1pp_operator_basis;
    use crate::su3_lattice::{
        deterministic_su3_site_gauge, single_traceless_su3_link_lattice, Su3Lattice4,
    };

    #[test]
    fn phase3b1_parameters_are_frozen() {
        assert_eq!(SMEARING_CONTRACT_ID, "YM-GLUEBALL-SPATIAL-STOUT-V1");
        assert_eq!(TRANSFORMATION_ID, "YM-SPATIAL-STOUT-SMEAR-V1");
        assert_eq!(CANONICAL_RHO, 0.10);
        assert_eq!(MAX_RHO, 0.25);
        assert_eq!(MAX_LEVELS, 8);
        assert_eq!(EUCLIDEAN_TIME_DIRECTION, 3);
        assert_eq!(SPATIAL_DIRECTIONS, [0, 1, 2]);
    }

    #[test]
    fn identity_is_exact_fixed_point() {
        let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, 3).unwrap();
        assert_eq!(max_link_residual(&lattice, &smeared).unwrap(), 0.0);
        let basis = scalar_a1pp_operator_basis(&smeared, 0).unwrap();
        assert_eq!(basis.spatial_plaquette_1x1, 1.0);
        assert_eq!(basis.spatial_rectangle_2x1, 1.0);
    }

    #[test]
    fn zero_levels_is_exact_noop() {
        let lattice = single_traceless_su3_link_lattice().unwrap();
        let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, 0).unwrap();
        assert_eq!(max_link_residual(&lattice, &smeared).unwrap(), 0.0);
    }

    #[test]
    fn temporal_links_remain_exactly_unchanged() {
        let lattice = single_traceless_su3_link_lattice().unwrap();
        let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, 1).unwrap();
        let extents = lattice.extents();
        for x0 in 0..extents[0] {
            for x1 in 0..extents[1] {
                for x2 in 0..extents[2] {
                    for x3 in 0..extents[3] {
                        let x = [x0, x1, x2, x3];
                        assert_eq!(
                            lattice.link(x, EUCLIDEAN_TIME_DIRECTION).unwrap(),
                            smeared.link(x, EUCLIDEAN_TIME_DIRECTION).unwrap()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn stout_smearing_is_gauge_covariant() {
        let lattice = single_traceless_su3_link_lattice().unwrap();
        let transformed = lattice
            .gauge_transform(deterministic_su3_site_gauge)
            .unwrap();
        let smeared_then_transformed = spatial_stout_smear(&lattice, CANONICAL_RHO, 1)
            .unwrap()
            .gauge_transform(deterministic_su3_site_gauge)
            .unwrap();
        let transformed_then_smeared =
            spatial_stout_smear(&transformed, CANONICAL_RHO, 1).unwrap();
        let residual = max_link_residual(&smeared_then_transformed, &transformed_then_smeared)
            .unwrap();
        assert!(residual <= GAUGE_COVARIANCE_TOLERANCE, "residual={residual:.17e}");
    }

    #[test]
    fn smeared_links_remain_in_su3() {
        let lattice = single_traceless_su3_link_lattice().unwrap();
        let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, MAX_LEVELS).unwrap();
        assert!(smeared.max_link_unitarity_residual() <= SU3_ACCEPTANCE_TOLERANCE);
        assert!(smeared.max_link_determinant_residual() <= SU3_ACCEPTANCE_TOLERANCE);
    }

    #[test]
    fn invalid_parameter_and_geometry_surfaces_fail_closed() {
        let lattice = Su3Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        assert_eq!(
            spatial_stout_smear(&lattice, f64::NAN, 1),
            Err(GlueballSmearingError::NonFiniteRho)
        );
        assert_eq!(
            spatial_stout_smear(&lattice, 0.0, 1),
            Err(GlueballSmearingError::RhoOutOfRange)
        );
        assert_eq!(
            spatial_stout_smear(&lattice, MAX_RHO + 1.0e-6, 1),
            Err(GlueballSmearingError::RhoOutOfRange)
        );
        assert_eq!(
            spatial_stout_smear(&lattice, CANONICAL_RHO, MAX_LEVELS + 1),
            Err(GlueballSmearingError::TooManyLevels)
        );

        let too_small = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();
        assert_eq!(
            spatial_stout_smear(&too_small, CANONICAL_RHO, 1),
            Err(GlueballSmearingError::SpatialExtentTooSmall)
        );

        let anisotropic = Su3Lattice4::new_identity([3, 4, 5, 3]).unwrap();
        assert_eq!(
            spatial_stout_smear(&anisotropic, CANONICAL_RHO, 1),
            Err(GlueballSmearingError::SpatialExtentsNotCubic)
        );
    }
}
