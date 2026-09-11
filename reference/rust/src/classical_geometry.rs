// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 1E finite-dimensional convention checks for oriented Euclidean R^4.
//!
//! This module does not attempt to represent a general manifold, principal
//! bundle, connection, or continuum Yang-Mills solver. It exists only to make
//! the four-dimensional Hodge-star and self-duality sign conventions executable
//! and regression-testable before later formalisation.

use core::fmt;

/// Ordered coefficients of
///
/// `f01 e0^e1 + f02 e0^e2 + f03 e0^e3 + f12 e1^e2 + f13 e1^e3 + f23 e2^e3`
///
/// in an oriented orthonormal Euclidean coframe `(e0,e1,e2,e3)` with
/// `e0^e1^e2^e3` positive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoForm4 {
    components: [f64; 6],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassicalGeometryError {
    NonFiniteComponent,
}

impl fmt::Display for ClassicalGeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteComponent => f.write_str("two-form components must be finite"),
        }
    }
}

impl std::error::Error for ClassicalGeometryError {}

impl TwoForm4 {
    pub const ZERO: Self = Self {
        components: [0.0; 6],
    };

    pub fn from_components(components: [f64; 6]) -> Result<Self, ClassicalGeometryError> {
        if !components.iter().all(|value| value.is_finite()) {
            return Err(ClassicalGeometryError::NonFiniteComponent);
        }
        Ok(Self { components })
    }

    pub const fn components(self) -> [f64; 6] {
        self.components
    }

    /// Hodge star on 2-forms for the frozen oriented Euclidean R^4 fixture:
    ///
    /// *01=23, *02=-13, *03=12, *12=03, *13=-02, *23=01.
    pub const fn hodge_star(self) -> Self {
        let [f01, f02, f03, f12, f13, f23] = self.components;
        Self {
            components: [f23, -f13, f12, f03, -f02, f01],
        }
    }

    pub fn add(self, rhs: Self) -> Self {
        let mut out = [0.0; 6];
        for (index, value) in out.iter_mut().enumerate() {
            *value = self.components[index] + rhs.components[index];
        }
        Self { components: out }
    }

    pub fn sub(self, rhs: Self) -> Self {
        let mut out = [0.0; 6];
        for (index, value) in out.iter_mut().enumerate() {
            *value = self.components[index] - rhs.components[index];
        }
        Self { components: out }
    }

    pub fn scale(self, factor: f64) -> Result<Self, ClassicalGeometryError> {
        if !factor.is_finite() {
            return Err(ClassicalGeometryError::NonFiniteComponent);
        }
        let mut out = [0.0; 6];
        for (index, value) in out.iter_mut().enumerate() {
            *value = self.components[index] * factor;
            if !value.is_finite() {
                return Err(ClassicalGeometryError::NonFiniteComponent);
            }
        }
        Ok(Self { components: out })
    }

    pub fn inner_product(self, rhs: Self) -> f64 {
        self.components
            .iter()
            .zip(rhs.components)
            .map(|(left, right)| left * right)
            .sum()
    }

    /// Coefficient of the positive oriented volume form in `self wedge rhs`.
    pub fn wedge_volume_coefficient(self, rhs: Self) -> f64 {
        let [a01, a02, a03, a12, a13, a23] = self.components;
        let [b01, b02, b03, b12, b13, b23] = rhs.components;
        a01 * b23 + a23 * b01 - a02 * b13 - a13 * b02 + a03 * b12 + a12 * b03
    }

    pub fn self_dual_part(self) -> Result<Self, ClassicalGeometryError> {
        self.add(self.hodge_star()).scale(0.5)
    }

    pub fn anti_self_dual_part(self) -> Result<Self, ClassicalGeometryError> {
        self.sub(self.hodge_star()).scale(0.5)
    }

    pub fn max_residual(self, rhs: Self) -> f64 {
        self.components
            .iter()
            .zip(rhs.components)
            .map(|(left, right)| (left - right).abs())
            .fold(0.0_f64, f64::max)
    }
}
