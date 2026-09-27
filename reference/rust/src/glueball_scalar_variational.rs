// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3J two-operator scalar variational/GEVP mechanics.
//!
//! This authority is mechanics-only. It does not assume a Phase 3I pass or
//! failure and does not activate a production recovery experiment. It builds
//! the symmetric connected cross-correlator matrix for the frozen plaquette
//! and rectangle A1++ operators, solves a bounded 2x2 generalized eigenproblem,
//! and projects the correlator onto the principal variational direction.

use core::fmt;

use crate::glueball_correlators::{
    scalar_connected_correlators, GlueballCorrelatorError, MAX_CONFIGURATIONS, MAX_TIME_EXTENT,
};
use crate::glueball_operators::{scalar_a1pp_operator_basis, GlueballOperatorError};
use crate::su3_lattice::Su3Lattice4;

pub const VARIATIONAL_CONTRACT_ID: &str = "YM-GLUEBALL-SCALAR-VARIATIONAL-BASIS-V1";
pub const VARIATIONAL_STATUS: &str = "phase-3j-candidate";
pub const BASIS_DIMENSION: usize = 2;
pub const REFERENCE_TIME: usize = 1;
pub const DIAGONALIZATION_TIME: usize = 2;
pub const SYMMETRY_TOLERANCE: f64 = 2.0e-12;
pub const GAUGE_INVARIANCE_TOLERANCE: f64 = 5.0e-11;
pub const NORMALIZATION_TOLERANCE: f64 = 2.0e-12;
pub const POSITIVE_DEFINITE_PIVOT_TOLERANCE: f64 = 1.0e-14;
pub const CANONICAL_TIME_EXTENT: usize = 16;
pub const CANONICAL_GROUND_MASS: f64 = 0.4;
pub const CANONICAL_EXCITED_MASS: f64 = 0.9;

pub type Matrix2 = [[f64; BASIS_DIMENSION]; BASIS_DIMENSION];

#[derive(Clone, Debug, PartialEq)]
pub struct ScalarVariationalCorrelator {
    pub configuration_count: usize,
    pub time_extent: usize,
    pub vacuum_means: [f64; BASIS_DIMENSION],
    pub matrices: Vec<Matrix2>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrincipalProjection {
    pub reference_time: usize,
    pub diagonalization_time: usize,
    pub principal_eigenvalue: f64,
    pub coefficients: [f64; BASIS_DIMENSION],
    pub projected_correlator: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariationalError {
    EmptyConfigurationSet,
    TooManyConfigurations,
    TimeExtentTooSmall,
    TimeExtentTooLarge,
    LatticeExtentsMismatch,
    SampleCountOverflow,
    AllocationFailed,
    NonFiniteObservable,
    NonFiniteAccumulator,
    CorrelatorShapeMismatch,
    MatrixNotSymmetric,
    InvalidReferenceTimes,
    ReferenceMatrixNotPositiveDefinite,
    DegeneratePrincipalEigenvector,
    NonPositivePrincipalEigenvalue,
    ProjectionNormalizationFailure,
    Operator(GlueballOperatorError),
    Correlator(GlueballCorrelatorError),
}

impl fmt::Display for VariationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyConfigurationSet => "Phase 3J requires at least one configuration",
            Self::TooManyConfigurations => "Phase 3J configuration count exceeds the frozen bound",
            Self::TimeExtentTooSmall => "Phase 3J requires at least three Euclidean time slices",
            Self::TimeExtentTooLarge => "Phase 3J time extent exceeds the frozen bound",
            Self::LatticeExtentsMismatch => {
                "Phase 3J configurations must have identical lattice extents"
            }
            Self::SampleCountOverflow => "Phase 3J sample-count arithmetic overflowed",
            Self::AllocationFailed => "Phase 3J allocation failed",
            Self::NonFiniteObservable => "Phase 3J operator input became non-finite",
            Self::NonFiniteAccumulator => "Phase 3J matrix arithmetic became non-finite",
            Self::CorrelatorShapeMismatch => "Phase 3J correlator matrix shape is invalid",
            Self::MatrixNotSymmetric => "Phase 3J correlator matrix violates symmetry tolerance",
            Self::InvalidReferenceTimes => {
                "Phase 3J requires reference_time < diagonalization_time < time_extent"
            }
            Self::ReferenceMatrixNotPositiveDefinite => {
                "Phase 3J reference correlator matrix is not positive definite"
            }
            Self::DegeneratePrincipalEigenvector => {
                "Phase 3J principal generalized eigenvector is not uniquely defined"
            }
            Self::NonPositivePrincipalEigenvalue => {
                "Phase 3J principal generalized eigenvalue is not positive"
            }
            Self::ProjectionNormalizationFailure => {
                "Phase 3J projected reference correlator is not normalized"
            }
            Self::Operator(_) => "Phase 3J scalar operator evaluation failed",
            Self::Correlator(_) => "Phase 3J frozen diagonal correlator replay failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for VariationalError {}

impl From<GlueballOperatorError> for VariationalError {
    fn from(value: GlueballOperatorError) -> Self {
        Self::Operator(value)
    }
}

impl From<GlueballCorrelatorError> for VariationalError {
    fn from(value: GlueballCorrelatorError) -> Self {
        Self::Correlator(value)
    }
}

fn matrix_finite(matrix: &Matrix2) -> bool {
    matrix.iter().flatten().all(|value| value.is_finite())
}

fn symmetry_residual(matrix: &Matrix2) -> f64 {
    (matrix[0][1] - matrix[1][0]).abs()
}

fn symmetry_within_tolerance(matrix: &Matrix2) -> bool {
    let residual = symmetry_residual(matrix);
    if !residual.is_finite() {
        return false;
    }
    let scale = matrix[0][1].abs().max(matrix[1][0].abs()).max(1.0);
    scale.is_finite() && residual <= SYMMETRY_TOLERANCE * scale
}

fn validate_matrix_series(matrices: &[Matrix2]) -> Result<(), VariationalError> {
    if matrices.len() < 3 || matrices.len() > MAX_TIME_EXTENT {
        return Err(VariationalError::CorrelatorShapeMismatch);
    }
    for matrix in matrices {
        if !matrix_finite(matrix) {
            return Err(VariationalError::NonFiniteAccumulator);
        }
        if !symmetry_within_tolerance(matrix) {
            return Err(VariationalError::MatrixNotSymmetric);
        }
    }
    Ok(())
}

fn quadratic_form(vector: [f64; 2], matrix: &Matrix2) -> f64 {
    vector[0] * (matrix[0][0] * vector[0] + matrix[0][1] * vector[1])
        + vector[1] * (matrix[1][0] * vector[0] + matrix[1][1] * vector[1])
}

fn matmul(first: Matrix2, second: Matrix2) -> Matrix2 {
    let mut out = [[0.0; 2]; 2];
    for row in 0..2 {
        for column in 0..2 {
            out[row][column] =
                first[row][0] * second[0][column] + first[row][1] * second[1][column];
        }
    }
    out
}

fn transpose(matrix: Matrix2) -> Matrix2 {
    [[matrix[0][0], matrix[1][0]], [matrix[0][1], matrix[1][1]]]
}

fn matrix_scale_basis(matrix: Matrix2, scales: [f64; 2]) -> Matrix2 {
    [
        [
            scales[0] * matrix[0][0] * scales[0],
            scales[0] * matrix[0][1] * scales[1],
        ],
        [
            scales[1] * matrix[1][0] * scales[0],
            scales[1] * matrix[1][1] * scales[1],
        ],
    ]
}

fn matrix_swap_basis(matrix: Matrix2) -> Matrix2 {
    [[matrix[1][1], matrix[1][0]], [matrix[0][1], matrix[0][0]]]
}

/// Build the finite-sample symmetric connected cross-correlator matrix
///
/// C_ij(tau) = 1/2 [<O_i(t) O_j(t+tau)> + <O_j(t) O_i(t+tau)>] - <O_i><O_j>.
///
/// The same configurations and periodic time origins receive equal arithmetic
/// weight. Diagonal entries therefore replay the frozen Phase 3C estimators.
pub fn scalar_variational_connected_correlator(
    configurations: &[Su3Lattice4],
) -> Result<ScalarVariationalCorrelator, VariationalError> {
    if configurations.is_empty() {
        return Err(VariationalError::EmptyConfigurationSet);
    }
    if configurations.len() > MAX_CONFIGURATIONS {
        return Err(VariationalError::TooManyConfigurations);
    }

    let extents = configurations[0].extents();
    let time_extent = extents[3];
    if time_extent < 3 {
        return Err(VariationalError::TimeExtentTooSmall);
    }
    if time_extent > MAX_TIME_EXTENT {
        return Err(VariationalError::TimeExtentTooLarge);
    }
    if configurations
        .iter()
        .any(|configuration| configuration.extents() != extents)
    {
        return Err(VariationalError::LatticeExtentsMismatch);
    }

    let sample_count = configurations
        .len()
        .checked_mul(time_extent)
        .ok_or(VariationalError::SampleCountOverflow)?;
    let mut samples = Vec::<[f64; 2]>::new();
    samples
        .try_reserve_exact(sample_count)
        .map_err(|_| VariationalError::AllocationFailed)?;

    let mut mean_sums = [0.0_f64; 2];
    for configuration in configurations {
        for time_slice in 0..time_extent {
            let basis = scalar_a1pp_operator_basis(configuration, time_slice)?;
            let values = [basis.spatial_plaquette_1x1, basis.spatial_rectangle_2x1];
            if !values.iter().all(|value| value.is_finite()) {
                return Err(VariationalError::NonFiniteObservable);
            }
            for index in 0..2 {
                mean_sums[index] += values[index];
                if !mean_sums[index].is_finite() {
                    return Err(VariationalError::NonFiniteAccumulator);
                }
            }
            samples.push(values);
        }
    }

    let vacuum_means = [
        mean_sums[0] / sample_count as f64,
        mean_sums[1] / sample_count as f64,
    ];
    if !vacuum_means.iter().all(|value| value.is_finite()) {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let mut matrices = Vec::new();
    matrices
        .try_reserve_exact(time_extent)
        .map_err(|_| VariationalError::AllocationFailed)?;

    for tau in 0..time_extent {
        let mut pp = 0.0_f64;
        let mut rr = 0.0_f64;
        let mut pr = 0.0_f64;
        let mut rp = 0.0_f64;
        for configuration in 0..configurations.len() {
            let offset = configuration * time_extent;
            for time in 0..time_extent {
                let first = samples[offset + time];
                let second = samples[offset + (time + tau) % time_extent];
                pp += first[0] * second[0];
                rr += first[1] * second[1];
                pr += first[0] * second[1];
                rp += first[1] * second[0];
                if !pp.is_finite() || !rr.is_finite() || !pr.is_finite() || !rp.is_finite() {
                    return Err(VariationalError::NonFiniteAccumulator);
                }
            }
        }
        let norm = sample_count as f64;
        let cross = 0.5 * (pr / norm + rp / norm) - vacuum_means[0] * vacuum_means[1];
        let matrix = [
            [pp / norm - vacuum_means[0] * vacuum_means[0], cross],
            [cross, rr / norm - vacuum_means[1] * vacuum_means[1]],
        ];
        if !matrix_finite(&matrix) {
            return Err(VariationalError::NonFiniteAccumulator);
        }
        matrices.push(matrix);
    }

    let frozen = scalar_connected_correlators(configurations)?;
    if frozen.plaquette_connected.len() != time_extent
        || frozen.rectangle_connected.len() != time_extent
        || matrices.iter().enumerate().any(|(tau, matrix)| {
            (matrix[0][0] - frozen.plaquette_connected[tau]).abs() > SYMMETRY_TOLERANCE
                || (matrix[1][1] - frozen.rectangle_connected[tau]).abs() > SYMMETRY_TOLERANCE
        })
    {
        return Err(VariationalError::CorrelatorShapeMismatch);
    }

    Ok(ScalarVariationalCorrelator {
        configuration_count: configurations.len(),
        time_extent,
        vacuum_means,
        matrices,
    })
}

/// Solve C(t1) v = lambda C(t0) v for the largest generalized eigenvalue,
/// using a fail-closed Cholesky whitening of the positive-definite C(t0).
/// The returned vector is normalized so v^T C(t0) v = 1.
pub fn principal_generalized_projection(
    matrices: &[Matrix2],
    reference_time: usize,
    diagonalization_time: usize,
) -> Result<PrincipalProjection, VariationalError> {
    validate_matrix_series(matrices)?;
    if reference_time != REFERENCE_TIME
        || diagonalization_time != DIAGONALIZATION_TIME
        || diagonalization_time >= matrices.len()
    {
        return Err(VariationalError::InvalidReferenceTimes);
    }

    let c0 = matrices[reference_time];
    let c1 = matrices[diagonalization_time];

    let a = c0[0][0];
    let d = c0[1][1];
    if !a.is_finite() || !d.is_finite() || a <= 0.0 || d <= 0.0 {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    // Positive-definiteness is tested through the dimensionless correlation
    // coefficient, so an invertible independent rescaling of either basis
    // operator cannot change admissibility merely by changing its units.
    let diagonal_scale = a.sqrt() * d.sqrt();
    if !diagonal_scale.is_finite() || diagonal_scale <= 0.0 {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }
    let normalized_off_diagonal = c0[0][1] / diagonal_scale;
    if !normalized_off_diagonal.is_finite() {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }
    let normalized_determinant = 1.0 - normalized_off_diagonal * normalized_off_diagonal;
    if !normalized_determinant.is_finite()
        || normalized_determinant <= POSITIVE_DEFINITE_PIVOT_TOLERANCE
    {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    let l00 = a.sqrt();
    let l10 = c0[1][0] / l00;
    let second_pivot = d * normalized_determinant;
    if !second_pivot.is_finite() || second_pivot <= 0.0 {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }
    let l11 = second_pivot.sqrt();

    let inv_l = [[1.0 / l00, 0.0], [-l10 / (l00 * l11), 1.0 / l11]];
    let whitened = matmul(matmul(inv_l, c1), transpose(inv_l));
    if !matrix_finite(&whitened) {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    if !symmetry_within_tolerance(&whitened) {
        return Err(VariationalError::MatrixNotSymmetric);
    }

    let whitened_a = whitened[0][0];
    let whitened_d = whitened[1][1];
    let off_diagonal = 0.5 * (whitened[0][1] + whitened[1][0]);
    let eigen_scale = whitened_a
        .abs()
        .max(whitened_d.abs())
        .max(off_diagonal.abs());
    if !eigen_scale.is_finite() || eigen_scale <= 0.0 {
        return Err(VariationalError::DegeneratePrincipalEigenvector);
    }

    // Work on a unit-scaled symmetric 2x2 matrix so the largest eigenvalue
    // can be recovered without overflow. When the midpoint is negative,
    // computing midpoint + radius can catastrophically cancel; use the
    // determinant/product identity against the well-resolved smaller root.
    let scaled_a = whitened_a / eigen_scale;
    let scaled_d = whitened_d / eigen_scale;
    let scaled_b = off_diagonal / eigen_scale;
    let midpoint = 0.5 * (scaled_a + scaled_d);
    let half_difference = 0.5 * (scaled_a - scaled_d);
    let radius = half_difference.hypot(scaled_b);
    if !midpoint.is_finite() || !radius.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    if radius <= POSITIVE_DEFINITE_PIVOT_TOLERANCE {
        return Err(VariationalError::DegeneratePrincipalEigenvector);
    }

    let scaled_principal = if midpoint >= 0.0 {
        midpoint + radius
    } else {
        let scaled_smaller = midpoint - radius;
        let scaled_determinant = scaled_a * scaled_d - scaled_b * scaled_b;
        if !scaled_smaller.is_finite() || scaled_smaller == 0.0 || !scaled_determinant.is_finite() {
            return Err(VariationalError::NonFiniteAccumulator);
        }
        scaled_determinant / scaled_smaller
    };
    let principal_eigenvalue = scaled_principal * eigen_scale;
    if !principal_eigenvalue.is_finite() || principal_eigenvalue <= 0.0 {
        return Err(VariationalError::NonPositivePrincipalEigenvalue);
    }

    let scaled_principal_for_vector = principal_eigenvalue / eigen_scale;
    let off_diagonal_is_resolved = scaled_b.abs() > POSITIVE_DEFINITE_PIVOT_TOLERANCE;
    let mut eigenvector = if off_diagonal_is_resolved {
        let first = [scaled_b, scaled_principal_for_vector - scaled_a];
        let second = [scaled_principal_for_vector - scaled_d, scaled_b];
        if first[0].hypot(first[1]) >= second[0].hypot(second[1]) {
            first
        } else {
            second
        }
    } else if scaled_a > scaled_d {
        [1.0, 0.0]
    } else {
        [0.0, 1.0]
    };
    let euclidean_norm = eigenvector[0].hypot(eigenvector[1]);
    if !euclidean_norm.is_finite() || euclidean_norm <= 0.0 {
        return Err(VariationalError::DegeneratePrincipalEigenvector);
    }
    eigenvector[0] /= euclidean_norm;
    eigenvector[1] /= euclidean_norm;

    let inv_lt = transpose(inv_l);
    let mut coefficients = [
        inv_lt[0][0] * eigenvector[0] + inv_lt[0][1] * eigenvector[1],
        inv_lt[1][0] * eigenvector[0] + inv_lt[1][1] * eigenvector[1],
    ];
    let metric_norm = quadratic_form(coefficients, &c0);
    if !metric_norm.is_finite() || metric_norm <= 0.0 {
        return Err(VariationalError::ProjectionNormalizationFailure);
    }
    let scale = metric_norm.sqrt();
    coefficients[0] /= scale;
    coefficients[1] /= scale;

    if coefficients[0].abs() > POSITIVE_DEFINITE_PIVOT_TOLERANCE {
        if coefficients[0] < 0.0 {
            coefficients[0] = -coefficients[0];
            coefficients[1] = -coefficients[1];
        }
    } else if coefficients[1] < 0.0 {
        coefficients[1] = -coefficients[1];
    }

    let mut projected_correlator = Vec::new();
    projected_correlator
        .try_reserve_exact(matrices.len())
        .map_err(|_| VariationalError::AllocationFailed)?;
    for matrix in matrices {
        let value = quadratic_form(coefficients, matrix);
        if !value.is_finite() {
            return Err(VariationalError::NonFiniteAccumulator);
        }
        projected_correlator.push(value);
    }

    if (projected_correlator[reference_time] - 1.0).abs() > NORMALIZATION_TOLERANCE {
        return Err(VariationalError::ProjectionNormalizationFailure);
    }

    Ok(PrincipalProjection {
        reference_time,
        diagonalization_time,
        principal_eigenvalue,
        coefficients,
        projected_correlator,
    })
}

fn periodic_weight(mass: f64, time: usize, time_extent: usize) -> f64 {
    (-mass * time as f64).exp() + (-mass * (time_extent - time) as f64).exp()
}

pub fn canonical_two_state_matrices() -> Vec<Matrix2> {
    let ground_coupling = [1.0_f64, 0.5_f64];
    let excited_coupling = [0.25_f64, 1.0_f64];
    let ground_amplitude = 1.2_f64;
    let excited_amplitude = 0.8_f64;

    (0..CANONICAL_TIME_EXTENT)
        .map(|time| {
            let ground = ground_amplitude
                * periodic_weight(CANONICAL_GROUND_MASS, time, CANONICAL_TIME_EXTENT);
            let excited = excited_amplitude
                * periodic_weight(CANONICAL_EXCITED_MASS, time, CANONICAL_TIME_EXTENT);
            let mut matrix = [[0.0_f64; 2]; 2];
            for row in 0..2 {
                for column in 0..2 {
                    matrix[row][column] = ground * ground_coupling[row] * ground_coupling[column]
                        + excited * excited_coupling[row] * excited_coupling[column];
                }
            }
            matrix
        })
        .collect()
}

pub fn max_matrix_series_residual(
    first: &[Matrix2],
    second: &[Matrix2],
) -> Result<f64, VariationalError> {
    validate_matrix_series(first)?;
    validate_matrix_series(second)?;
    if first.len() != second.len() {
        return Err(VariationalError::CorrelatorShapeMismatch);
    }
    let mut maximum = 0.0_f64;
    for (left, right) in first.iter().zip(second) {
        for row in 0..2 {
            for column in 0..2 {
                let residual = (left[row][column] - right[row][column]).abs();
                if !residual.is_finite() {
                    return Err(VariationalError::NonFiniteAccumulator);
                }
                maximum = maximum.max(residual);
            }
        }
    }
    Ok(maximum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glueball_correlators::canonical_shifted_defect_configurations;
    use crate::su3_lattice::deterministic_su3_site_gauge;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
        );
    }

    fn assert_relative_close(actual: f64, expected: f64, tolerance: f64) {
        let residual = (actual - expected).abs();
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!(
            residual <= tolerance * scale,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} relative_tolerance={tolerance:.17e}"
        );
    }

    #[test]
    fn phase3j_identity_and_scope_are_predeclared() {
        assert_eq!(
            VARIATIONAL_CONTRACT_ID,
            "YM-GLUEBALL-SCALAR-VARIATIONAL-BASIS-V1"
        );
        assert_eq!(VARIATIONAL_STATUS, "phase-3j-candidate");
        assert_eq!(BASIS_DIMENSION, 2);
        assert_eq!(REFERENCE_TIME, 1);
        assert_eq!(DIAGONALIZATION_TIME, 2);
        assert_eq!(GAUGE_INVARIANCE_TOLERANCE, 5.0e-11);
        assert_eq!(CANONICAL_TIME_EXTENT, 16);
    }

    #[test]
    fn cross_correlator_diagonals_replay_frozen_phase3c() {
        let configurations = canonical_shifted_defect_configurations().unwrap();
        let matrix_result = scalar_variational_connected_correlator(&configurations).unwrap();
        let diagonal_result = scalar_connected_correlators(&configurations).unwrap();
        for tau in 0..matrix_result.time_extent {
            assert_close(
                matrix_result.matrices[tau][0][0],
                diagonal_result.plaquette_connected[tau],
                SYMMETRY_TOLERANCE,
            );
            assert_close(
                matrix_result.matrices[tau][1][1],
                diagonal_result.rectangle_connected[tau],
                SYMMETRY_TOLERANCE,
            );
            assert_eq!(
                matrix_result.matrices[tau][0][1].to_bits(),
                matrix_result.matrices[tau][1][0].to_bits()
            );
        }
    }

    #[test]
    fn cross_correlator_matrix_is_gauge_invariant() {
        let configurations = canonical_shifted_defect_configurations().unwrap();
        let before = scalar_variational_connected_correlator(&configurations).unwrap();
        let transformed: Vec<_> = configurations
            .iter()
            .map(|configuration| {
                configuration
                    .gauge_transform(deterministic_su3_site_gauge)
                    .unwrap()
            })
            .collect();
        let after = scalar_variational_connected_correlator(&transformed).unwrap();
        let residual = max_matrix_series_residual(&before.matrices, &after.matrices).unwrap();
        assert!(residual <= GAUGE_INVARIANCE_TOLERANCE);
    }

    #[test]
    fn canonical_two_state_gevp_isolates_ground_state() {
        let matrices = canonical_two_state_matrices();
        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        let expected_lambda =
            periodic_weight(
                CANONICAL_GROUND_MASS,
                DIAGONALIZATION_TIME,
                CANONICAL_TIME_EXTENT,
            ) / periodic_weight(CANONICAL_GROUND_MASS, REFERENCE_TIME, CANONICAL_TIME_EXTENT);
        assert_close(projection.principal_eigenvalue, expected_lambda, 2.0e-12);

        for time in 0..CANONICAL_TIME_EXTENT {
            let expected = periodic_weight(CANONICAL_GROUND_MASS, time, CANONICAL_TIME_EXTENT)
                / periodic_weight(CANONICAL_GROUND_MASS, REFERENCE_TIME, CANONICAL_TIME_EXTENT);
            assert_close(projection.projected_correlator[time], expected, 2.0e-11);
        }
    }

    #[test]
    fn projection_is_invariant_under_basis_rescaling() {
        let matrices = canonical_two_state_matrices();
        let scaled: Vec<_> = matrices
            .iter()
            .copied()
            .map(|matrix| matrix_scale_basis(matrix, [3.0, 0.25]))
            .collect();
        let first =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        let second =
            principal_generalized_projection(&scaled, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_close(
            first.principal_eigenvalue,
            second.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in first
            .projected_correlator
            .iter()
            .zip(&second.projected_correlator)
        {
            assert_close(left, right, 2.0e-11);
        }

        let tiny_scaled: Vec<_> = matrices
            .iter()
            .copied()
            .map(|matrix| matrix_scale_basis(matrix, [1.0e-8, 1.0]))
            .collect();
        let tiny =
            principal_generalized_projection(&tiny_scaled, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_close(
            first.principal_eigenvalue,
            tiny.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in first
            .projected_correlator
            .iter()
            .zip(&tiny.projected_correlator)
        {
            assert_close(left, right, 2.0e-11);
        }
    }

    #[test]
    fn projection_is_invariant_under_basis_permutation() {
        let matrices = canonical_two_state_matrices();
        let swapped: Vec<_> = matrices.iter().copied().map(matrix_swap_basis).collect();
        let first =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        let second =
            principal_generalized_projection(&swapped, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_close(
            first.principal_eigenvalue,
            second.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in first
            .projected_correlator
            .iter()
            .zip(&second.projected_correlator)
        {
            assert_close(left, right, 2.0e-11);
        }
    }

    #[test]
    fn whitened_symmetry_check_is_basis_permutation_invariant_at_large_scale() {
        let c0 = [[1.0e8, 0.25], [0.25, 1.0e-8]];
        let c1 = [[1.0e10, -1.0e10], [-1.0e10, 1.0e10]];
        let matrices = vec![c0, c0, c1];
        let swapped: Vec<_> = matrices.iter().copied().map(matrix_swap_basis).collect();

        let first =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        let second =
            principal_generalized_projection(&swapped, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_relative_close(
            first.principal_eigenvalue,
            second.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in first
            .projected_correlator
            .iter()
            .zip(&second.projected_correlator)
        {
            assert_relative_close(left, right, 2.0e-12);
        }
    }

    #[test]
    fn principal_eigenvalue_avoids_negative_midpoint_cancellation() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrices = vec![identity, identity, [[-1.0e16, 0.0], [0.0, 1.0]]];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_eq!(projection.principal_eigenvalue, 1.0);
        assert_eq!(projection.coefficients, [0.0, 1.0]);
        assert_eq!(projection.projected_correlator, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn invalid_reference_matrix_and_asymmetry_fail_closed() {
        let mut matrices = canonical_two_state_matrices();
        matrices[REFERENCE_TIME] = [[1.0, 1.0], [1.0, 1.0]];
        assert_eq!(
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap_err(),
            VariationalError::ReferenceMatrixNotPositiveDefinite
        );

        let mut matrices = canonical_two_state_matrices();
        matrices[3][0][1] += 1.0e-6;
        assert_eq!(
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap_err(),
            VariationalError::MatrixNotSymmetric
        );
    }

    #[test]
    fn residual_subtraction_overflow_fails_closed() {
        let first = vec![
            [[f64::MAX, 0.0], [0.0, 1.0]],
            [[f64::MAX, 0.0], [0.0, 1.0]],
            [[f64::MAX, 0.0], [0.0, 1.0]],
        ];
        let second = vec![
            [[-f64::MAX, 0.0], [0.0, 1.0]],
            [[-f64::MAX, 0.0], [0.0, 1.0]],
            [[-f64::MAX, 0.0], [0.0, 1.0]],
        ];
        assert_eq!(
            max_matrix_series_residual(&first, &second).unwrap_err(),
            VariationalError::NonFiniteAccumulator
        );
    }

    #[test]
    fn invalid_times_and_nonfinite_inputs_fail_closed() {
        let matrices = canonical_two_state_matrices();
        assert_eq!(
            principal_generalized_projection(&matrices, 2, 2).unwrap_err(),
            VariationalError::InvalidReferenceTimes
        );
        assert_eq!(
            principal_generalized_projection(&matrices, 0, 1).unwrap_err(),
            VariationalError::InvalidReferenceTimes
        );

        let mut nonfinite = matrices;
        nonfinite[0][0][0] = f64::NAN;
        assert_eq!(
            principal_generalized_projection(&nonfinite, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap_err(),
            VariationalError::NonFiniteAccumulator
        );
    }
}
