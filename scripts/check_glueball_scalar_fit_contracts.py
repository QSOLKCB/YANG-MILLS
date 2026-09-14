#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3E scalar correlated-fit surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-FIT-V1",
    "status": "phase-3e-frozen",
    "change_classification": {
        "classification": "implementation-only",
        "reason": "PR #16 merged the reviewed Phase 3E authority; this lifecycle transition changes no window-selection, covariance, spectral-fit, uncertainty, fixture, or claim semantics",
    },
    "scope": "bounded scalar A1++ predeclared plateau-window selection and correlated periodic one-state fit with delete-one jackknife parameter uncertainty",
    "depends_on": [
        {"contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1", "required_status": "phase-3d-frozen"},
        {"contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1", "required_status": "phase-3c-frozen"},
    ],
    "method_origin": {
        "classification": "standard jackknife covariance and periodic one-state model with project-defined deterministic bounded window-selection and profile-grid reference mechanics",
        "canonical_fixture_is_physical_ensemble": False,
        "production_scalar_analysis_claimed": False,
        "physical_glueball_mass_claimed": False,
    },
    "input_surface": {
        "channel": "scalar A1++ / continuum-target 0++",
        "full_connected_correlator_required": True,
        "delete_one_jackknife_correlators_required": True,
        "candidate_windows_must_be_supplied_before_fit": True,
        "automatic_data_dependent_candidate_generation": False,
        "time_reflection_symmetrization_applied": False,
    },
    "bounds": {
        "minimum_window_points": 3,
        "maximum_window_points": 8,
        "maximum_candidate_windows": 16,
        "window_rule": "contiguous integer tau with start>=1 and end<N_t/2",
        "covariance_rank_rule": "jackknife replica count must exceed selected window point count",
        "mass_grid_min": 0.05,
        "mass_grid_max": 2.0,
        "mass_grid_step": 0.001,
        "mass_grid_points": 1951,
    },
    "plateau_selection": {
        "effective_mass_authority": "YM-GLUEBALL-EFFECTIVE-MASS-V1",
        "weighted_mean": "sum_t m_eff(t)/sigma(t)^2 divided by sum_t 1/sigma(t)^2",
        "criterion": "max_t abs(m_eff(t)-m_bar)/sigma(t) <= 2.5",
        "maximum_standardized_deviation": 2.5,
        "tie_break_rule": "largest point count first; if tied choose later start tau",
        "positive_finite_jackknife_error_required": True,
    },
    "covariance": {
        "scheme": "delete-one-configuration jackknife",
        "formula": "Cov_ab=(N-1)/N*sum_i (C_i(a)-C_bar(a))(C_i(b)-C_bar(b))",
        "complete_matrix_used": True,
        "diagonal_only_fallback": False,
        "positive_definite_required": True,
        "relative_pivot_tolerance": 1e-12,
        "same_covariance_used_for_full_and_replica_parameter_fits": True,
    },
    "spectral_fit": {
        "model": "C(tau)=A*(exp(-m*tau)+exp(-m*(N_t-tau)))",
        "fit_class": "correlated periodic one-state profile-grid fit",
        "profiled_amplitude": "A=(f^T Cov^-1 C)/(f^T Cov^-1 f)",
        "mass_search": "evaluate every frozen mass-grid point and select minimum chi-square; exact ties choose lower mass",
        "chi_square": "(C-Af)^T Cov^-1 (C-Af)",
        "degrees_of_freedom": "N_window-2",
        "positive_amplitude_required": True,
    },
    "parameter_uncertainty": {
        "scheme": "delete-one-configuration jackknife refit",
        "replica_fit_rule": "refit every jackknife correlator on the selected full-sample window using the same full jackknife covariance authority",
        "standard_error": "sqrt((N-1)/N*sum_i (theta_i-theta_bar)^2)",
        "mass_uncertainty_reported": True,
        "amplitude_uncertainty_reported": True,
    },
    "canonical_fixture": {
        "time_extent": 16,
        "full_mass": 0.5,
        "full_amplitude": 1.0,
        "jackknife_replicates": 8,
        "perturbed_time_range": [1, 7],
        "relative_perturbation": 0.005,
        "shared_correlation_component": 0.25,
        "candidate_windows": [[1, 3], [2, 5], [3, 6]],
        "expected_selected_window": [3, 6],
        "expected_plateau_weighted_mass": 0.5,
        "expected_fitted_mass": 0.5,
        "expected_fitted_amplitude": 1.0,
        "expected_chi_square": 0.0,
        "expected_degrees_of_freedom": 2,
        "expected_mass_jackknife_mean": 0.5,
        "expected_mass_standard_error": 0.005916079783099616,
        "expected_maximum_off_diagonal_correlation": 0.058823529411764705,
        "amplitude_standard_error_must_be_positive": True,
        "analytic_replay_tolerance": 2e-10,
        "fixture_is_physical_ensemble": False,
    },
    "validation": {
        "empty_candidate_windows_rule": "reject",
        "duplicate_candidate_windows_rule": "reject",
        "invalid_or_midpoint_window_rule": "reject",
        "degenerate_plateau_uncertainty_rule": "candidate does not pass",
        "no_accepted_plateau_rule": "reject",
        "insufficient_covariance_rank_rule": "reject",
        "non_positive_definite_covariance_rule": "reject",
        "non_finite_statistic_rule": "reject",
        "allocation_failure_rule": "propagate through Result",
    },
    "future_surfaces": {
        "production_scalar_ensemble_analysis_defined": False,
        "fit_window_sensitivity_scan_defined": False,
        "operator_basis_stability_scan_defined": False,
        "variational_gevp_defined": False,
        "tensor_2pp_channel_defined": False,
        "pseudoscalar_0mp_channel_defined": False,
        "finite_volume_spectrum_extracted": False,
    },
    "claim_boundary": {
        "canonical_fixture_is_physical_measurement": False,
        "correlated_fit_parameter_is_physical_glueball_mass": False,
        "scalar_pipeline_fully_validated": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-FIT-V1",
    "vector_set": "GLUEBALL-SCALAR-FIT-PHASE3E-V1",
    "status": "phase-3e-frozen",
    "vectors": [
        {
            "id": "canonical-correlated-single-state-fit",
            "time_extent": 16,
            "full_mass": 0.5,
            "full_amplitude": 1.0,
            "jackknife_replicates": 8,
            "candidate_windows": [[1, 3], [2, 5], [3, 6]],
            "expected_selected_window": [3, 6],
            "expected_plateau_weighted_effective_mass": 0.5,
            "expected_fitted_mass": 0.5,
            "expected_fitted_amplitude": 1.0,
            "expected_chi_square": 0.0,
            "expected_degrees_of_freedom": 2,
            "absolute_tolerance": 2e-10,
            "claim_class": "C1",
        },
        {
            "id": "correlated-covariance-not-diagonal",
            "selected_window": [3, 6],
            "expected_maximum_off_diagonal_correlation": 0.058823529411764705,
            "diagonal_only_fallback": False,
            "absolute_tolerance": 2e-10,
            "claim_class": "C1",
        },
        {
            "id": "jackknife-parameter-refit",
            "replica_mass_grid_results": [0.5, 0.498, 0.5, 0.504, 0.503, 0.499, 0.497, 0.499],
            "expected_mass_jackknife_mean": 0.5,
            "expected_mass_standard_error": 0.005916079783099616,
            "amplitude_standard_error_must_be_positive": True,
            "standard_error_formula": "sqrt((N-1)/N*sum_i (theta_i-theta_bar)^2)",
            "absolute_tolerance": 2e-10,
            "claim_class": "C1",
        },
        {
            "id": "window-tie-prefers-later-start",
            "passing_equal_length_windows": [[2, 5], [3, 6]],
            "expected_selected_window": [3, 6],
            "claim_class": "C1",
        },
        {
            "id": "duplicate-window-rejected",
            "candidate_windows": [[2, 5], [2, 5]],
            "expected": "DuplicateCandidateWindow",
            "claim_class": "C0",
        },
        {
            "id": "midpoint-window-rejected",
            "time_extent": 16,
            "candidate_window": [3, 8],
            "expected": "InvalidCandidateWindow",
            "claim_class": "C0",
        },
        {
            "id": "insufficient-covariance-rank-rejected",
            "selected_window_points": 4,
            "jackknife_replicates": 4,
            "expected": "InsufficientReplicatesForCovariance",
            "claim_class": "C0",
        },
        {
            "id": "singular-covariance-rejected",
            "configuration": "eight identical jackknife correlators",
            "expected": "CovarianceNotPositiveDefinite",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "canonical_fixture_is_physical_ensemble": False,
            "fit_window_selected": True,
            "full_jackknife_covariance_used": True,
            "correlated_single_state_fit_performed": True,
            "production_ensemble_analyzed": False,
            "physical_glueball_mass_reported": False,
            "scalar_pipeline_fully_validated": False,
            "variational_gevp_solved": False,
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
    validate_exact("contracts/YM-GLUEBALL-SCALAR-FIT-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-scalar-fit-phase3e-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SCALAR-FIT-V1.json"))
    validate_vectors(load("test-vectors/glueball-scalar-fit-phase3e-v1.json"))
    print("Phase 3E scalar correlated-fit contract checks: passed")


if __name__ == "__main__":
    main()
