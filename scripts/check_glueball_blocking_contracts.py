#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3B2 spatial blocking surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1",
    "status": "phase-3b2-frozen",
    "scope": "deterministic stride-2 spatial coarse-link blocking for finite periodic SU(3) lattices compatible with the frozen Phase 3A scalar operator surface",
    "depends_on": [
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
        {"contract_id": "YM-GLUEBALL-OPERATORS-V1", "required_status": "phase-3a-frozen"},
    ],
    "method_origin": {
        "classification": "project-defined correctness-first reference coarse-link map",
        "published_blocking_scheme_claimed": False,
        "improved_action_or_variational_optimality_claimed": False,
    },
    "geometry": {
        "euclidean_time_direction": 3,
        "spatial_directions": [0, 1, 2],
        "block_factor": 2,
        "minimum_fine_spatial_extent": 6,
        "fine_spatial_extents_equal": True,
        "fine_spatial_extents_even": True,
        "periodic_boundary_conditions": True,
        "coarse_extent_rule": "[Lx/2, Ly/2, Lz/2, Nt]",
        "temporal_extent_unchanged": True,
        "phase3a_compatibility_rule": "coarse spatial extent is at least 3",
    },
    "algorithm": {
        "coarse_site_embedding": "y -> x=(2*y0,2*y1,2*y2,y3)",
        "spatial_link_rule": "V_mu(y)=U_mu(x) U_mu(x+mu) for mu in {0,1,2}",
        "temporal_link_rule": "V_3(y)=U_3(x) sampled exactly at the embedded fine site",
        "update_semantics": "all coarse links are computed only from the immutable fine lattice",
        "projection_step_used": False,
        "smearing_step_used": False,
        "averaging_step_used": False,
        "allocation_failure_rule": "propagate through Su3Lattice4::new_identity Result",
    },
    "gauge_covariance": {
        "fine_gauge_rule": "U_mu(x)^G=G(x)U_mu(x)G(x+mu)^dagger",
        "coarse_gauge_restriction": "G_c(y)=G_f(2*y0,2*y1,2*y2,y3)",
        "expected_blocking_relation": "block(U^G)=block(U)^{G_c}",
    },
    "validation": {
        "identity_configuration_blocks_exactly_to_identity": True,
        "identity_tolerance": 0.0,
        "temporal_sampling_tolerance": 0.0,
        "gauge_covariance_residual_max": 5e-11,
        "su3_unitarity_residual_max": 5e-12,
        "su3_determinant_residual_max": 5e-12,
        "frozen_output_abs_tolerance": 1e-12,
        "frozen_coarse_plaquette_1x1": 0.9506172839506173,
        "frozen_coarse_rectangle_2x1": 0.9259259259259259,
    },
    "composition_boundary": {
        "stout_smearing_required": False,
        "stout_smearing_may_precede_blocking_under_future_analysis_contract": True,
        "combined_smear_block_tuning_validated": False,
    },
    "future_surfaces": {
        "connected_correlator_constructed": False,
        "vacuum_subtraction_defined": False,
        "effective_mass_extracted": False,
        "fit_window_selected": False,
        "glueball_spectrum_extracted": False,
    },
    "claim_boundary": {
        "blocking_improves_ground_state_overlap_established": False,
        "blocking_is_glueball_mass_measurement": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SPATIAL-BLOCK-V1",
    "vector_set": "GLUEBALL-SPATIAL-BLOCK-PHASE3B2-V1",
    "status": "phase-3b2-frozen",
    "vectors": [
        {
            "id": "identity-6cubed-to-3cubed",
            "fine_extents": [6, 6, 6, 3],
            "configuration": "all links identity",
            "observable": "maximum coarse-link residual from an all-identity 3x3x3x3 lattice",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "canonical-coarse-operator-output",
            "fine_extents": [6, 6, 6, 3],
            "coarse_extents": [3, 3, 3, 3],
            "configuration": "single traceless SU(3) cycle spatial link at fine origin",
            "observable": "frozen Phase 3A scalar operator output after one stride-2 block",
            "expected": {
                "spatial_plaquette_1x1": 0.9506172839506173,
                "spatial_rectangle_2x1": 0.9259259259259259,
            },
            "absolute_tolerance": 1e-12,
            "claim_class": "C2",
        },
        {
            "id": "temporal-link-sampled-exactly",
            "fine_extents": [6, 6, 6, 3],
            "configuration": "single traceless SU(3) cycle temporal link at fine origin",
            "observable": "matrix residual between fine temporal link at the embedded site and the corresponding coarse temporal link",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "deterministic-gauge-covariance",
            "fine_extents": [6, 6, 6, 3],
            "configuration": "single traceless SU(3) cycle spatial link at fine origin",
            "gauge_field": "deterministic_su3_site_gauge",
            "coarse_gauge_field": "deterministic_su3_site_gauge restricted by y -> (2y0,2y1,2y2,y3)",
            "observable": "maximum link residual between block(gauge(U)) and gauge_restricted(block(U))",
            "expected_max": 5e-11,
            "claim_class": "C2",
        },
        {
            "id": "blocked-link-group-residuals",
            "fine_extents": [6, 6, 6, 3],
            "configuration": "single traceless SU(3) cycle spatial link at fine origin",
            "unitarity_residual_max": 5e-12,
            "determinant_residual_max": 5e-12,
            "claim_class": "C2",
        },
        {
            "id": "too-small-spatial-lattice-rejected",
            "fine_extents": [4, 4, 4, 3],
            "expected": "SpatialExtentTooSmall",
            "claim_class": "C0",
        },
        {
            "id": "odd-spatial-lattice-rejected",
            "fine_extents": [7, 7, 7, 3],
            "expected": "SpatialExtentNotEven",
            "claim_class": "C0",
        },
        {
            "id": "anisotropic-spatial-lattice-rejected",
            "fine_extents": [6, 8, 6, 3],
            "expected": "SpatialExtentsNotCubic",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "transformation": "YM-SPATIAL-STRIDE2-BLOCK-V1",
            "stout_smearing_required": False,
            "combined_smear_block_tuning_validated": False,
            "connected_correlator_constructed": False,
            "effective_mass_extracted": False,
            "glueball_spectrum_extracted": False,
            "finite_volume_mass_gap_established": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact("contracts/YM-GLUEBALL-SPATIAL-BLOCK-V1.json", payload, EXPECTED_CONTRACT)


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-spatial-block-phase3b2-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SPATIAL-BLOCK-V1.json"))
    validate_vectors(load("test-vectors/glueball-spatial-block-phase3b2-v1.json"))
    print("Phase 3B2 spatial blocking contract checks: passed")


if __name__ == "__main__":
    main()
