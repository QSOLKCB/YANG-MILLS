#!/usr/bin/env python3
"""Negative regressions for Phase 1B SU(3) contract semantics."""

from __future__ import annotations

import copy

from check_contracts import validate_exact
from check_su3_contracts import (
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

    print("Negative Phase 1B SU(3) contract regression checks: passed")


if __name__ == "__main__":
    main()
