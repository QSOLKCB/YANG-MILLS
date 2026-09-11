// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 1C reference surface for the abstract cyclic group C3, a fixed
//! non-central three-cycle embedding in SU(3), and the genuine SU(3) center.
//!
//! The abstract group, the non-central permutation subgroup, and the center
//! are deliberately represented as distinct objects even though each is
//! abstractly isomorphic to C3 ~= Z3.

use core::fmt;

use crate::su3::{Complex64, Mat3, Su3, Su3Error, SU3_ACCEPTANCE_TOLERANCE};
use crate::su3_lattice::{Su3Lattice4, Su3LatticeError, DIM};

pub const CENTER_ORDER: u8 = 3;
const SQRT_3_OVER_2: f64 = 0.8660254037844386;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C3 {
    exponent: u8,
}

impl C3 {
    pub const IDENTITY: Self = Self { exponent: 0 };
    pub const GENERATOR: Self = Self { exponent: 1 };

    pub const fn new(exponent: u8) -> Self {
        Self {
            exponent: exponent % CENTER_ORDER,
        }
    }

    pub const fn exponent(self) -> u8 {
        self.exponent
    }

    pub const fn compose(self, rhs: Self) -> Self {
        Self::new(self.exponent + rhs.exponent)
    }

    pub const fn inverse(self) -> Self {
        Self::new(CENTER_ORDER - self.exponent)
    }

    pub const fn pow(self, exponent: u8) -> Self {
        Self::new(self.exponent * (exponent % CENTER_ORDER))
    }
}

pub fn center_phase(element: C3) -> Complex64 {
    match element.exponent() {
        0 => Complex64::ONE,
        1 => Complex64::new(-0.5, SQRT_3_OVER_2),
        2 => Complex64::new(-0.5, -SQRT_3_OVER_2),
        _ => unreachable!("C3 exponent is reduced modulo three"),
    }
}

pub fn center_element(element: C3) -> Result<Su3, Su3Error> {
    let z = center_phase(element);
    Su3::from_checked_matrix(Mat3::from_rows([
        [z, Complex64::ZERO, Complex64::ZERO],
        [Complex64::ZERO, z, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ZERO, z],
    ]))
}

/// Canonical forward three-cycle satisfying Omega |k> = |k+1 mod 3>.
pub fn cycle_generator() -> Result<Su3, Su3Error> {
    Su3::from_checked_matrix(Mat3::from_rows([
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
        [Complex64::ONE, Complex64::ZERO, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ONE, Complex64::ZERO],
    ]))
}

/// Exact SU(3) witness diag(-1,-1,1), used to prove the cycle embedding is
/// non-central without relying on a merely infinitesimal Lie-algebra witness.
pub fn noncentral_witness() -> Result<Su3, Su3Error> {
    Su3::from_checked_matrix(Mat3::from_rows([
        [Complex64::new(-1.0, 0.0), Complex64::ZERO, Complex64::ZERO],
        [Complex64::ZERO, Complex64::new(-1.0, 0.0), Complex64::ZERO],
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
    ]))
}

pub fn cycle_noncentral_residual() -> Result<f64, Su3Error> {
    let omega = cycle_generator()?.matrix();
    let witness = noncentral_witness()?.matrix();
    Ok(omega.mul(witness).max_residual(witness.mul(omega)))
}

pub fn center_commutator_residual(element: C3, value: Su3) -> Result<f64, Su3Error> {
    let center = center_element(element)?.matrix();
    let value = value.matrix();
    Ok(center.mul(value).max_residual(value.mul(center)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Su3CenterError {
    DirectionOutOfRange,
    SliceOutOfRange,
    Group(Su3Error),
    Lattice(Su3LatticeError),
}

impl fmt::Display for Su3CenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::DirectionOutOfRange => "center-transform direction must be in 0..4",
            Self::SliceOutOfRange => "center-transform slice is outside the periodic extent",
            Self::Group(_) => "SU(3) center element failed the frozen group contract",
            Self::Lattice(_) => "SU(3) lattice operation failed during center transformation",
        };
        f.write_str(message)
    }
}

impl std::error::Error for Su3CenterError {}

impl From<Su3Error> for Su3CenterError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

impl From<Su3LatticeError> for Su3CenterError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

/// Multiply every positive-direction link U_mu(x) on x_mu = slice by one
/// genuine center element on the left. Because the factor is central, every
/// plaquette contains cancelling z/z^dagger factors, while a Polyakov loop in
/// direction mu crosses the transformed slice exactly once and acquires z.
pub fn apply_center_transform(
    lattice: &Su3Lattice4,
    direction: usize,
    slice: usize,
    element: C3,
) -> Result<Su3Lattice4, Su3CenterError> {
    if direction >= DIM {
        return Err(Su3CenterError::DirectionOutOfRange);
    }
    let extents = lattice.extents();
    if slice >= extents[direction] {
        return Err(Su3CenterError::SliceOutOfRange);
    }

    let z = center_element(element)?;
    let mut transformed = Su3Lattice4::new_identity(extents)?;
    for x0 in 0..extents[0] {
        for x1 in 0..extents[1] {
            for x2 in 0..extents[2] {
                for x3 in 0..extents[3] {
                    let x = [x0, x1, x2, x3];
                    for mu in 0..DIM {
                        let value = lattice.link(x, mu)?;
                        let updated = if mu == direction && x[direction] == slice {
                            z.mul(value)?
                        } else {
                            value
                        };
                        transformed.set_link(x, mu, updated)?;
                    }
                }
            }
        }
    }
    Ok(transformed)
}

pub fn center_acceptance_tolerance() -> f64 {
    SU3_ACCEPTANCE_TOLERANCE
}
