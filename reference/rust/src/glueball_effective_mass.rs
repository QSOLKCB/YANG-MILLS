// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Phase 3D effective-mass mechanics for periodic scalar glueball correlators.
//!
//! This surface consumes a full connected correlator together with
//! delete-one-configuration jackknife correlator replicas. It evaluates the
//! nearest-neighbour periodic cosh effective-mass estimator and propagates the
//! non-linear transformation through the jackknife replicas. It deliberately
//! does not select a fit window, fit a spectral model, solve a GEVP, or claim a
//! finite-volume or continuum mass gap.

use core::fmt;

pub const EFFECTIVE_MASS_CONTRACT_ID: &str = "YM-GLUEBALL-EFFECTIVE-MASS-V1";
pub const EFFECTIVE_MASS_STATUS: &str = "phase-3d-candidate";
pub const MIN_JACKKNIFE_REPLICATES: usize = 3;
pub const MAX_JACKKNIFE_REPLICATES: usize = 64;
pub const MIN_TIME_EXTENT: usize = 4;
pub const MAX_TIME_EXTENT: usize = 64;
pub const ANALYTIC_REPLAY_TOLERANCE: f64 = 2.0e-12;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveMassPoint {
    pub tau: usize,
    pub full_estimate: f64,
    pub jackknife_mean: f64,
    pub standard_error: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EffectiveMassSeries {
    pub time_extent: usize,
    pub jackknife_replicates: usize,
    pub points: Vec<EffectiveMassPoint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectiveMassError {
    TimeExtentTooSmall,
    TimeExtentTooLarge,
    TooFewJackknifeReplicates,
    TooManyJackknifeReplicates,
    EmptyAnalysisWindow,
    InvalidAnalysisLag,
    ShapeMismatch,
    NonFiniteCorrelator,
    NonPositiveCentralCorrelator,
    AcoshDomainViolation,
    NonFiniteStatistic,
    AllocationFailed,
    InvalidFixtureParameter,
}

impl fmt::Display for EffectiveMassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TimeExtentTooSmall => "Phase 3D requires Euclidean time extent >= 4",
            Self::TimeExtentTooLarge => "Phase 3D time extent exceeds the bounded reference limit",
            Self::TooFewJackknifeReplicates => {
                "Phase 3D requires at least three delete-one jackknife replicas"
            }
            Self::TooManyJackknifeReplicates => {
                "Phase 3D jackknife replica count exceeds the bounded reference limit"
            }
            Self::EmptyAnalysisWindow => "Phase 3D analysis lag set must not be empty",
            Self::InvalidAnalysisLag => {
                "Phase 3D lags must be strictly increasing and lie before the temporal midpoint"
            }
            Self::ShapeMismatch => {
                "Phase 3D full and jackknife correlators must share one time extent"
            }
            Self::NonFiniteCorrelator => "Phase 3D correlator input must be finite",
            Self::NonPositiveCentralCorrelator => {
                "Phase 3D cosh effective mass requires C(tau) > 0"
            }
            Self::AcoshDomainViolation => {
                "Phase 3D cosh effective-mass acosh argument must be >= 1"
            }
            Self::NonFiniteStatistic => "Phase 3D effective-mass statistic became non-finite",
            Self::AllocationFailed => "Phase 3D bounded allocation failed",
            Self::InvalidFixtureParameter => "Phase 3D analytic fixture parameters are invalid",
        };
        f.write_str(message)
    }
}

impl std::error::Error for EffectiveMassError {}

fn validate_inputs(
    full: &[f64],
    jackknife: &[Vec<f64>],
    lags: &[usize],
) -> Result<(), EffectiveMassError> {
    let time_extent = full.len();
    if time_extent < MIN_TIME_EXTENT {
        return Err(EffectiveMassError::TimeExtentTooSmall);
    }
    if time_extent > MAX_TIME_EXTENT {
        return Err(EffectiveMassError::TimeExtentTooLarge);
    }
    if jackknife.len() < MIN_JACKKNIFE_REPLICATES {
        return Err(EffectiveMassError::TooFewJackknifeReplicates);
    }
    if jackknife.len() > MAX_JACKKNIFE_REPLICATES {
        return Err(EffectiveMassError::TooManyJackknifeReplicates);
    }
    if lags.is_empty() {
        return Err(EffectiveMassError::EmptyAnalysisWindow);
    }
    if full.iter().any(|value| !value.is_finite()) {
        return Err(EffectiveMassError::NonFiniteCorrelator);
    }
    if jackknife.iter().any(|replica| {
        replica.len() != time_extent || replica.iter().any(|value| !value.is_finite())
    }) {
        return Err(EffectiveMassError::ShapeMismatch);
    }

    let mut previous = 0usize;
    for (index, &tau) in lags.iter().enumerate() {
        if tau == 0
            || tau + 1 >= time_extent
            || tau.saturating_mul(2) >= time_extent
            || (index != 0 && tau <= previous)
        {
            return Err(EffectiveMassError::InvalidAnalysisLag);
        }
        previous = tau;
    }
    Ok(())
}

fn cosh_effective_mass(correlator: &[f64], tau: usize) -> Result<f64, EffectiveMassError> {
    let center = correlator[tau];
    if center <= 0.0 {
        return Err(EffectiveMassError::NonPositiveCentralCorrelator);
    }
    let numerator = correlator[tau - 1] + correlator[tau + 1];
    let denominator = 2.0 * center;
    let argument = numerator / denominator;
    if !numerator.is_finite() || !denominator.is_finite() || !argument.is_finite() {
        return Err(EffectiveMassError::NonFiniteStatistic);
    }
    if argument < 1.0 {
        return Err(EffectiveMassError::AcoshDomainViolation);
    }
    let mass = argument.acosh();
    if !mass.is_finite() {
        return Err(EffectiveMassError::NonFiniteStatistic);
    }
    Ok(mass)
}

/// Evaluate the periodic nearest-neighbour cosh effective mass
///
/// `m_eff(tau) = acosh((C(tau-1)+C(tau+1))/(2 C(tau)))`
///
/// on a predeclared set of lags. `jackknife` must contain one correlator per
/// delete-one-configuration replica, with the connected correlator (including
/// vacuum subtraction) recomputed inside each replica. The reported point
/// estimate is the full-sample transform. The standard error is
///
/// `sqrt((N-1)/N * sum_i (m_i - m_bar)^2)`.
///
/// This function cannot prove that callers constructed replicas correctly; the
/// Phase 3D contract makes that provenance requirement explicit.
pub fn cosh_effective_mass_with_jackknife(
    full: &[f64],
    jackknife: &[Vec<f64>],
    lags: &[usize],
) -> Result<EffectiveMassSeries, EffectiveMassError> {
    validate_inputs(full, jackknife, lags)?;

    let mut points = Vec::new();
    points
        .try_reserve_exact(lags.len())
        .map_err(|_| EffectiveMassError::AllocationFailed)?;

    for &tau in lags {
        let full_estimate = cosh_effective_mass(full, tau)?;

        let mut replicate_masses = Vec::new();
        replicate_masses
            .try_reserve_exact(jackknife.len())
            .map_err(|_| EffectiveMassError::AllocationFailed)?;
        let mut sum = 0.0_f64;
        for replica in jackknife {
            let mass = cosh_effective_mass(replica, tau)?;
            sum += mass;
            if !sum.is_finite() {
                return Err(EffectiveMassError::NonFiniteStatistic);
            }
            replicate_masses.push(mass);
        }

        let n = replicate_masses.len() as f64;
        let jackknife_mean = sum / n;
        if !jackknife_mean.is_finite() {
            return Err(EffectiveMassError::NonFiniteStatistic);
        }

        let mut squared_deviation_sum = 0.0_f64;
        for mass in replicate_masses {
            let deviation = mass - jackknife_mean;
            squared_deviation_sum += deviation * deviation;
            if !squared_deviation_sum.is_finite() {
                return Err(EffectiveMassError::NonFiniteStatistic);
            }
        }
        let variance = ((n - 1.0) / n) * squared_deviation_sum;
        let standard_error = variance.sqrt();
        if !variance.is_finite() || variance < 0.0 || !standard_error.is_finite() {
            return Err(EffectiveMassError::NonFiniteStatistic);
        }

        points.push(EffectiveMassPoint {
            tau,
            full_estimate,
            jackknife_mean,
            standard_error,
        });
    }

    Ok(EffectiveMassSeries {
        time_extent: full.len(),
        jackknife_replicates: jackknife.len(),
        points,
    })
}

/// Deterministic single-state periodic correlator used only for analytic
/// mechanics regression:
/// `C(tau)=A(exp(-m tau)+exp(-m (N_t-tau)))`.
pub fn analytic_periodic_single_state(
    time_extent: usize,
    mass: f64,
    amplitude: f64,
) -> Result<Vec<f64>, EffectiveMassError> {
    if !(MIN_TIME_EXTENT..=MAX_TIME_EXTENT).contains(&time_extent)
        || !mass.is_finite()
        || mass <= 0.0
        || !amplitude.is_finite()
        || amplitude <= 0.0
    {
        return Err(EffectiveMassError::InvalidFixtureParameter);
    }
    let mut correlator = Vec::new();
    correlator
        .try_reserve_exact(time_extent)
        .map_err(|_| EffectiveMassError::AllocationFailed)?;
    for tau in 0..time_extent {
        let left = (-mass * tau as f64).exp();
        let right = (-mass * (time_extent - tau) as f64).exp();
        let value = amplitude * (left + right);
        if !value.is_finite() {
            return Err(EffectiveMassError::NonFiniteStatistic);
        }
        correlator.push(value);
    }
    Ok(correlator)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let residual = (actual - expected).abs();
        assert!(
            residual <= ANALYTIC_REPLAY_TOLERANCE,
            "actual={actual:.17e} expected={expected:.17e} residual={residual:.17e}"
        );
    }

    fn canonical_fixture() -> (Vec<f64>, Vec<Vec<f64>>, Vec<usize>) {
        let full = analytic_periodic_single_state(16, 0.5, 1.0).unwrap();
        let masses = [0.48, 0.49, 0.50, 0.51, 0.52];
        let jackknife = masses
            .iter()
            .map(|&mass| analytic_periodic_single_state(16, mass, 1.0).unwrap())
            .collect::<Vec<_>>();
        (full, jackknife, vec![1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn phase3d_identity_is_frozen() {
        assert_eq!(EFFECTIVE_MASS_CONTRACT_ID, "YM-GLUEBALL-EFFECTIVE-MASS-V1");
        assert_eq!(EFFECTIVE_MASS_STATUS, "phase-3d-candidate");
        assert_eq!(MIN_JACKKNIFE_REPLICATES, 3);
        assert_eq!(MAX_JACKKNIFE_REPLICATES, 64);
        assert_eq!(MIN_TIME_EXTENT, 4);
        assert_eq!(MAX_TIME_EXTENT, 64);
    }

    #[test]
    fn periodic_single_state_recovers_exact_mass_and_jackknife_error() {
        let (full, jackknife, lags) = canonical_fixture();
        let result = cosh_effective_mass_with_jackknife(&full, &jackknife, &lags).unwrap();
        let expected_error = 0.0008_f64.sqrt();
        assert_eq!(result.time_extent, 16);
        assert_eq!(result.jackknife_replicates, 5);
        for point in result.points {
            assert_close(point.full_estimate, 0.5);
            assert_close(point.jackknife_mean, 0.5);
            assert_close(point.standard_error, expected_error);
        }
    }

    #[test]
    fn overall_correlator_amplitude_cancels_from_effective_mass() {
        let full = analytic_periodic_single_state(16, 0.5, 7.25).unwrap();
        let jackknife = [0.48, 0.49, 0.50, 0.51, 0.52]
            .iter()
            .map(|&mass| analytic_periodic_single_state(16, mass, 3.5).unwrap())
            .collect::<Vec<_>>();
        let result = cosh_effective_mass_with_jackknife(&full, &jackknife, &[3]).unwrap();
        assert_close(result.points[0].full_estimate, 0.5);
    }

    #[test]
    fn non_positive_center_fails_closed() {
        let (mut full, jackknife, _) = canonical_fixture();
        full[2] = 0.0;
        assert_eq!(
            cosh_effective_mass_with_jackknife(&full, &jackknife, &[2]).unwrap_err(),
            EffectiveMassError::NonPositiveCentralCorrelator
        );
    }

    #[test]
    fn acosh_domain_violation_fails_closed() {
        let (mut full, jackknife, _) = canonical_fixture();
        full[2] = 10.0;
        assert_eq!(
            cosh_effective_mass_with_jackknife(&full, &jackknife, &[2]).unwrap_err(),
            EffectiveMassError::AcoshDomainViolation
        );
    }

    #[test]
    fn jackknife_shape_and_count_fail_closed() {
        let (full, mut jackknife, _) = canonical_fixture();
        jackknife[0].pop();
        assert_eq!(
            cosh_effective_mass_with_jackknife(&full, &jackknife, &[2]).unwrap_err(),
            EffectiveMassError::ShapeMismatch
        );
        let too_few = vec![full.clone(), full.clone()];
        assert_eq!(
            cosh_effective_mass_with_jackknife(&full, &too_few, &[2]).unwrap_err(),
            EffectiveMassError::TooFewJackknifeReplicates
        );
    }

    #[test]
    fn analysis_lags_must_be_pre_midpoint_and_strictly_increasing() {
        let (full, jackknife, _) = canonical_fixture();
        for lags in [vec![0], vec![8], vec![2, 2], vec![3, 2]] {
            assert_eq!(
                cosh_effective_mass_with_jackknife(&full, &jackknife, &lags).unwrap_err(),
                EffectiveMassError::InvalidAnalysisLag
            );
        }
    }

    #[test]
    fn non_finite_input_fails_closed() {
        let (mut full, jackknife, _) = canonical_fixture();
        full[1] = f64::NAN;
        assert_eq!(
            cosh_effective_mass_with_jackknife(&full, &jackknife, &[2]).unwrap_err(),
            EffectiveMassError::NonFiniteCorrelator
        );
    }
}
