#!/usr/bin/env python3
"""Negative regressions for SU(3) algebra, lattice, and center contract semantics."""

from __future__ import annotations

import copy

from check_contracts import validate_exact
from check_su3_contracts import (
    EXPECTED_SU3_CENTER_CONTRACT,
    EXPECTED_SU3_CENTER_VECTORS,
    EXPECTED_SU3_CONTRACT,
    EXPECTED_SU3_LATTICE_CONTRACT,
    EXPECTED_SU3_LATTICE_VECTORS,
    EXPECTED_SU3_VECTORS,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative SU(3) contract test failed: {label} was accepted")


def main() -> None:
    broken = copy.deepcopy(EXPECTED_SU3_CONTRACT)
    broken["representation"]["generator_count"] = 7
    expect_reject(
        lambda payload: validate_exact("contracts/YM-SU3-ALGEBRA-V1.json", payload, EXPECTED_SU3_CONTRACT),
        broken,
        "generator-count drift",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CONTRACT)
    broken["exponentiation"]["projection_to_su3_used"] = True
    expect_reject(
        lambda payload: validate_exact("contracts/YM-SU3-ALGEBRA-V1.json", payload, EXPECTED_SU3_CONTRACT),
        broken,
        "hidden SU(3) projection",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CONTRACT)
    broken["exponentiation"]["determinant_residual"] = "abs(det(U)-1)"
    expect_reject(
        lambda payload: validate_exact("contracts/YM-SU3-ALGEBRA-V1.json", payload, EXPECTED_SU3_CONTRACT),
        broken,
        "one-sided determinant residual",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CONTRACT)
    broken["lattice_links_enabled"] = 0
    expect_reject(
        lambda payload: validate_exact("contracts/YM-SU3-ALGEBRA-V1.json", payload, EXPECTED_SU3_CONTRACT),
        broken,
        "boolean algebra lattice-link boundary replaced by numeric zero",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CONTRACT)
    broken["cross_domain_semantic_ontology_imported"] = True
    expect_reject(
        lambda payload: validate_exact("contracts/YM-SU3-ALGEBRA-V1.json", payload, EXPECTED_SU3_CONTRACT),
        broken,
        "cross-domain ontology import",
    )

    broken = copy.deepcopy(EXPECTED_SU3_VECTORS)
    broken["vectors"][1]["expected"] = True
    expect_reject(
        lambda payload: validate_exact("test-vectors/su3-algebra-v1.json", payload, EXPECTED_SU3_VECTORS),
        broken,
        "boolean replacing numeric f123",
    )

    broken = copy.deepcopy(EXPECTED_SU3_VECTORS)
    broken["vectors"][6]["expected_multiple_of_identity"] = 1.0
    expect_reject(
        lambda payload: validate_exact("test-vectors/su3-algebra-v1.json", payload, EXPECTED_SU3_VECTORS),
        broken,
        "cubic-Casimir drift",
    )

    broken = copy.deepcopy(EXPECTED_SU3_VECTORS)
    broken["vectors"][7]["unitarity_tolerance"] = 5e-6
    expect_reject(
        lambda payload: validate_exact("test-vectors/su3-algebra-v1.json", payload, EXPECTED_SU3_VECTORS),
        broken,
        "weakened exponentiation acceptance tolerance",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_CONTRACT)
    broken["depends_on"]["required_status"] = "phase-1b-review-candidate"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-LATTICE-V1.json", payload, EXPECTED_SU3_LATTICE_CONTRACT
        ),
        broken,
        "weakened frozen algebra dependency",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_CONTRACT)
    broken["group_acceptance"]["projection_to_su3_used"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-LATTICE-V1.json", payload, EXPECTED_SU3_LATTICE_CONTRACT
        ),
        broken,
        "hidden lattice group projection",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_CONTRACT)
    broken["fixture_boundary"]["center_symmetry_interpretation_claimed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-LATTICE-V1.json", payload, EXPECTED_SU3_LATTICE_CONTRACT
        ),
        broken,
        "premature center-symmetry interpretation",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_CONTRACT)
    broken["confinement_claimed"] = 0
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-LATTICE-V1.json", payload, EXPECTED_SU3_LATTICE_CONTRACT
        ),
        broken,
        "boolean confinement boundary replaced by numeric zero",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_VECTORS)
    broken["vectors"][1]["expected"] = 11.0
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su3-lattice-v1.json", payload, EXPECTED_SU3_LATTICE_VECTORS
        ),
        broken,
        "SU(3) single-link analytic-action drift",
    )

    broken = copy.deepcopy(EXPECTED_SU3_LATTICE_VECTORS)
    broken["vectors"][2]["claim_class"] = "C1"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su3-lattice-v1.json", payload, EXPECTED_SU3_LATTICE_VECTORS
        ),
        broken,
        "gauge-invariance numerical vector overclassification",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_CONTRACT)
    broken["abstract_group"]["order"] = 4
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-CENTER-V1.json", payload, EXPECTED_SU3_CENTER_CONTRACT
        ),
        broken,
        "abstract C3 order drift",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_CONTRACT)
    broken["cycle_embedding"]["central"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-CENTER-V1.json", payload, EXPECTED_SU3_CENTER_CONTRACT
        ),
        broken,
        "non-central cycle embedding promoted to center",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_CONTRACT)
    broken["center_embedding"]["same_as_cycle_embedding"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-CENTER-V1.json", payload, EXPECTED_SU3_CENTER_CONTRACT
        ),
        broken,
        "distinct subgroup embeddings collapsed",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_CONTRACT)
    broken["polyakov_center_transform"]["confinement_claimed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-CENTER-V1.json", payload, EXPECTED_SU3_CENTER_CONTRACT
        ),
        broken,
        "premature confinement claim",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_CONTRACT)
    broken["polyakov_center_transform"]["order_parameter_inference_claimed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU3-CENTER-V1.json", payload, EXPECTED_SU3_CENTER_CONTRACT
        ),
        broken,
        "premature Polyakov order-parameter inference",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_VECTORS)
    broken["vectors"][2]["expected"] = 0.0
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su3-center-v1.json", payload, EXPECTED_SU3_CENTER_VECTORS
        ),
        broken,
        "Omega non-central witness erased",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_VECTORS)
    broken["vectors"][5]["claim_class"] = "C1"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su3-center-v1.json", payload, EXPECTED_SU3_CENTER_VECTORS
        ),
        broken,
        "Polyakov center-transform numerical vector overclassification",
    )

    broken = copy.deepcopy(EXPECTED_SU3_CENTER_VECTORS)
    broken["vectors"][5]["polyakov_after"] = [1.0, 0.0]
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su3-center-v1.json", payload, EXPECTED_SU3_CENTER_VECTORS
        ),
        broken,
        "Polyakov center rotation drift",
    )

    print("Negative SU(3) algebra, lattice, and center contract regression checks: passed")


if __name__ == "__main__":
    main()
