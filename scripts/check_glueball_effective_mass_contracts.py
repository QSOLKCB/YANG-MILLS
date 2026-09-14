#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3D effective-mass surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1",
    "status": "phase-3d-candidate",
    "scope": "bounded scalar A1++ periodic cosh effective-mass estimator with delete-one-configuration jackknife uncertainty propagation",
    "depends_on": [
        {
            "contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1",
            "required_status": "phase-3c-frozen",
        }
    ],
    "method_origin": {
        "classification": "standard periodic cosh effective-mass estimator with standard delete-one jackknife uncertainty propagation",
        "canonical_fixture_is_physical_ensemble": False,
        "physical_mass_measurement_claimed": False,
    },
    "input_surface": {
        "channel": "scalar A1++ / continuum-target 0++",
        "full_connected_correlator_required": True,
        "jackknife_replicas_required": True,
        "jackknife_replica_origin": "for N configurations, replica i omits configuration i and recomputes the complete connected correlator including vacuum subtraction from the retained configurations",
        "replicas_may_not_reuse_full_sample_vacuum_mean": True,
        "time_reflection_symmetrization_applied": False,
        "fit_window_selected": False,
    },
    "bounds": {
        "minimum_jackknife_replicates": 3,
        "maximum_jackknife_replicates": 64,
        "minimum_time_extent": 4,
        "maximum_time_extent": 64,
        "analysis_lag_rule": "strictly increasing integer tau with 1 <= tau < N_t/2",
        "periodic_euclidean_time": True,
    },
    "estimator": {
        "name": "periodic-nearest-neighbour-cosh",
        "formula": "m_eff(tau)=acosh((C(tau-1)+C(tau+1))/(2*C(tau)))",
        "central_correlator_rule": "C(tau)>0",
        "acosh_domain_rule": "(C(tau-1)+C(tau+1))/(2*C(tau))>=1",
        "point_estimate": "evaluate the non-linear estimator on the full connected correlator",
        "hidden_symmetrization": False,
    },
    "uncertainty": {
        "scheme": "delete-one-configuration jackknife",
        "replica_transform": "evaluate the same cosh effective-mass estimator independently on every recomputed jackknife connected correlator",
        "jackknife_mean": "m_bar=(1/N)*sum_i m_i",
        "standard_error": "sqrt((N-1)/N*sum_i (m_i-m_bar)^2)",
        "nonlinear_transform_recomputed_per_replica": True,
        "correlator_covariance_fit_performed": False,
    },
    "canonical_fixture": {
        "time_extent": 16,
        "full_single_state_mass": 0.5,
        "full_amplitude": 1.0,
        "jackknife_replica_masses": [0.48, 0.49, 0.5, 0.51, 0.52],
        "jackknife_replica_amplitude": 1.0,
        "analysis_lags": [1, 2, 3, 4, 5, 6, 7],
        "correlator_formula": "C(tau)=A*(exp(-m*tau)+exp(-m*(N_t-tau)))",
        "expected_full_effective_mass": 0.5,
        "expected_jackknife_mean": 0.5,
        "expected_standard_error": 0.0282842712474619,
        "analytic_replay_tolerance": 2e-12,
        "fixture_is_physical_ensemble": False,
    },
    "validation": {
        "shape_mismatch_rule": "reject",
        "non_finite_correlator_rule": "reject",
        "non_positive_central_correlator_rule": "reject",
        "acosh_domain_violation_rule": "reject",
        "invalid_analysis_lag_rule": "reject",
        "allocation_failure_rule": "propagate through Result",
    },
    "future_surfaces": {
        "fit_window_selection_defined": False,
        "correlated_spectral_fit_defined": False,
        "variational_gevp_defined": False,
        "tensor_2pp_channel_defined": False,
        "pseudoscalar_0mp_channel_defined": False,
        "finite_volume_spectrum_extracted": False,
    },
    "claim_boundary": {
        "analytic_fixture_is_physical_measurement": False,
        "effective_mass_point_is_fitted_glueball_mass": False,
        "scalar_pipeline_fully_validated": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1",
    "vector_set": "GLUEBALL-EFFECTIVE-MASS-PHASE3D-V1",
    "status": "phase-3d-candidate",
    "vectors": [
        {
            "id": "periodic-single-state-cosh-mass",
            "time_extent": 16,
            "correlator_formula": "C(tau)=A*(exp(-m*tau)+exp(-m*(N_t-tau)))",
            "full_mass": 0.5,
            "amplitude": 1.0,
            "analysis_lags": [1, 2, 3, 4, 5, 6, 7],
            "expected_effective_mass_each_lag": 0.5,
            "absolute_tolerance": 2e-12,
            "claim_class": "C1",
        },
        {
            "id": "delete-one-jackknife-propagation",
            "replica_masses": [0.48, 0.49, 0.5, 0.51, 0.52],
            "replica_count": 5,
            "expected_jackknife_mean_each_lag": 0.5,
            "expected_standard_error_each_lag": 0.0282842712474619,
            "standard_error_formula": "sqrt((N-1)/N*sum_i (m_i-m_bar)^2)",
            "absolute_tolerance": 2e-12,
            "claim_class": "C1",
        },
        {
            "id": "amplitude-cancellation",
            "full_mass": 0.5,
            "full_amplitude": 7.25,
            "replica_amplitude": 3.5,
            "analysis_lag": 3,
            "expected_effective_mass": 0.5,
            "absolute_tolerance": 2e-12,
            "claim_class": "C1",
        },
        {
            "id": "non-positive-center-rejected",
            "mutation": "set C(2)=0",
            "analysis_lag": 2,
            "expected": "NonPositiveCentralCorrelator",
            "claim_class": "C0",
        },
        {
            "id": "acosh-domain-rejected",
            "mutation": "inflate C(2) so neighbor ratio is below one",
            "analysis_lag": 2,
            "expected": "AcoshDomainViolation",
            "claim_class": "C0",
        },
        {
            "id": "jackknife-shape-mismatch-rejected",
            "mutation": "remove one time sample from one replica",
            "expected": "ShapeMismatch",
            "claim_class": "C0",
        },
        {
            "id": "too-few-jackknife-replicas-rejected",
            "replica_count": 2,
            "expected": "TooFewJackknifeReplicates",
            "claim_class": "C0",
        },
        {
            "id": "midpoint-or-unsorted-lags-rejected",
            "time_extent": 16,
            "invalid_lag_sets": [[0], [8], [2, 2], [3, 2]],
            "expected": "InvalidAnalysisLag",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "canonical_fixture_is_physical_ensemble": False,
            "uncertainty_propagation_defined": True,
            "effective_mass_estimator_evaluated": True,
            "fit_window_selected": False,
            "spectral_fit_performed": False,
            "variational_gevp_solved": False,
            "glueball_mass_measured": False,
            "finite_volume_mass_gap_established": False,
            "confinement_established": False,
            "continuum_qft_constructed": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact("contracts/YM-GLUEBALL-EFFECTIVE-MASS-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-effective-mass-phase3d-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-EFFECTIVE-MASS-V1.json"))
    validate_vectors(load("test-vectors/glueball-effective-mass-phase3d-v1.json"))
    print("Phase 3D effective-mass contract checks: passed")


if __name__ == "__main__":
    main()
