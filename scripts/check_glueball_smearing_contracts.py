#!/usr/bin/env python3
"""Fail-closed checks for the Phase 3B1 spatial stout-smearing surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SPATIAL-STOUT-V1",
    "status": "phase-3b1-candidate",
    "scope": "deterministic synchronous spatial stout-link smearing for the frozen Phase 3A finite-lattice SU(3) glueball operator surface only",
    "depends_on": [
        {"contract_id": "YM-SU3-ALGEBRA-V1", "required_status": "phase-1b-algebra-frozen"},
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
        {"contract_id": "YM-GLUEBALL-OPERATORS-V1", "required_status": "phase-3a-frozen"},
    ],
    "primary_source": {
        "title": "Analytic smearing of SU(3) link variables in lattice QCD",
        "authors": ["Colin Morningstar", "Mike Peardon"],
        "journal": "Physical Review D 69, 054501 (2004)",
        "doi": "10.1103/PhysRevD.69.054501",
        "method": "stout-link smearing",
        "project_specialization": "spatial links only; the temporal direction is excluded from both staples and updates",
    },
    "geometry": {
        "euclidean_time_direction": 3,
        "spatial_directions": [0, 1, 2],
        "minimum_spatial_extent": 3,
        "spatial_extents_equal": True,
        "periodic_boundary_conditions": True,
        "temporal_links_modified": False,
    },
    "parameters": {
        "canonical_rho": 0.1,
        "rho_rule": "finite and 0 < rho <= 0.25",
        "maximum_rho": 0.25,
        "canonical_levels": 1,
        "maximum_levels": 8,
        "zero_levels_is_exact_noop": True,
    },
    "algorithm": {
        "level_semantics": "synchronous: every updated spatial link in level n+1 is computed only from level n",
        "spatial_staple": "C_mu(x)=rho*sum_{nu in {0,1,2},nu!=mu}[U_nu(x)U_mu(x+nu)U_nu(x+mu)^dagger + U_nu(x-nu)^dagger U_mu(x-nu) U_nu(x-nu+mu)]",
        "omega": "Omega_mu(x)=C_mu(x) U_mu(x)^dagger",
        "hermitian_traceless_generator": "Q=(i/2)(Omega^dagger-Omega)-(i/6)Tr(Omega^dagger-Omega)I",
        "generator_coefficients": "q_a=Tr(Q lambda_a) under the frozen Gell-Mann normalization",
        "link_update": "U'_mu(x)=exp(i sum_a q_a T^a) U_mu(x) for spatial mu only",
        "temporal_link_rule": "U'_3(x)=U_3(x) exactly",
        "projection_step_used": False,
        "blocking_step_used": False,
        "lattice_copy_allocation_failure_rule": "propagate via Result; no infallible Vec clone in the public smearing path",
    },
    "validation": {
        "identity_configuration_is_exact_fixed_point": True,
        "zero_level_noop_tolerance": 0.0,
        "algebra_hermitian_traceless_residual_max": 2e-12,
        "gauge_covariance_residual_max": 5e-11,
        "su3_unitarity_residual_max": 5e-12,
        "su3_determinant_residual_max": 5e-12,
        "canonical_nontrivial_fixture_must_change_spatial_links": True,
        "frozen_output_abs_tolerance": 1e-12,
        "frozen_spatial_change": 0.2158247298307305,
        "frozen_smeared_plaquette_1x1": 0.9693015447000209,
        "frozen_smeared_rectangle_2x1": 0.9511047606560571,
        "non_finite_intermediate_rule": "reject",
    },
    "future_surfaces": {
        "spatial_blocking_defined": False,
        "connected_correlator_constructed": False,
        "vacuum_subtraction_defined": False,
        "effective_mass_extracted": False,
        "fit_window_selected": False,
        "glueball_spectrum_extracted": False,
    },
    "claim_boundary": {
        "smearing_improves_ground_state_overlap_established": False,
        "smearing_parameter_is_physical_parameter": False,
        "smearing_is_glueball_mass_measurement": False,
        "finite_volume_mass_gap_established": False,
        "confinement_established": False,
        "continuum_qft_constructed": False,
        "positive_continuum_mass_gap_proved": False,
        "clay_solution_claimed": False,
    },
}

EXPECTED_VECTORS: dict[str, Any] = {
    "contract_id": "YM-GLUEBALL-SPATIAL-STOUT-V1",
    "vector_set": "GLUEBALL-SPATIAL-STOUT-PHASE3B1-V1",
    "status": "phase-3b1-candidate",
    "vectors": [
        {
            "id": "identity-fixed-point",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 3,
            "configuration": "all links identity",
            "observable": "maximum link-matrix residual before/after smearing",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "zero-level-exact-noop",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 0,
            "configuration": "single traceless SU(3) cycle link",
            "observable": "maximum link-matrix residual before/after smearing",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "temporal-links-unchanged",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 1,
            "configuration": "single traceless SU(3) cycle link",
            "observable": "maximum temporal-link residual before/after smearing",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "deterministic-gauge-covariance",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 1,
            "configuration": "single traceless SU(3) cycle link",
            "gauge_field": "deterministic_su3_site_gauge",
            "observable": "maximum link residual between smear(gauge(U)) and gauge(smear(U))",
            "expected_max": 5e-11,
            "claim_class": "C2",
        },
        {
            "id": "bounded-level-group-residuals",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 8,
            "configuration": "single traceless SU(3) cycle link",
            "unitarity_residual_max": 5e-12,
            "determinant_residual_max": 5e-12,
            "claim_class": "C2",
        },
        {
            "id": "canonical-nontrivial-fixture-changes-spatial-links",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 1,
            "configuration": "single traceless SU(3) cycle link",
            "observable": "canonical one-level stout output on the deterministic nontrivial fixture",
            "expected": {
                "maximum_all_link_residual": 0.2158247298307305,
                "spatial_plaquette_1x1": 0.9693015447000209,
                "spatial_rectangle_2x1": 0.9511047606560571,
            },
            "absolute_tolerance": 1e-12,
            "claim_class": "C2",
        },
        {
            "id": "invalid-rho-rejected",
            "extents": [3, 3, 3, 3],
            "rho_cases": [0.0, 0.250001],
            "expected": "RhoOutOfRange",
            "claim_class": "C0",
        },
        {
            "id": "excess-levels-rejected",
            "extents": [3, 3, 3, 3],
            "rho": 0.1,
            "levels": 9,
            "expected": "TooManyLevels",
            "claim_class": "C0",
        },
        {
            "id": "anisotropic-spatial-lattice-rejected",
            "extents": [3, 4, 5, 3],
            "rho": 0.1,
            "levels": 1,
            "expected": "SpatialExtentsNotCubic",
            "claim_class": "C0",
        },
        {
            "id": "claim-boundary",
            "transformation": "YM-SPATIAL-STOUT-SMEAR-V1",
            "spatial_blocking_defined": False,
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
    validate_exact(
        "contracts/YM-GLUEBALL-SPATIAL-STOUT-V1.json", payload, EXPECTED_CONTRACT
    )


def validate_vectors(payload: dict[str, Any]) -> None:
    validate_exact(
        "test-vectors/glueball-spatial-stout-phase3b1-v1.json",
        payload,
        EXPECTED_VECTORS,
    )


def main() -> None:
    validate_contract(load("contracts/YM-GLUEBALL-SPATIAL-STOUT-V1.json"))
    validate_vectors(load("test-vectors/glueball-spatial-stout-phase3b1-v1.json"))
    print("Phase 3B1 spatial stout-smearing contract checks: passed")


if __name__ == "__main__":
    main()
