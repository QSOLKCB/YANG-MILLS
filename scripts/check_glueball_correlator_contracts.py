#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3C connected-correlator surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1",
    "status": "phase-3c-frozen",
    "scope": "bounded periodic connected Euclidean two-point estimator with vacuum subtraction for the frozen Phase 3A scalar A1++ operator basis",
    "depends_on": [
        {"contract_id": "YM-GLUEBALL-OPERATORS-V1", "required_status": "phase-3a-frozen"},
        {"contract_id": "YM-GLUEBALL-SPATIAL-STOUT-V1", "required_status": "phase-3b1-frozen"},
        {"contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1", "required_status": "phase-3b2-frozen"},
    ],
    "method_origin": {
        "classification": "standard connected two-point definition with a project-defined bounded finite-sample reference estimator",
        "canonical_fixture_is_physical_ensemble": False,
        "production_statistical_claimed": False,
    },
    "input_surface": {
        "operator_basis": "YM-GLUEBALL-SPATIAL-A1PP-V1",
        "basis_elements": ["spatial-plaquette-1x1", "spatial-rectangle-2x1"],
        "cross_operator_matrix_constructed": False,
        "raw_unsmeared_input_allowed": True,
        "stout_smeared_input_allowed": True,
        "stride2_blocked_input_allowed": True,
        "combined_smear_block_tuning_validated": False,
    },
    "bounds": {
        "minimum_configurations": 1,
        "maximum_configurations": 64,
        "minimum_time_extent": 2,
        "maximum_time_extent": 64,
        "identical_lattice_extents_required": True,
        "periodic_euclidean_time": True,
    },
    "estimator": {
        "configuration_weighting": "equal arithmetic weight",
        "time_origin_weighting": "equal arithmetic weight over every periodic Euclidean time origin",
        "vacuum_mean": "O_bar=(1/(N_cfg*N_t))*sum_{c,t} O_c(t)",
        "raw_two_point": "G_O(tau)=(1/(N_cfg*N_t))*sum_{c,t} O_c(t) O_c((t+tau) mod N_t)",
        "connected_two_point": "C_O(tau)=G_O(tau)-O_bar^2",
        "vacuum_subtraction_uses_same_samples": True,
        "time_reflection_symmetrization_applied": False,
        "uncertainty_estimated": False,
    },
    "canonical_fixture": {
        "configuration_count": 4,
        "extents": [3, 3, 3, 4],
        "construction": "configuration c is identity except for one exact traceless-cycle spatial link at [0,0,0,c] in direction 0",
        "plaquette_vacuum_mean": 0.9876543209876543,
        "rectangle_vacuum_mean": 0.9814814814814815,
        "plaquette_raw": [0.9759183051364121, 0.9753086419753086, 0.9753086419753086, 0.9753086419753086],
        "rectangle_raw": [0.9643347050754458, 0.9629629629629629, 0.9629629629629629, 0.9629629629629629],
        "plaquette_connected": [0.0004572473708276177, -0.00015241579027587258, -0.00015241579027587258, -0.00015241579027587258],
        "rectangle_connected": [0.00102880658436214, -0.0003429355281207133, -0.0003429355281207133, -0.0003429355281207133],
        "analytic_replay_tolerance": 2e-12,
    },
    "validation": {
        "identity_input_connected_correlator_is_exact_zero": True,
        "gauge_invariance_residual_max": 5e-11,
        "empty_configuration_set_rule": "reject",
        "mixed_lattice_extents_rule": "reject",
        "non_finite_observable_rule": "reject",
        "allocation_failure_rule": "propagate through Result",
    },
    "future_surfaces": {
        "uncertainty_propagation_defined": False,
        "effective_mass_extracted": False,
        "fit_window_selected": False,
        "variational_gevp_defined": False,
        "glueball_spectrum_extracted": False,
    },
    "claim_boundary": {
        "canonical_fixture_is_physical_measurement": False,
        "connected_correlator_is_glueball_mass": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1",
    "vector_set": "GLUEBALL-CONNECTED-CORRELATOR-PHASE3C-V1",
    "status": "phase-3c-frozen",
    "vectors": [
        {
            "id": "identity-connected-zero",
            "configuration_count": 2,
            "extents": [3, 3, 3, 4],
            "configuration": "all links identity",
            "expected": {
                "plaquette_vacuum_mean": 1.0,
                "rectangle_vacuum_mean": 1.0,
                "plaquette_raw": [1.0, 1.0, 1.0, 1.0],
                "rectangle_raw": [1.0, 1.0, 1.0, 1.0],
                "plaquette_connected": [0.0, 0.0, 0.0, 0.0],
                "rectangle_connected": [0.0, 0.0, 0.0, 0.0],
            },
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "shifted-defect-analytic-correlators",
            "configuration_count": 4,
            "extents": [3, 3, 3, 4],
            "configuration": "configuration c has one traceless-cycle spatial link at [0,0,0,c] in direction 0",
            "expected": {
                "plaquette_vacuum_mean": 0.9876543209876543,
                "rectangle_vacuum_mean": 0.9814814814814815,
                "plaquette_raw": [0.9759183051364121, 0.9753086419753086, 0.9753086419753086, 0.9753086419753086],
                "rectangle_raw": [0.9643347050754458, 0.9629629629629629, 0.9629629629629629, 0.9629629629629629],
                "plaquette_connected": [0.0004572473708276177, -0.00015241579027587258, -0.00015241579027587258, -0.00015241579027587258],
                "rectangle_connected": [0.00102880658436214, -0.0003429355281207133, -0.0003429355281207133, -0.0003429355281207133],
            },
            "absolute_tolerance": 2e-12,
            "claim_class": "C2",
        },
        {
            "id": "deterministic-gauge-invariance",
            "configuration_count": 4,
            "extents": [3, 3, 3, 4],
            "configuration": "shifted-defect analytic fixture",
            "gauge_field": "deterministic_su3_site_gauge applied independently to every configuration",
            "observable": "maximum residual across vacuum means, raw correlators, and connected correlators",
            "expected_max": 5e-11,
            "claim_class": "C2",
        },
        {
            "id": "empty-configuration-set-rejected",
            "configuration_count": 0,
            "expected": "EmptyConfigurationSet",
            "claim_class": "C0",
        },
        {
            "id": "mixed-lattice-extents-rejected",
            "extents": [[3, 3, 3, 4], [4, 4, 4, 4]],
            "expected": "LatticeExtentsMismatch",
            "claim_class": "C0",
        },
        {
            "id": "configuration-limit-rejected",
            "configuration_count": 65,
            "expected": "TooManyConfigurations",
            "claim_class": "C0",
        },
        {
            "id": "time-extent-limit-rejected",
            "configuration_count": 1,
            "extents": [3, 3, 3, 65],
            "expected": "TimeExtentTooLarge",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "canonical_fixture_is_physical_ensemble": False,
            "vacuum_subtraction_defined": True,
            "connected_correlator_constructed": True,
            "uncertainty_estimated": False,
            "effective_mass_extracted": False,
            "fit_window_selected": False,
            "glueball_spectrum_extracted": False,
            "finite_volume_mass_gap_established": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact("contracts/YM-GLUEBALL-CONNECTED-CORRELATOR-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-connected-correlator-phase3c-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-CONNECTED-CORRELATOR-V1.json"))
    validate_vectors(load("test-vectors/glueball-connected-correlator-phase3c-v1.json"))
    print("Phase 3C connected-correlator contract checks: passed")


if __name__ == "__main__":
    main()
