// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Standalone Phase 1B SU(3) algebra and group reference surface.
//!
//! This module deliberately contains no Yang--Mills lattice dynamics. It
//! establishes the canonical Gell-Mann basis, structure constants, Casimir
//! regressions, and a bounded dependency-free map
//!
//!     theta_a -> exp(i sum_a theta_a T^a) in SU(3),
//!
//! where T^a = lambda_a / 2. Lattice links may depend on this surface only
//! after its contracts and tests are sealed.

use core::fmt;

pub const SU3_DIM: usize = 3;
pub const SU3_GENERATORS: usize = 8;
pub const SU3_ACCEPTANCE_TOLERANCE: f64 = 5.0e-12;
pub const SU3_EXP_MAX_COEFFICIENT: f64 = 64.0;
const INV_SQRT_3: f64 = 0.5773502691896258;
const EXP_TAYLOR_TERMS: usize = 64;
const EXP_SCALE_TARGET: f64 = 0.5;
const EXP_TERM_CUTOFF: f64 = 1.0e-18;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub const fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }

    pub const fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }

    pub const fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }

    pub const fn scale(self, factor: f64) -> Self {
        Self::new(self.re * factor, self.im * factor)
    }

    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }

    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    m: [[Complex64; SU3_DIM]; SU3_DIM],
}

impl Mat3 {
    pub const ZERO: Self = Self {
        m: [[Complex64::ZERO; SU3_DIM]; SU3_DIM],
    };

    pub const IDENTITY: Self = Self {
        m: [
            [Complex64::ONE, Complex64::ZERO, Complex64::ZERO],
            [Complex64::ZERO, Complex64::ONE, Complex64::ZERO],
            [Complex64::ZERO, Complex64::ZERO, Complex64::ONE],
        ],
    };

    pub const fn from_rows(m: [[Complex64; SU3_DIM]; SU3_DIM]) -> Self {
        Self { m }
    }

    pub const fn entry(self, row: usize, col: usize) -> Complex64 {
        self.m[row][col]
    }

    pub fn is_finite(self) -> bool {
        self.m.iter().flatten().all(|value| value.is_finite())
    }

    pub fn add(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                out.m[row][col] = self.m[row][col].add(rhs.m[row][col]);
            }
        }
        out
    }

    pub fn sub(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                out.m[row][col] = self.m[row][col].sub(rhs.m[row][col]);
            }
        }
        out
    }

    pub fn scale_real(self, factor: f64) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                out.m[row][col] = self.m[row][col].scale(factor);
            }
        }
        out
    }

    pub fn scale_complex(self, factor: Complex64) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                out.m[row][col] = self.m[row][col].mul(factor);
            }
        }
        out
    }

    pub fn mul(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                let mut value = Complex64::ZERO;
                for inner in 0..SU3_DIM {
                    value = value.add(self.m[row][inner].mul(rhs.m[inner][col]));
                }
                out.m[row][col] = value;
            }
        }
        out
    }

    pub fn dagger(self) -> Self {
        let mut out = Self::ZERO;
        for row in 0..SU3_DIM {
            for col in 0..SU3_DIM {
                out.m[row][col] = self.m[col][row].conj();
            }
        }
        out
    }

    pub fn trace(self) -> Complex64 {
        self.m[0][0].add(self.m[1][1]).add(self.m[2][2])
    }

    pub fn determinant(self) -> Complex64 {
        let a = self.m[0][0];
        let b = self.m[0][1];
        let c = self.m[0][2];
        let d = self.m[1][0];
        let e = self.m[1][1];
        let f = self.m[1][2];
        let g = self.m[2][0];
        let h = self.m[2][1];
        let i = self.m[2][2];

        a.mul(e.mul(i).sub(f.mul(h)))
            .sub(b.mul(d.mul(i).sub(f.mul(g))))
            .add(c.mul(d.mul(h).sub(e.mul(g))))
    }

    pub fn max_abs_entry(self) -> f64 {
        self.m
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max)
    }

    pub fn one_norm(self) -> f64 {
        let mut max_column = 0.0_f64;
        for col in 0..SU3_DIM {
            let mut sum = 0.0;
            for row in 0..SU3_DIM {
                sum += self.m[row][col].abs();
            }
            max_column = max_column.max(sum);
        }
        max_column
    }

    pub fn max_residual(self, rhs: Self) -> f64 {
        self.sub(rhs).max_abs_entry()
    }

    pub fn hermiticity_residual(self) -> f64 {
        self.max_residual(self.dagger())
    }

    pub fn unitarity_residual(self) -> f64 {
        let left = self.dagger().mul(self).max_residual(Self::IDENTITY);
        let right = self.mul(self.dagger()).max_residual(Self::IDENTITY);
        left.max(right)
    }

    pub fn determinant_residual(self) -> f64 {
        let direct = self.determinant().sub(Complex64::ONE).abs();
        let dagger = self
            .dagger()
            .determinant()
            .sub(Complex64::ONE)
            .abs();
        direct.max(dagger)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Su3 {
    matrix: Mat3,
}

impl Su3 {
    pub const IDENTITY: Self = Self {
        matrix: Mat3::IDENTITY,
    };

    pub fn from_checked_matrix(matrix: Mat3) -> Result<Self, Su3Error> {
        if !matrix.is_finite() {
            return Err(Su3Error::NonFiniteMatrix);
        }
        if matrix.unitarity_residual() > SU3_ACCEPTANCE_TOLERANCE {
            return Err(Su3Error::NonUnitaryMatrix);
        }
        if matrix.determinant_residual() > SU3_ACCEPTANCE_TOLERANCE {
            return Err(Su3Error::DeterminantNotOne);
        }
        Ok(Self { matrix })
    }

    pub fn from_algebra_coefficients(
        coefficients: [f64; SU3_GENERATORS],
    ) -> Result<Self, Su3Error> {
        let hermitian = algebra_matrix(coefficients)?;
        let anti_hermitian = hermitian.scale_complex(Complex64::I);
        let matrix = matrix_exponential(anti_hermitian)?;
        Self::from_checked_matrix(matrix)
    }

    pub const fn matrix(self) -> Mat3 {
        self.matrix
    }

    pub fn determinant(self) -> Complex64 {
        self.matrix.determinant()
    }

    pub fn unitarity_residual(self) -> f64 {
        self.matrix.unitarity_residual()
    }

    pub fn determinant_residual(self) -> f64 {
        self.matrix.determinant_residual()
    }

    pub fn dagger(self) -> Self {
        Self {
            matrix: self.matrix.dagger(),
        }
    }

    pub fn mul(self, rhs: Self) -> Result<Self, Su3Error> {
        Self::from_checked_matrix(self.matrix.mul(rhs.matrix))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Su3Error {
    GeneratorIndexOutOfRange,
    NonFiniteCoefficient,
    AlgebraCoefficientTooLarge,
    NonFiniteMatrix,
    ExponentialScalingOverflow,
    NonUnitaryMatrix,
    DeterminantNotOne,
}

impl fmt::Display for Su3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::GeneratorIndexOutOfRange => "SU(3) generator index must be in 0..8",
            Self::NonFiniteCoefficient => "SU(3) algebra coefficients must be finite",
            Self::AlgebraCoefficientTooLarge => {
                "SU(3) reference exponentiation coefficient exceeds its bounded domain"
            }
            Self::NonFiniteMatrix => "SU(3) matrix contains a non-finite component",
            Self::ExponentialScalingOverflow => {
                "SU(3) matrix-exponential scaling bound was exceeded"
            }
            Self::NonUnitaryMatrix => "matrix is outside the SU(3) unitarity acceptance tolerance",
            Self::DeterminantNotOne => {
                "matrix determinant is outside the SU(3) determinant-one acceptance tolerance"
            }
        };
        f.write_str(message)
    }
}

impl std::error::Error for Su3Error {}

pub fn gell_mann(index: usize) -> Result<Mat3, Su3Error> {
    let z = Complex64::ZERO;
    let one = Complex64::ONE;
    let minus_one = Complex64::new(-1.0, 0.0);
    let i = Complex64::I;
    let minus_i = Complex64::new(0.0, -1.0);

    let matrix = match index {
        0 => Mat3::from_rows([[z, one, z], [one, z, z], [z, z, z]]),
        1 => Mat3::from_rows([[z, minus_i, z], [i, z, z], [z, z, z]]),
        2 => Mat3::from_rows([[one, z, z], [z, minus_one, z], [z, z, z]]),
        3 => Mat3::from_rows([[z, z, one], [z, z, z], [one, z, z]]),
        4 => Mat3::from_rows([[z, z, minus_i], [z, z, z], [i, z, z]]),
        5 => Mat3::from_rows([[z, z, z], [z, z, one], [z, one, z]]),
        6 => Mat3::from_rows([[z, z, z], [z, z, minus_i], [z, i, z]]),
        7 => Mat3::from_rows([
            [Complex64::new(INV_SQRT_3, 0.0), z, z],
            [z, Complex64::new(INV_SQRT_3, 0.0), z],
            [z, z, Complex64::new(-2.0 * INV_SQRT_3, 0.0)],
        ]),
        _ => return Err(Su3Error::GeneratorIndexOutOfRange),
    };
    Ok(matrix)
}

pub fn generator(index: usize) -> Result<Mat3, Su3Error> {
    Ok(gell_mann(index)?.scale_real(0.5))
}

pub fn structure_f(a: usize, b: usize, c: usize) -> Result<f64, Su3Error> {
    let ta = generator(a)?;
    let tb = generator(b)?;
    let tc = generator(c)?;
    let commutator = ta.mul(tb).sub(tb.mul(ta));
    Ok(2.0 * commutator.mul(tc).trace().im)
}

pub fn structure_d(a: usize, b: usize, c: usize) -> Result<f64, Su3Error> {
    let ta = generator(a)?;
    let tb = generator(b)?;
    let tc = generator(c)?;
    let anticommutator = ta.mul(tb).add(tb.mul(ta));
    Ok(2.0 * anticommutator.mul(tc).trace().re)
}

pub fn quadratic_casimir_fundamental() -> Mat3 {
    let mut sum = Mat3::ZERO;
    for a in 0..SU3_GENERATORS {
        let ta = generator(a).expect("fixed canonical generator index");
        sum = sum.add(ta.mul(ta));
    }
    sum
}

pub fn cubic_casimir_fundamental() -> Mat3 {
    let mut sum = Mat3::ZERO;
    for a in 0..SU3_GENERATORS {
        let ta = generator(a).expect("fixed canonical generator index");
        for b in 0..SU3_GENERATORS {
            let tb = generator(b).expect("fixed canonical generator index");
            for c in 0..SU3_GENERATORS {
                let tc = generator(c).expect("fixed canonical generator index");
                let d = structure_d(a, b, c).expect("fixed canonical generator index");
                sum = sum.add(ta.mul(tb).mul(tc).scale_real(d));
            }
        }
    }
    sum
}

fn algebra_matrix(coefficients: [f64; SU3_GENERATORS]) -> Result<Mat3, Su3Error> {
    let mut matrix = Mat3::ZERO;
    for (index, coefficient) in coefficients.into_iter().enumerate() {
        if !coefficient.is_finite() {
            return Err(Su3Error::NonFiniteCoefficient);
        }
        if coefficient.abs() > SU3_EXP_MAX_COEFFICIENT {
            return Err(Su3Error::AlgebraCoefficientTooLarge);
        }
        matrix = matrix.add(generator(index)?.scale_real(coefficient));
    }
    Ok(matrix)
}

fn matrix_exponential(matrix: Mat3) -> Result<Mat3, Su3Error> {
    if !matrix.is_finite() {
        return Err(Su3Error::NonFiniteMatrix);
    }

    let norm = matrix.one_norm();
    if !norm.is_finite() {
        return Err(Su3Error::NonFiniteMatrix);
    }

    let mut squarings = 0usize;
    let mut scaled_norm = norm;
    while scaled_norm > EXP_SCALE_TARGET {
        squarings += 1;
        if squarings > 32 {
            return Err(Su3Error::ExponentialScalingOverflow);
        }
        scaled_norm *= 0.5;
    }

    let scale = 2.0_f64.powi(-(squarings as i32));
    let scaled = matrix.scale_real(scale);
    let mut sum = Mat3::IDENTITY;
    let mut term = Mat3::IDENTITY;

    for order in 1..=EXP_TAYLOR_TERMS {
        term = term.mul(scaled).scale_real(1.0 / order as f64);
        sum = sum.add(term);
        if term.max_abs_entry() <= EXP_TERM_CUTOFF {
            break;
        }
    }

    for _ in 0..squarings {
        sum = sum.mul(sum);
    }

    if !sum.is_finite() {
        return Err(Su3Error::NonFiniteMatrix);
    }
    Ok(sum)
}
