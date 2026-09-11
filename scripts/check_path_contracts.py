#!/usr/bin/env python3
"""Fail-closed checks for the Phase 1D path/holonomy surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_PATH_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-PATH-HOLONOMY-V1",
    "status": "phase-1d-frozen",
    "depends_on": [
        {"contract_id": "YM-WILSON-LOOP-V1", "required_status": "phase-1a-frozen"},
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
        {"contract_id": "YM-SU3-CENTER-V1", "required_status": "phase-1c-frozen"},
    ],
    "continuum_path": {
        "domain": "[0,1]",
        "class": "oriented-piecewise-smooth",
        "map": "gamma:[0,1]->M",
        "tangent": "gamma_dot(t) in T_{gamma(t)}M",
        "local_connection": "A in Omega^1(U,g) in a chosen local trivialization",
        "pullback_coefficient": "a_gamma(t)=A_{gamma(t)}(gamma_dot(t))",
    },
    "continuum_transport_convention": {
        "evolution": "dU_gamma(t)/dt=U_gamma(t) a_gamma(t), U_gamma(0)=I",
        "ordered_exponential": "U_gamma=P_R exp(integral_0^1 a_gamma(t) dt)",
        "ordering": "segments/later infinitesimal factors multiply on the right; traversal order is left-to-right",
        "local_gauge_transform": "A^Omega=Omega A Omega^-1-dOmega Omega^-1",
        "endpoint_covariance": "U_gamma^Omega=Omega(gamma(0)) U_gamma Omega(gamma(1))^-1",
        "orientation_preserving_reparameterization_invariant": True,
        "reversal": "U_{gamma^-1}=U_gamma^-1",
        "concatenation": "if gamma2 o gamma1 means traverse gamma1 then gamma2, U_{gamma2 o gamma1}=U_gamma1 U_gamma2",
    },
    "lattice_discretization": {
        "path_nonempty": True,
        "forward_step": "+mu uses U_mu(x) then advances x->x+mu",
        "backward_step": "-mu first advances x->x-mu then uses U_mu(x-mu)^dagger",
        "product_order": "segments-multiplied-in-traversal-order-from-left-to-right",
        "open_path": "returns group-valued transport plus final periodic coordinate",
        "closed_path": "final periodic coordinate equals start coordinate",
        "max_path_steps": 1048576,
        "su2_compatibility_contract": "YM-WILSON-LOOP-V1",
    },
    "observables": {
        "open_wilson_line": "group-valued transport with endpoint covariance; not gauge invariant by itself",
        "su2_closed_loop": "(1/2)ReTr(U_C) inherited from YM-WILSON-LOOP-V1",
        "su3_closed_loop": "(1/3)Tr(U_C) is gauge invariant and may be complex on an individual configuration",
        "polyakov_loop": "normalized trace of holonomy wrapping one full periodic extent",
        "periodic_local_gauge_transform_rule": "Polyakov normalized trace is invariant under periodic local gauge transformations",
        "center_transform_rule": "Phase 1C center transformations are distinct from periodic local gauge transformations",
    },
    "small_loop_bridge": {
        "continuum_curvature": "F_A=dA+A wedge A under the frozen local convention",
        "relation": "for an infinitesimal oriented loop, U_boundary=I+F_A(X,Y)*oriented_area+higher-order terms",
        "finite_loop_equals_pointwise_curvature": False,
        "finite_plaquette_equals_continuum_field_strength": False,
    },
    "curve_geometry_firewall": {
        "frenet_curvature_is_gauge_curvature": False,
        "torsion_is_gauge_curvature": False,
        "osculating_geometry_is_gauge_curvature": False,
        "separate_theorem_required_for_any_connection": True,
    },
    "static_potential_future_surface": {
        "relation": "<W(R,T)> approximately exp(-V(R) T) only in an appropriate large-T ensemble-analysis regime",
        "ensemble_expectation_required": True,
        "static_potential_extracted": False,
        "area_law_claimed": False,
        "confinement_claimed": False,
    },
    "continuum_parallel_transport_numerically_solved": False,
    "cross_domain_semantic_ontology_imported": False,
    "claim_scope": "continuum path/transport definitions plus bounded finite-lattice holonomy implementation and regression facts only",
}

EXPECTED_PATH_VECTORS: dict[str, Any] = {
    "contract_id": "YM-PATH-HOLONOMY-V1",
    "vector_set": "PATH-HOLONOMY-PHASE1D-V1",
    "status": "phase-1d-frozen",
    "vectors": [
        {
            "id": "su3-open-path-concatenation-left-to-right",
            "configuration": "deterministic two-link noncommuting SU(3) fixture",
            "first": ["+0"],
            "second": ["+1"],
            "observable": "max_abs(U_concat-U_first U_second)",
            "expected_max": 5e-12,
            "claim_class": "C2",
        },
        {
            "id": "su3-path-reversal",
            "configuration": "deterministic two-link noncommuting SU(3) fixture",
            "path": ["+0", "+1"],
            "observable": "max_abs(U_reverse-U_path^dagger)",
            "expected_max": 5e-12,
            "claim_class": "C2",
        },
        {
            "id": "su3-open-endpoint-gauge-covariance",
            "configuration": "deterministic two-link noncommuting SU(3) fixture",
            "path": ["+0", "+1", "+2"],
            "gauge_field": "deterministic_su3_site_gauge",
            "observable": "max_abs(U'_gamma-Omega(start) U_gamma Omega(end)^dagger)",
            "expected_max": 2e-11,
            "claim_class": "C2",
        },
        {
            "id": "su3-closed-loop-gauge-invariance",
            "configuration": "deterministic two-link noncommuting SU(3) fixture",
            "path": ["+0", "+1", "-0", "-1"],
            "gauge_field": "deterministic_su3_site_gauge",
            "observable": "abs((1/3)Tr(U'_C)-(1/3)Tr(U_C))",
            "expected_max": 2e-11,
            "claim_class": "C2",
        },
        {
            "id": "su3-polyakov-periodic-gauge-invariance",
            "configuration": "deterministic two-link noncommuting SU(3) fixture",
            "direction": 0,
            "gauge_field": "deterministic_su3_site_gauge",
            "observable": "abs(P'-P)",
            "expected_max": 2e-11,
            "claim_class": "C2",
        },
        {
            "id": "su3-nonplanar-chair-exact-cycle",
            "extents": [3, 3, 3, 3],
            "configuration": "all identity except U_0([0,0,0,0])=forward three-cycle permutation matrix",
            "path": ["+0", "+1", "+2", "-0", "-1", "-2"],
            "observable": "(1/3)Tr(U_C)",
            "expected": [0.0, 0.0],
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "su3-identity-rectangle",
            "extents": [4, 4, 4, 4],
            "configuration": "all links identity",
            "rectangle": {"mu": 0, "nu": 3, "R": 2, "T": 3},
            "observable": "(1/3)Tr(U_C)",
            "expected": [1.0, 0.0],
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "frozen-su2-step-orientation-compatible",
            "source_contract": "YM-WILSON-LOOP-V1",
            "phase1d_forward": "+mu uses U_mu(x) then advances x->x+mu",
            "phase1d_backward": "-mu first advances x->x-mu then uses U_mu(x-mu)^dagger",
            "expected": "exact-semantic-match",
            "claim_class": "C0",
        },
    ],
}


def main() -> None:
    validate_exact(
        "contracts/YM-PATH-HOLONOMY-V1.json",
        load("contracts/YM-PATH-HOLONOMY-V1.json"),
        EXPECTED_PATH_CONTRACT,
    )
    validate_exact(
        "test-vectors/path-holonomy-v1.json",
        load("test-vectors/path-holonomy-v1.json"),
        EXPECTED_PATH_VECTORS,
    )
    print("Phase 1D path/holonomy contract checks: passed")


if __name__ == "__main__":
    main()
