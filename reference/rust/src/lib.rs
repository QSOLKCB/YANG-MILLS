// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Deterministic finite-lattice SU(2) reference core.
//!
//! The mathematical convention is
//! `U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)`.
//! Unit-quaternion multiplication implements SU(2) matrix multiplication,
//! quaternion conjugation implements Hermitian conjugation, and Phase 1A
//! extends the Phase 0 plaquette/action surface with oriented paths, Wilson
//! loops, Polyakov loops, staples, local action differences, bounded reference
//! lattices, and exact deterministic configuration serialization.

use core::fmt;
use core::fmt::Write as _;

pub const DIM: usize = 4;
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;
pub const SERIALIZATION_TOLERANCE: f64 = 2.0e-12;
pub const MAX_REFERENCE_LINKS: usize = 1 << 20;
pub const MAX_PATH_STEPS: usize = 1 << 20;
pub const CONFIG_FORMAT_ID: &str = "YM-SU2-CONFIG-V1";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Su2 {
    c: [f64; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathStep {
    Forward(usize),
    Backward(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Staple {
    c: [f64; 4],
    terms: usize,
}

impl Staple {
    pub const fn components(self) -> [f64; 4] {
        self.c
    }

    pub const fn term_count(self) -> usize {
        self.terms
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceError {
    NonFiniteSu2Component,
    ZeroSu2Norm,
    NonUnitSu2,
    InvalidAxis,
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
    EmptyPath,
    PathTooLong,
    PathNotClosed,
    ZeroRectangleExtent,
    InvalidConfigurationEncoding,
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NonFiniteSu2Component => "SU(2) components must be finite",
            Self::ZeroSu2Norm => "SU(2) components must have non-zero norm",
            Self::NonUnitSu2 => "serialized SU(2) components must satisfy the unit invariant",
            Self::InvalidAxis => "axis must be finite and have non-zero norm",
            Self::InvalidLatticeExtent => "every lattice extent must be >= 2",
            Self::LatticeSizeOverflow => "lattice size arithmetic overflowed",
            Self::LatticeTooLarge => "lattice exceeds the bounded reference link limit",
            Self::AllocationFailed => "reference lattice allocation failed",
            Self::CoordinateOutOfRange => {
                "lattice coordinate is outside its periodic fundamental domain"
            }
            Self::DirectionOutOfRange => "lattice direction must be in 0..4",
            Self::DirectionsMustDiffer => "directions must differ",
            Self::NonFiniteBeta => "beta must be finite",
            Self::NegativeBeta => "beta must be non-negative",
            Self::NonFiniteAction => "Wilson action evaluation produced a non-finite result",
            Self::EmptyPath => "path must contain at least one oriented step",
            Self::PathTooLong => "path exceeds the bounded reference step limit",
            Self::PathNotClosed => "Wilson loop path does not close on the periodic lattice",
            Self::ZeroRectangleExtent => "rectangular Wilson-loop extents must be positive",
            Self::InvalidConfigurationEncoding => "invalid canonical SU(2) configuration encoding",
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
    if scale == 0.0 || !scale.is_finite() {
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

fn validate_beta(beta: f64) -> Result<(), ReferenceError> {
    if !beta.is_finite() {
        return Err(ReferenceError::NonFiniteBeta);
    }
    if beta < 0.0 {
        return Err(ReferenceError::NegativeBeta);
    }
    Ok(())
}

fn checked_action(value: f64) -> Result<f64, ReferenceError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ReferenceError::NonFiniteAction)
    }
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

    fn from_preserved_unit_components(c: [f64; 4]) -> Result<Self, ReferenceError> {
        if !c.iter().all(|v| v.is_finite()) {
            return Err(ReferenceError::NonFiniteSu2Component);
        }
        if c.iter()
            .any(|value| value.abs() > 1.0 + SERIALIZATION_TOLERANCE)
        {
            return Err(ReferenceError::NonUnitSu2);
        }
        let norm_sq = c.iter().map(|value| value * value).sum::<f64>();
        if !norm_sq.is_finite() || (norm_sq - 1.0).abs() > SERIALIZATION_TOLERANCE {
            return Err(ReferenceError::NonUnitSu2);
        }
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
            c: [half.cos(), s * axis[0], s * axis[1], s * axis[2]],
        })
    }

    pub const fn components(self) -> [f64; 4] {
        self.c
    }

    pub fn norm_sq(self) -> f64 {
        self.c.iter().map(|v| v * v).sum()
    }

    pub fn is_unit(self, tolerance: f64) -> bool {
        tolerance.is_finite()
            && tolerance >= 0.0
            && self.norm_sq().is_finite()
            && (self.norm_sq() - 1.0).abs() <= tolerance
    }

    pub const fn dagger(self) -> Self {
        Self {
            c: [self.c[0], -self.c[1], -self.c[2], -self.c[3]],
        }
    }

    pub fn mul(self, rhs: Self) -> Self {
        let [a0, a1, a2, a3] = self.c;
        let [b0, b1, b2, b3] = rhs.c;
        let raw = [
            a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
            a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
            a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
            a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0,
        ];
        let c = normalize_scaled(raw)
            .expect("product of finite unit quaternions must have non-zero finite norm");
        Self { c }
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
            .ok_or(ReferenceError::LatticeSizeOverflow)?;
        let link_count = sites
            .checked_mul(DIM)
            .ok_or(ReferenceError::LatticeSizeOverflow)?;
        if link_count > MAX_REFERENCE_LINKS {
            return Err(ReferenceError::LatticeTooLarge);
        }

        let mut links = Vec::new();
        links
            .try_reserve_exact(link_count)
            .map_err(|_| ReferenceError::AllocationFailed)?;
        links.resize(link_count, Su2::IDENTITY);
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

    pub fn neighbor_minus(
        &self,
        mut x: [usize; DIM],
        mu: usize,
    ) -> Result<[usize; DIM], ReferenceError> {
        Self::validate_direction(mu)?;
        self.site_index(x)?;
        x[mu] = if x[mu] == 0 {
            self.extents[mu] - 1
        } else {
            x[mu] - 1
        };
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

    pub fn left_multiply_link(
        &mut self,
        x: [usize; DIM],
        mu: usize,
        update: Su2,
    ) -> Result<Su2, ReferenceError> {
        let next = update.mul(self.link(x, mu)?);
        self.set_link(x, mu, next)?;
        Ok(next)
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
        validate_beta(beta)?;

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

        if !sum.is_finite() {
            return Err(ReferenceError::NonFiniteAction);
        }
        checked_action(beta * sum)
    }

    pub fn path_holonomy(
        &self,
        start: [usize; DIM],
        steps: &[PathStep],
    ) -> Result<(Su2, [usize; DIM]), ReferenceError> {
        self.site_index(start)?;
        if steps.is_empty() {
            return Err(ReferenceError::EmptyPath);
        }
        if steps.len() > MAX_PATH_STEPS {
            return Err(ReferenceError::PathTooLong);
        }

        let mut current = start;
        let mut holonomy = Su2::IDENTITY;
        for step in steps {
            let segment = match *step {
                PathStep::Forward(mu) => {
                    Self::validate_direction(mu)?;
                    let value = self.link(current, mu)?;
                    current = self.neighbor_plus(current, mu)?;
                    value
                }
                PathStep::Backward(mu) => {
                    Self::validate_direction(mu)?;
                    current = self.neighbor_minus(current, mu)?;
                    self.link(current, mu)?.dagger()
                }
            };
            holonomy = holonomy.mul(segment);
        }
        Ok((holonomy, current))
    }

    pub fn closed_path_holonomy(
        &self,
        start: [usize; DIM],
        steps: &[PathStep],
    ) -> Result<Su2, ReferenceError> {
        let (holonomy, end) = self.path_holonomy(start, steps)?;
        if end != start {
            return Err(ReferenceError::PathNotClosed);
        }
        Ok(holonomy)
    }

    pub fn wilson_loop(
        &self,
        start: [usize; DIM],
        steps: &[PathStep],
    ) -> Result<f64, ReferenceError> {
        Ok(self.closed_path_holonomy(start, steps)?.half_real_trace())
    }

    pub fn rectangular_path(
        mu: usize,
        nu: usize,
        r: usize,
        t: usize,
    ) -> Result<Vec<PathStep>, ReferenceError> {
        Self::validate_direction(mu)?;
        Self::validate_direction(nu)?;
        if mu == nu {
            return Err(ReferenceError::DirectionsMustDiffer);
        }
        if r == 0 || t == 0 {
            return Err(ReferenceError::ZeroRectangleExtent);
        }
        let perimeter = r
            .checked_add(t)
            .and_then(|value| value.checked_mul(2))
            .ok_or(ReferenceError::PathTooLong)?;
        if perimeter > MAX_PATH_STEPS {
            return Err(ReferenceError::PathTooLong);
        }

        let mut steps = Vec::new();
        steps
            .try_reserve_exact(perimeter)
            .map_err(|_| ReferenceError::AllocationFailed)?;
        steps.extend(core::iter::repeat(PathStep::Forward(mu)).take(r));
        steps.extend(core::iter::repeat(PathStep::Forward(nu)).take(t));
        steps.extend(core::iter::repeat(PathStep::Backward(mu)).take(r));
        steps.extend(core::iter::repeat(PathStep::Backward(nu)).take(t));
        Ok(steps)
    }

    pub fn rectangular_wilson_loop(
        &self,
        start: [usize; DIM],
        mu: usize,
        nu: usize,
        r: usize,
        t: usize,
    ) -> Result<f64, ReferenceError> {
        let steps = Self::rectangular_path(mu, nu, r, t)?;
        self.wilson_loop(start, &steps)
    }

    pub fn polyakov_loop(
        &self,
        start: [usize; DIM],
        direction: usize,
    ) -> Result<f64, ReferenceError> {
        Self::validate_direction(direction)?;
        self.site_index(start)?;
        let winding = self.extents[direction];
        if winding > MAX_PATH_STEPS {
            return Err(ReferenceError::PathTooLong);
        }

        let mut current = start;
        let mut holonomy = Su2::IDENTITY;
        for _ in 0..winding {
            holonomy = holonomy.mul(self.link(current, direction)?);
            current = self.neighbor_plus(current, direction)?;
        }
        if current != start {
            return Err(ReferenceError::PathNotClosed);
        }
        Ok(holonomy.half_real_trace())
    }

    pub fn staple(&self, x: [usize; DIM], mu: usize) -> Result<Staple, ReferenceError> {
        Self::validate_direction(mu)?;
        self.site_index(x)?;
        let mut components = [0.0; 4];
        let mut terms = 0usize;

        for nu in 0..DIM {
            if nu == mu {
                continue;
            }

            let x_plus_mu = self.neighbor_plus(x, mu)?;
            let x_plus_nu = self.neighbor_plus(x, nu)?;
            let forward = self
                .link(x_plus_mu, nu)?
                .mul(self.link(x_plus_nu, mu)?.dagger())
                .mul(self.link(x, nu)?.dagger());

            let x_minus_nu = self.neighbor_minus(x, nu)?;
            let x_minus_nu_plus_mu = self.neighbor_plus(x_minus_nu, mu)?;
            let backward = self
                .link(x_minus_nu_plus_mu, nu)?
                .dagger()
                .mul(self.link(x_minus_nu, mu)?.dagger())
                .mul(self.link(x_minus_nu, nu)?);

            for term in [forward, backward] {
                let c = term.components();
                for index in 0..4 {
                    components[index] += c[index];
                }
                terms += 1;
            }
        }

        Ok(Staple {
            c: components,
            terms,
        })
    }

    pub fn local_link_action(
        &self,
        x: [usize; DIM],
        mu: usize,
        beta: f64,
    ) -> Result<f64, ReferenceError> {
        validate_beta(beta)?;
        Self::validate_direction(mu)?;
        self.site_index(x)?;

        let mut sum = 0.0;
        for nu in 0..DIM {
            if nu == mu {
                continue;
            }
            let forward = self.plaquette(x, mu, nu)?;
            let x_minus_nu = self.neighbor_minus(x, nu)?;
            let backward = self.plaquette(x_minus_nu, mu, nu)?;
            sum += 1.0 - forward.half_real_trace();
            sum += 1.0 - backward.half_real_trace();
        }
        checked_action(beta * sum)
    }

    pub fn local_action_difference(
        &self,
        x: [usize; DIM],
        mu: usize,
        candidate: Su2,
        beta: f64,
    ) -> Result<f64, ReferenceError> {
        validate_beta(beta)?;
        let old_local = self.local_link_action(x, mu, beta)?;
        let mut proposed = self.clone();
        proposed.set_link(x, mu, candidate)?;
        let new_local = proposed.local_link_action(x, mu, beta)?;
        checked_action(new_local - old_local)
    }

    pub fn to_canonical_config(&self) -> Result<String, ReferenceError> {
        if self.max_link_unit_residual() > SERIALIZATION_TOLERANCE {
            return Err(ReferenceError::NonUnitSu2);
        }

        let mut output = String::new();
        writeln!(&mut output, "{CONFIG_FORMAT_ID}")
            .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        writeln!(
            &mut output,
            "extents={},{},{},{}",
            self.extents[0], self.extents[1], self.extents[2], self.extents[3]
        )
        .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        writeln!(&mut output, "link_count={}", self.links.len())
            .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        for link in &self.links {
            let [a0, a1, a2, a3] = link.components();
            writeln!(
                &mut output,
                "{:016x},{:016x},{:016x},{:016x}",
                a0.to_bits(),
                a1.to_bits(),
                a2.to_bits(),
                a3.to_bits()
            )
            .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        }
        Ok(output)
    }

    pub fn from_canonical_config(text: &str) -> Result<Self, ReferenceError> {
        let mut lines = text.lines();
        if lines.next() != Some(CONFIG_FORMAT_ID) {
            return Err(ReferenceError::InvalidConfigurationEncoding);
        }

        let extents_line = lines
            .next()
            .ok_or(ReferenceError::InvalidConfigurationEncoding)?;
        let extents_text = extents_line
            .strip_prefix("extents=")
            .ok_or(ReferenceError::InvalidConfigurationEncoding)?;
        let mut extent_parts = extents_text.split(',');
        let mut extents = [0usize; DIM];
        for extent in &mut extents {
            *extent = extent_parts
                .next()
                .ok_or(ReferenceError::InvalidConfigurationEncoding)?
                .parse::<usize>()
                .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        }
        if extent_parts.next().is_some() {
            return Err(ReferenceError::InvalidConfigurationEncoding);
        }

        let count_line = lines
            .next()
            .ok_or(ReferenceError::InvalidConfigurationEncoding)?;
        let declared_link_count = count_line
            .strip_prefix("link_count=")
            .ok_or(ReferenceError::InvalidConfigurationEncoding)?
            .parse::<usize>()
            .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
        if declared_link_count > MAX_REFERENCE_LINKS {
            return Err(ReferenceError::LatticeTooLarge);
        }

        let mut lattice = Self::new_identity(extents)?;
        if declared_link_count != lattice.links.len() {
            return Err(ReferenceError::InvalidConfigurationEncoding);
        }

        for index in 0..declared_link_count {
            let line = lines
                .next()
                .ok_or(ReferenceError::InvalidConfigurationEncoding)?;
            let mut parts = line.split(',');
            let mut components = [0.0; 4];
            for component in &mut components {
                let token = parts
                    .next()
                    .ok_or(ReferenceError::InvalidConfigurationEncoding)?;
                if token.len() != 16
                    || !token
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(ReferenceError::InvalidConfigurationEncoding);
                }
                let bits = u64::from_str_radix(token, 16)
                    .map_err(|_| ReferenceError::InvalidConfigurationEncoding)?;
                *component = f64::from_bits(bits);
            }
            if parts.next().is_some() {
                return Err(ReferenceError::InvalidConfigurationEncoding);
            }
            lattice.links[index] = Su2::from_preserved_unit_components(components)?;
        }
        if lines.next().is_some() {
            return Err(ReferenceError::InvalidConfigurationEncoding);
        }

        if lattice.to_canonical_config()?.as_str() != text {
            return Err(ReferenceError::InvalidConfigurationEncoding);
        }
        Ok(lattice)
    }

    pub fn gauge_transform<F>(&self, omega: F) -> Result<Self, ReferenceError>
    where
        F: Fn([usize; DIM]) -> Su2,
    {
        let mut site_gauges = Vec::new();
        site_gauges
            .try_reserve_exact(self.site_count())
            .map_err(|_| ReferenceError::AllocationFailed)?;
        site_gauges.resize(self.site_count(), Su2::IDENTITY);
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
        let mut max_residual: f64 = 0.0;
        for link in &self.links {
            let norm_sq = link.norm_sq();
            if !norm_sq.is_finite() {
                return f64::INFINITY;
            }
            let residual = (norm_sq - 1.0).abs();
            if !residual.is_finite() {
                return f64::INFINITY;
            }
            max_residual = max_residual.max(residual);
        }
        max_residual
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

    fn single_traceless_link_lattice() -> Lattice4 {
        let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        lattice
            .set_link(
                [0, 0, 0, 0],
                0,
                Su2::from_normalized_components([0.0, 1.0, 0.0, 0.0]).unwrap(),
            )
            .unwrap();
        lattice
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
    fn repeated_multiplication_preserves_unit_quaternion_invariant() {
        let mut q = Su2::from_axis_angle([1.0, 2.0, 3.0], 0.91).unwrap();
        for _ in 0..100 {
            q = q.mul(q);
            assert_close(q.norm_sq(), 1.0, DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn lattice_bounds_fail_closed_before_large_allocation() {
        assert_eq!(
            Lattice4::new_identity([usize::MAX, 2, 2, 2]).unwrap_err(),
            ReferenceError::LatticeSizeOverflow
        );
        assert_eq!(
            Lattice4::new_identity([512, 512, 2, 2]).unwrap_err(),
            ReferenceError::LatticeTooLarge
        );
    }

    #[test]
    fn periodic_neighbors_wrap_both_directions() {
        let lattice = Lattice4::new_identity([2, 3, 4, 5]).unwrap();
        assert_eq!(
            lattice.neighbor_plus([1, 2, 3, 4], 0).unwrap(),
            [0, 2, 3, 4]
        );
        assert_eq!(
            lattice.neighbor_minus([0, 0, 0, 0], 3).unwrap(),
            [0, 0, 0, 4]
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
        let lattice = single_traceless_link_lattice();
        assert_close(
            lattice.wilson_action(beta).unwrap(),
            6.0 * beta,
            DEFAULT_TOLERANCE,
        );
    }

    #[test]
    fn non_finite_action_result_fails_closed() {
        let lattice = single_traceless_link_lattice();
        assert_eq!(
            lattice.wilson_action(f64::MAX).unwrap_err(),
            ReferenceError::NonFiniteAction
        );
    }

    #[test]
    fn identity_rectangles_are_unity() {
        let lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        for (r, t) in [(1, 1), (2, 1), (1, 2), (2, 2)] {
            assert_eq!(
                lattice
                    .rectangular_wilson_loop([0, 0, 0, 0], 0, 1, r, t)
                    .unwrap(),
                1.0
            );
        }
    }

    #[test]
    fn path_reverse_link_handling_cancels_exactly() {
        let lattice = single_traceless_link_lattice();
        let steps = [PathStep::Forward(0), PathStep::Backward(0)];
        let (holonomy, end) = lattice.path_holonomy([0, 0, 0, 0], &steps).unwrap();
        assert_eq!(end, [0, 0, 0, 0]);
        assert_close(holonomy.half_real_trace(), 1.0, DEFAULT_TOLERANCE);
    }

    #[test]
    fn single_traceless_link_gives_zero_rectangular_and_polyakov_observables() {
        let lattice = single_traceless_link_lattice();
        assert_close(
            lattice
                .rectangular_wilson_loop([0, 0, 0, 0], 0, 1, 2, 1)
                .unwrap(),
            0.0,
            DEFAULT_TOLERANCE,
        );
        assert_close(
            lattice.polyakov_loop([0, 0, 0, 0], 0).unwrap(),
            0.0,
            DEFAULT_TOLERANCE,
        );
    }

    #[test]
    fn closed_loop_trace_is_cyclically_invariant() {
        let lattice = deterministic_nontrivial_field();
        let first = [
            PathStep::Forward(0),
            PathStep::Forward(1),
            PathStep::Backward(0),
            PathStep::Backward(1),
        ];
        let shifted = [
            PathStep::Forward(1),
            PathStep::Backward(0),
            PathStep::Backward(1),
            PathStep::Forward(0),
        ];
        let value_a = lattice.wilson_loop([0, 0, 0, 0], &first).unwrap();
        let value_b = lattice.wilson_loop([1, 0, 0, 0], &shifted).unwrap();
        assert_close(value_a, value_b, 2.0e-12);
    }

    #[test]
    fn wilson_and_polyakov_loops_are_gauge_invariant() {
        let lattice = deterministic_nontrivial_field();
        let transformed = lattice.gauge_transform(deterministic_site_gauge).unwrap();
        let rectangle = Lattice4::rectangular_path(0, 1, 2, 1).unwrap();
        let w_a = lattice.wilson_loop([0, 0, 0, 0], &rectangle).unwrap();
        let w_b = transformed.wilson_loop([0, 0, 0, 0], &rectangle).unwrap();
        assert_close(w_a, w_b, 2.0e-11);

        let p_a = lattice.polyakov_loop([0, 0, 0, 0], 0).unwrap();
        let p_b = transformed.polyakov_loop([0, 0, 0, 0], 0).unwrap();
        assert_close(p_a, p_b, 2.0e-11);
    }

    #[test]
    fn open_path_and_nonclosing_wilson_loop_are_distinguished() {
        let lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        let steps = [PathStep::Forward(0)];
        let (_, end) = lattice.path_holonomy([0, 0, 0, 0], &steps).unwrap();
        assert_eq!(end, [1, 0, 0, 0]);
        assert_eq!(
            lattice.wilson_loop([0, 0, 0, 0], &steps).unwrap_err(),
            ReferenceError::PathNotClosed
        );
    }

    #[test]
    fn staple_and_local_action_match_full_action_difference() {
        let beta = 2.0;
        let lattice = single_traceless_link_lattice();
        let staple = lattice.staple([0, 0, 0, 0], 0).unwrap();
        assert_eq!(staple.term_count(), 6);
        assert_eq!(staple.components(), [6.0, 0.0, 0.0, 0.0]);
        assert_close(
            lattice.local_link_action([0, 0, 0, 0], 0, beta).unwrap(),
            12.0,
            DEFAULT_TOLERANCE,
        );

        let candidate = Su2::IDENTITY;
        let local_delta = lattice
            .local_action_difference([0, 0, 0, 0], 0, candidate, beta)
            .unwrap();
        let old_action = lattice.wilson_action(beta).unwrap();
        let mut replaced = lattice.clone();
        replaced.set_link([0, 0, 0, 0], 0, candidate).unwrap();
        let full_delta = replaced.wilson_action(beta).unwrap() - old_action;
        assert_close(local_delta, full_delta, DEFAULT_TOLERANCE);
        assert_close(local_delta, -12.0, DEFAULT_TOLERANCE);
    }

    #[test]
    fn near_unit_serialized_link_local_delta_matches_full_action() {
        let identity = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
        let encoded = identity.to_canonical_config().unwrap();
        let near_one = 1.0 - 5.0e-13;
        let near_hex = format!("{:016x}", near_one.to_bits());
        let mutated = encoded.replacen("3ff0000000000000", &near_hex, 1);
        let lattice = Lattice4::from_canonical_config(&mutated).unwrap();

        let local_delta = lattice
            .local_action_difference([0, 0, 0, 0], 0, Su2::IDENTITY, 2.0)
            .unwrap();
        let old_action = lattice.wilson_action(2.0).unwrap();
        let mut replaced = lattice.clone();
        replaced.set_link([0, 0, 0, 0], 0, Su2::IDENTITY).unwrap();
        let full_delta = replaced.wilson_action(2.0).unwrap() - old_action;
        assert_close(local_delta, full_delta, DEFAULT_TOLERANCE);
        assert_close(local_delta, 0.0, DEFAULT_TOLERANCE);
    }

    #[test]
    fn left_update_primitive_preserves_group_invariant() {
        let mut lattice = Lattice4::new_identity([3, 3, 3, 3]).unwrap();
        let update = Su2::from_axis_angle([1.0, 1.0, 0.0], 0.5).unwrap();
        let updated = lattice.left_multiply_link([0, 0, 0, 0], 0, update).unwrap();
        assert!(updated.is_unit(DEFAULT_TOLERANCE));
        assert!(lattice.max_link_unit_residual() <= DEFAULT_TOLERANCE);
    }

    #[test]
    fn canonical_configuration_round_trip_is_exact() {
        let lattice = deterministic_nontrivial_field();
        let encoded = lattice.to_canonical_config().unwrap();
        let decoded = Lattice4::from_canonical_config(&encoded).unwrap();
        assert_eq!(decoded.extents(), lattice.extents());
        assert_eq!(decoded.link_count(), lattice.link_count());
        assert_eq!(decoded.to_canonical_config().unwrap(), encoded);
        assert_close(
            decoded.wilson_action(1.7).unwrap(),
            lattice.wilson_action(1.7).unwrap(),
            DEFAULT_TOLERANCE,
        );
    }

    #[test]
    fn canonical_configuration_parser_rejects_noncanonical_or_nonunit_data() {
        let lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
        let encoded = lattice.to_canonical_config().unwrap();
        let uppercase = encoded.replacen("3ff0000000000000", "3FF0000000000000", 1);
        assert_eq!(
            Lattice4::from_canonical_config(&uppercase).unwrap_err(),
            ReferenceError::InvalidConfigurationEncoding
        );

        let nonunit = encoded.replacen("3ff0000000000000", "4000000000000000", 1);
        assert_eq!(
            Lattice4::from_canonical_config(&nonunit).unwrap_err(),
            ReferenceError::NonUnitSu2
        );
    }

    #[test]
    fn canonical_configuration_parser_bounds_declared_count_before_body() {
        let malformed = format!(
            "{CONFIG_FORMAT_ID}\nextents=2,2,2,2\nlink_count={}\n",
            MAX_REFERENCE_LINKS + 1
        );
        assert_eq!(
            Lattice4::from_canonical_config(&malformed).unwrap_err(),
            ReferenceError::LatticeTooLarge
        );
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
        assert_eq!(
            lattice
                .rectangular_wilson_loop([0, 0, 0, 0], 0, 0, 1, 1)
                .unwrap_err(),
            ReferenceError::DirectionsMustDiffer
        );
        assert_eq!(
            lattice
                .rectangular_wilson_loop([0, 0, 0, 0], 0, 1, 0, 1)
                .unwrap_err(),
            ReferenceError::ZeroRectangleExtent
        );
        assert_eq!(
            lattice.wilson_loop([0, 0, 0, 0], &[]).unwrap_err(),
            ReferenceError::EmptyPath
        );
    }
}
