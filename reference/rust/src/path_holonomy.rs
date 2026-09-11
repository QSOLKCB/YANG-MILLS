// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 1D lattice path/holonomy reference surface.
//!
//! The traversal convention is inherited from frozen `YM-WILSON-LOOP-V1`:
//! segments are multiplied from left to right in traversal order. Thus if
//! `gamma2 o gamma1` means traverse `gamma1` first and then `gamma2`, the
//! corresponding transporter is `U(gamma1) U(gamma2)`.

use core::fmt;

use crate::su3::{Complex64, Su3, Su3Error};
use crate::su3_lattice::{Su3Lattice4, Su3LatticeError, DIM};

pub const MAX_PATH_STEPS: usize = 1 << 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrientedStep {
    Forward(usize),
    Backward(usize),
}

impl OrientedStep {
    pub const fn reversed(self) -> Self {
        match self {
            Self::Forward(mu) => Self::Backward(mu),
            Self::Backward(mu) => Self::Forward(mu),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathHolonomyError {
    EmptyPath,
    PathTooLong,
    PathNotClosed,
    DirectionsMustDiffer,
    ZeroRectangleExtent,
    AllocationFailed,
    Group(Su3Error),
    Lattice(Su3LatticeError),
}

impl fmt::Display for PathHolonomyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyPath => "path must contain at least one oriented step",
            Self::PathTooLong => "path exceeds the bounded Phase 1D step limit",
            Self::PathNotClosed => {
                "closed holonomy requires the final periodic coordinate to equal the start"
            }
            Self::DirectionsMustDiffer => "rectangular path directions must differ",
            Self::ZeroRectangleExtent => "rectangular path extents must be positive",
            Self::AllocationFailed => "path allocation failed",
            Self::Group(_) => "SU(3) path product failed the frozen group acceptance contract",
            Self::Lattice(_) => "SU(3) lattice operation failed during path transport",
        };
        f.write_str(message)
    }
}

impl std::error::Error for PathHolonomyError {}

impl From<Su3Error> for PathHolonomyError {
    fn from(value: Su3Error) -> Self {
        Self::Group(value)
    }
}

impl From<Su3LatticeError> for PathHolonomyError {
    fn from(value: Su3LatticeError) -> Self {
        Self::Lattice(value)
    }
}

fn validate_steps(steps: &[OrientedStep]) -> Result<(), PathHolonomyError> {
    if steps.is_empty() {
        return Err(PathHolonomyError::EmptyPath);
    }
    if steps.len() > MAX_PATH_STEPS {
        return Err(PathHolonomyError::PathTooLong);
    }
    Ok(())
}

fn validate_coordinate(lattice: &Su3Lattice4, x: [usize; DIM]) -> Result<(), PathHolonomyError> {
    let extents = lattice.extents();
    if x.iter()
        .zip(extents)
        .any(|(&value, extent)| value >= extent)
    {
        return Err(PathHolonomyError::Lattice(
            Su3LatticeError::CoordinateOutOfRange,
        ));
    }
    Ok(())
}

fn neighbor_minus(
    lattice: &Su3Lattice4,
    mut x: [usize; DIM],
    direction: usize,
) -> Result<[usize; DIM], PathHolonomyError> {
    if direction >= DIM {
        return Err(PathHolonomyError::Lattice(
            Su3LatticeError::DirectionOutOfRange,
        ));
    }
    validate_coordinate(lattice, x)?;
    let extent = lattice.extents()[direction];
    x[direction] = if x[direction] == 0 {
        extent - 1
    } else {
        x[direction] - 1
    };
    Ok(x)
}

pub fn path_holonomy(
    lattice: &Su3Lattice4,
    start: [usize; DIM],
    steps: &[OrientedStep],
) -> Result<(Su3, [usize; DIM]), PathHolonomyError> {
    validate_steps(steps)?;
    validate_coordinate(lattice, start)?;

    let mut current = start;
    let mut holonomy = Su3::IDENTITY;
    for step in steps {
        let segment = match *step {
            OrientedStep::Forward(direction) => {
                let value = lattice.link(current, direction)?;
                current = lattice.neighbor_plus(current, direction)?;
                value
            }
            OrientedStep::Backward(direction) => {
                current = neighbor_minus(lattice, current, direction)?;
                lattice.link(current, direction)?.dagger()
            }
        };
        holonomy = holonomy.mul(segment)?;
    }
    Ok((holonomy, current))
}

pub fn closed_path_holonomy(
    lattice: &Su3Lattice4,
    start: [usize; DIM],
    steps: &[OrientedStep],
) -> Result<Su3, PathHolonomyError> {
    let (holonomy, end) = path_holonomy(lattice, start, steps)?;
    if end != start {
        return Err(PathHolonomyError::PathNotClosed);
    }
    Ok(holonomy)
}

/// Gauge-invariant normalized complex trace of a closed SU(3) holonomy.
pub fn wilson_loop(
    lattice: &Su3Lattice4,
    start: [usize; DIM],
    steps: &[OrientedStep],
) -> Result<Complex64, PathHolonomyError> {
    Ok(closed_path_holonomy(lattice, start, steps)?
        .matrix()
        .trace()
        .scale(1.0 / 3.0))
}

/// Reverse traversal: reverse the step sequence and reverse each orientation.
pub fn reverse_path(steps: &[OrientedStep]) -> Result<Vec<OrientedStep>, PathHolonomyError> {
    validate_steps(steps)?;
    let mut reversed = Vec::new();
    reversed
        .try_reserve_exact(steps.len())
        .map_err(|_| PathHolonomyError::AllocationFailed)?;
    for step in steps.iter().rev() {
        reversed.push(step.reversed());
    }
    Ok(reversed)
}

/// Concatenate two non-empty traversal sequences, first then second.
pub fn concatenate_paths(
    first: &[OrientedStep],
    second: &[OrientedStep],
) -> Result<Vec<OrientedStep>, PathHolonomyError> {
    validate_steps(first)?;
    validate_steps(second)?;
    let combined = first
        .len()
        .checked_add(second.len())
        .ok_or(PathHolonomyError::PathTooLong)?;
    if combined > MAX_PATH_STEPS {
        return Err(PathHolonomyError::PathTooLong);
    }

    let mut out = Vec::new();
    out.try_reserve_exact(combined)
        .map_err(|_| PathHolonomyError::AllocationFailed)?;
    out.extend_from_slice(first);
    out.extend_from_slice(second);
    Ok(out)
}

pub fn rectangular_path(
    mu: usize,
    nu: usize,
    r: usize,
    t: usize,
) -> Result<Vec<OrientedStep>, PathHolonomyError> {
    if mu >= DIM || nu >= DIM {
        return Err(PathHolonomyError::Lattice(
            Su3LatticeError::DirectionOutOfRange,
        ));
    }
    if mu == nu {
        return Err(PathHolonomyError::DirectionsMustDiffer);
    }
    if r == 0 || t == 0 {
        return Err(PathHolonomyError::ZeroRectangleExtent);
    }
    let perimeter = r
        .checked_add(t)
        .and_then(|value| value.checked_mul(2))
        .ok_or(PathHolonomyError::PathTooLong)?;
    if perimeter > MAX_PATH_STEPS {
        return Err(PathHolonomyError::PathTooLong);
    }

    let mut out = Vec::new();
    out.try_reserve_exact(perimeter)
        .map_err(|_| PathHolonomyError::AllocationFailed)?;
    for _ in 0..r {
        out.push(OrientedStep::Forward(mu));
    }
    for _ in 0..t {
        out.push(OrientedStep::Forward(nu));
    }
    for _ in 0..r {
        out.push(OrientedStep::Backward(mu));
    }
    for _ in 0..t {
        out.push(OrientedStep::Backward(nu));
    }
    Ok(out)
}

/// Endpoint-covariance prediction for the frozen start-left transporter
/// convention: Omega(start) U_gamma Omega(end)^dagger.
pub fn endpoint_covariance_prediction(
    start_gauge: Su3,
    transport: Su3,
    end_gauge: Su3,
) -> Result<Su3, PathHolonomyError> {
    Ok(start_gauge.mul(transport)?.mul(end_gauge.dagger())?)
}
