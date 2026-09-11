#!/usr/bin/env python3
"""Negative regressions proving YANG-MILLS contract gates fail closed."""

from __future__ import annotations

import copy

from check_contracts import (
    EXPECTED_CONTRACTS,
    EXPECTED_PHASE0_VECTORS,
    EXPECTED_PHASE1A_VECTORS,
    parse_json_text,
    validate_exact,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative contract test failed: {label} was accepted")


def main() -> None:
    broken = copy.deepcopy(EXPECTED_CONTRACTS["contracts/YM-LATTICE-V1.json"])
    del broken["action"]
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-LATTICE-V1.json",
            payload,
            EXPECTED_CONTRACTS["contracts/YM-LATTICE-V1.json"],
        ),
        broken,
        "missing Phase 0 lattice action",
    )

    broken = copy.deepcopy(EXPECTED_CONTRACTS["contracts/YM-CLAIM-POLICY-V1.json"])
    broken["forbidden_equivalences"]["formal_finite_lattice_theorem_implies_clay_solution"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLAIM-POLICY-V1.json",
            payload,
            EXPECTED_CONTRACTS["contracts/YM-CLAIM-POLICY-V1.json"],
        ),
        broken,
        "weakened claim policy",
    )

    broken = copy.deepcopy(EXPECTED_CONTRACTS["contracts/YM-SU2-LATTICE-V1.json"])
    broken["reference_limits"]["max_links"] += 1
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-SU2-LATTICE-V1.json",
            payload,
            EXPECTED_CONTRACTS["contracts/YM-SU2-LATTICE-V1.json"],
        ),
        broken,
        "Phase 1A lattice limit drift",
    )

    broken = copy.deepcopy(EXPECTED_CONTRACTS["contracts/YM-WILSON-LOOP-V1.json"])
    broken["confinement_claimed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-WILSON-LOOP-V1.json",
            payload,
            EXPECTED_CONTRACTS["contracts/YM-WILSON-LOOP-V1.json"],
        ),
        broken,
        "premature confinement claim",
    )

    broken = copy.deepcopy(EXPECTED_PHASE0_VECTORS)
    broken["vectors"][1]["expected_action"] = 11.0
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su2-wilson-v1.json",
            payload,
            EXPECTED_PHASE0_VECTORS,
        ),
        broken,
        "Phase 0 expected-action drift",
    )

    broken = copy.deepcopy(EXPECTED_PHASE1A_VECTORS)
    broken["vectors"][0]["expected"] = 0.5
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su2-observables-v1.json",
            payload,
            EXPECTED_PHASE1A_VECTORS,
        ),
        broken,
        "identity Wilson-loop drift",
    )

    broken = copy.deepcopy(EXPECTED_PHASE1A_VECTORS)
    broken["vectors"][2]["observable"] = "confinement_order_parameter"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/su2-observables-v1.json",
            payload,
            EXPECTED_PHASE1A_VECTORS,
        ),
        broken,
        "Polyakov semantic drift",
    )

    duplicate_key_json = '{"dimension": 3, "dimension": 4}'
    expect_reject(
        lambda text: parse_json_text(text, "duplicate-key fixture"),
        duplicate_key_json,
        "duplicate JSON object key",
    )

    nonstandard_number_json = '{"beta": NaN}'
    expect_reject(
        lambda text: parse_json_text(text, "non-standard-number fixture"),
        nonstandard_number_json,
        "non-standard JSON numeric constant",
    )

    print("Negative contract regression checks: passed")


if __name__ == "__main__":
    main()
