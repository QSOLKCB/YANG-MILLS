// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3E scalar plateau-window selection and correlated one-state fitting.

use core::fmt;

use crate::glueball_effective_mass::{
    analytic_periodic_single_state, cosh_effective_mass_with_jackknife, EffectiveMassError,
};

pub const SCALAR_FIT_CONTRACT_ID: &str = "YM-GLUEBALL-SCALAR-FIT-V1";
pub const SCALAR_FIT_STATUS: &str = "phase-3e-candidate";
pub const MIN_WINDOW_POINTS: usize = 3;
pub const MAX_WINDOW_POINTS: usize = 8;
pub const MAX_CANDIDATE_WINDOWS: usize = 16;
pub const PLATEAU_MAX_Z: f64 = 2.5;
pub const MASS_GRID_MIN: f64 = 0.05;
pub const MASS_GRID_MAX: f64 = 2.0;
pub const MASS_GRID_STEP: f64 = 0.001;
pub const MASS_GRID_STEPS: usize = 1950;
pub const COVARIANCE_PIVOT_REL_TOLERANCE: f64 = 1.0e-12;
pub const ANALYTIC_REPLAY_TOLERANCE: f64 = 2.0e-10;
pub const CANONICAL_PERTURBATION: f64 = 0.005;
pub const CANONICAL_SHARED_CORRELATION: f64 = 0.25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FitWindow {
    pub start: usize,
    pub end: usize,
}

impl FitWindow {
    pub fn point_count(self) -> Option<usize> {
        self.end.checked_sub(self.start)?.checked_add(1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSelection {
    pub window: FitWindow,
    pub weighted_effective_mass: f64,
    pub maximum_standardized_deviation: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScalarFitResult {
    pub time_extent: usize,
    pub jackknife_replicates: usize,
    pub selected_window: FitWindow,
    pub plateau_weighted_effective_mass: f64,
    pub plateau_maximum_z: f64,
    pub fitted_mass: f64,
    pub fitted_amplitude: f64,
    pub chi_square: f64,
    pub degrees_of_freedom: usize,
    pub chi_square_per_dof: f64,
    pub mass_jackknife_mean: f64,
    pub mass_standard_error: f64,
    pub amplitude_jackknife_mean: f64,
    pub amplitude_standard_error: f64,
    pub maximum_off_diagonal_correlation: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarFitError {
    EmptyCandidateWindows,
    TooManyCandidateWindows,
    InvalidCandidateWindow,
    DuplicateCandidateWindow,
    DegeneratePlateauUncertainty,
    NoAcceptedPlateauWindow,
    InsufficientReplicatesForCovariance,
    CovarianceNotPositiveDefinite,
    SingularCovariance,
    NonFiniteStatistic,
    NoFiniteGridFit,
    AllocationFailed,
    EffectiveMass(EffectiveMassError),
}

impl fmt::Display for ScalarFitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyCandidateWindows => {
                "Phase 3E requires at least one predeclared candidate window"
            }
            Self::TooManyCandidateWindows => {
                "Phase 3E candidate-window count exceeds the bounded reference limit"
            }
            Self::InvalidCandidateWindow => {
                "Phase 3E candidate windows must be bounded before the temporal midpoint"
            }
            Self::DuplicateCandidateWindow => "Phase 3E candidate windows must be unique",
            Self::DegeneratePlateauUncertainty => {
                "Phase 3E plateau selection requires positive finite jackknife errors"
            }
            Self::NoAcceptedPlateauWindow => {
                "Phase 3E no predeclared candidate passed the plateau criterion"
            }
            Self::InsufficientReplicatesForCovariance => {
                "Phase 3E requires more jackknife replicas than fitted time points"
            }
            Self::CovarianceNotPositiveDefinite => {
                "Phase 3E jackknife covariance must be positive definite"
            }
            Self::SingularCovariance => "Phase 3E jackknife covariance inversion failed closed",
            Self::NonFiniteStatistic => "Phase 3E fit statistic became non-finite",
            Self::NoFiniteGridFit => {
                "Phase 3E bounded profile grid produced no admissible finite fit"
            }
            Self::AllocationFailed => "Phase 3E bounded allocation failed",
            Self::EffectiveMass(_) => "Phase 3E effective-mass plateau evaluation failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for ScalarFitError {}

impl From<EffectiveMassError> for ScalarFitError {
    fn from(value: EffectiveMassError) -> Self {
        Self::EffectiveMass(value)
    }
}

fn zero_vec(len: usize) -> Result<Vec<f64>, ScalarFitError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    values.resize(len, 0.0);
    Ok(values)
}

fn zero_matrix(size: usize) -> Result<Vec<Vec<f64>>, ScalarFitError> {
    let mut matrix = Vec::new();
    matrix
        .try_reserve_exact(size)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for _ in 0..size {
        matrix.push(zero_vec(size)?);
    }
    Ok(matrix)
}

fn validate_candidate_windows(
    time_extent: usize,
    candidates: &[FitWindow],
) -> Result<(), ScalarFitError> {
    if candidates.is_empty() {
        return Err(ScalarFitError::EmptyCandidateWindows);
    }
    if candidates.len() > MAX_CANDIDATE_WINDOWS {
        return Err(ScalarFitError::TooManyCandidateWindows);
    }
    for (index, &candidate) in candidates.iter().enumerate() {
        let count = candidate
            .point_count()
            .ok_or(ScalarFitError::InvalidCandidateWindow)?;
        if candidate.start == 0
            || count < MIN_WINDOW_POINTS
            || count > MAX_WINDOW_POINTS
            || candidate.end.saturating_mul(2) >= time_extent
        {
            return Err(ScalarFitError::InvalidCandidateWindow);
        }
        if candidates[..index].contains(&candidate) {
            return Err(ScalarFitError::DuplicateCandidateWindow);
        }
    }
    Ok(())
}

fn window_lags(window: FitWindow) -> Result<Vec<usize>, ScalarFitError> {
    let count = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    let mut lags = Vec::new();
    lags.try_reserve_exact(count)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for tau in window.start..=window.end {
        lags.push(tau);
    }
    Ok(lags)
}

fn evaluate_plateau_window(
    full: &[f64],
    jackknife: &[Vec<f64>],
    window: FitWindow,
) -> Result<WindowSelection, ScalarFitError> {
    let lags = window_lags(window)?;
    let series = cosh_effective_mass_with_jackknife(full, jackknife, &lags)?;
    let mut weighted_sum = 0.0_f64;
    let mut weight_sum = 0.0_f64;
    for point in &series.points {
        if !point.standard_error.is_finite() || point.standard_error <= 0.0 {
            return Err(ScalarFitError::DegeneratePlateauUncertainty);
        }
        let weight = 1.0 / (point.standard_error * point.standard_error);
        weighted_sum += weight * point.full_estimate;
        weight_sum += weight;
        if !weighted_sum.is_finite() || !weight_sum.is_finite() {
            return Err(ScalarFitError::NonFiniteStatistic);
        }
    }
    if weight_sum <= 0.0 {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    let weighted_effective_mass = weighted_sum / weight_sum;
    if !weighted_effective_mass.is_finite() {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    let mut maximum_standardized_deviation = 0.0_f64;
    for point in &series.points {
        let z = ((point.full_estimate - weighted_effective_mass) / point.standard_error).abs();
        if !z.is_finite() {
            return Err(ScalarFitError::NonFiniteStatistic);
        }
        maximum_standardized_deviation = maximum_standardized_deviation.max(z);
    }
    Ok(WindowSelection {
        window,
        weighted_effective_mass,
        maximum_standardized_deviation,
    })
}

fn choose_plateau_window(
    full: &[f64],
    jackknife: &[Vec<f64>],
    candidates: &[FitWindow],
) -> Result<WindowSelection, ScalarFitError> {
    validate_candidate_windows(full.len(), candidates)?;
    let mut best: Option<WindowSelection> = None;
    for &candidate in candidates {
        let selection = match evaluate_plateau_window(full, jackknife, candidate) {
            Ok(value) => value,
            Err(ScalarFitError::DegeneratePlateauUncertainty) => continue,
            Err(error) => return Err(error),
        };
        if selection.maximum_standardized_deviation > PLATEAU_MAX_Z {
            continue;
        }
        let candidate_count = candidate
            .point_count()
            .ok_or(ScalarFitError::InvalidCandidateWindow)?;
        let replace = match best {
            None => true,
            Some(current) => {
                let current_count = current
                    .window
                    .point_count()
                    .ok_or(ScalarFitError::InvalidCandidateWindow)?;
                candidate_count > current_count
                    || (candidate_count == current_count && candidate.start > current.window.start)
            }
        };
        if replace {
            best = Some(selection);
        }
    }
    best.ok_or(ScalarFitError::NoAcceptedPlateauWindow)
}

fn jackknife_covariance(
    jackknife: &[Vec<f64>],
    window: FitWindow,
) -> Result<Vec<Vec<f64>>, ScalarFitError> {
    let points = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    if jackknife.len() <= points {
        return Err(ScalarFitError::InsufficientReplicatesForCovariance);
    }
    let mut means = zero_vec(points)?;
    for replica in jackknife {
        for (column, tau) in (window.start..=window.end).enumerate() {
            let value = *replica
                .get(tau)
                .ok_or(ScalarFitError::InvalidCandidateWindow)?;
            if !value.is_finite() {
                return Err(ScalarFitError::NonFiniteStatistic);
            }
            means[column] += value;
        }
    }
    let n = jackknife.len() as f64;
    for mean in &mut means {
        *mean /= n;
    }
    let mut covariance = zero_matrix(points)?;
    for replica in jackknife {
        for row in 0..points {
            let left = replica[window.start + row] - means[row];
            for column in 0..points {
                let right = replica[window.start + column] - means[column];
                covariance[row][column] += left * right;
            }
        }
    }
    let factor = (n - 1.0) / n;
    for row in &mut covariance {
        for value in row {
            *value *= factor;
            if !value.is_finite() {
                return Err(ScalarFitError::NonFiniteStatistic);
            }
        }
    }
    Ok(covariance)
}

fn validate_positive_definite(matrix: &[Vec<f64>]) -> Result<(), ScalarFitError> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return Err(ScalarFitError::CovarianceNotPositiveDefinite);
    }
    let scale = matrix
        .iter()
        .flatten()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    if !scale.is_finite() || scale <= 0.0 {
        return Err(ScalarFitError::CovarianceNotPositiveDefinite);
    }
    let mut lower = zero_matrix(n)?;
    for i in 0..n {
        for j in 0..=i {
            let mut sum = matrix[i][j];
            for k in 0..j {
                sum -= lower[i][k] * lower[j][k];
            }
            if i == j {
                if !sum.is_finite() || sum <= scale * COVARIANCE_PIVOT_REL_TOLERANCE {
                    return Err(ScalarFitError::CovarianceNotPositiveDefinite);
                }
                lower[i][j] = sum.sqrt();
            } else {
                let pivot = lower[j][j];
                if !pivot.is_finite() || pivot <= 0.0 {
                    return Err(ScalarFitError::CovarianceNotPositiveDefinite);
                }
                lower[i][j] = sum / pivot;
            }
        }
    }
    Ok(())
}

fn invert_matrix(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, ScalarFitError> {
    validate_positive_definite(matrix)?;
    let n = matrix.len();
    let scale = matrix
        .iter()
        .flatten()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    let mut augmented = Vec::new();
    augmented
        .try_reserve_exact(n)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for row in 0..n {
        let mut values = zero_vec(2 * n)?;
        values[..n].copy_from_slice(&matrix[row]);
        values[n + row] = 1.0;
        augmented.push(values);
    }
    for column in 0..n {
        let mut pivot_row = column;
        let mut pivot_abs = augmented[column][column].abs();
        for (row, values) in augmented.iter().enumerate().skip(column + 1) {
            let candidate = values[column].abs();
            if candidate > pivot_abs {
                pivot_abs = candidate;
                pivot_row = row;
            }
        }
        if !pivot_abs.is_finite() || pivot_abs <= scale * COVARIANCE_PIVOT_REL_TOLERANCE {
            return Err(ScalarFitError::SingularCovariance);
        }
        if pivot_row != column {
            augmented.swap(pivot_row, column);
        }
        let pivot = augmented[column][column];
        for value in &mut augmented[column] {
            *value /= pivot;
        }
        let pivot_values = augmented[column].clone();
        for (row, values) in augmented.iter_mut().enumerate() {
            if row == column {
                continue;
            }
            let factor = values[column];
            for index in 0..(2 * n) {
                values[index] -= factor * pivot_values[index];
            }
        }
    }
    let mut inverse = zero_matrix(n)?;
    for row in 0..n {
        for column in 0..n {
            let value = augmented[row][n + column];
            if !value.is_finite() {
                return Err(ScalarFitError::SingularCovariance);
            }
            inverse[row][column] = value;
        }
    }
    Ok(inverse)
}

fn quadratic_form(left: &[f64], matrix: &[Vec<f64>], right: &[f64]) -> Result<f64, ScalarFitError> {
    if left.len() != matrix.len() || right.len() != matrix.len() {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    let mut sum = 0.0_f64;
    for row in 0..matrix.len() {
        let mut inner = 0.0_f64;
        for column in 0..matrix.len() {
            inner += matrix[row][column] * right[column];
        }
        sum += left[row] * inner;
    }
    if !sum.is_finite() {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    Ok(sum)
}

fn periodic_basis(
    time_extent: usize,
    window: FitWindow,
    mass: f64,
) -> Result<Vec<f64>, ScalarFitError> {
    let points = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    let mut basis = Vec::new();
    basis
        .try_reserve_exact(points)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for tau in window.start..=window.end {
        let value = (-mass * tau as f64).exp() + (-mass * (time_extent - tau) as f64).exp();
        if !value.is_finite() || value <= 0.0 {
            return Err(ScalarFitError::NonFiniteStatistic);
        }
        basis.push(value);
    }
    Ok(basis)
}

#[derive(Clone, Copy, Debug)]
struct ProfileFit {
    mass: f64,
    amplitude: f64,
    chi_square: f64,
}

fn profile_grid_fit(
    time_extent: usize,
    window: FitWindow,
    values: &[f64],
    inverse_covariance: &[Vec<f64>],
) -> Result<ProfileFit, ScalarFitError> {
    let expected = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    if values.len() != expected || values.iter().any(|value| !value.is_finite()) {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    let mut best: Option<ProfileFit> = None;
    for grid_index in 0..=MASS_GRID_STEPS {
        let mass = MASS_GRID_MIN + MASS_GRID_STEP * grid_index as f64;
        if mass > MASS_GRID_MAX + 1.0e-15 {
            break;
        }
        let basis = periodic_basis(time_extent, window, mass)?;
        let denominator = quadratic_form(&basis, inverse_covariance, &basis)?;
        if denominator <= 0.0 || !denominator.is_finite() {
            continue;
        }
        let numerator = quadratic_form(&basis, inverse_covariance, values)?;
        let amplitude = numerator / denominator;
        if amplitude <= 0.0 || !amplitude.is_finite() {
            continue;
        }
        let mut residual = Vec::new();
        residual
            .try_reserve_exact(expected)
            .map_err(|_| ScalarFitError::AllocationFailed)?;
        for (&value, &model) in values.iter().zip(&basis) {
            residual.push(value - amplitude * model);
        }
        let chi_square = quadratic_form(&residual, inverse_covariance, &residual)?;
        if chi_square < 0.0 || !chi_square.is_finite() {
            continue;
        }
        let candidate = ProfileFit {
            mass,
            amplitude,
            chi_square,
        };
        let replace = match best {
            None => true,
            Some(current) => {
                chi_square < current.chi_square
                    || (chi_square == current.chi_square && mass < current.mass)
            }
        };
        if replace {
            best = Some(candidate);
        }
    }
    best.ok_or(ScalarFitError::NoFiniteGridFit)
}

fn window_values(values: &[f64], window: FitWindow) -> Result<Vec<f64>, ScalarFitError> {
    let points = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    let mut selected = Vec::new();
    selected
        .try_reserve_exact(points)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for tau in window.start..=window.end {
        let value = *values
            .get(tau)
            .ok_or(ScalarFitError::InvalidCandidateWindow)?;
        if !value.is_finite() {
            return Err(ScalarFitError::NonFiniteStatistic);
        }
        selected.push(value);
    }
    Ok(selected)
}

fn jackknife_standard_error(values: &[f64]) -> Result<(f64, f64), ScalarFitError> {
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let mut sum = 0.0_f64;
    for &value in values {
        let delta = value - mean;
        sum += delta * delta;
    }
    let variance = ((n - 1.0) / n) * sum;
    let standard_error = variance.sqrt();
    if !mean.is_finite() || !variance.is_finite() || variance < 0.0 || !standard_error.is_finite() {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    Ok((mean, standard_error))
}

fn maximum_off_diagonal_correlation(covariance: &[Vec<f64>]) -> Result<f64, ScalarFitError> {
    let mut maximum = 0.0_f64;
    for row in 0..covariance.len() {
        if covariance[row][row] <= 0.0 || !covariance[row][row].is_finite() {
            return Err(ScalarFitError::CovarianceNotPositiveDefinite);
        }
        for column in 0..row {
            let denominator = (covariance[row][row] * covariance[column][column]).sqrt();
            let correlation = (covariance[row][column] / denominator).abs();
            if !correlation.is_finite() {
                return Err(ScalarFitError::NonFiniteStatistic);
            }
            maximum = maximum.max(correlation);
        }
    }
    Ok(maximum)
}

pub fn correlated_scalar_fit(
    full: &[f64],
    jackknife: &[Vec<f64>],
    candidate_windows: &[FitWindow],
) -> Result<ScalarFitResult, ScalarFitError> {
    let selection = choose_plateau_window(full, jackknife, candidate_windows)?;
    let window = selection.window;
    let covariance = jackknife_covariance(jackknife, window)?;
    let inverse_covariance = invert_matrix(&covariance)?;
    let maximum_off_diagonal_correlation = maximum_off_diagonal_correlation(&covariance)?;
    let full_values = window_values(full, window)?;
    let full_fit = profile_grid_fit(full.len(), window, &full_values, &inverse_covariance)?;

    let mut replica_masses = Vec::new();
    let mut replica_amplitudes = Vec::new();
    replica_masses
        .try_reserve_exact(jackknife.len())
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    replica_amplitudes
        .try_reserve_exact(jackknife.len())
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for replica in jackknife {
        let values = window_values(replica, window)?;
        let fit = profile_grid_fit(full.len(), window, &values, &inverse_covariance)?;
        replica_masses.push(fit.mass);
        replica_amplitudes.push(fit.amplitude);
    }
    let (mass_jackknife_mean, mass_standard_error) = jackknife_standard_error(&replica_masses)?;
    let (amplitude_jackknife_mean, amplitude_standard_error) =
        jackknife_standard_error(&replica_amplitudes)?;
    let points = window
        .point_count()
        .ok_or(ScalarFitError::InvalidCandidateWindow)?;
    let degrees_of_freedom = points - 2;
    let chi_square_per_dof = full_fit.chi_square / degrees_of_freedom as f64;
    if !chi_square_per_dof.is_finite() {
        return Err(ScalarFitError::NonFiniteStatistic);
    }
    Ok(ScalarFitResult {
        time_extent: full.len(),
        jackknife_replicates: jackknife.len(),
        selected_window: window,
        plateau_weighted_effective_mass: selection.weighted_effective_mass,
        plateau_maximum_z: selection.maximum_standardized_deviation,
        fitted_mass: full_fit.mass,
        fitted_amplitude: full_fit.amplitude,
        chi_square: full_fit.chi_square,
        degrees_of_freedom,
        chi_square_per_dof,
        mass_jackknife_mean,
        mass_standard_error,
        amplitude_jackknife_mean,
        amplitude_standard_error,
        maximum_off_diagonal_correlation,
    })
}

const HADAMARD_8: [[f64; 8]; 8] = [
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    [1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
    [1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0],
    [1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0],
    [1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0],
];

pub fn canonical_scalar_fit_fixture(
) -> Result<(Vec<f64>, Vec<Vec<f64>>, Vec<FitWindow>), ScalarFitError> {
    let full = analytic_periodic_single_state(16, 0.5, 1.0)?;
    let mut jackknife = Vec::new();
    jackknife
        .try_reserve_exact(HADAMARD_8.len())
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    for row in HADAMARD_8 {
        let mut replica = full.clone();
        for tau in 1..=7 {
            let pattern = if tau == 1 {
                row[1]
            } else {
                row[tau] + CANONICAL_SHARED_CORRELATION * row[1]
            };
            replica[tau] = full[tau] * (1.0 + CANONICAL_PERTURBATION * pattern);
        }
        jackknife.push(replica);
    }
    let mut candidates = Vec::new();
    candidates
        .try_reserve_exact(3)
        .map_err(|_| ScalarFitError::AllocationFailed)?;
    candidates.push(FitWindow { start: 1, end: 3 });
    candidates.push(FitWindow { start: 2, end: 5 });
    candidates.push(FitWindow { start: 3, end: 6 });
    Ok((full, jackknife, candidates))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e} tolerance={tolerance:.17e}"
        );
    }

    #[test]
    fn phase3e_identity_is_frozen() {
        assert_eq!(SCALAR_FIT_CONTRACT_ID, "YM-GLUEBALL-SCALAR-FIT-V1");
        assert_eq!(SCALAR_FIT_STATUS, "phase-3e-candidate");
        assert_eq!(MIN_WINDOW_POINTS, 3);
        assert_eq!(MAX_WINDOW_POINTS, 8);
        assert_eq!(MAX_CANDIDATE_WINDOWS, 16);
        assert_eq!(PLATEAU_MAX_Z, 2.5);
        assert_eq!(MASS_GRID_MIN, 0.05);
        assert_eq!(MASS_GRID_MAX, 2.0);
        assert_eq!(MASS_GRID_STEP, 0.001);
        assert_eq!(MASS_GRID_STEPS, 1950);
    }

    #[test]
    fn canonical_fixture_selects_later_tie_and_recovers_single_state() {
        let (full, jackknife, candidates) = canonical_scalar_fit_fixture().unwrap();
        let result = correlated_scalar_fit(&full, &jackknife, &candidates).unwrap();
        assert_eq!(result.selected_window, FitWindow { start: 3, end: 6 });
        assert_close(
            result.plateau_weighted_effective_mass,
            0.5,
            ANALYTIC_REPLAY_TOLERANCE,
        );
        assert!(result.plateau_maximum_z <= 1.0e-10);
        assert_close(result.fitted_mass, 0.5, 1.0e-15);
        assert_close(result.fitted_amplitude, 1.0, ANALYTIC_REPLAY_TOLERANCE);
        assert_close(result.chi_square, 0.0, ANALYTIC_REPLAY_TOLERANCE);
        assert_eq!(result.degrees_of_freedom, 2);
        assert_close(result.mass_jackknife_mean, 0.5, 1.0e-15);
        assert_close(
            result.mass_standard_error,
            0.005916079783099616,
            ANALYTIC_REPLAY_TOLERANCE,
        );
        assert_close(
            result.maximum_off_diagonal_correlation,
            1.0 / 17.0,
            ANALYTIC_REPLAY_TOLERANCE,
        );
        assert!(result.amplitude_standard_error > 0.0);
    }

    #[test]
    fn candidate_window_rules_fail_closed() {
        let (full, jackknife, _) = canonical_scalar_fit_fixture().unwrap();
        for candidates in [
            vec![],
            vec![FitWindow { start: 0, end: 3 }],
            vec![FitWindow { start: 2, end: 3 }],
            vec![FitWindow { start: 3, end: 8 }],
        ] {
            assert!(correlated_scalar_fit(&full, &jackknife, &candidates).is_err());
        }
        let duplicate = [
            FitWindow { start: 2, end: 5 },
            FitWindow { start: 2, end: 5 },
        ];
        assert_eq!(
            correlated_scalar_fit(&full, &jackknife, &duplicate).unwrap_err(),
            ScalarFitError::DuplicateCandidateWindow
        );
    }

    #[test]
    fn covariance_requires_more_replicas_than_window_points() {
        let (full, jackknife, _) = canonical_scalar_fit_fixture().unwrap();
        let too_few = jackknife[..4].to_vec();
        assert_eq!(
            correlated_scalar_fit(&full, &too_few, &[FitWindow { start: 3, end: 6 }]).unwrap_err(),
            ScalarFitError::InsufficientReplicatesForCovariance
        );
    }

    #[test]
    fn singular_covariance_fails_closed() {
        let full = analytic_periodic_single_state(16, 0.5, 1.0).unwrap();
        let jackknife = vec![full.clone(); 8];
        let covariance = jackknife_covariance(&jackknife, FitWindow { start: 3, end: 6 }).unwrap();
        assert_eq!(
            invert_matrix(&covariance).unwrap_err(),
            ScalarFitError::CovarianceNotPositiveDefinite
        );
    }

    #[test]
    fn canonical_covariance_is_genuinely_correlated() {
        let (_, jackknife, _) = canonical_scalar_fit_fixture().unwrap();
        let covariance = jackknife_covariance(&jackknife, FitWindow { start: 3, end: 6 }).unwrap();
        let maximum = maximum_off_diagonal_correlation(&covariance).unwrap();
        assert!(maximum > 0.05);
        assert_close(maximum, 1.0 / 17.0, ANALYTIC_REPLAY_TOLERANCE);
    }
}
