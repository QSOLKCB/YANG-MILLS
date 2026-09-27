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
pub const CANONICAL_GROUND_AMPLITUDE: f64 = 1.2;
pub const CANONICAL_EXCITED_AMPLITUDE: f64 = 0.8;
pub const CANONICAL_GROUND_COUPLING: [f64; 2] = [1.0, 0.5];
pub const CANONICAL_EXCITED_COUPLING: [f64; 2] = [0.25, 1.0];
pub const GOLDEN_CANONICAL_PRINCIPAL_EIGENVALUE: f64 = 0.673_346_666_250_657;

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

fn stable_average(first: f64, second: f64) -> Result<f64, VariationalError> {
    if !first.is_finite() || !second.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    if first.to_bits() == second.to_bits() {
        return Ok(first);
    }
    let difference = second - first;
    if !difference.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    let average = first + 0.5 * difference;
    if !average.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    Ok(average)
}

fn symmetrized_matrix(matrix: Matrix2) -> Result<Matrix2, VariationalError> {
    let cross = stable_average(matrix[0][1], matrix[1][0])?;
    Ok([[matrix[0][0], cross], [cross, matrix[1][1]]])
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

fn cholesky_metric_norm(
    vector: [f64; 2],
    l00: f64,
    l10: f64,
    l11: f64,
) -> Result<f64, VariationalError> {
    let first = l00 * vector[0] + l10 * vector[1];
    let second = l11 * vector[1];
    let norm = first.hypot(second);
    if !first.is_finite() || !second.is_finite() || !norm.is_finite() || norm <= 0.0 {
        return Err(VariationalError::ProjectionNormalizationFailure);
    }
    Ok(norm)
}

fn stable_divide_by_product(
    numerator: f64,
    first_denominator: f64,
    second_denominator: f64,
) -> Result<f64, VariationalError> {
    if !numerator.is_finite()
        || !first_denominator.is_finite()
        || !second_denominator.is_finite()
        || first_denominator == 0.0
        || second_denominator == 0.0
    {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    // Never form the denominator product directly: for subnormal factors its
    // relative precision can collapse even when the final quotient is well
    // represented. Divide by the larger-magnitude factor first to suppress
    // intermediate overflow, then fall back to the opposite order only if
    // that path underflows or becomes non-finite.
    let (large, small) = if first_denominator.abs() >= second_denominator.abs() {
        (first_denominator, second_denominator)
    } else {
        (second_denominator, first_denominator)
    };

    let first_intermediate = numerator / large;
    let first_value = first_intermediate / small;
    if first_intermediate.is_finite()
        && first_value.is_finite()
        && (first_intermediate != 0.0 || numerator == 0.0)
    {
        return Ok(first_value);
    }

    let second_intermediate = numerator / small;
    let second_value = second_intermediate / large;
    if second_intermediate.is_finite() && second_value.is_finite() {
        return Ok(second_value);
    }

    Err(VariationalError::NonFiniteAccumulator)
}

fn compensated_symmetric_determinant(
    diagonal_first: f64,
    diagonal_second: f64,
    off_diagonal: f64,
) -> Result<f64, VariationalError> {
    if !diagonal_first.is_finite() || !diagonal_second.is_finite() || !off_diagonal.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let diagonal_product = diagonal_first * diagonal_second;
    let off_diagonal_square = off_diagonal * off_diagonal;
    if !diagonal_product.is_finite() || !off_diagonal_square.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    // Recover the low-order bits lost by each rounded product with FMA before
    // subtracting them. This preserves determinants that are representably
    // nonzero even when the two leading products round to the same binary64.
    let diagonal_error = diagonal_first.mul_add(diagonal_second, -diagonal_product);
    let off_diagonal_error = off_diagonal.mul_add(off_diagonal, -off_diagonal_square);
    let leading = diagonal_product - off_diagonal_square;
    let correction = diagonal_error - off_diagonal_error;
    let determinant = leading + correction;
    if !diagonal_error.is_finite()
        || !off_diagonal_error.is_finite()
        || !leading.is_finite()
        || !correction.is_finite()
        || !determinant.is_finite()
    {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    Ok(determinant)
}

fn solve_scaled_generalized_characteristic(
    reference_determinant: f64,
    reference_off_diagonal: f64,
    diagonal_first: f64,
    diagonal_second: f64,
    off_diagonal: f64,
) -> Result<f64, VariationalError> {
    if !reference_determinant.is_finite()
        || reference_determinant <= 0.0
        || !reference_off_diagonal.is_finite()
        || !diagonal_first.is_finite()
        || !diagonal_second.is_finite()
        || !off_diagonal.is_finite()
    {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let trace_coefficient = diagonal_first + diagonal_second
        - 2.0 * reference_off_diagonal * off_diagonal;
    let determinant =
        compensated_symmetric_determinant(diagonal_first, diagonal_second, off_diagonal)?;
    if !trace_coefficient.is_finite() || !determinant.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let discriminant_correction = -4.0 * reference_determinant * determinant;
    let discriminant = trace_coefficient.mul_add(trace_coefficient, discriminant_correction);
    if !discriminant.is_finite() || discriminant < 0.0 {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let discriminant_root = discriminant.sqrt();
    let denominator = 2.0 * reference_determinant;
    if !discriminant_root.is_finite() || !denominator.is_finite() || denominator <= 0.0 {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    if trace_coefficient >= 0.0 {
        let root = (trace_coefficient + discriminant_root) / denominator;
        if !root.is_finite() {
            return Err(VariationalError::NonFiniteAccumulator);
        }
        return Ok(root);
    }

    let smaller = (trace_coefficient - discriminant_root) / denominator;
    if !smaller.is_finite() || smaller == 0.0 {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    stable_divide_by_product(determinant, reference_determinant, smaller)
}

fn stable_generalized_principal_eigenvalue(
    reference: Matrix2,
    diagonalization: Matrix2,
) -> Result<f64, VariationalError> {
    let reference = symmetrized_matrix(reference)?;
    let diagonalization = symmetrized_matrix(diagonalization)?;

    let reference_first = reference[0][0];
    let reference_second = reference[1][1];
    if !reference_first.is_finite()
        || !reference_second.is_finite()
        || reference_first <= 0.0
        || reference_second <= 0.0
    {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    let reference_scale = reference_first.sqrt() * reference_second.sqrt();
    if !reference_scale.is_finite() || reference_scale <= 0.0 {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    let reference_off_diagonal = reference[0][1] / reference_scale;
    let reference_determinant =
        1.0 - reference_off_diagonal * reference_off_diagonal;
    if !reference_off_diagonal.is_finite()
        || !reference_determinant.is_finite()
        || reference_determinant <= POSITIVE_DEFINITE_PIVOT_TOLERANCE
    {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    let diagonal_first = diagonalization[0][0] / reference_first;
    let diagonal_second = diagonalization[1][1] / reference_second;
    let off_diagonal = diagonalization[0][1] / reference_scale;
    if !diagonal_first.is_finite()
        || !diagonal_second.is_finite()
        || !off_diagonal.is_finite()
    {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    if let Ok(root) = solve_scaled_generalized_characteristic(
        reference_determinant,
        reference_off_diagonal,
        diagonal_first,
        diagonal_second,
        off_diagonal,
    ) {
        return Ok(root);
    }

    let scale = diagonal_first
        .abs()
        .max(diagonal_second.abs())
        .max(off_diagonal.abs());
    if !scale.is_finite() || scale <= 0.0 {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let scaled_root = solve_scaled_generalized_characteristic(
        reference_determinant,
        reference_off_diagonal,
        diagonal_first / scale,
        diagonal_second / scale,
        off_diagonal / scale,
    )?;
    let root = scale * scaled_root;
    if !root.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    Ok(root)
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

fn stable_whitened_projection(
    matrix: Matrix2,
    reference_matrix: &Matrix2,
    inv_l: Matrix2,
    metric_vector: [f64; 2],
) -> Result<f64, VariationalError> {
    let matrix = symmetrized_matrix(matrix)?;
    if matrix == *reference_matrix {
        return Ok(metric_vector[0] * metric_vector[0] + metric_vector[1] * metric_vector[1]);
    }

    let whitened = matmul(matmul(inv_l, matrix), transpose(inv_l));
    if !matrix_finite(&whitened) {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    if !symmetry_within_tolerance(&whitened) {
        return Err(VariationalError::MatrixNotSymmetric);
    }
    let whitened = symmetrized_matrix(whitened)?;

    let row0 = whitened[0][0].mul_add(metric_vector[0], whitened[0][1] * metric_vector[1]);
    let row1 = whitened[1][0].mul_add(metric_vector[0], whitened[1][1] * metric_vector[1]);
    let value = metric_vector[0].mul_add(row0, metric_vector[1] * row1);
    if !value.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    Ok(value)
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

    let c0 = symmetrized_matrix(matrices[reference_time])?;
    let c1 = symmetrized_matrix(matrices[diagonalization_time])?;

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
    let l11 = d.sqrt() * normalized_determinant.sqrt();
    if !l00.is_finite() || !l10.is_finite() || !l11.is_finite() || l00 <= 0.0 || l11 <= 0.0 {
        return Err(VariationalError::ReferenceMatrixNotPositiveDefinite);
    }

    let inverse_lower = -stable_divide_by_product(l10, l00, l11)?;
    let inv_l = [[1.0 / l00, 0.0], [inverse_lower, 1.0 / l11]];
    let whitened = matmul(matmul(inv_l, c1), transpose(inv_l));
    if !matrix_finite(&whitened) {
        return Err(VariationalError::NonFiniteAccumulator);
    }
    if !symmetry_within_tolerance(&whitened) {
        return Err(VariationalError::MatrixNotSymmetric);
    }

    let whitened = symmetrized_matrix(whitened)?;
    let whitened_a = whitened[0][0];
    let whitened_d = whitened[1][1];
    let off_diagonal = whitened[0][1];
    if !whitened_a.is_finite() || !whitened_d.is_finite() || !off_diagonal.is_finite() {
        return Err(VariationalError::NonFiniteAccumulator);
    }

    let principal_eigenvalue = stable_generalized_principal_eigenvalue(c0, c1)?;
    if !principal_eigenvalue.is_finite() || principal_eigenvalue <= 0.0 {
        return Err(VariationalError::NonPositivePrincipalEigenvalue);
    }

    // Build the principal direction from the symmetric 2x2 radius directly,
    // not from lambda-a/lambda-d subtraction. This preserves a unique
    // eigenvector even when the positive eigenvalue increment rounds away.
    let mut eigenvector = if off_diagonal != 0.0 {
        let half_difference = 0.5 * whitened_a - 0.5 * whitened_d;
        let radius = half_difference.hypot(off_diagonal);
        if !half_difference.is_finite() || !radius.is_finite() || radius == 0.0 {
            return Err(VariationalError::DegeneratePrincipalEigenvector);
        }

        let component_scale = radius
            .abs()
            .max(half_difference.abs())
            .max(off_diagonal.abs());
        if !component_scale.is_finite() || component_scale <= 0.0 {
            return Err(VariationalError::DegeneratePrincipalEigenvector);
        }
        if half_difference >= 0.0 {
            [
                radius / component_scale + half_difference / component_scale,
                off_diagonal / component_scale,
            ]
        } else {
            [
                off_diagonal / component_scale,
                radius / component_scale - half_difference / component_scale,
            ]
        }
    } else if whitened_a > whitened_d {
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
    let metric_norm = cholesky_metric_norm(coefficients, l00, l10, l11)?;
    coefficients[0] /= metric_norm;
    coefficients[1] /= metric_norm;

    if coefficients[0].abs() > POSITIVE_DEFINITE_PIVOT_TOLERANCE {
        if coefficients[0] < 0.0 {
            coefficients[0] = -coefficients[0];
            coefficients[1] = -coefficients[1];
        }
    } else if coefficients[1] < 0.0 {
        coefficients[0] = -coefficients[0];
        coefficients[1] = -coefficients[1];
    }

    let metric_vector = [
        l00 * coefficients[0] + l10 * coefficients[1],
        l11 * coefficients[1],
    ];
    if !metric_vector[0].is_finite() || !metric_vector[1].is_finite() {
        return Err(VariationalError::ProjectionNormalizationFailure);
    }
    let normalized_metric_norm = metric_vector[0].hypot(metric_vector[1]);
    let reference_projection = normalized_metric_norm * normalized_metric_norm;
    if !reference_projection.is_finite()
        || (reference_projection - 1.0).abs() > NORMALIZATION_TOLERANCE
    {
        return Err(VariationalError::ProjectionNormalizationFailure);
    }

    let mut projected_correlator = Vec::new();
    projected_correlator
        .try_reserve_exact(matrices.len())
        .map_err(|_| VariationalError::AllocationFailed)?;
    for &matrix in matrices {
        projected_correlator.push(stable_whitened_projection(
            matrix,
            &c0,
            inv_l,
            metric_vector,
        )?);
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
    (0..CANONICAL_TIME_EXTENT)
        .map(|time| {
            let ground = CANONICAL_GROUND_AMPLITUDE
                * periodic_weight(CANONICAL_GROUND_MASS, time, CANONICAL_TIME_EXTENT);
            let excited = CANONICAL_EXCITED_AMPLITUDE
                * periodic_weight(CANONICAL_EXCITED_MASS, time, CANONICAL_TIME_EXTENT);
            let mut matrix = [[0.0_f64; 2]; 2];
            for row in 0..2 {
                for column in 0..2 {
                    matrix[row][column] =
                        ground * CANONICAL_GROUND_COUPLING[row] * CANONICAL_GROUND_COUPLING[column]
                            + excited
                                * CANONICAL_EXCITED_COUPLING[row]
                                * CANONICAL_EXCITED_COUPLING[column];
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
        assert_eq!(CANONICAL_GROUND_MASS.to_bits(), 0.4_f64.to_bits());
        assert_eq!(CANONICAL_EXCITED_MASS.to_bits(), 0.9_f64.to_bits());
        assert_eq!(CANONICAL_GROUND_AMPLITUDE.to_bits(), 1.2_f64.to_bits());
        assert_eq!(CANONICAL_EXCITED_AMPLITUDE.to_bits(), 0.8_f64.to_bits());
        assert_eq!(CANONICAL_GROUND_COUPLING, [1.0, 0.5]);
        assert_eq!(CANONICAL_EXCITED_COUPLING, [0.25, 1.0]);
        assert_eq!(
            GOLDEN_CANONICAL_PRINCIPAL_EIGENVALUE.to_bits(),
            0.673_346_666_250_657_f64.to_bits()
        );
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

        assert_close(
            projection.principal_eigenvalue,
            GOLDEN_CANONICAL_PRINCIPAL_EIGENVALUE,
            2.0e-15,
        );

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

        let huge_scaled: Vec<_> = matrices
            .iter()
            .copied()
            .map(|matrix| matrix_scale_basis(matrix, [1.0e16, 1.0]))
            .collect();
        let huge =
            principal_generalized_projection(&huge_scaled, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_close(
            first.principal_eigenvalue,
            huge.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in first
            .projected_correlator
            .iter()
            .zip(&huge.projected_correlator)
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
    fn distinct_representable_eigenvalues_below_pivot_tolerance_are_solved() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let principal = 1.0 + 1.0e-14;
        let matrices = vec![identity, identity, [[principal, 0.0], [0.0, 1.0]]];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_eq!(projection.principal_eigenvalue, principal);
        assert_eq!(projection.coefficients, [1.0, 0.0]);
        assert_eq!(projection.projected_correlator, vec![1.0, 1.0, principal]);
    }

    #[test]
    fn ill_conditioned_positive_definite_reference_uses_stable_normalization() {
        let c0 = [[1.0, 0.999_999], [0.999_999, 1.0]];
        let c1 = [[2.0, 0.3], [0.3, 1.0]];
        let matrices = vec![c0, c0, c1];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert!(projection.principal_eigenvalue.is_finite());
        assert!(projection.principal_eigenvalue > 0.0);
        assert_close(
            projection.projected_correlator[REFERENCE_TIME],
            1.0,
            NORMALIZATION_TOLERANCE,
        );
    }

    #[test]
    fn subnormal_positive_principal_root_survives_extreme_dynamic_range() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrices = vec![identity, identity, [[-1.0e308, 0.0], [0.0, 1.0e-308]]];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_eq!(projection.principal_eigenvalue, 1.0e-308);
        assert_eq!(projection.coefficients, [0.0, 1.0]);
        assert_eq!(projection.projected_correlator, vec![1.0, 1.0, 1.0e-308]);
    }

    #[test]
    fn tolerated_reference_asymmetry_is_symmetrized_before_positivity() {
        let c0 = [[1.0, 1.0 - 1.0e-13], [1.0 + 1.0e-13, 1.0]];
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrices = vec![c0, c0, identity];

        assert_eq!(
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap_err(),
            VariationalError::ReferenceMatrixNotPositiveDefinite
        );
    }

    #[test]
    fn tiny_off_diagonal_preserves_unique_principal_direction() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrices = vec![identity, identity, [[1.0, 1.0e-16], [1.0e-16, 1.0]]];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        let expected = core::f64::consts::FRAC_1_SQRT_2;
        assert_close(projection.coefficients[0], expected, 2.0e-15);
        assert_close(projection.coefficients[1], expected, 2.0e-15);
    }

    #[test]
    fn repeated_ill_conditioned_reference_lags_project_identically() {
        let e = 2.0e-14;
        let c0 = [[1.0, 1.0 - e], [1.0 - e, 1.0]];
        let c1 = [
            [1.0 + e / 2.0, 1.0 - 3.0 * e / 2.0],
            [1.0 - 3.0 * e / 2.0, 1.0 + e / 2.0],
        ];
        let matrices = vec![c0, c0, c1];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        assert_close(
            projection.projected_correlator[0],
            1.0,
            NORMALIZATION_TOLERANCE,
        );
        assert_close(
            projection.projected_correlator[1],
            1.0,
            NORMALIZATION_TOLERANCE,
        );
        assert_eq!(
            projection.projected_correlator[0].to_bits(),
            projection.projected_correlator[1].to_bits()
        );
    }

    #[test]
    fn symmetrization_preserves_equal_subnormal_entries_bitwise() {
        let minimum_subnormal = f64::from_bits(1);
        let cross = 3.0 * minimum_subnormal;
        let matrix = [
            [16.0 * minimum_subnormal, cross],
            [cross, minimum_subnormal],
        ];
        let symmetric = symmetrized_matrix(matrix).unwrap();
        assert_eq!(symmetric[0][1].to_bits(), cross.to_bits());
        assert_eq!(symmetric[1][0].to_bits(), cross.to_bits());
    }

    #[test]
    fn uniform_min_subnormal_basis_rescaling_preserves_projection() {
        let c0 = [[16.0, 3.0], [3.0, 1.0]];
        let c1 = [[8.0, 1.0], [1.0, 1.0]];
        let matrices = vec![c0, c0, c1];
        let baseline =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        let minimum_subnormal = f64::from_bits(1);
        let scale = minimum_subnormal.sqrt();
        let scaled: Vec<_> = matrices
            .iter()
            .copied()
            .map(|matrix| matrix_scale_basis(matrix, [scale, scale]))
            .collect();
        let rescaled =
            principal_generalized_projection(&scaled, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_relative_close(
            baseline.principal_eigenvalue,
            rescaled.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in baseline
            .projected_correlator
            .iter()
            .zip(&rescaled.projected_correlator)
        {
            assert_relative_close(left, right, 2.0e-12);
        }
    }

    #[test]
    fn extreme_nonuniform_basis_rescaling_preserves_projection() {
        let c0 = [[1.0, 0.5], [0.5, 1.0]];
        let c1 = [[2.0, 0.3], [0.3, 1.0]];
        let matrices = vec![c0, c0, c1];

        let baseline =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        let scale = [f64::from_bits(1).sqrt(), 1.0e154];
        let scaled: Vec<_> = matrices
            .iter()
            .copied()
            .map(|matrix| matrix_scale_basis(matrix, scale))
            .collect();

        assert_eq!(
            scaled[REFERENCE_TIME][0][0].to_bits(),
            f64::from_bits(1).to_bits()
        );
        assert!(scaled[REFERENCE_TIME][0][1].is_finite());
        assert!(scaled[REFERENCE_TIME][1][1].is_finite());

        let rescaled =
            principal_generalized_projection(&scaled, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_relative_close(
            baseline.principal_eigenvalue,
            rescaled.principal_eigenvalue,
            2.0e-12,
        );
        for (&left, &right) in baseline
            .projected_correlator
            .iter()
            .zip(&rescaled.projected_correlator)
        {
            assert_relative_close(left, right, 2.0e-12);
        }
    }

    #[test]
    fn inverse_cholesky_product_division_avoids_intermediate_overflow() {
        let minimum_subnormal = f64::from_bits(1);
        let l00 = minimum_subnormal.sqrt();
        let l10 = 0.5 * 1.0e154;
        let l11 = 0.5_f64.sqrt() * 1.0e154;

        assert!((l10 / l00).is_infinite());

        let stable = stable_divide_by_product(l10, l00, l11).unwrap();
        assert!(stable.is_finite());
        assert!(stable > 0.0);

        let tiny_l00 = (16.0 * minimum_subnormal).sqrt();
        let tiny_l10 = (3.0 * minimum_subnormal) / tiny_l00;
        let tiny_l11 = minimum_subnormal.sqrt() * (7.0_f64 / 16.0).sqrt();
        let sequential = (tiny_l10 / tiny_l00) / tiny_l11;
        let stable_tiny = stable_divide_by_product(tiny_l10, tiny_l00, tiny_l11).unwrap();
        assert_eq!(stable_tiny.to_bits(), sequential.to_bits());
    }

    #[test]
    fn tiny_positive_root_survives_near_zero_determinant_cancellation() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrix = [
            [-0.1, 0.761_577_310_586_390_9],
            [0.761_577_310_586_390_9, -5.8],
        ];
        let matrices = vec![identity, identity, matrix];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_close(
            projection.principal_eigenvalue,
            1.279_778_358_347_230_5e-17,
            1.0e-30,
        );
        assert!(projection.principal_eigenvalue > 0.0);
    }

    #[test]
    fn compensated_determinant_recovers_bits_lost_by_rounded_products() {
        let a = -0.1_f64;
        let d = -5.8_f64;
        let b = 0.761_577_310_586_390_9_f64;

        let naive = a * d - b * b;
        let compensated = compensated_symmetric_determinant(a, d, b).unwrap();

        assert_eq!(naive, -1.110_223_024_625_156_5e-16);
        assert!(compensated < 0.0);
        assert!(compensated.is_finite());
        assert!(compensated.abs() < naive.abs());
    }

    #[test]
    fn huge_finite_matrix_preserves_positive_principal_root() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrix = [[-8.0e307, 1.0e308], [1.0e308, -8.0e307]];
        let matrices = vec![identity, identity, matrix];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert_close(
            projection.principal_eigenvalue,
            2.000_000_000_000_000_2e307,
            4.0e292,
        );
        let expected = core::f64::consts::FRAC_1_SQRT_2;
        assert_close(projection.coefficients[0], expected, 2.0e-15);
        assert_close(projection.coefficients[1], expected, 2.0e-15);
    }

    #[test]
    fn scaled_generalized_recovery_survives_product_overflow() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let matrix = [[-8.0e307, 1.0e308], [1.0e308, -8.0e307]];
        assert!((matrix[0][0] * matrix[1][1]).is_infinite());
        assert!((matrix[0][1] * matrix[0][1]).is_infinite());

        let root = stable_generalized_principal_eigenvalue(identity, matrix).unwrap();
        assert_close(root, 2.000_000_000_000_000_2e307, 4.0e292);
        assert!(root.is_finite());
        assert!(root > 0.0);
    }

    #[test]
    fn negative_definite_diagonalization_matrix_rejects_spurious_positive_root() {
        let c0 = [[1.0, 0.5], [0.5, 1.0]];
        let c1 = [[-1.0e10, 0.0], [0.0, -1.0e-10]];
        let matrices = vec![c0, c0, c1];

        assert_eq!(
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap_err(),
            VariationalError::NonPositivePrincipalEigenvalue
        );
    }

    #[test]
    fn generalized_characteristic_preserves_inertia_signs() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let negative_definite = [[-1.0e10, 0.0], [0.0, -1.0e-10]];
        let negative_root =
            stable_generalized_principal_eigenvalue(identity, negative_definite).unwrap();
        assert!(negative_root < 0.0);

        let indefinite = [
            [-0.1, 0.761_577_310_586_390_9],
            [0.761_577_310_586_390_9, -5.8],
        ];
        let positive_root =
            stable_generalized_principal_eigenvalue(identity, indefinite).unwrap();
        assert!(positive_root > 0.0);
    }

    #[test]
    fn near_zero_positive_mode_is_permutation_invariant() {
        let c0 = [
            [9.999_999_999_999_992e-301, 4.999_999_999_999_997e-295],
            [4.999_999_999_999_997e-295, 9.999_999_999_999_997e-289],
        ];
        let c1 = [
            [-9.999_999_999_999_992e-302, 7.615_773_105_863_904e-295],
            [7.615_773_105_863_904e-295, -5.799_999_999_999_998e-288],
        ];
        let matrices = vec![c0, c0, c1];
        let swapped: Vec<_> = matrices.iter().copied().map(matrix_swap_basis).collect();

        let first =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();
        let second =
            principal_generalized_projection(&swapped, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert!(first.principal_eigenvalue > 0.0);
        assert_eq!(
            first.principal_eigenvalue.to_bits(),
            second.principal_eigenvalue.to_bits()
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
    fn near_singular_indefinite_matrix_preserves_positive_mode() {
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let c1 = [
            [-9.999_999_999_999_994e-301, 3.162_277_660_168_378_4e-150],
            [3.162_277_660_168_378_4e-150, -10.0],
        ];
        let matrices = vec![identity, identity, c1];

        let projection =
            principal_generalized_projection(&matrices, REFERENCE_TIME, DIAGONALIZATION_TIME)
                .unwrap();

        assert!(projection.principal_eigenvalue > 0.0);
        assert_close(
            projection.principal_eigenvalue,
            5.872_016_694_893_03e-317,
            5.0e-320,
        );
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
