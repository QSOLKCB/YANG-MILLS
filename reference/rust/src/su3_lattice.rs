// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Minimal Phase 1B SU(3) lattice-link layer built strictly on the frozen
//! `YM-SU3-ALGEBRA-V1` conventions.
//!
//! This layer adds only bounded periodic links, plaquettes, the Wilson action,
//! and local gauge transformations. It does not add Monte Carlo sampling,
//! center-symmetry physics, confinement claims, or glueball observables.

use core::fmt;

use crate::su3::{Complex64, Mat3, Su3, Su3Error, SU3_ACCEPTANCE_TOLERANCE};

pub const DIM: usize = 4;
pub const MAX_SU3_REFERENCE_LINKS: usize = 1 << 18;
const DETERMINISTIC_GAUGE_CODE_MODULUS: usize = 1024;

#[derive(Clone, Debug)]
pub struct Su3Lattice4 {
    extents: [usize; DIM],
    links: Vec<Su3>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Su3LatticeError {
    InvalidLatticeExtent,
    LatticeSizeOverflow,
    LatticeTooLarge,
    AllocationFailed,
    CoordinateOutOfRange,
    DirectionOutOfRange,
    DirectionsMustDiffer,
    NonFiniteBeta,
    NegativeBeta,
    NonFiniteAction,
    Group(Su3Error),
}

impl fmt::Display for Su3LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidLatticeExtent => "every SU(3) lattice extent must be >= 2",
            Self::LatticeSizeOverflow => "SU(3) lattice size arithmetic overflowed",
            Self::LatticeTooLarge => "SU(3) lattice exceeds the bounded reference link limit",
            Self::AllocationFailed => "SU(3) reference lattice allocation failed",
            Self::CoordinateOutOfRange => {
                "SU(3) lattice coordinate is outside its periodic fundamental domain"
            }
            Self::DirectionOutOfRange => "SU(3) lattice direction must be in 0..4",
            Self::DirectionsMustDiffer => "SU(3) plaquette directions must differ",
            Self::NonFiniteBeta => "SU(3) Wilson beta must be finite",
            Self::NegativeBeta => "SU(3) Wilson beta must be non-negative",
            Self::NonFiniteAction => "SU(3) Wilson action produced a non-finite result",
            Self::Group(_) => "SU(3) group operation failed its frozen acceptance contract",
        };
        f.write_str(message)
    }
}

impl std::error::Error for Su3LatticeError {}

impl From<Su3Error> for Su3LatticeError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

fn validate_beta(beta: f64) -> Result<(), Su3LatticeError> {
    if !beta.is_finite() {
        return Err(Su3LatticeError::NonFiniteBeta);
    }
    if beta < 0.0 {
        return Err(Su3LatticeError::NegativeBeta);
    }
    Ok(())
}

fn normalized_real_trace(value: Su3) -> f64 {
    value.matrix().trace().re / 3.0
}

impl Su3Lattice4 {
    pub fn new_identity(extents: [usize; DIM]) -> Result<Self, Su3LatticeError> {
        if extents.iter().any(|&extent| extent < 2) {
            return Err(Su3LatticeError::InvalidLatticeExtent);
        }
        let sites = extents
            .iter()
            .try_fold(1usize, |acc, &extent| acc.checked_mul(extent))
            .ok_or(Su3LatticeError::LatticeSizeOverflow)?;
        let link_count = sites
            .checked_mul(DIM)
            .ok_or(Su3LatticeError::LatticeSizeOverflow)?;
        if link_count > MAX_SU3_REFERENCE_LINKS {
            return Err(Su3LatticeError::LatticeTooLarge);
        }

        let mut links = Vec::new();
        links
            .try_reserve_exact(link_count)
            .map_err(|_| Su3LatticeError::AllocationFailed)?;
        links.resize(link_count, Su3::IDENTITY);
        Ok(Self { extents, links })
    }

    pub const fn extents(&self) -> [usize; DIM] {
        self.extents
    }

    pub fn site_count(&self) -> usize {
        self.links.len() / DIM
    }

    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    fn validate_direction(mu: usize) -> Result<(), Su3LatticeError> {
        if mu >= DIM {
            return Err(Su3LatticeError::DirectionOutOfRange);
        }
        Ok(())
    }

    fn site_index(&self, x: [usize; DIM]) -> Result<usize, Su3LatticeError> {
        if x.iter()
            .zip(self.extents)
            .any(|(&value, extent)| value >= extent)
        {
            return Err(Su3LatticeError::CoordinateOutOfRange);
        }
        Ok((((x[0] * self.extents[1] + x[1]) * self.extents[2] + x[2]) * self.extents[3]) + x[3])
    }

    fn link_index(&self, x: [usize; DIM], mu: usize) -> Result<usize, Su3LatticeError> {
        Self::validate_direction(mu)?;
        Ok(self.site_index(x)? * DIM + mu)
    }

    pub fn neighbor_plus(
        &self,
        mut x: [usize; DIM],
        mu: usize,
    ) -> Result<[usize; DIM], Su3LatticeError> {
        Self::validate_direction(mu)?;
        self.site_index(x)?;
        x[mu] = (x[mu] + 1) % self.extents[mu];
        Ok(x)
    }

    pub fn link(&self, x: [usize; DIM], mu: usize) -> Result<Su3, Su3LatticeError> {
        Ok(self.links[self.link_index(x, mu)?])
    }

    pub fn set_link(
        &mut self,
        x: [usize; DIM],
        mu: usize,
        value: Su3,
    ) -> Result<(), Su3LatticeError> {
        let index = self.link_index(x, mu)?;
        self.links[index] = value;
        Ok(())
    }

    pub fn plaquette(&self, x: [usize; DIM], mu: usize, nu: usize) -> Result<Su3, Su3LatticeError> {
        Self::validate_direction(mu)?;
        Self::validate_direction(nu)?;
        if mu == nu {
            return Err(Su3LatticeError::DirectionsMustDiffer);
        }
        let x_mu = self.neighbor_plus(x, mu)?;
        let x_nu = self.neighbor_plus(x, nu)?;
        Ok(self
            .link(x, mu)?
            .mul(self.link(x_mu, nu)?)?
            .mul(self.link(x_nu, mu)?.dagger())?
            .mul(self.link(x, nu)?.dagger())?)
    }

    pub fn wilson_action(&self, beta: f64) -> Result<f64, Su3LatticeError> {
        validate_beta(beta)?;
        let mut sum = 0.0_f64;

        for x0 in 0..self.extents[0] {
            for x1 in 0..self.extents[1] {
                for x2 in 0..self.extents[2] {
                    for x3 in 0..self.extents[3] {
                        let x = [x0, x1, x2, x3];
                        for mu in 0..DIM {
                            for nu in (mu + 1)..DIM {
                                let plaquette = self.plaquette(x, mu, nu)?;
                                sum += 1.0 - normalized_real_trace(plaquette);
                                if !sum.is_finite() {
                                    return Err(Su3LatticeError::NonFiniteAction);
                                }
                            }
                        }
                    }
                }
            }
        }

        let action = beta * sum;
        if !action.is_finite() {
            return Err(Su3LatticeError::NonFiniteAction);
        }
        Ok(action)
    }

    pub fn gauge_transform<F>(&self, omega: F) -> Result<Self, Su3LatticeError>
    where
        F: Fn([usize; DIM]) -> Su3,
    {
        let mut site_gauges = Vec::new();
        site_gauges
            .try_reserve_exact(self.site_count())
            .map_err(|_| Su3LatticeError::AllocationFailed)?;
        site_gauges.resize(self.site_count(), Su3::IDENTITY);
        self.for_each_site(|x| {
            let index = self.site_index(x).expect("enumerated SU(3) site is valid");
            site_gauges[index] = omega(x);
        });

        let mut transformed = Self::new_identity(self.extents)?;
        let mut failure: Option<Su3LatticeError> = None;
        self.for_each_site(|x| {
            if failure.is_some() {
                return;
            }
            let gx = site_gauges[self.site_index(x).expect("enumerated SU(3) site is valid")];
            for mu in 0..DIM {
                let result = (|| {
                    let xp = self.neighbor_plus(x, mu)?;
                    let gxp = site_gauges[self.site_index(xp)?];
                    let value = gx.mul(self.link(x, mu)?)?.mul(gxp.dagger())?;
                    transformed.set_link(x, mu, value)?;
                    Ok::<(), Su3LatticeError>(())
                })();
                if let Err(error) = result {
                    failure = Some(error);
                    break;
                }
            }
        });
        if let Some(error) = failure {
            return Err(error);
        }
        Ok(transformed)
    }

    pub fn max_link_unitarity_residual(&self) -> f64 {
        self.links
            .iter()
            .map(|link| link.unitarity_residual())
            .fold(0.0_f64, f64::max)
    }

    pub fn max_link_determinant_residual(&self) -> f64 {
        self.links
            .iter()
            .map(|link| link.determinant_residual())
            .fold(0.0_f64, f64::max)
    }

    fn for_each_site<F>(&self, mut f: F)
    where
        F: FnMut([usize; DIM]),
    {
        for x0 in 0..self.extents[0] {
            for x1 in 0..self.extents[1] {
                for x2 in 0..self.extents[2] {
                    for x3 in 0..self.extents[3] {
                        f([x0, x1, x2, x3]);
                    }
                }
            }
        }
    }
}

/// Exact floating-point SU(3) regression fixture: the 3-cycle permutation
/// matrix has entries only in {0,1}, trace zero, determinant +1, and exact
/// unitarity in binary64 arithmetic. No center-symmetry interpretation is made.
pub fn traceless_cycle_link() -> Result<Su3, Su3LatticeError> {
    let matrix = Mat3::from_rows([
        [Complex64::ZERO, Complex64::ONE, Complex64::ZERO],
        [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
        [Complex64::ONE, Complex64::ZERO, Complex64::ZERO],
    ]);
    Ok(Su3::from_checked_matrix(matrix)?)
}

pub fn deterministic_su3_site_gauge(x: [usize; DIM]) -> Su3 {
    let reduced = [
        x[0] % DETERMINISTIC_GAUGE_CODE_MODULUS,
        x[1] % DETERMINISTIC_GAUGE_CODE_MODULUS,
        x[2] % DETERMINISTIC_GAUGE_CODE_MODULUS,
        x[3] % DETERMINISTIC_GAUGE_CODE_MODULUS,
    ];
    let code = (reduced[0] + 3 * reduced[1] + 7 * reduced[2] + 11 * reduced[3])
        % DETERMINISTIC_GAUGE_CODE_MODULUS;
    let mut coefficients = [0.0; 8];
    coefficients[2] = 0.031 * (code as f64 + 1.0);
    Su3::from_algebra_coefficients(coefficients)
        .expect("modulo-bounded deterministic SU(3) gauge coefficient")
}

pub fn single_traceless_su3_link_lattice() -> Result<Su3Lattice4, Su3LatticeError> {
    let mut lattice = Su3Lattice4::new_identity([3, 3, 3, 3])?;
    lattice.set_link([0, 0, 0, 0], 0, traceless_cycle_link()?)?;
    Ok(lattice)
}

pub fn reference_acceptance_tolerance() -> f64 {
    SU3_ACCEPTANCE_TOLERANCE
}
