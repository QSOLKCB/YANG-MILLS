#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3F production scalar surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1",
    "status": "phase-3f-candidate",
    "scope": "bounded finite-lattice scalar A1++ production ensemble evidence with predeclared fit-window, operator, and stout-transformation stability scans",
    "depends_on": [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2", "required_status": "phase-2c2-frozen"},
        {"contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1", "required_status": "phase-3c-frozen"},
        {"contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1", "required_status": "phase-3d-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-FIT-V1", "required_status": "phase-3e-frozen"},
    ],
    "change_discipline": {
        "predeclared_before_production_run": True,
        "failed_scientific_gate_must_be_preserved": True,
        "thresholds_may_not_be_weakened_after_observation": True,
        "fresh_stream_or_semantic_change_requires_new_contract_identifier": True,
    },
    "evidence_class": {
        "classification": "finite-lattice production numerical observation",
        "production_monte_carlo_configurations": True,
        "physical_glueball_mass_claimed": False,
        "continuum_inference_claimed": False,
    },
    "ensemble": {
        "sampler_id": "YM-SU3-ENSEMBLE-V1",
        "proposal_id": "YM-SU3-GENERATOR-METROPOLIS-V1",
        "rng_id": "SPLITMIX64-V1",
        "stream_id": "YM-SU3-STREAM-V1",
        "extents": [4, 4, 4, 10],
        "beta": 5.7,
        "master_seed_hex": "0x3f00202609150001",
        "independent_chain_count": 16,
        "chain_ids": list(range(16)),
        "cold_start_each_chain": True,
        "thermalization_sweeps": 64,
        "post_thermalization_sweeps": 4,
        "total_sweeps_per_chain": 68,
        "proposal_angle": 0.35,
        "retained_configurations_per_chain": 1,
        "sequential_configurations_pooled_within_chain": False,
        "expected_proposals_per_chain": 1392640,
        "minimum_acceptance": 0.1,
        "maximum_acceptance": 0.95,
        "maximum_link_unitarity_residual": 5e-12,
        "maximum_link_determinant_residual": 5e-12,
    },
    "operator_and_transform_scan": {
        "operators": ["plaquette-1x1", "rectangle-2x1"],
        "transformations": [
            {"id": "raw", "stout_rho": None, "stout_levels": 0},
            {"id": "stout-1", "stout_rho": 0.1, "stout_levels": 1},
            {"id": "stout-2", "stout_rho": 0.1, "stout_levels": 2},
        ],
        "blocking_scan_defined": False,
        "blocking_exclusion_reason": "the predeclared 4^3 spatial production geometry is below the frozen stride-2 blocking minimum fine extent of 6",
    },
    "correlator": {
        "authority": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1",
        "full_sample_vacuum_subtraction_recomputed": True,
        "delete_one_configuration_vacuum_subtraction_recomputed_per_replica": True,
        "independent_chain_configuration_is_jackknife_unit": True,
        "full_sample_replay_against_phase3c_absolute_tolerance": 2e-12,
    },
    "fit": {
        "authority": "YM-GLUEBALL-SCALAR-FIT-V1",
        "candidate_windows": [[1, 3], [2, 4], [1, 4]],
        "candidates_declared_before_observation": True,
        "primary_surface": {"transformation": "stout-2", "operator": "plaquette-1x1"},
        "surface_fit_rejection_is_recorded_evidence": True,
        "surface_fit_rejection_is_not_implementation_failure": True,
    },
    "stability": {
        "maximum_pairwise_mass_z": 5.0,
        "pairwise_scale": "sqrt(sigma_a^2+sigma_b^2)",
        "fit_window_scan": {
            "primary_surface_only": True,
            "minimum_successful_windows": 2,
            "maximum_pairwise_z": 5.0,
        },
        "operator_scan": {
            "transformation": "stout-2",
            "operators": ["plaquette-1x1", "rectangle-2x1"],
            "both_fits_required_for_validation": True,
            "maximum_pairwise_z": 5.0,
        },
        "transformation_scan": {
            "operator": "plaquette-1x1",
            "transformations": ["raw", "stout-1", "stout-2"],
            "minimum_successful_transformations": 2,
            "maximum_pairwise_z": 5.0,
        },
    },
    "scientific_gate": {
        "sampler_diagnostics_must_pass": True,
        "primary_surface_fit_must_succeed": True,
        "fit_window_stability_must_pass": True,
        "operator_stability_must_pass": True,
        "transformation_stability_must_pass": True,
        "scientific_gate_failure_causes_ci_failure": False,
        "scientific_gate_failure_must_be_reported": True,
    },
    "future_surfaces": {
        "blocked_operator_stability_scan_defined": False,
        "larger_volume_scalar_confirmation_defined": False,
        "tensor_2pp_channel_defined": False,
        "pseudoscalar_0mp_channel_defined": False,
        "finite_volume_spectrum_extracted": False,
    },
    "claim_boundary": {
        "production_numerical_observation_is_physical_glueball_mass": False,
        "scalar_pipeline_validation_is_continuum_mass_gap": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1",
    "vector_set": "GLUEBALL-SCALAR-PRODUCTION-PHASE3F-V1",
    "status": "phase-3f-candidate",
    "vectors": [
        {
            "id": "predeclared-independent-chain-production-schedule",
            "extents": [4, 4, 4, 10],
            "beta": 5.7,
            "master_seed_hex": "0x3f00202609150001",
            "chain_count": 16,
            "thermalization_sweeps": 64,
            "post_thermalization_sweeps": 4,
            "proposal_angle": 0.35,
            "retained_configurations_per_chain": 1,
            "expected_proposals_per_chain": 1392640,
            "claim_class": "C0",
        },
        {
            "id": "predeclared-fit-windows",
            "candidate_windows": [[1, 3], [2, 4], [1, 4]],
            "primary_surface": "stout-2/plaquette-1x1",
            "maximum_pairwise_mass_z": 5.0,
            "claim_class": "C0",
        },
        {
            "id": "predeclared-operator-transformation-scan",
            "operators": ["plaquette-1x1", "rectangle-2x1"],
            "transformations": ["raw", "stout-1", "stout-2"],
            "stout_rho": 0.1,
            "blocked_scan_defined": False,
            "claim_class": "C0",
        },
        {
            "id": "scientific-gate-boundary",
            "production_monte_carlo_configurations": True,
            "scientific_gate_failure_causes_ci_failure": False,
            "failed_scientific_gate_must_be_preserved": True,
            "physical_glueball_mass_claimed": False,
            "finite_volume_mass_gap_established": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact("contracts/YM-GLUEBALL-SCALAR-PRODUCTION-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-scalar-production-phase3f-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SCALAR-PRODUCTION-V1.json"))
    validate_vectors(load("test-vectors/glueball-scalar-production-phase3f-v1.json"))
    print("Phase 3F production scalar contract checks: passed")


if __name__ == "__main__":
    main()
