#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3H blocking-capable geometry recovery surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1",
    "status": "phase-3h-candidate",
    "scope": "blocking-capable scalar A1++ geometry/operator-overlap recovery after the preserved Phase 3F and Phase 3G negative observations",
    "depends_on": [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2", "required_status": "phase-2c2-frozen"},
        {"contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1", "required_status": "phase-3b2-frozen"},
        {"contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1", "required_status": "phase-3c-frozen"},
        {"contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1", "required_status": "phase-3d-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-FIT-V1", "required_status": "phase-3e-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1", "required_status": "phase-3f-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-RECOVERY-V1", "required_status": "phase-3g-frozen"},
    ],
    "change_discipline": {
        "predeclared_before_first_phase3h_observation": True,
        "phase3f_and_phase3g_negative_results_must_remain_unchanged": True,
        "fresh_stream_required": True,
        "same_contract_may_not_be_retuned_after_observation": True,
        "scientific_gate_failure_must_be_preserved": True,
        "no_outcome_by_construction": True,
    },
    "recovery_hypothesis": {
        "id": "blocking-geometry-overlap-recovery",
        "statement": "moving from spatial extent 4 to the minimum blocking-capable spatial extent 6 and adding the already-frozen stride-2 blocking surface may improve scalar operator overlap enough for the predeclared effective-mass and stability gates to become admissible",
        "phase3g_extents": [4, 4, 4, 10],
        "phase3h_extents": [6, 6, 6, 10],
        "phase3g_independent_chains": 32,
        "phase3h_independent_chains": 32,
        "geometry_changed": True,
        "blocking_surface_added": True,
        "chain_count_changed": False,
        "fit_semantics_changed": False,
        "thresholds_changed": False,
        "causal_attribution_if_successful": "composite geometry/operator-overlap intervention; a successful result requires fresh independent confirmation before attribution",
    },
    "ensemble": {
        "sampler_id": "YM-SU3-ENSEMBLE-V1",
        "proposal_id": "YM-SU3-GENERATOR-METROPOLIS-V1",
        "rng_id": "SPLITMIX64-V1",
        "stream_id": "YM-SU3-STREAM-V1",
        "extents": [6, 6, 6, 10],
        "beta": 5.7,
        "master_seed_hex": "0x3f02202609150001",
        "phase3g_master_seed_hex": "0x3f01202609150001",
        "independent_chain_count": 32,
        "chain_ids": list(range(32)),
        "cold_start_each_chain": True,
        "thermalization_sweeps": 64,
        "post_thermalization_sweeps": 4,
        "total_sweeps_per_chain": 68,
        "proposal_angle": 0.35,
        "retained_configurations_per_chain": 1,
        "sequential_configurations_pooled_within_chain": False,
        "expected_proposals_per_chain": 4700160,
        "expected_total_proposals": 150405120,
        "minimum_acceptance": 0.1,
        "maximum_acceptance": 0.95,
        "maximum_link_unitarity_residual": 5e-12,
        "maximum_link_determinant_residual": 5e-12,
        "group_residual_violation_is_implementation_failure": True,
    },
    "analysis": {
        "operators": ["plaquette-1x1", "rectangle-2x1"],
        "transformations": [
            {"id": "raw", "source": "fine", "stout_rho": None, "stout_levels": 0},
            {"id": "stout-1", "source": "fine", "stout_rho": 0.1, "stout_levels": 1},
            {"id": "stout-2", "source": "fine", "stout_rho": 0.1, "stout_levels": 2},
            {"id": "block-1", "source": "fine", "blocking_contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1", "block_factor": 2, "coarse_extents": [3, 3, 3, 10]},
        ],
        "candidate_windows": [[1, 3], [2, 4], [1, 4]],
        "primary_surface": {"transformation": "block-1", "operator": "plaquette-1x1"},
        "maximum_pairwise_mass_z": 5.0,
        "correlator_replay_absolute_tolerance": 2e-12,
        "all_successful_fit_pairs_must_be_scorable": True,
        "computational_fit_failure_is_implementation_failure": True,
        "data_admissibility_rejection_is_scientific_evidence": True,
        "blocked_configurations_must_satisfy_frozen_phase3b2_geometry": True,
    },
    "scientific_gate": {
        "sampler_diagnostics_must_pass": True,
        "primary_surface_fit_must_succeed": True,
        "minimum_successful_primary_windows": 2,
        "fit_window_stability_must_pass": True,
        "operator_stability_must_pass_on_primary_transformation": True,
        "minimum_successful_transformations": 2,
        "transformation_stability_must_pass": True,
        "maximum_pairwise_mass_z": 5.0,
        "scientific_gate_failure_causes_ci_failure": False,
        "scientific_gate_failure_must_be_reported": True,
    },
    "decision_boundary": {
        "if_gate_passes": "freeze Phase 3H and run a fresh-stream independent confirmation of the same blocking-capable geometry before opening additional irreducible channels",
        "if_gate_fails": "freeze Phase 3H and predeclare a new scalar authority that changes time extent and/or variational operator basis rather than retuning this contract",
        "tensor_2pp_channel_enabled": False,
        "pseudoscalar_0mp_channel_enabled": False,
    },
    "claim_boundary": {
        "geometry_recovery_is_physical_glueball_mass": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1",
    "vector_set": "GLUEBALL-SCALAR-GEOMETRY-RECOVERY-PHASE3H-V1",
    "status": "phase-3h-candidate",
    "vectors": [
        {
            "id": "phase3g-frozen-negative-baseline",
            "phase3g_contract_id": "YM-GLUEBALL-SCALAR-RECOVERY-V1",
            "phase3g_observation_head": "65dacc9f03cb0fd74826cd211ee7bd586668e08a",
            "phase3g_workflow_run_id": 34875522823,
            "phase3g_independent_chains": 32,
            "phase3g_scalar_pipeline_validated": False,
            "phase3g_all_six_surfaces_effective_mass_rejected": True,
            "claim_class": "C2",
        },
        {
            "id": "blocking-capable-fresh-stream-geometry",
            "extents": [6, 6, 6, 10],
            "beta": 5.7,
            "master_seed_hex": "0x3f02202609150001",
            "phase3g_master_seed_hex": "0x3f01202609150001",
            "chain_count": 32,
            "thermalization_sweeps": 64,
            "post_thermalization_sweeps": 4,
            "proposal_angle": 0.35,
            "retained_configurations_per_chain": 1,
            "expected_proposals_per_chain": 4700160,
            "expected_total_proposals": 150405120,
            "claim_class": "C0",
        },
        {
            "id": "geometry-overlap-analysis-surface",
            "operators": ["plaquette-1x1", "rectangle-2x1"],
            "transformations": ["raw", "stout-1", "stout-2", "block-1"],
            "block_factor": 2,
            "blocked_extents": [3, 3, 3, 10],
            "stout_rho": 0.1,
            "candidate_windows": [[1, 3], [2, 4], [1, 4]],
            "primary_surface": "block-1/plaquette-1x1",
            "maximum_pairwise_mass_z": 5.0,
            "correlator_replay_absolute_tolerance": 2e-12,
            "claim_class": "C0",
        },
        {
            "id": "scientific-integrity-boundary",
            "no_outcome_by_construction": True,
            "same_contract_may_not_be_retuned_after_observation": True,
            "scientific_gate_failure_causes_ci_failure": False,
            "tensor_2pp_channel_enabled": False,
            "pseudoscalar_0mp_channel_enabled": False,
            "physical_glueball_mass_claimed": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact("contracts/YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-scalar-geometry-recovery-phase3h-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1.json"))
    validate_vectors(load("test-vectors/glueball-scalar-geometry-recovery-phase3h-v1.json"))
    print("Phase 3H scalar geometry recovery contract checks: passed")


if __name__ == "__main__":
    main()
