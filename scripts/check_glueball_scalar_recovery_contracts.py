#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3G fresh-stream scalar recovery surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-RECOVERY-V1",
    "status": "phase-3g-candidate",
    "scope": "fresh-stream scalar A1++ recovery confirmation testing increased independent statistics while preserving the frozen Phase 3F geometry and analysis semantics",
    "depends_on": [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2", "required_status": "phase-2c2-frozen"},
        {"contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1", "required_status": "phase-3c-frozen"},
        {"contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1", "required_status": "phase-3d-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-FIT-V1", "required_status": "phase-3e-frozen"},
        {"contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1", "required_status": "phase-3f-frozen"},
    ],
    "change_discipline": {
        "predeclared_before_first_phase3g_observation": True,
        "phase3f_negative_result_must_remain_unchanged": True,
        "fresh_stream_required": True,
        "same_contract_may_not_be_retuned_after_observation": True,
        "scientific_gate_failure_must_be_preserved": True,
    },
    "recovery_hypothesis": {
        "id": "independent-statistics-recovery",
        "statement": "doubling independently seeded retained configurations from 16 to 32 at fixed geometry and frozen analysis semantics may reduce scalar-correlator noise enough for the predeclared effective-mass and stability gates to become admissible",
        "phase3f_independent_chains": 16,
        "phase3g_independent_chains": 32,
        "chain_count_ratio": 2.0,
        "geometry_changed": False,
        "fit_semantics_changed": False,
        "thresholds_changed": False,
    },
    "ensemble": {
        "sampler_id": "YM-SU3-ENSEMBLE-V1",
        "proposal_id": "YM-SU3-GENERATOR-METROPOLIS-V1",
        "rng_id": "SPLITMIX64-V1",
        "stream_id": "YM-SU3-STREAM-V1",
        "extents": [4, 4, 4, 10],
        "beta": 5.7,
        "master_seed_hex": "0x3f01202609150001",
        "phase3f_master_seed_hex": "0x3f00202609150001",
        "independent_chain_count": 32,
        "chain_ids": list(range(32)),
        "cold_start_each_chain": True,
        "thermalization_sweeps": 64,
        "post_thermalization_sweeps": 4,
        "total_sweeps_per_chain": 68,
        "proposal_angle": 0.35,
        "retained_configurations_per_chain": 1,
        "sequential_configurations_pooled_within_chain": False,
        "expected_proposals_per_chain": 1392640,
        "expected_total_proposals": 44564480,
        "minimum_acceptance": 0.1,
        "maximum_acceptance": 0.95,
        "maximum_link_unitarity_residual": 5e-12,
        "maximum_link_determinant_residual": 5e-12,
        "group_residual_violation_is_implementation_failure": True,
    },
    "analysis_inheritance": {
        "source_authority": "YM-GLUEBALL-SCALAR-PRODUCTION-V1",
        "operators": ["plaquette-1x1", "rectangle-2x1"],
        "transformations": [
            {"id": "raw", "stout_rho": None, "stout_levels": 0},
            {"id": "stout-1", "stout_rho": 0.1, "stout_levels": 1},
            {"id": "stout-2", "stout_rho": 0.1, "stout_levels": 2},
        ],
        "blocking_scan_defined": False,
        "blocking_exclusion_reason": "Phase 3G intentionally holds the Phase 3F 4^3 spatial geometry fixed to isolate independent-statistics recovery; frozen Phase 3B2 blocking still requires fine spatial extent at least 6",
        "candidate_windows": [[1, 3], [2, 4], [1, 4]],
        "primary_surface": {"transformation": "stout-2", "operator": "plaquette-1x1"},
        "maximum_pairwise_mass_z": 5.0,
        "correlator_replay_absolute_tolerance": 2e-12,
        "all_successful_fit_pairs_must_be_scorable": True,
        "computational_fit_failure_is_implementation_failure": True,
        "data_admissibility_rejection_is_scientific_evidence": True,
    },
    "scientific_gate": {
        "sampler_diagnostics_must_pass": True,
        "primary_surface_fit_must_succeed": True,
        "minimum_successful_primary_windows": 2,
        "fit_window_stability_must_pass": True,
        "operator_stability_must_pass": True,
        "minimum_successful_transformations": 2,
        "transformation_stability_must_pass": True,
        "maximum_pairwise_mass_z": 5.0,
        "scientific_gate_failure_causes_ci_failure": False,
        "scientific_gate_failure_must_be_reported": True,
    },
    "decision_boundary": {
        "if_gate_passes": "freeze Phase 3G and predeclare a larger-spatial-volume scalar confirmation whose geometry admits the frozen blocking authority before opening additional irreducible channels",
        "if_gate_fails": "freeze Phase 3G and predeclare a new geometry/operator-overlap recovery experiment rather than increasing statistics again under this identifier",
        "tensor_2pp_channel_enabled": False,
        "pseudoscalar_0mp_channel_enabled": False,
    },
    "claim_boundary": {
        "recovery_confirmation_is_physical_glueball_mass": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SCALAR-RECOVERY-V1",
    "vector_set": "GLUEBALL-SCALAR-RECOVERY-PHASE3G-V1",
    "status": "phase-3g-candidate",
    "vectors": [
        {
            "id": "phase3f-frozen-negative-baseline",
            "phase3f_contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1",
            "phase3f_observation_head": "88b7f450701884e6351878e7890410e4095e3d6d",
            "phase3f_workflow_run_id": 34862755015,
            "phase3f_independent_chains": 16,
            "phase3f_scalar_pipeline_validated": False,
            "phase3f_all_six_surfaces_effective_mass_rejected": True,
            "claim_class": "C2",
        },
        {
            "id": "fresh-stream-doubled-independent-statistics",
            "extents": [4, 4, 4, 10],
            "beta": 5.7,
            "master_seed_hex": "0x3f01202609150001",
            "phase3f_master_seed_hex": "0x3f00202609150001",
            "chain_count": 32,
            "thermalization_sweeps": 64,
            "post_thermalization_sweeps": 4,
            "proposal_angle": 0.35,
            "retained_configurations_per_chain": 1,
            "expected_proposals_per_chain": 1392640,
            "expected_total_proposals": 44564480,
            "claim_class": "C0",
        },
        {
            "id": "analysis-semantics-held-fixed",
            "operators": ["plaquette-1x1", "rectangle-2x1"],
            "transformations": ["raw", "stout-1", "stout-2"],
            "stout_rho": 0.1,
            "candidate_windows": [[1, 3], [2, 4], [1, 4]],
            "primary_surface": "stout-2/plaquette-1x1",
            "maximum_pairwise_mass_z": 5.0,
            "correlator_replay_absolute_tolerance": 2e-12,
            "blocked_scan_defined": False,
            "claim_class": "C0",
        },
        {
            "id": "recovery-decision-boundary",
            "scientific_gate_failure_causes_ci_failure": False,
            "phase3f_result_may_not_be_rewritten": True,
            "if_phase3g_passes_next_step": "larger-spatial-volume-blocking-capable-scalar-confirmation",
            "if_phase3g_fails_next_step": "new-geometry-or-operator-overlap-recovery-contract",
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
    validate_exact("contracts/YM-GLUEBALL-SCALAR-RECOVERY-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-scalar-recovery-phase3g-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SCALAR-RECOVERY-V1.json"))
    validate_vectors(load("test-vectors/glueball-scalar-recovery-phase3g-v1.json"))
    print("Phase 3G scalar recovery contract checks: passed")


if __name__ == "__main__":
    main()
