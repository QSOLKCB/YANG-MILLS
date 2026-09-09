// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Minimal Phase 0 SU(2) Wilson-lattice reference core.
//!
//! The mathematical convention is
//! `U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)`.
//! Unit-quaternion multiplication therefore implements SU(2) matrix
//! multiplication, and quaternion conjugation implements Hermitian conjugation.

use core::fmt;

pub const DIM: usize = 4;
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Su2 {
    c: [f64; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceError {
    NonFiniteSu2Component,
    ZeroSu2Norm,
    InvalidAxis,
    InvalidLatticeExtent,
    CoordinateOutOfRange,
    DirectionOutOfRange,
    DirectionsMustDiffer,
    NonFiniteBeta,
    NegativeBeta,
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NonFiniteSu2Component => "SU(2) components must be finite",
            Self::ZeroSu2Norm => "SU(2) components must have non-zero norm",
            Self::InvalidAxis => "axis must be finite and have non-zero norm",
            Self::InvalidLatticeExtent => "every lattice extent must be >= 2",
            Self::CoordinateOutOfRange => {
                "lattice coordinate is outside its periodic fundamental domain"
            }
            Self::DirectionOutOfRange => "lattice direction must be in 0..4",
            Self::DirectionsMustDiffer => "plaquette directions must differ",
            Self::NonFiniteBeta => "beta must be finite",
            Self::NegativeBeta => "beta must be non-negative",
        };
        f.write_str(message)
    }
}

impl std::error::Error for ReferenceError {}

fn normalize_scaled<const N: usize>(values: [f64; N]) -> Option<[f64; N]> {
    let scale = values
        .iter()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return None;
    }

    let mut scaled = [0.0; N];
    for index in 0..N {
        scaled[index] = values[index] / scale;
    }

    let norm = scaled.iter().map(|value| value * value).sum::<f64>().sqrt();
    if norm == 0.0 || !norm.is_finite() {
        return None;
    }

    let mut normalized = [0.0; N];
    for index in 0..N {
        normalized[index] = scaled[index] / norm;
    }
    Some(normalized)
}

impl Su2 {
    pub const IDENTITY: Self = Self {
        c: [1.0, 0.0, 0.0, 0.0],
    };

    pub fn from_normalized_components(c: [f64; 4]) -> Result<Self, ReferenceError> {
        if !c.iter().all(|v| v.is_finite()) {
            return Err(ReferenceError::NonFiniteSu2Component);
        }
        let c = normalize_scaled(c).ok_or(ReferenceError::ZeroSu2Norm)?;
        Ok(Self { c })
    }

    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Result<Self, ReferenceError> {
        if !angle.is_finite() || !axis.iter().all(|v| v.is_finite()) {
            return Err(ReferenceError::InvalidAxis);
        }
        let axis = normalize_scaled(axis).ok_or(ReferenceError::InvalidAxis)?;
        let half = 0.5 * angle;
        let s = half.sin();
        Ok(Self {
            c: [
                half.cos(),
                s * axis[0],
                s * axis[1],
                s * axis[2],
            ],
        })
    }

    pub const fn components(self) -> [f64; 4] {
        self.c
    }

    pub fn norm_sq(self) -> f64 {
        self.c.iter().map(|v| v * v).sum()
    }

    pub fn is_unit(self, tolerance: f64) -> bool {
        (self.norm_sq() - 1.0).abs() <= tolerance
    }

    pub const fn dagger(self) -> Self {
        Self {
            c: [self.c[0], -self.c[1], -self.c[2], -self.c[3]],
        }
    }

    pub fn mul(self, rhs: Self) -> Self {
        let [a0, a1, a2, a3] = self.c;
        let [b0, b1, b2, b3] = rhs.c;
        Self {
            c: [
                a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
                a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
                a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
                a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0,
            ],
        }
    }

    pub const fn half_real_trace(self) -> f64 {
        self.c[0]
    }

    pub const fn real_trace(self) -> f64 {
        2.0 * self.c[0]
    }
}

#[derive(Clone, Debug)]
pub struct Lattice4 {
    extents: [usize; DIM],
    links: Vec<Su2>,
}

impl Lattice4 {
    pub fn new_identity(extents: [usize; DIM]) -> Result<Self, ReferenceError> {
        if extents.iter().any(|&n| n < 2) {
            return Err(ReferenceError::InvalidLatticeExtent);
        }
        let sites = extents
            .iter()
            .try_fold(1usize, |acc, &n| acc.checked_mul(n))
            .ok_or(ReferenceError::InvalidLatticeExtent)?;
        let link_count = sites
            .checked_mul(DIM)
            .ok_or(ReferenceError::InvalidLatticeExtent)?;
        Ok(Self {
            extents,
            links: vec![Su2::IDENTITY; link_count],
        })
    }

    pub const fn extents(&self) -> [usize; DIM] {
        self.extents
    }

    pub fn site_count(&self) -> usize {
        self.links.len() / DIM
    }

    fn validate_direction(mu: usize) -> Result<(), ReferenceError> {
        if mu >= DIM {
            return Err(ReferenceError::DirectionOutOfRange);
        }
        Ok(())
    }

    fn site_index(&self, x: [usize; DIM]) -> Result<usize, ReferenceError> {
        if x.iter().zip(self.extents).any(|(&v, n)| v >= n) {
            return Err(ReferenceError::CoordinateOutOfRange);
        }
        Ok((((x[0] * self.extents[1] + x[1]) * self.extents[2] + x[2]) * self.extents[3]) + x[3])
    }

    fn link_index(&self, x: [usize; DIM], mu: usize) -> Result<usize, ReferenceError> {
        Self::validate_direction(mu)?;
        Ok(self.site_index(x)? * DIM + mu)
    }

    pub fn neighbor_plus(
        &self,
        mut x: [usize; DIM],
        mu: usize,
    ) -> Result<[usize; DIM], ReferenceError> {
        Self::validate_direction(mu)?;
        self.site_index(x)?;
        x[mu] = (x[mu] + 1) % self.extents[mu];
        Ok(x)
    }

    pub fn link(&self, x: [usize; DIM], mu: usize) -> Result<Su2, ReferenceError> {
        Ok(self.links[self.link_index(x, mu)?])
    }

    pub fn set_link(
        &mut self,
        x: [usize; DIM],
        mu: usize,
        value: Su2,
    ) -> Result<(), ReferenceError> {
        let index = self.link_index(x, mu)?;
        self.links[index] = value;
        Ok(())
    }

    pub fn plaquette(&self, x: [usize; DIM], mu: usize, nu: usize) -> Result<Su2, ReferenceError> {
        Self::validate_direction(mu)?;
        Self::validate_direction(nu)?;
        if mu == nu {
            return Err(ReferenceError::DirectionsMustDiffer);
        }
        let x_mu = self.neighbor_plus(x, mu)?;
        let x_nu = self.neighbor_plus(x, nu)?;
        Ok(self
            .link(x, mu)?
            .mul(self.link(x_mu, nu)?)
            .mul(self.link(x_nu, mu)?.dagger())
            .mul(self.link(x, nu)?.dagger()))
    }

    pub fn wilson_action(&self, beta: f64) -> Result<f64, ReferenceError> {
        if !beta.is_finite() {
            return Err(ReferenceError::NonFiniteBeta);
        }
        if beta < 0.0 {
            return Err(ReferenceError::NegativeBeta);
        }

        let mut sum = 0.0;
        self.for_each_site(|x| {
            for mu in 0..DIM {
                for nu in (mu + 1)..DIM {
                    let p = self
                        .plaquette(x, mu, nu)
                        .expect("valid plaquette directions");
                    sum += 1.0 - p.half_real_trace();
                }
            }
        });
        Ok(beta * sum)
    }

    pub fn gauge_transform<F>(&self, omega: F) -> Result<Self, ReferenceError>
    where
        F: Fn([usize; DIM]) -> Su2,
    {
        let mut site_gauges = vec![Su2::IDENTITY; self.site_count()];
        self.for_each_site(|x| {
            let index = self.site_index(x).expect("enumerated site is valid");
            site_gauges[index] = omega(x);
        });

        let mut transformed = Self::new_identity(self.extents)?;
        self.for_each_site(|x| {
            let x_index = self.site_index(x).expect("enumerated site is valid");
            let gx = site_gauges[x_index];
            for mu in 0..DIM {
                let xp = self.neighbor_plus(x, mu).expect("valid direction");
                let xp_index = self.site_index(xp).expect("periodic neighbor is valid");
                let value = gx
                    .mul(self.link(x, mu).expect("valid direction"))
                    .mul(site_gauges[xp_index].dagger());
                transformed.set_link(x, mu, value).expect("valid direction");
            }
        });
        Ok(transformed)
    }

    pub fn max_link_unit_residual(&self) -> f64 {
        self.links
            .iter()
            .map(|u| (u.norm_sq() - 1.0).abs())
            .fold(0.0, f64::max)
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

pub fn deterministic_site_gauge(x: [usize; DIM]) -> Su2 {
    let code = x[0] + 3 * x[1] + 7 * x[2] + 11 * x[3];
    let angle = 0.137 * (code as f64 + 1.0);
    Su2::from_axis_angle([1.0, -0.5, 0.25], angle).expect("fixed valid axis")
}

pub fn deterministic_nontrivial_field() -> Lattice4 {
    let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).expect("fixed valid lattice");
    let samples = [
        ([0, 0, 0, 0], 0, [1.0, 0.0, 0.0], 0.41),
        ([1, 2, 0, 1], 1, [0.0, 1.0, 0.0], -0.73),
        ([2, 1, 2, 0], 2, [0.0, 0.0, 1.0], 1.11),
        ([0, 2, 1, 2], 3, [1.0, 1.0, -1.0], 0.29),
    ];
    for (x, mu, axis, angle) in samples {
        lattice
            .set_link(
                x,
                mu,
                Su2::from_axis_angle(axis, angle).expect("fixed valid axis-angle"),
            )
            .expect("fixed valid direction");
    }
    lattice
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
        );
    }

    #[test]
    fn normalized_quaternion_is_unit() {
        let q = Su2::from_normalized_components([1.0, 2.0, -3.0, 4.0]).unwrap();
        assert_close(q.norm_sq(), 1.0, DEFAULT_TOLERANCE);
    }

    #[test]
    fn normalization_handles_extreme_finite_magnitudes() {
        for magnitude in [f64::MAX, f64::MIN_POSITIVE] {
            let q = Su2::from_normalized_components([magnitude, 0.0, 0.0, 0.0]).unwrap();
            assert_eq!(q.components(), [1.0, 0.0, 0.0, 0.0]);

            let axis_q = Su2::from_axis_angle([magnitude, 0.0, 0.0], 0.73).unwrap();
            assert_close(axis_q.norm_sq(), 1.0, DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn multiplication_and_dagger_give_identity() {
        let q = Su2::from_axis_angle([1.0, 2.0, 3.0], 0.91).unwrap();
        let product = q.mul(q.dagger());
        let c = product.components();
        assert_close(c[0], 1.0, DEFAULT_TOLERANCE);
        assert_close(c[1], 0.0, DEFAULT_TOLERANCE);
        assert_close(c[2], 0.0, DEFAULT_TOLERANCE);
        assert_close(c[3], 0.0, DEFAULT_TOLERANCE);
    }

    #[test]
    fn periodic_neighbor_wraps() {
        let lattice = Lattice4::new_identity([2, 3, 4, 5]).unwrap();
        assert_eq!(
            lattice.neighbor_plus([1, 2, 3, 4], 0).unwrap(),
            [0, 2, 3, 4]
        );
        assert_eq!(
            lattice.neighbor_plus([1, 2, 3, 4], 3).unwrap(),
            [1, 2, 3, 0]
        );
    }

    #[test]
    fn identity_plaquette_and_action_are_exact() {
        let lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
        assert_eq!(
            lattice.plaquette([0, 0, 0, 0], 0, 1).unwrap(),
            Su2::IDENTITY
        );
        assert_eq!(lattice.wilson_action(2.0).unwrap(), 0.0);
    }

    #[test]
    fn single_traceless_link_has_analytic_action() {
        let beta = 2.0;
        let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        let traceless = Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0]).unwrap();
        lattice.set_link([0, 0, 0, 0], 0, traceless).unwrap();
        assert_close(
            lattice.wilson_action(beta).unwrap(),
            6.0 * beta,
            DEFAULT_TOLERANCE,
        );
    }

    #[test]
    fn wilson_action_is_gauge_invariant_numerically() {
        let beta = 2.3;
        let lattice = deterministic_nontrivial_field();
        let transformed = lattice.gauge_transform(deterministic_site_gauge).unwrap();
        assert_close(
            lattice.wilson_action(beta).unwrap(),
            transformed.wilson_action(beta).unwrap(),
            2.0e-11,
        );
        assert!(transformed.max_link_unit_residual() <= 2.0e-12);
    }

    #[test]
    fn gauge_transform_samples_each_site_field_exactly_once() {
        let lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
        let calls = Cell::new(0usize);
        let transformed = lattice
            .gauge_transform(|x| {
                let call = calls.get();
                calls.set(call + 1);
                let coordinate_code = x[0] + 2 * x[1] + 3 * x[2] + 5 * x[3];
                let angle = 0.031 * (call as f64 + 1.0) + 0.017 * coordinate_code as f64;
                Su2::from_axis_angle([1.0, 2.0, -1.0], angle).unwrap()
            })
            .unwrap();

        assert_eq!(calls.get(), lattice.site_count());
        assert_close(transformed.wilson_action(2.0).unwrap(), 0.0, 2.0e-11);
    }

    #[test]
    fn invalid_inputs_fail_closed() {
        assert_eq!(
            Lattice4::new_identity([1, 2, 2, 2]).unwrap_err(),
            ReferenceError::InvalidLatticeExtent
        );
        assert_eq!(
            Su2::from_normalized_components([0.0; 4]).unwrap_err(),
            ReferenceError::ZeroSu2Norm
        );
        assert_eq!(
            Su2::from_axis_angle([0.0; 3], 1.0).unwrap_err(),
            ReferenceError::InvalidAxis
        );
        let lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
        assert_eq!(
            lattice.link([2, 0, 0, 0], 0).unwrap_err(),
            ReferenceError::CoordinateOutOfRange
        );
        assert_eq!(
            lattice.plaquette([0, 0, 0, 0], 1, 1).unwrap_err(),
            ReferenceError::DirectionsMustDiffer
        );
        assert_eq!(
            lattice.wilson_action(f64::NAN).unwrap_err(),
            ReferenceError::NonFiniteBeta
        );
        assert_eq!(
            lattice.wilson_action(-1.0).unwrap_err(),
            ReferenceError::NegativeBeta
        );
    }
}
