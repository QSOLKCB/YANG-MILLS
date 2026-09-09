#!/usr/bin/env python3
"""Fail-closed Phase 0 checks for machine-readable and semantic contract surfaces."""

from __future__ import annotations

import json
import math
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]

EXPECTED = {
    "contracts/YM-GAUGE-GROUP-V1.json": "YM-GAUGE-GROUP-V1",
    "contracts/YM-LATTICE-V1.json": "YM-LATTICE-V1",
    "contracts/YM-EVIDENCE-V1.json": "YM-EVIDENCE-V1",
    "contracts/YM-CLAIM-POLICY-V1.json": "YM-CLAIM-POLICY-V1",
}

EXPECTED_EVIDENCE_IDENTITY_CLASSES = [
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
]

EXPECTED_CLAIM_CLASSES = {
    "C0": "software-contract",
    "C1": "finite-lattice-exact",
    "C2": "finite-lattice-numerical",
    "C3": "physical-scaling-inference",
    "C4": "continuum-mathematical-theorem",
    "C5": "clay-solution-claim",
}

EXPECTED_FORBIDDEN_EQUIVALENCES = {
    "positive_finite_lattice_mass_implies_positive_continuum_mass_gap",
    "stable_fit_implies_theorem",
    "continuum_extrapolation_implies_continuum_qft_existence",
    "cpu_gpu_agreement_implies_physical_truth",
    "literature_agreement_implies_proof",
    "formal_finite_lattice_theorem_implies_clay_solution",
}

REQUIRED_NUMERICAL_RELEASE_SENTENCE = (
    "A numerical release must explicitly say that its numerical results are "
    "evidence/observations within a specified lattice model and are not a proof "
    "of the Yang–Mills existence and mass-gap problem."
)

PHASE0_VECTOR_IDS = {
    "identity-3x3x3x3-beta2",
    "single-traceless-link-3x3x3x3-beta2",
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
    path = ROOT / relative
    return parse_json_text(path.read_text(encoding="utf-8"), relative)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"contract check failed: {message}")


def require_type(value: Any, expected: type, context: str) -> None:
    require(
        type(value) is expected,
        f"{context}: expected {expected.__name__}, got {type(value).__name__}",
    )


def require_exact_keys(value: dict[str, Any], expected: set[str], context: str) -> None:
    actual = set(value)
    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    require(
        not missing and not extra,
        f"{context}: key mismatch; missing={missing}, extra={extra}",
    )


def require_finite_number(value: Any, context: str) -> float:
    require(
        type(value) in (int, float) and type(value) is not bool,
        f"{context} must be numeric",
    )
    numeric = float(value)
    require(math.isfinite(numeric), f"{context} must be finite")
    return numeric


def validate_header(
    data: Any,
    relative: str,
    contract_id: str,
    required_keys: set[str],
) -> dict[str, Any]:
    require_type(data, dict, relative)
    require_exact_keys(data, required_keys, relative)
    require_type(data["contract_id"], str, f"{relative}.contract_id")
    require(data["contract_id"] == contract_id, f"{relative}: contract_id drift")
    require_type(data["status"], str, f"{relative}.status")
    require(data["status"] == "phase-0-draft", f"{relative}: unexpected status")
    return data


def validate_gauge_contract(data: Any) -> None:
    relative = "contracts/YM-GAUGE-GROUP-V1.json"
    gauge = validate_header(
        data,
        relative,
        "YM-GAUGE-GROUP-V1",
        {"contract_id", "status", "gauge_group", "representation", "claim_scope"},
    )
    require_type(gauge["gauge_group"], str, f"{relative}.gauge_group")
    require(gauge["gauge_group"] == "SU(2)", "Phase 0 gauge group must be SU(2)")

    representation = gauge["representation"]
    require_type(representation, dict, f"{relative}.representation")
    require_exact_keys(
        representation,
        {"storage", "components", "matrix_convention", "unit_condition", "dagger", "real_trace"},
        f"{relative}.representation",
    )
    require(representation["storage"] == "unit-quaternion-f64", "SU(2) storage drift")
    require(
        representation["components"] == ["a0", "a1", "a2", "a3"],
        "SU(2) component ordering drift",
    )
    for key, expected in {
        "matrix_convention": "U(q)=a0*I-i*(a1*sigma1+a2*sigma2+a3*sigma3)",
        "unit_condition": "a0^2+a1^2+a2^2+a3^2=1",
        "dagger": "quaternion-conjugate",
        "real_trace": "2*a0",
    }.items():
        require_type(representation[key], str, f"{relative}.representation.{key}")
        require(representation[key] == expected, f"SU(2) {key} drift")
    require(
        gauge["claim_scope"] == "finite-lattice reference mathematics only",
        "SU(2) claim scope drift",
    )


def validate_lattice_contract(data: Any) -> None:
    relative = "contracts/YM-LATTICE-V1.json"
    lattice = validate_header(
        data,
        relative,
        "YM-LATTICE-V1",
        {
            "contract_id",
            "status",
            "dimension",
            "boundary_conditions",
            "link_directions_stored",
            "minimum_extent_per_dimension",
            "plaquette_orientation",
            "plaquette_sum",
            "action",
            "numerics",
        },
    )
    require_type(lattice["dimension"], int, f"{relative}.dimension")
    require(lattice["dimension"] == 4, "lattice dimension must remain four")
    require(lattice["boundary_conditions"] == "periodic", "boundary condition drift")
    require(lattice["link_directions_stored"] == "positive-only", "stored-link direction drift")
    require_type(
        lattice["minimum_extent_per_dimension"],
        int,
        f"{relative}.minimum_extent_per_dimension",
    )
    require(lattice["minimum_extent_per_dimension"] == 2, "minimum lattice extent drift")
    require(
        lattice["plaquette_orientation"]
        == "U_mu(x) U_nu(x+mu) U_mu(x+nu)^dagger U_nu(x)^dagger",
        "plaquette orientation drift",
    )
    require(lattice["plaquette_sum"] == "mu<nu", "plaquette counting drift")

    action = lattice["action"]
    require_type(action, dict, f"{relative}.action")
    require_exact_keys(action, {"name", "formula", "beta_domain"}, f"{relative}.action")
    require(action["name"] == "Wilson", "Wilson action name drift")
    require(
        action["formula"] == "S=beta*sum_p(1-(1/2)*ReTr(U_p))",
        "Wilson action formula drift",
    )
    require(action["beta_domain"] == "finite and >=0", "Wilson beta domain drift")

    numerics = lattice["numerics"]
    require_type(numerics, dict, f"{relative}.numerics")
    require_exact_keys(
        numerics,
        {"scalar_type", "bitwise_cross_platform_identity_claimed"},
        f"{relative}.numerics",
    )
    require(numerics["scalar_type"] == "f64", "lattice scalar type drift")
    require_type(
        numerics["bitwise_cross_platform_identity_claimed"],
        bool,
        f"{relative}.numerics.bitwise_cross_platform_identity_claimed",
    )
    require(
        numerics["bitwise_cross_platform_identity_claimed"] is False,
        "cross-platform bitwise identity claim drift",
    )


def validate_evidence_contract(data: Any) -> None:
    relative = "contracts/YM-EVIDENCE-V1.json"
    evidence = validate_header(
        data,
        relative,
        "YM-EVIDENCE-V1",
        {
            "contract_id",
            "status",
            "required_identity_classes",
            "hash_semantics",
            "numerical_evidence_is_proof",
            "accelerator_is_mathematical_authority",
            "clay_solution_claimed",
        },
    )
    classes = evidence["required_identity_classes"]
    require_type(classes, list, f"{relative}.required_identity_classes")
    require(classes == EXPECTED_EVIDENCE_IDENTITY_CLASSES, "evidence identity-class drift")
    require(
        evidence["hash_semantics"] == "identity-and-integrity-only",
        "hash semantics drift",
    )
    for key in (
        "numerical_evidence_is_proof",
        "accelerator_is_mathematical_authority",
        "clay_solution_claimed",
    ):
        require_type(evidence[key], bool, f"{relative}.{key}")
        require(evidence[key] is False, f"{key} boundary weakened")


def validate_claim_policy(data: Any) -> None:
    relative = "contracts/YM-CLAIM-POLICY-V1.json"
    policy = validate_header(
        data,
        relative,
        "YM-CLAIM-POLICY-V1",
        {
            "contract_id",
            "status",
            "claim_classes",
            "forbidden_equivalences",
            "numerical_release_disclaimer_required",
            "required_numerical_release_sentence",
            "clay_solution_requires_official_problem_proof",
        },
    )

    classes = policy["claim_classes"]
    require_type(classes, dict, f"{relative}.claim_classes")
    require(classes == EXPECTED_CLAIM_CLASSES, "claim-class semantics drift")

    forbidden = policy["forbidden_equivalences"]
    require_type(forbidden, dict, f"{relative}.forbidden_equivalences")
    require_exact_keys(
        forbidden,
        EXPECTED_FORBIDDEN_EQUIVALENCES,
        f"{relative}.forbidden_equivalences",
    )
    for key in EXPECTED_FORBIDDEN_EQUIVALENCES:
        require_type(forbidden[key], bool, f"{relative}.forbidden_equivalences.{key}")
        require(forbidden[key] is False, f"forbidden equivalence weakened: {key}")

    require_type(
        policy["numerical_release_disclaimer_required"],
        bool,
        f"{relative}.numerical_release_disclaimer_required",
    )
    require(
        policy["numerical_release_disclaimer_required"] is True,
        "numerical release disclaimer requirement weakened",
    )
    require_type(
        policy["required_numerical_release_sentence"],
        str,
        f"{relative}.required_numerical_release_sentence",
    )
    require(
        policy["required_numerical_release_sentence"] == REQUIRED_NUMERICAL_RELEASE_SENTENCE,
        "required numerical release sentence drift",
    )
    require_type(
        policy["clay_solution_requires_official_problem_proof"],
        bool,
        f"{relative}.clay_solution_requires_official_problem_proof",
    )
    require(
        policy["clay_solution_requires_official_problem_proof"] is True,
        "Clay solution proof requirement weakened",
    )


def validate_identity_vector(
    vector: dict[str, Any],
    context: str,
    beta: float,
    expected_action: float,
    tolerance: float,
) -> None:
    require_exact_keys(
        vector,
        {"id", "extents", "beta", "configuration", "expected_action", "tolerance", "claim_class"},
        context,
    )
    require(vector["extents"] == [3, 3, 3, 3], f"{context}: identity extents drift")
    require(beta == 2.0, f"{context}: identity beta drift")
    require(vector["configuration"] == "all links identity", f"{context}: identity configuration drift")
    require(expected_action == 0.0, f"{context}: identity action must be exactly zero")
    require(tolerance == 0.0, f"{context}: identity tolerance drift")


def validate_single_traceless_vector(
    vector: dict[str, Any],
    context: str,
    beta: float,
    expected_action: float,
    tolerance: float,
) -> None:
    require_exact_keys(
        vector,
        {
            "id",
            "extents",
            "beta",
            "configuration",
            "affected_plaquettes",
            "expected_action",
            "tolerance",
            "claim_class",
        },
        context,
    )
    require(vector["extents"] == [3, 3, 3, 3], f"{context}: single-link extents drift")
    require(beta == 2.0, f"{context}: single-link beta drift")
    require(
        vector["configuration"] == "all identity except U_0([0,0,0,0])=(0,1,0,0)",
        f"{context}: single-link configuration drift",
    )
    require_type(vector["affected_plaquettes"], int, f"{context}.affected_plaquettes")
    require(vector["affected_plaquettes"] == 6, f"{context}: affected-plaquette count drift")
    require(expected_action == 6.0 * beta, f"{context}: expected action must equal analytic 6*beta")
    require(tolerance == 1.0e-12, f"{context}: single-link tolerance drift")


def validate_vectors(data: Any) -> None:
    relative = "test-vectors/su2-wilson-v1.json"
    require_type(data, dict, relative)
    require_exact_keys(data, {"contract_id", "vector_set", "status", "vectors"}, relative)
    require(data["contract_id"] == "YM-LATTICE-V1", "test-vector contract drift")
    require(data["vector_set"] == "SU2-WILSON-PHASE0-V1", "vector-set identity drift")
    require(data["status"] == "phase-0-draft", "test-vector status drift")
    require_type(data["vectors"], list, f"{relative}.vectors")
    require(len(data["vectors"]) == 2, "Phase 0 vector set must contain exactly two vectors")

    seen_ids: set[str] = set()
    for index, vector in enumerate(data["vectors"]):
        context = f"{relative}.vectors[{index}]"
        require_type(vector, dict, context)
        require_type(vector.get("id"), str, f"{context}.id")
        vector_id = vector["id"]
        require(vector_id in PHASE0_VECTOR_IDS, f"{context}: unknown Phase 0 vector id {vector_id!r}")
        require(vector_id not in seen_ids, f"{context}: duplicate vector id {vector_id!r}")
        seen_ids.add(vector_id)

        require_type(vector.get("extents"), list, f"{context}.extents")
        require(
            len(vector["extents"]) == 4
            and all(type(n) is int and n >= 2 for n in vector["extents"]),
            f"{context}.extents must be four integer extents >= 2",
        )
        beta = require_finite_number(vector.get("beta"), f"{context}.beta")
        require(beta >= 0.0, f"{context}.beta must be non-negative")
        expected_action = require_finite_number(
            vector.get("expected_action"),
            f"{context}.expected_action",
        )
        tolerance = require_finite_number(vector.get("tolerance"), f"{context}.tolerance")
        require(tolerance >= 0.0, f"{context}.tolerance must be non-negative")
        require_type(vector.get("configuration"), str, f"{context}.configuration")
        require_type(vector.get("claim_class"), str, f"{context}.claim_class")
        require(vector["claim_class"] == "C1", f"{context}: claim class drift")

        if vector_id == "identity-3x3x3x3-beta2":
            validate_identity_vector(vector, context, beta, expected_action, tolerance)
        else:
            validate_single_traceless_vector(vector, context, beta, expected_action, tolerance)

    require(seen_ids == PHASE0_VECTOR_IDS, "Phase 0 vector identity set drift")


def main() -> None:
    loaded = {relative: load(relative) for relative in EXPECTED}
    validate_gauge_contract(loaded["contracts/YM-GAUGE-GROUP-V1.json"])
    validate_lattice_contract(loaded["contracts/YM-LATTICE-V1.json"])
    validate_evidence_contract(loaded["contracts/YM-EVIDENCE-V1.json"])
    validate_claim_policy(loaded["contracts/YM-CLAIM-POLICY-V1.json"])
    validate_vectors(load("test-vectors/su2-wilson-v1.json"))

    claim_text = (ROOT / "docs/CLAIM_BOUNDARY.md").read_text(encoding="utf-8")
    require(
        "`contracts/YM-CLAIM-POLICY-V1.json` is the machine-readable authority"
        in claim_text,
        "claim boundary must identify the machine-readable authority",
    )
    require(
        REQUIRED_NUMERICAL_RELEASE_SENTENCE in claim_text,
        "claim boundary lost required numerical-release refusal language",
    )

    donor_text = (ROOT / "docs/DONOR_BOUNDARY.md").read_text(encoding="utf-8")
    require("engineering donor" in donor_text, "donor semantics drift")
    require(
        "80941183d14531093117e122da0fc32c13d2464b" in donor_text,
        "GLUBALL donor SHA drift",
    )

    print("Phase 0 contract checks: passed")


if __name__ == "__main__":
    main()
