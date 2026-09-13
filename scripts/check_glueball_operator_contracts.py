#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3A glueball operator surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-OPERATORS-V1",
    "status": "phase-3a-frozen",
    "scope": "unsmeared gauge-invariant spatial Wilson-loop operator basis for finite periodic SU(3) lattices only",
    "depends_on": [
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
        {"contract_id": "YM-PATH-HOLONOMY-V1", "required_status": "phase-1d-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2", "required_status": "phase-2c2-frozen"},
    ],
    "geometry": {
        "euclidean_time_direction": 3,
        "spatial_directions": [0, 1, 2],
        "operator_support": "closed spatial loops on one fixed Euclidean time slice",
        "minimum_spatial_extent": 3,
        "spatial_extents_equal": True,
        "time_slice_rule": "0 <= t < extent[3]",
        "periodic_boundary_conditions": True,
    },
    "basis": {
        "basis_id": "YM-GLUEBALL-SPATIAL-A1PP-V1",
        "smearing": "none",
        "blocking": "none",
        "normalization": "arithmetic mean of Re[(1/3)Tr U_C] over every spatial translation and every listed orientation",
        "elements": [
            {
                "id": "spatial-plaquette-1x1",
                "shape": "planar rectangle 1x1",
                "orientation_orbit": [[0, 1], [0, 2], [1, 2]],
            },
            {
                "id": "spatial-rectangle-2x1",
                "shape": "planar rectangle 2x1",
                "orientation_orbit": [[0, 1], [0, 2], [1, 0], [1, 2], [2, 0], [2, 1]],
                "orientation_semantics": "first direction has extent 2; second direction has extent 1",
            },
        ],
    },
    "channel_semantics": {
        "finite_lattice_irrep": "A1++",
        "continuum_channel_intent": "0++",
        "continuum_spin_assignment_exact_at_finite_spacing": False,
        "translation_average": True,
        "cubic_orientation_average": True,
        "parity_even_surface": "ReTr makes loop reversal/orientation conjugation even",
        "charge_conjugation_even_surface": "ReTr is invariant under complex conjugation",
    },
    "validation": {
        "identity_configuration_each_basis_element": 1.0,
        "identity_tolerance": 0.0,
        "gauge_invariance_tolerance": 2e-11,
        "expected_plaquette_loops_on_3cubed_slice": 81,
        "expected_rectangle_loops_on_3cubed_slice": 162,
        "non_finite_observable_rule": "reject",
    },
    "future_surfaces": {
        "smearing_or_blocking_applied": False,
        "connected_correlator_constructed": False,
        "vacuum_subtraction_defined": False,
        "effective_mass_extracted": False,
        "fit_window_selected": False,
        "tensor_2pp_channel_enabled": False,
        "pseudoscalar_0mp_channel_enabled": False,
    },
    "claim_boundary": {
        "operator_definition_is_glueball_mass_measurement": False,
        "finite_lattice_a1pp_is_exact_continuum_spin_zero": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-OPERATORS-V1",
    "vector_set": "GLUEBALL-OPERATORS-PHASE3A-V1",
    "status": "phase-3a-frozen",
    "vectors": [
        {
            "id": "identity-3cubed-spatial-slice",
            "extents": [3, 3, 3, 3],
            "time_slice": 0,
            "configuration": "all links identity",
            "basis_id": "YM-GLUEBALL-SPATIAL-A1PP-V1",
            "expected": {
                "spatial_plaquette_1x1": 1.0,
                "spatial_rectangle_2x1": 1.0,
                "plaquette_loop_count": 81,
                "rectangle_loop_count": 162,
            },
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "all-time-slices-identity",
            "extents": [3, 3, 3, 3],
            "configuration": "all links identity",
            "time_slices": [0, 1, 2],
            "expected_each_basis_element": 1.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "deterministic-gauge-invariance",
            "extents": [3, 3, 3, 3],
            "time_slice": 0,
            "configuration": "single traceless SU(3) cycle link",
            "gauge_field": "deterministic_su3_site_gauge",
            "observable": "max absolute basis-element difference before/after local gauge transform",
            "expected_max": 2e-11,
            "claim_class": "C2",
        },
        {
            "id": "time-slice-out-of-range-rejected",
            "extents": [3, 3, 3, 3],
            "time_slice": 3,
            "expected": "TimeSliceOutOfRange",
            "claim_class": "C0",
        },
        {
            "id": "wrapped-2x1-basis-not-admitted-on-spatial-extent-2",
            "extents": [2, 3, 3, 3],
            "time_slice": 0,
            "expected": "SpatialExtentTooSmall",
            "claim_class": "C0",
        },
        {
            "id": "anisotropic-spatial-lattice-rejected",
            "extents": [3, 4, 5, 3],
            "time_slice": 0,
            "expected": "SpatialExtentsNotCubic",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "finite_lattice_irrep": "A1++",
            "continuum_channel_intent": "0++",
            "smearing_or_blocking_applied": False,
            "connected_correlator_constructed": False,
            "effective_mass_extracted": False,
            "finite_volume_mass_gap_established": False,
            "positive_continuum_mass_gap_proved": False,
            "clay_solution_claimed": False,
            "claim_class": "C0",
        },
    ],
}


def validate_contract(payload: dict[str, Any]) -> None:
    validate_exact(
        "contracts/YM-GLUEBALL-OPERATORS-V1.json",
        payload,
        EXPECTED_CONTRACT,
    )


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-operators-phase3a-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-OPERATORS-V1.json"))
    validate_vectors(load("test-vectors/glueball-operators-phase3a-v1.json"))
    print("Phase 3A glueball operator contract checks: passed")


if __name__ == "__main__":
    main()
