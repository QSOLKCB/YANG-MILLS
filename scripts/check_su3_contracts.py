#!/usr/bin/env python3
"""Fail-closed checks for the SU(3) algebra, lattice, and center surfaces."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_SU3_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU3-ALGEBRA-V1",
    "status": "phase-1b-algebra-frozen",
    "scope": "standalone SU(3) algebra and group reference mathematics",
    "representation": {
        "fundamental_dimension": 3,
        "scalar_type": "complex-f64",
        "generator_basis": "canonical-gell-mann",
        "generator_count": 8,
        "T_definition": "T^a=lambda_a/2",
        "trace_normalization": "Tr(lambda_a lambda_b)=2 delta_ab",
    },
    "structure_constants": {
        "commutator": "[T^a,T^b]=i f^{abc} T^c",
        "anticommutator": "{T^a,T^b}=(1/3)delta_ab I+d^{abc}T^c",
        "f_extraction": "f^{abc}=2 Im Tr([T^a,T^b]T^c)",
        "d_extraction": "d^{abc}=2 Re Tr({T^a,T^b}T^c)",
    },
    "casimirs": {
        "quadratic_fundamental": "sum_a T^a T^a=(4/3)I",
        "cubic_operator": "sum_abc d^{abc}T^aT^bT^c=(10/9)I",
    },
    "exponentiation": {
        "map": "U(theta)=exp(i sum_a theta_a T^a)",
        "coefficient_domain": "all theta_a finite and abs(theta_a)<=64",
        "method": "scaling-and-squaring-taylor-64",
        "scale_target_one_norm": 0.5,
        "unitarity_residual": "max(max_abs(U^dagger U-I),max_abs(U U^dagger-I))",
        "unitarity_acceptance_tolerance": 5e-12,
        "determinant_residual": "max(abs(det(U)-1),abs(det(U^dagger)-1))",
        "determinant_one_acceptance_tolerance": 5e-12,
        "projection_to_su3_used": False,
    },
    "lattice_links_enabled": False,
    "cross_domain_semantic_ontology_imported": False,
    "claim_scope": "finite-dimensional SU(3) algebra/group reference only",
}

EXPECTED_SU3_VECTORS: dict[str, Any] = {
    "contract_id": "YM-SU3-ALGEBRA-V1",
    "vector_set": "SU3-ALGEBRA-PHASE1B-V1",
    "status": "phase-1b-algebra-frozen",
    "vectors": [
        {"id": "gell-mann-trace-normalization", "observable": "Tr(lambda_a lambda_b)", "expected_rule": "2 delta_ab", "tolerance": 2e-15, "claim_class": "C0"},
        {"id": "f123", "observable": "f", "indices_one_based": [1, 2, 3], "expected": 1.0, "tolerance": 2e-15, "claim_class": "C0"},
        {"id": "f156", "observable": "f", "indices_one_based": [1, 5, 6], "expected": -0.5, "tolerance": 2e-15, "claim_class": "C0"},
        {"id": "d118", "observable": "d", "indices_one_based": [1, 1, 8], "expected": 0.5773502691896258, "tolerance": 2e-15, "claim_class": "C0"},
        {"id": "d888", "observable": "d", "indices_one_based": [8, 8, 8], "expected": -0.5773502691896258, "tolerance": 2e-15, "claim_class": "C0"},
        {"id": "quadratic-casimir-fundamental", "observable": "sum_a T^a T^a", "expected_multiple_of_identity": 1.3333333333333333, "tolerance": 3e-15, "claim_class": "C0"},
        {"id": "cubic-casimir-fundamental", "observable": "sum_abc d^{abc} T^a T^b T^c", "expected_multiple_of_identity": 1.1111111111111112, "tolerance": 8e-15, "claim_class": "C0"},
        {"id": "exp-theta3-0p7", "observable": "U=exp(i theta T^3)", "theta": 0.7, "expected_diagonal": "diag(exp(i*0.35),exp(-i*0.35),1)", "matrix_tolerance": 2e-14, "unitarity_tolerance": 5e-12, "determinant_one_tolerance": 5e-12, "claim_class": "C0"},
    ],
}

EXPECTED_SU3_LATTICE_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU3-LATTICE-V1",
    "status": "phase-1b-frozen",
    "depends_on": {
        "contract_id": "YM-SU3-ALGEBRA-V1",
        "required_status": "phase-1b-algebra-frozen",
    },
    "gauge_group": "SU(3)",
    "dimension": 4,
    "boundary_conditions": "periodic",
    "link_storage": "positive-directions-only",
    "negative_transport_rule": "U_mu(x-mu)^dagger",
    "plaquette_orientation": "U_mu(x) U_nu(x+mu) U_mu(x+nu)^dagger U_nu(x)^dagger",
    "wilson_action": {
        "formula": "S=beta*sum_x,sum_mu<nu(1-(1/3)*ReTr(P_mu_nu(x)))",
        "beta_domain": "finite and >=0",
        "non_finite_result": "reject",
    },
    "gauge_transform": {
        "formula": "U_mu(x)->Omega(x) U_mu(x) Omega(x+mu)^dagger",
        "site_field_sampling": "exactly-once-per-site-before-link-transform",
        "action_invariant_within_declared_tolerance": True,
    },
    "group_acceptance": {
        "source_contract": "YM-SU3-ALGEBRA-V1",
        "projection_to_su3_used": False,
        "link_products_fail_closed": True,
    },
    "reference_limits": {
        "max_links": 262144,
        "allocation": "try_reserve_exact-before-resize",
        "overflow": "reject-before-allocation",
    },
    "fixture_boundary": {
        "traceless_cycle_link": "3-cycle permutation matrix used only as exact SU(3) regression fixture",
        "center_symmetry_interpretation_claimed": False,
    },
    "monte_carlo_enabled": False,
    "confinement_claimed": False,
    "claim_scope": "finite-lattice SU(3) reference mechanics only",
}

EXPECTED_SU3_LATTICE_VECTORS: dict[str, Any] = {
    "contract_id": "YM-SU3-LATTICE-V1",
    "vector_set": "SU3-LATTICE-PHASE1B-V1",
    "status": "phase-1b-frozen",
    "vectors": [
        {
            "id": "identity-3x3x3x3-beta2",
            "extents": [3, 3, 3, 3],
            "beta": 2.0,
            "configuration": "all links identity",
            "observable": "wilson_action",
            "expected": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-cycle-link-3x3x3x3-beta2",
            "extents": [3, 3, 3, 3],
            "beta": 2.0,
            "configuration": "all identity except U_0([0,0,0,0])=three-cycle permutation matrix",
            "fixture_trace": 0.0,
            "fixture_determinant": 1.0,
            "affected_plaquettes": 6,
            "observable": "wilson_action",
            "expected": 12.0,
            "tolerance": 1e-12,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-cycle-link-gauge-invariance",
            "extents": [3, 3, 3, 3],
            "beta": 1.7,
            "configuration": "single-traceless-cycle-link",
            "gauge_field": "deterministic-theta3-site-field",
            "observable": "absolute_wilson_action_difference_after_local_gauge_transform",
            "expected_max": 1e-10,
            "link_unitarity_residual_max": 5e-12,
            "link_determinant_residual_max": 5e-12,
            "claim_class": "C2",
        },
    ],
}

EXPECTED_SU3_CENTER_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU3-CENTER-V1",
    "status": "phase-1c-frozen",
    "depends_on": [
        {"contract_id": "YM-SU3-ALGEBRA-V1", "required_status": "phase-1b-algebra-frozen"},
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
    ],
    "abstract_group": {
        "name": "C3",
        "order": 3,
        "presentation": "<g | g^3=e>",
        "isomorphic_to": "Z3-additive",
        "composition": "exponents-add-mod-3",
    },
    "cycle_embedding": {
        "symbol": "Omega",
        "basis_action": "Omega|k>=|k+1 mod 3>",
        "matrix": "[[0,0,1],[1,0,0],[0,1,0]]",
        "order": 3,
        "unitary": True,
        "determinant": 1,
        "central": False,
        "noncentral_witness": "diag(-1,-1,1)",
        "noncentral_commutator_residual": 2.0,
    },
    "center_embedding": {
        "definition": "Z(SU(3))={I,omega I,omega^2 I}",
        "omega": "exp(2*pi*i/3)",
        "omega_components": [-0.5, 0.8660254037844386],
        "order": 3,
        "unitarity_tolerance": 5e-12,
        "determinant_one_tolerance": 5e-12,
        "tested_commutator_tolerance": 2e-15,
        "abstract_isomorphism": "Z(SU(3))~=C3~=Z3",
        "same_as_cycle_embedding": False,
    },
    "embedding_boundary": {
        "same_abstract_group_does_not_imply_same_subgroup_action": True,
        "cycle_embedding_is_noncentral": True,
        "scalar_center_embedding_is_central": True,
    },
    "polyakov_center_transform": {
        "operation": "left-multiply U_mu(x) by z in Z(SU(3)) for every x with x_mu=slice",
        "plaquette_action_rule": "center factors cancel pairwise in every plaquette",
        "polyakov_rule": "L_mu -> z L_mu and (1/3)Tr(L_mu)->z(1/3)Tr(L_mu)",
        "action_invariant_within_declared_tolerance": True,
        "confinement_claimed": False,
        "deconfinement_claimed": False,
        "order_parameter_inference_claimed": False,
    },
    "cross_domain_semantic_ontology_imported": False,
    "claim_scope": "finite-dimensional center/group facts and bounded finite-lattice center-transform observations only",
}

EXPECTED_SU3_CENTER_VECTORS: dict[str, Any] = {
    "contract_id": "YM-SU3-CENTER-V1",
    "vector_set": "SU3-CENTER-PHASE1C-V1",
    "status": "phase-1c-frozen",
    "vectors": [
        {"id": "abstract-c3-generator-cubed", "observable": "g^3", "expected": "identity", "claim_class": "C0"},
        {"id": "omega-cubed", "observable": "Omega^3", "expected": "identity", "tolerance": 0.0, "claim_class": "C0"},
        {"id": "omega-noncentral-witness", "observable": "max_abs(Omega D-D Omega)", "witness": "D=diag(-1,-1,1)", "expected": 2.0, "tolerance": 0.0, "claim_class": "C0"},
        {"id": "genuine-center-generator", "observable": "z=omega I", "omega_components": [-0.5, 0.8660254037844386], "unitarity_residual_max": 5e-12, "determinant_residual_max": 5e-12, "tested_commutator_residual_max": 2e-15, "claim_class": "C0"},
        {"id": "cycle-and-center-embeddings-distinct", "observable": "max_abs(Omega-z_k I)", "for_all_k": [0, 1, 2], "expected_min": 1.0, "claim_class": "C0"},
        {
            "id": "identity-polyakov-center-transform",
            "extents": [3, 3, 3, 3],
            "configuration": "all links identity",
            "direction": 0,
            "slice": 0,
            "center_exponent": 1,
            "polyakov_before": [1.0, 0.0],
            "polyakov_after": [-0.5, 0.8660254037844386],
            "polyakov_tolerance": 2e-15,
            "wilson_action_difference_max": 2e-12,
            "claim_class": "C2",
        },
        {
            "id": "nontrivial-action-center-transform",
            "extents": [3, 3, 3, 3],
            "configuration": "single-traceless-cycle-link",
            "beta": 2.0,
            "direction": 0,
            "slice": 1,
            "center_exponent": 2,
            "action_before": 12.0,
            "action_before_tolerance": 1e-12,
            "action_difference_max": 1e-10,
            "claim_class": "C2",
        },
    ],
}


def main() -> None:
    validate_exact(
        "contracts/YM-SU3-ALGEBRA-V1.json",
        load("contracts/YM-SU3-ALGEBRA-V1.json"),
        EXPECTED_SU3_CONTRACT,
    )
    validate_exact(
        "test-vectors/su3-algebra-v1.json",
        load("test-vectors/su3-algebra-v1.json"),
        EXPECTED_SU3_VECTORS,
    )
    validate_exact(
        "contracts/YM-SU3-LATTICE-V1.json",
        load("contracts/YM-SU3-LATTICE-V1.json"),
        EXPECTED_SU3_LATTICE_CONTRACT,
    )
    validate_exact(
        "test-vectors/su3-lattice-v1.json",
        load("test-vectors/su3-lattice-v1.json"),
        EXPECTED_SU3_LATTICE_VECTORS,
    )
    validate_exact(
        "contracts/YM-SU3-CENTER-V1.json",
        load("contracts/YM-SU3-CENTER-V1.json"),
        EXPECTED_SU3_CENTER_CONTRACT,
    )
    validate_exact(
        "test-vectors/su3-center-v1.json",
        load("test-vectors/su3-center-v1.json"),
        EXPECTED_SU3_CENTER_VECTORS,
    )
    print("SU(3) algebra, lattice, and center contract checks: passed")


if __name__ == "__main__":
    main()
