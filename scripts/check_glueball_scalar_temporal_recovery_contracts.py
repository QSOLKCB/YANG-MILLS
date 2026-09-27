#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3I temporal-extent recovery surface."""

from __future__ import annotations

import json
from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = json.loads(r'''{
  "contract_id": "YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1",
  "status": "phase-3i-candidate",
  "scope": "scalar A1++ temporal-extent recovery after the preserved Phase 3H blocking-capable negative observation",
  "depends_on": [
    {
      "contract_id": "YM-SU3-ENSEMBLE-V1",
      "required_status": "phase-2c1-frozen"
    },
    {
      "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2",
      "required_status": "phase-2c2-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1",
      "required_status": "phase-3b2-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-CONNECTED-CORRELATOR-V1",
      "required_status": "phase-3c-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-EFFECTIVE-MASS-V1",
      "required_status": "phase-3d-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-SCALAR-FIT-V1",
      "required_status": "phase-3e-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-SCALAR-PRODUCTION-V1",
      "required_status": "phase-3f-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-SCALAR-RECOVERY-V1",
      "required_status": "phase-3g-frozen"
    },
    {
      "contract_id": "YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1",
      "required_status": "phase-3h-frozen"
    }
  ],
  "change_discipline": {
    "predeclared_before_first_phase3i_observation": true,
    "phase3h_negative_result_must_remain_unchanged": true,
    "fresh_stream_required": true,
    "only_temporal_extent_changes": true,
    "same_contract_may_not_be_retuned_after_observation": true,
    "scientific_gate_failure_must_be_preserved": true,
    "no_outcome_by_construction": true
  },
  "recovery_hypothesis": {
    "id": "temporal-extent-wraparound-recovery",
    "statement": "increasing only the fine and blocked temporal extent from 10 to 16 may reduce finite-periodic wraparound contamination and provide a longer Euclidean-time domain while preserving the Phase 3H spatial geometry, operator/transformation surface, chain count, fit windows, fit mechanics, and stability thresholds",
    "phase3h_extents": [
      6,
      6,
      6,
      10
    ],
    "phase3i_extents": [
      6,
      6,
      6,
      16
    ],
    "phase3h_independent_chains": 32,
    "phase3i_independent_chains": 32,
    "time_extent_changed": true,
    "spatial_geometry_changed": false,
    "blocking_surface_changed": false,
    "operator_basis_changed": false,
    "chain_count_changed": false,
    "fit_semantics_changed": false,
    "thresholds_changed": false,
    "causal_attribution_if_successful": "temporal-extent intervention only; a successful result still requires fresh independent confirmation before channel expansion"
  },
  "ensemble": {
    "sampler_id": "YM-SU3-ENSEMBLE-V1",
    "proposal_id": "YM-SU3-GENERATOR-METROPOLIS-V1",
    "rng_id": "SPLITMIX64-V1",
    "stream_id": "YM-SU3-STREAM-V1",
    "extents": [
      6,
      6,
      6,
      16
    ],
    "beta": 5.7,
    "master_seed_hex": "0x3f03202609270001",
    "phase3h_master_seed_hex": "0x3f02202609150001",
    "independent_chain_count": 32,
    "chain_ids": [
      0,
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
      9,
      10,
      11,
      12,
      13,
      14,
      15,
      16,
      17,
      18,
      19,
      20,
      21,
      22,
      23,
      24,
      25,
      26,
      27,
      28,
      29,
      30,
      31
    ],
    "cold_start_each_chain": true,
    "thermalization_sweeps": 64,
    "post_thermalization_sweeps": 4,
    "total_sweeps_per_chain": 68,
    "proposal_angle": 0.35,
    "retained_configurations_per_chain": 1,
    "sequential_configurations_pooled_within_chain": false,
    "expected_proposals_per_chain": 7520256,
    "expected_total_proposals": 240648192,
    "minimum_acceptance": 0.1,
    "maximum_acceptance": 0.95,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
    "group_residual_violation_is_implementation_failure": true
  },
  "analysis": {
    "operators": [
      "plaquette-1x1",
      "rectangle-2x1"
    ],
    "transformations": [
      {
        "id": "raw",
        "source": "fine",
        "stout_rho": null,
        "stout_levels": 0
      },
      {
        "id": "stout-1",
        "source": "fine",
        "stout_rho": 0.1,
        "stout_levels": 1
      },
      {
        "id": "stout-2",
        "source": "fine",
        "stout_rho": 0.1,
        "stout_levels": 2
      },
      {
        "id": "block-1",
        "source": "fine",
        "blocking_contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1",
        "block_factor": 2,
        "coarse_extents": [
          3,
          3,
          3,
          16
        ]
      }
    ],
    "candidate_windows": [
      [
        1,
        3
      ],
      [
        2,
        4
      ],
      [
        1,
        4
      ]
    ],
    "primary_surface": {
      "transformation": "block-1",
      "operator": "plaquette-1x1"
    },
    "maximum_pairwise_mass_z": 5,
    "correlator_replay_absolute_tolerance": 2e-12,
    "all_successful_fit_pairs_must_be_scorable": true,
    "computational_fit_failure_is_implementation_failure": true,
    "data_admissibility_rejection_is_scientific_evidence": true,
    "blocked_configurations_must_satisfy_frozen_phase3b2_geometry": true
  },
  "scientific_gate": {
    "sampler_diagnostics_must_pass": true,
    "primary_surface_fit_must_succeed": true,
    "minimum_successful_primary_windows": 2,
    "fit_window_stability_must_pass": true,
    "operator_stability_must_pass_on_primary_transformation": true,
    "minimum_successful_transformations": 2,
    "transformation_stability_must_pass": true,
    "maximum_pairwise_mass_z": 5,
    "scientific_gate_failure_causes_ci_failure": false,
    "scientific_gate_failure_must_be_reported": true
  },
  "decision_boundary": {
    "if_gate_passes": "freeze Phase 3I and run a fresh-stream independent confirmation of the same 6 x 6 x 6 x 16 blocking-capable surface before opening additional irreducible channels",
    "if_gate_fails": "freeze Phase 3I and predeclare a variational scalar operator-basis authority; do not retune thresholds, windows, chain count, or this temporal contract",
    "tensor_2pp_channel_enabled": false,
    "pseudoscalar_0mp_channel_enabled": false
  },
  "claim_boundary": {
    "temporal_recovery_is_physical_glueball_mass": false,
    "finite_volume_mass_gap_established": false,
    "confinement_established": false,
    "continuum_qft_constructed": false,
    "positive_continuum_mass_gap_proved": false,
    "clay_solution_claimed": false
  }
}''')
EXPECTED_VECTORS: dict[str, Any] = json.loads(r'''{
  "contract_id": "YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1",
  "vector_set": "GLUEBALL-SCALAR-TEMPORAL-RECOVERY-PHASE3I-V1",
  "status": "phase-3i-candidate",
  "vectors": [
    {
      "id": "phase3h-frozen-negative-baseline",
      "phase3h_contract_id": "YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1",
      "phase3h_observation_head": "c241991e58860795d247f70d7f92b9e984c96a64",
      "phase3h_workflow_run_id": 34901768744,
      "phase3h_independent_chains": 32,
      "phase3h_scalar_pipeline_validated": false,
      "phase3h_all_eight_surfaces_effective_mass_rejected": true,
      "claim_class": "C2"
    },
    {
      "id": "fresh-stream-temporal-extent",
      "extents": [
        6,
        6,
        6,
        16
      ],
      "phase3h_extents": [
        6,
        6,
        6,
        10
      ],
      "beta": 5.7,
      "master_seed_hex": "0x3f03202609270001",
      "phase3h_master_seed_hex": "0x3f02202609150001",
      "chain_count": 32,
      "thermalization_sweeps": 64,
      "post_thermalization_sweeps": 4,
      "proposal_angle": 0.35,
      "retained_configurations_per_chain": 1,
      "expected_proposals_per_chain": 7520256,
      "expected_total_proposals": 240648192,
      "claim_class": "C0"
    },
    {
      "id": "unchanged-analysis-surface",
      "operators": [
        "plaquette-1x1",
        "rectangle-2x1"
      ],
      "transformations": [
        "raw",
        "stout-1",
        "stout-2",
        "block-1"
      ],
      "block_factor": 2,
      "blocked_extents": [
        3,
        3,
        3,
        16
      ],
      "stout_rho": 0.1,
      "candidate_windows": [
        [
          1,
          3
        ],
        [
          2,
          4
        ],
        [
          1,
          4
        ]
      ],
      "primary_surface": "block-1/plaquette-1x1",
      "maximum_pairwise_mass_z": 5,
      "correlator_replay_absolute_tolerance": 2e-12,
      "claim_class": "C0"
    },
    {
      "id": "scientific-integrity-boundary",
      "only_temporal_extent_changes": true,
      "no_outcome_by_construction": true,
      "same_contract_may_not_be_retuned_after_observation": true,
      "scientific_gate_failure_causes_ci_failure": false,
      "tensor_2pp_channel_enabled": false,
      "pseudoscalar_0mp_channel_enabled": false,
      "physical_glueball_mass_claimed": false,
      "positive_continuum_mass_gap_proved": false,
      "clay_solution_claimed": false,
      "claim_class": "C0"
    }
  ]
}''')


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact(
        "contracts/YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1.json",
        payload,
        EXPECTED_CONTRACT,
    )


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-scalar-temporal-recovery-phase3i-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1.json"))
    validate_vectors(load("test-vectors/glueball-scalar-temporal-recovery-phase3i-v1.json"))
    print("Phase 3I scalar temporal recovery contract checks: passed")


if __name__ == "__main__":
    main()
