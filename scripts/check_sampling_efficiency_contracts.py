#!/usr/bin/env python3
"""Fail-closed reporting contract and analytic fixture checks."""
import json
from check_contracts import load, validate_exact
EXPECTED_CONTRACT = json.loads(r'''{
  "contract_id": "YM-SAMPLING-EFFICIENCY-V1",
  "status": "reporting-mechanics-candidate",
  "change_class": "additive new reporting authority; frozen samplers and physics gates unchanged",
  "source": {
    "url": "https://arxiv.org/abs/2610.20747v1",
    "equations": [
      "14",
      "16"
    ],
    "comparison": "Figure 5",
    "role": "metric inspiration only; no reproduction or algorithm implementation"
  },
  "limits": {
    "min_samples": 2,
    "max_samples": 4096,
    "min_lag": 1,
    "max_lag": 256,
    "lag_must_be_less_than_sample_count": true
  },
  "autocorrelation": {
    "authority": "YM-SU2-ENSEMBLE-V1 diagnostic arithmetic reused without mutation",
    "units": "retained measurement intervals",
    "estimator": "positive-window; tau=0.5+sum positive rho until first nonpositive lag",
    "ess": "N/(2*tau)",
    "zero_variance": "reject; mixing unidentifiable",
    "max_lag": "predeclare; inspect truncation and estimator sensitivity before scientific use"
  },
  "reweighting": {
    "input": "optional finite log weights with one weight per measurement; omitted means unit weights",
    "estimator": "(sum(exp(logw-max(logw))))^2 / (N*sum(exp(2*(logw-max(logw)))))",
    "weight_only_ess": "N*reweighting_efficiency",
    "not_autocorrelation_ess": true
  },
  "cost": {
    "components": [
      "setup",
      "thermalization",
      "sampling"
    ],
    "components_same_explicit_unit": true,
    "setup_and_thermalization_nonnegative": true,
    "sampling_strictly_positive": true,
    "total": "setup+thermalization+sampling",
    "combined_ess_proxy": "autocorrelation_ess*reweighting_efficiency",
    "sampling_cost_per_effective_sample_proxy": "sampling/combined_ess_proxy",
    "total_cost_per_effective_sample_proxy": "total/combined_ess_proxy",
    "proxy_not_validated_weighted_estimator_ess": true,
    "includes_variance_factor_from_equation16": false
  },
  "claim_boundary": {
    "flow_implemented": false,
    "topology_observable_implemented": false,
    "topological_freezing_resolved": false,
    "new_production_recovery": false,
    "mass_gap_proved": false
  },
  "canonical_runtime": {
    "basis": "existing Phase 2C1 beta-zero 16-measurement mechanics chain",
    "observable": "average plaquette",
    "cost_unit": "generator proposals",
    "setup": 0,
    "thermalization": 2048,
    "sampling": 8192,
    "total": 10240,
    "scope": "sampler proposals only; excludes analysis, IO, wall time, and hardware comparisons",
    "weights": "unit",
    "equilibrium_validation": false
  },
  "failure_policy": "reject malformed, nonfinite, degenerate, overflowing or nonpositive results; no silent fallback"
}''')
EXPECTED_VECTORS = json.loads(r'''{
  "fixture_id": "YM-SAMPLING-EFFICIENCY-REGRESSION-V1",
  "evidence_class": "analytic mechanics, not Monte Carlo physics",
  "tolerance": 1e-12,
  "values": [
    0,
    1,
    0,
    1
  ],
  "max_lag": 2,
  "cost": {
    "setup": 4,
    "thermalization": 8,
    "sampling": 20
  },
  "unit_weights": {
    "tau_int": 0.5,
    "autocorrelation_ess": 4,
    "weight_efficiency": 1,
    "combined_ess_proxy": 4,
    "sampling_cost_per_proxy": 5,
    "total_cost_per_proxy": 8
  },
  "weights_1_1_1_3": {
    "weight_efficiency": 0.75,
    "weight_only_ess": 3,
    "combined_ess_proxy": 3,
    "total_cost_per_proxy": 10.666666666666666
  },
  "correlated_values": [
    0,
    0,
    0,
    1,
    1,
    1
  ],
  "correlated_max_lag": 3,
  "correlated_tau_int": 1.1
}''')
def validate_contract(payload):
    validate_exact("sampling efficiency contract", payload, EXPECTED_CONTRACT)
def validate_vectors(payload):
    validate_exact("sampling efficiency vectors", payload, EXPECTED_VECTORS)
def main():
    validate_contract(load("contracts/YM-SAMPLING-EFFICIENCY-V1.json"))
    validate_vectors(load("test-vectors/sampling-efficiency-v1.json"))
    print("Sampling efficiency contract checks: passed")
if __name__ == "__main__":
    main()
