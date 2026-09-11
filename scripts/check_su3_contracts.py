#!/usr/bin/env python3
"""Fail-closed checks for the Phase 1B standalone SU(3) algebra surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_SU3_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU3-ALGEBRA-V1",
    "status": "phase-1b-review-candidate",
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
        "unitarity_acceptance_tolerance": 5e-12,
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
    "status": "phase-1b-review-candidate",
    "vectors": [
        {
            "id": "gell-mann-trace-normalization",
            "observable": "Tr(lambda_a lambda_b)",
            "expected_rule": "2 delta_ab",
            "tolerance": 2e-15,
            "claim_class": "C0",
        },
        {
            "id": "f123",
            "observable": "f",
            "indices_one_based": [1, 2, 3],
            "expected": 1.0,
            "tolerance": 2e-15,
            "claim_class": "C0",
        },
        {
            "id": "f156",
            "observable": "f",
            "indices_one_based": [1, 5, 6],
            "expected": -0.5,
            "tolerance": 2e-15,
            "claim_class": "C0",
        },
        {
            "id": "d118",
            "observable": "d",
            "indices_one_based": [1, 1, 8],
            "expected": 0.5773502691896258,
            "tolerance": 2e-15,
            "claim_class": "C0",
        },
        {
            "id": "d888",
            "observable": "d",
            "indices_one_based": [8, 8, 8],
            "expected": -0.5773502691896258,
            "tolerance": 2e-15,
            "claim_class": "C0",
        },
        {
            "id": "quadratic-casimir-fundamental",
            "observable": "sum_a T^a T^a",
            "expected_multiple_of_identity": 1.3333333333333333,
            "tolerance": 3e-15,
            "claim_class": "C0",
        },
        {
            "id": "cubic-casimir-fundamental",
            "observable": "sum_abc d^{abc} T^a T^b T^c",
            "expected_multiple_of_identity": 1.1111111111111112,
            "tolerance": 8e-15,
            "claim_class": "C0",
        },
        {
            "id": "exp-theta3-0p7",
            "observable": "U=exp(i theta T^3)",
            "theta": 0.7,
            "expected_diagonal": "diag(exp(i*0.35),exp(-i*0.35),1)",
            "matrix_tolerance": 2e-14,
            "unitarity_tolerance": 5e-12,
            "determinant_one_tolerance": 5e-12,
            "claim_class": "C0",
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
    print("Phase 1B SU(3) contract checks: passed")


if __name__ == "__main__":
    main()
