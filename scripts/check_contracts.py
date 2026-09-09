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


def load(relative: str) -> Any:
    path = ROOT / relative
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


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
    require_type(representation["storage"], str, f"{relative}.representation.storage")
    require(representation["storage"] == "unit-quaternion-f64", "SU(2) storage drift")
    require_type(representation["components"], list, f"{relative}.representation.components")
    require(
        representation["components"] == ["a0", "a1", "a2", "a3"],
        "SU(2) component ordering drift",
    )
    require(
        all(type(component) is str for component in representation["components"]),
        "SU(2) components must be strings",
    )
    for key, expected in {
        "matrix_convention": "U(q)=a0*I-i*(a1*sigma1+a2*sigma2+a3*sigma3)",
        "unit_condition": "a0^2+a1^2+a2^2+a3^2=1",
        "dagger": "quaternion-conjugate",
        "real_trace": "2*a0",
    }.items():
        require_type(representation[key], str, f"{relative}.representation.{key}")
        require(representation[key] == expected, f"SU(2) {key} drift")

    require_type(gauge["claim_scope"], str, f"{relative}.claim_scope")
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
    require_type(lattice["boundary_conditions"], str, f"{relative}.boundary_conditions")
    require(lattice["boundary_conditions"] == "periodic", "boundary condition drift")
    require_type(lattice["link_directions_stored"], str, f"{relative}.link_directions_stored")
    require(lattice["link_directions_stored"] == "positive-only", "stored-link direction drift")
    require_type(
        lattice["minimum_extent_per_dimension"],
        int,
        f"{relative}.minimum_extent_per_dimension",
    )
    require(lattice["minimum_extent_per_dimension"] == 2, "minimum lattice extent drift")
    require_type(lattice["plaquette_orientation"], str, f"{relative}.plaquette_orientation")
    require(
        lattice["plaquette_orientation"]
        == "U_mu(x) U_nu(x+mu) U_mu(x+nu)^dagger U_nu(x)^dagger",
        "plaquette orientation drift",
    )
    require_type(lattice["plaquette_sum"], str, f"{relative}.plaquette_sum")
    require(lattice["plaquette_sum"] == "mu<nu", "plaquette counting drift")

    action = lattice["action"]
    require_type(action, dict, f"{relative}.action")
    require_exact_keys(action, {"name", "formula", "beta_domain"}, f"{relative}.action")
    for key in ("name", "formula", "beta_domain"):
        require_type(action[key], str, f"{relative}.action.{key}")
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
    require_type(numerics["scalar_type"], str, f"{relative}.numerics.scalar_type")
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
    require(
        all(type(item) is str for item in classes),
        "evidence identity classes must all be strings",
    )
    require(classes == EXPECTED_EVIDENCE_IDENTITY_CLASSES, "evidence identity-class drift")
    require_type(evidence["hash_semantics"], str, f"{relative}.hash_semantics")
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


def validate_vectors(data: Any) -> None:
    relative = "test-vectors/su2-wilson-v1.json"
    require_type(data, dict, relative)
    require_exact_keys(data, {"contract_id", "vector_set", "status", "vectors"}, relative)
    require_type(data["contract_id"], str, f"{relative}.contract_id")
    require(data["contract_id"] == "YM-LATTICE-V1", "test-vector contract drift")
    require_type(data["vector_set"], str, f"{relative}.vector_set")
    require(data["vector_set"] == "SU2-WILSON-PHASE0-V1", "vector-set identity drift")
    require_type(data["status"], str, f"{relative}.status")
    require(data["status"] == "phase-0-draft", "test-vector status drift")
    require_type(data["vectors"], list, f"{relative}.vectors")
    require(len(data["vectors"]) >= 2, "expected Phase 0 regression vectors")

    required_vector_keys = {
        "id",
        "extents",
        "beta",
        "configuration",
        "expected_action",
        "tolerance",
        "claim_class",
    }
    for index, vector in enumerate(data["vectors"]):
        context = f"{relative}.vectors[{index}]"
        require_type(vector, dict, context)
        missing = required_vector_keys - set(vector)
        require(not missing, f"{context}: missing required keys {sorted(missing)}")
        require_type(vector["id"], str, f"{context}.id")
        require_type(vector["extents"], list, f"{context}.extents")
        require(
            len(vector["extents"]) == 4 and all(type(n) is int and n >= 2 for n in vector["extents"]),
            f"{context}.extents must be four integer extents >= 2",
        )
        beta = require_finite_number(vector["beta"], f"{context}.beta")
        require(beta >= 0.0, f"{context}.beta must be non-negative")
        require_finite_number(vector["expected_action"], f"{context}.expected_action")
        tolerance = require_finite_number(vector["tolerance"], f"{context}.tolerance")
        require(tolerance >= 0.0, f"{context}.tolerance must be non-negative")
        require_type(vector["configuration"], str, f"{context}.configuration")
        require_type(vector["claim_class"], str, f"{context}.claim_class")
        require(vector["claim_class"] == "C1", f"{context}: claim class drift")
        if "affected_plaquettes" in vector:
            require_type(vector["affected_plaquettes"], int, f"{context}.affected_plaquettes")
            require(vector["affected_plaquettes"] > 0, f"{context}.affected_plaquettes must be positive")


def main() -> None:
    loaded = {relative: load(relative) for relative in EXPECTED}
    validate_gauge_contract(loaded["contracts/YM-GAUGE-GROUP-V1.json"])
    validate_lattice_contract(loaded["contracts/YM-LATTICE-V1.json"])
    validate_evidence_contract(loaded["contracts/YM-EVIDENCE-V1.json"])
    validate_vectors(load("test-vectors/su2-wilson-v1.json"))

    claim_text = (ROOT / "docs/CLAIM_BOUNDARY.md").read_text(encoding="utf-8")
    require("finite-lattice" in claim_text.lower(), "claim boundary lost finite-lattice distinction")
    require("Clay" in claim_text, "claim boundary must retain Clay distinction")

    donor_text = (ROOT / "docs/DONOR_BOUNDARY.md").read_text(encoding="utf-8")
    require("engineering donor" in donor_text, "donor semantics drift")
    require("80941183d14531093117e122da0fc32c13d2464b" in donor_text, "GLUBALL donor SHA drift")

    print("Phase 0 contract checks: passed")


if __name__ == "__main__":
    main()
