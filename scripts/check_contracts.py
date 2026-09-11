#!/usr/bin/env python3
"""Fail-closed checks for frozen/candidate machine-readable YANG-MILLS contracts."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_NUMERICAL_RELEASE_SENTENCE = (
    "A numerical release must explicitly say that its numerical results are "
    "evidence/observations within a specified lattice model and are not a proof "
    "of the Yang–Mills existence and mass-gap problem."
)

EXPECTED_CONTRACTS: dict[str, dict[str, Any]] = {
    "contracts/YM-GAUGE-GROUP-V1.json": {
        "contract_id": "YM-GAUGE-GROUP-V1",
        "status": "phase-0-draft",
        "gauge_group": "SU(2)",
        "representation": {
            "storage": "unit-quaternion-f64",
            "components": ["a0", "a1", "a2", "a3"],
            "matrix_convention": "U(q)=a0*I-i*(a1*sigma1+a2*sigma2+a3*sigma3)",
            "unit_condition": "a0^2+a1^2+a2^2+a3^2=1",
            "dagger": "quaternion-conjugate",
            "real_trace": "2*a0",
        },
        "claim_scope": "finite-lattice reference mathematics only",
    },
    "contracts/YM-LATTICE-V1.json": {
        "contract_id": "YM-LATTICE-V1",
        "status": "phase-0-draft",
        "dimension": 4,
        "boundary_conditions": "periodic",
        "link_directions_stored": "positive-only",
        "minimum_extent_per_dimension": 2,
        "plaquette_orientation": "U_mu(x) U_nu(x+mu) U_mu(x+nu)^dagger U_nu(x)^dagger",
        "plaquette_sum": "mu<nu",
        "action": {
            "name": "Wilson",
            "formula": "S=beta*sum_p(1-(1/2)*ReTr(U_p))",
            "beta_domain": "finite and >=0",
        },
        "numerics": {
            "scalar_type": "f64",
            "bitwise_cross_platform_identity_claimed": False,
        },
    },
    "contracts/YM-EVIDENCE-V1.json": {
        "contract_id": "YM-EVIDENCE-V1",
        "status": "phase-0-draft",
        "required_identity_classes": [
            "source_commit",
            "contract_ids",
            "gauge_group",
            "lattice",
            "algorithm",
            "parameters",
            "runtime_toolchain",
            "numerical_tolerances",
            "evidence_reference",
            "analysis_version",
            "claim_class",
        ],
        "hash_semantics": "identity-and-integrity-only",
        "numerical_evidence_is_proof": False,
        "accelerator_is_mathematical_authority": False,
        "clay_solution_claimed": False,
    },
    "contracts/YM-CLAIM-POLICY-V1.json": {
        "contract_id": "YM-CLAIM-POLICY-V1",
        "status": "phase-0-draft",
        "claim_classes": {
            "C0": "software-contract",
            "C1": "finite-lattice-exact",
            "C2": "finite-lattice-numerical",
            "C3": "physical-scaling-inference",
            "C4": "continuum-mathematical-theorem",
            "C5": "clay-solution-claim",
        },
        "forbidden_equivalences": {
            "positive_finite_lattice_mass_implies_positive_continuum_mass_gap": False,
            "stable_fit_implies_theorem": False,
            "continuum_extrapolation_implies_continuum_qft_existence": False,
            "cpu_gpu_agreement_implies_physical_truth": False,
            "literature_agreement_implies_proof": False,
            "formal_finite_lattice_theorem_implies_clay_solution": False,
        },
        "numerical_release_disclaimer_required": True,
        "required_numerical_release_sentence": REQUIRED_NUMERICAL_RELEASE_SENTENCE,
        "clay_solution_requires_official_problem_proof": True,
    },
    "contracts/YM-SU2-LATTICE-V1.json": {
        "contract_id": "YM-SU2-LATTICE-V1",
        "status": "phase-1a-review-candidate",
        "gauge_group": "SU(2)",
        "dimension": 4,
        "boundary_conditions": "periodic",
        "link_storage": "positive-directions-only",
        "negative_link_rule": "U_mu(x-mu)^dagger",
        "plaquette_orientation": "U_mu(x) U_nu(x+mu) U_mu(x+nu)^dagger U_nu(x)^dagger",
        "wilson_action": {
            "formula": "S=beta*sum_x,sum_mu<nu(1-(1/2)*ReTr(P_mu_nu(x)))",
            "beta_domain": "finite and >=0",
            "non_finite_result": "reject",
        },
        "reference_limits": {
            "max_links": 1048576,
            "allocation": "try_reserve_exact-before-resize",
            "overflow": "reject-before-allocation",
        },
        "configuration_encoding": {
            "contract_id": "YM-SU2-CONFIG-V1",
            "header": "YM-SU2-CONFIG-V1",
            "component_encoding": "lowercase-16-hex-digit-ieee754-binary64-bits",
            "link_order": "site-major-x0-x1-x2-x3-then-direction-0-1-2-3",
            "round_trip": "exact-canonical-text",
            "unit_tolerance": 2e-12,
        },
        "local_update_surface": {
            "staple_terms_per_link": 6,
            "local_action_difference": "new-local-action-minus-old-local-action",
            "left_update": "U_mu(x)<-Delta*U_mu(x)",
        },
        "claim_scope": "finite-lattice exact and reference-numerical mechanics only",
    },
    "contracts/YM-WILSON-LOOP-V1.json": {
        "contract_id": "YM-WILSON-LOOP-V1",
        "status": "phase-1a-review-candidate",
        "gauge_group": "SU(2)",
        "path_step": {
            "forward": "+mu uses U_mu(x) then advances x->x+mu",
            "backward": "-mu first advances x->x-mu then uses U_mu(x-mu)^dagger",
        },
        "path_product_order": "segments-multiplied-in-traversal-order-from-left-to-right",
        "closed_path_rule": "final periodic lattice coordinate equals start coordinate",
        "observable": "W(C)=(1/2)*ReTr(product_C U)",
        "identity_rule": "W(C)=1 for every valid closed path on the identity configuration",
        "reverse_rule": "immediate +mu,-mu transport multiplies to identity",
        "gauge_rule": "closed-loop normalized trace is invariant under local gauge transformations",
        "cyclic_rule": "normalized trace is invariant under cyclic choice of closed-loop basepoint",
        "rectangle_rule": "+mu^R,+nu^T,-mu^R,-nu^T",
        "polyakov_rule": "positive-direction holonomy wrapping one full periodic extent",
        "reference_limits": {
            "max_path_steps": 1048576,
            "zero_rectangle_extent": "reject",
            "open_path_as_wilson_loop": "reject",
        },
        "confinement_claimed": False,
        "continuum_curvature_identity_claimed_at_finite_spacing": False,
    },
}

EXPECTED_PHASE0_VECTORS = {
    "contract_id": "YM-LATTICE-V1",
    "vector_set": "SU2-WILSON-PHASE0-V1",
    "status": "phase-0-draft",
    "vectors": [
        {
            "id": "identity-3x3x3x3-beta2",
            "extents": [3, 3, 3, 3],
            "beta": 2.0,
            "configuration": "all links identity",
            "expected_action": 0.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-link-3x3x3x3-beta2",
            "extents": [3, 3, 3, 3],
            "beta": 2.0,
            "configuration": "all identity except U_0([0,0,0,0])=(0,1,0,0)",
            "affected_plaquettes": 6,
            "expected_action": 12.0,
            "tolerance": 1e-12,
            "claim_class": "C1",
        },
    ],
}

EXPECTED_PHASE1A_VECTORS = {
    "contract_id": "YM-WILSON-LOOP-V1",
    "vector_set": "SU2-OBSERVABLES-PHASE1A-V1",
    "status": "phase-1a-review-candidate",
    "vectors": [
        {
            "id": "identity-rectangle-2x1",
            "extents": [3, 3, 3, 3],
            "configuration": "all links identity",
            "observable": "rectangular_wilson_loop",
            "start": [0, 0, 0, 0],
            "mu": 0,
            "nu": 1,
            "r": 2,
            "t": 1,
            "expected": 1.0,
            "tolerance": 0.0,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-rectangle-2x1",
            "extents": [3, 3, 3, 3],
            "configuration": "all identity except U_0([0,0,0,0])=(0,1,0,0)",
            "observable": "rectangular_wilson_loop",
            "start": [0, 0, 0, 0],
            "mu": 0,
            "nu": 1,
            "r": 2,
            "t": 1,
            "expected": 0.0,
            "tolerance": 1e-12,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-polyakov-dir0",
            "extents": [3, 3, 3, 3],
            "configuration": "all identity except U_0([0,0,0,0])=(0,1,0,0)",
            "observable": "polyakov_loop",
            "start": [0, 0, 0, 0],
            "direction": 0,
            "expected": 0.0,
            "tolerance": 1e-12,
            "claim_class": "C1",
        },
        {
            "id": "single-traceless-local-delta-to-identity",
            "extents": [3, 3, 3, 3],
            "configuration": "all identity except U_0([0,0,0,0])=(0,1,0,0)",
            "observable": "local_action_difference",
            "site": [0, 0, 0, 0],
            "direction": 0,
            "candidate": "identity",
            "beta": 2.0,
            "expected": -12.0,
            "tolerance": 1e-12,
            "claim_class": "C1",
        },
    ],
}


class DuplicateKeyError(ValueError):
    """Raised when a JSON object contains the same key more than once."""


def reject_duplicate_object_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateKeyError(f"duplicate object key {key!r}")
        result[key] = value
    return result


def reject_nonstandard_constant(token: str) -> Any:
    raise ValueError(f"non-standard JSON numeric constant {token!r}")


def parse_json_text(text: str, context: str) -> Any:
    try:
        return json.loads(
            text,
            object_pairs_hook=reject_duplicate_object_pairs,
            parse_constant=reject_nonstandard_constant,
        )
    except (json.JSONDecodeError, DuplicateKeyError, ValueError) as exc:
        raise SystemExit(f"contract check failed: {context}: invalid JSON: {exc}") from exc


def load(relative: str) -> Any:
    return parse_json_text((ROOT / relative).read_text(encoding="utf-8"), relative)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"contract check failed: {message}")


def validate_exact(relative: str, payload: Any, expected: Any) -> None:
    require(payload == expected, f"{relative}: contract/vector semantics drift")


def main() -> None:
    for relative, expected in EXPECTED_CONTRACTS.items():
        validate_exact(relative, load(relative), expected)

    validate_exact(
        "test-vectors/su2-wilson-v1.json",
        load("test-vectors/su2-wilson-v1.json"),
        EXPECTED_PHASE0_VECTORS,
    )
    validate_exact(
        "test-vectors/su2-observables-v1.json",
        load("test-vectors/su2-observables-v1.json"),
        EXPECTED_PHASE1A_VECTORS,
    )

    claim_text = (ROOT / "docs/CLAIM_BOUNDARY.md").read_text(encoding="utf-8")
    require(
        "contracts/YM-CLAIM-POLICY-V1.json` is the machine-readable authority" in claim_text,
        "claim-boundary prose lost machine-policy authority",
    )
    require(
        REQUIRED_NUMERICAL_RELEASE_SENTENCE in claim_text,
        "claim-boundary prose lost the required numerical release sentence",
    )

    donor_text = (ROOT / "docs/DONOR_BOUNDARY.md").read_text(encoding="utf-8")
    require("engineering donor" in donor_text, "donor semantics drift")
    require(
        "80941183d14531093117e122da0fc32c13d2464b" in donor_text,
        "GLUBALL donor SHA drift",
    )

    print("YANG-MILLS contract checks: passed")


if __name__ == "__main__":
    main()
