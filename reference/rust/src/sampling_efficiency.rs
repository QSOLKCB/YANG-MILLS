// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Additive reporting mechanics; never changes a frozen sampler or its gates.
use crate::su2_ensemble::{diagnostics, ChainDiagnostics};
use core::fmt;

pub const CONTRACT_ID: &str = "YM-SAMPLING-EFFICIENCY-V1";
pub const MAX_SAMPLES: usize = 4096;
pub const MAX_LAG: usize = 256;

/// All components must use the same declared unit (seconds, proposals, etc.).
/// Include training/setup and discarded thermalisation work explicitly.
#[derive(Clone, Copy, Debug)]
pub struct SamplingCost {
    pub setup: f64,
    pub thermalization: f64,
    pub sampling: f64,
}

#[derive(Clone, Debug)]
pub struct EfficiencyReport {
    pub diagnostics: ChainDiagnostics,
    /// Kish weight-only ESS / N. This is not autocorrelation ESS.
    pub reweighting_efficiency: f64,
    pub weight_only_ess: f64,
    /// N * weight_efficiency / (2*tau). A comparison proxy, not a
    /// validated ESS for a correlated self-normalized weighted estimator.
    pub combined_ess_proxy: f64,
    pub total_cost: f64,
    pub sampling_cost_per_effective_sample_proxy: f64,
    pub total_cost_per_effective_sample_proxy: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfficiencyError {
    InvalidLength,
    InvalidLag,
    InvalidWeight,
    InvalidCost,
    InvalidDiagnostic,
    DegenerateObservable,
    NonFiniteResult,
}
impl fmt::Display for EfficiencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sampling efficiency input/result rejected: {self:?}")
    }
}
impl std::error::Error for EfficiencyError {}

/// Positive weights supplied as finite log weights. None means unit weights.
/// Uses max-shifting so the result is invariant to a common log-weight shift
/// without exponentiating huge absolute values. Constant series fail closed:
/// no observed variance cannot establish mixing or useful independent samples.
pub fn evaluate(
    values: &[f64],
    log_weights: Option<&[f64]>,
    max_lag: usize,
    cost: SamplingCost,
) -> Result<EfficiencyReport, EfficiencyError> {
    let n = values.len();
    if !(2..=MAX_SAMPLES).contains(&n) {
        return Err(EfficiencyError::InvalidLength);
    }
    if max_lag == 0 || max_lag >= n || max_lag > MAX_LAG {
        return Err(EfficiencyError::InvalidLag);
    }
    if [cost.setup, cost.thermalization, cost.sampling]
        .iter()
        .any(|x| !x.is_finite() || *x < 0.0)
        || cost.sampling == 0.0
    {
        return Err(EfficiencyError::InvalidCost);
    }
    let total_cost = cost.setup + cost.thermalization + cost.sampling;
    if !total_cost.is_finite() {
        return Err(EfficiencyError::InvalidCost);
    }
    let efficiency = match log_weights {
        None => 1.0,
        Some(weights) => {
            if weights.len() != n || weights.iter().any(|w| !w.is_finite()) {
                return Err(EfficiencyError::InvalidWeight);
            }
            let max = weights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let (mut sum, mut squares) = (0.0, 0.0);
            for &log_weight in weights {
                let weight = (log_weight - max).exp();
                sum += weight;
                squares += weight * weight;
            }
            ((sum / n as f64) * (sum / squares)).min(1.0)
        }
    };
    // A rounded sample mean can spuriously give identical non-binary-exact
    // values positive variance. Reject exact constancy before frozen arithmetic.
    if values.iter().all(|value| *value == values[0]) {
        return Err(EfficiencyError::DegenerateObservable);
    }
    let diagnostics =
        diagnostics(values, max_lag).map_err(|_| EfficiencyError::InvalidDiagnostic)?;
    if diagnostics.variance <= 0.0 {
        return Err(EfficiencyError::DegenerateObservable);
    }
    if !diagnostics.standard_error.is_finite() || diagnostics.standard_error <= 0.0 {
        return Err(EfficiencyError::NonFiniteResult);
    }
    let weight_only_ess = n as f64 * efficiency;
    let combined_ess_proxy = diagnostics.effective_sample_size * efficiency;
    let sampling_cost_per_effective_sample_proxy = cost.sampling / combined_ess_proxy;
    let total_cost_per_effective_sample_proxy = total_cost / combined_ess_proxy;
    if [
        efficiency,
        weight_only_ess,
        combined_ess_proxy,
        sampling_cost_per_effective_sample_proxy,
        total_cost_per_effective_sample_proxy,
    ]
    .iter()
    .any(|x| !x.is_finite() || *x <= 0.0)
    {
        return Err(EfficiencyError::NonFiniteResult);
    }
    Ok(EfficiencyReport {
        diagnostics,
        reweighting_efficiency: efficiency,
        weight_only_ess,
        combined_ess_proxy,
        total_cost,
        sampling_cost_per_effective_sample_proxy,
        total_cost_per_effective_sample_proxy,
    })
}
