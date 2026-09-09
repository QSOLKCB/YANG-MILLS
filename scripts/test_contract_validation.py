#!/usr/bin/env python3
"""Regression tests proving malformed Phase 0 contracts are rejected."""

from __future__ import annotations

import copy

from check_contracts import (
    load,
    parse_json_text,
    validate_evidence_contract,
    validate_gauge_contract,
    validate_lattice_contract,
    validate_vectors,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative contract test failed: {label} was accepted")


def main() -> None:
    gauge = load("contracts/YM-GAUGE-GROUP-V1.json")
    lattice = load("contracts/YM-LATTICE-V1.json")
    evidence = load("contracts/YM-EVIDENCE-V1.json")
    vectors = load("test-vectors/su2-wilson-v1.json")

    validate_gauge_contract(gauge)
    validate_lattice_contract(lattice)
    validate_evidence_contract(evidence)
    validate_vectors(vectors)

    broken = copy.deepcopy(lattice)
    del broken["action"]
    expect_reject(validate_lattice_contract, broken, "missing lattice action")

    broken = copy.deepcopy(lattice)
    broken["action"]["formula"] = 17
    expect_reject(validate_lattice_contract, broken, "non-string Wilson formula")

    broken = copy.deepcopy(lattice)
    del broken["numerics"]["scalar_type"]
    expect_reject(validate_lattice_contract, broken, "incomplete lattice numerics")

    broken = copy.deepcopy(gauge)
    del broken["representation"]["dagger"]
    expect_reject(validate_gauge_contract, broken, "incomplete gauge representation")

    broken = copy.deepcopy(evidence)
    broken["required_identity_classes"] = broken["required_identity_classes"][:-1]
    expect_reject(validate_evidence_contract, broken, "incomplete evidence identity classes")

    broken = copy.deepcopy(vectors)
    del broken["vectors"][0]["expected_action"]
    expect_reject(validate_vectors, broken, "incomplete regression vector")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["beta"] = float("nan")
    expect_reject(validate_vectors, broken, "NaN regression beta")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected_action"] = float("inf")
    expect_reject(validate_vectors, broken, "infinite expected action")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["tolerance"] = float("-inf")
    expect_reject(validate_vectors, broken, "non-finite tolerance")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["beta"] = -1.0
    expect_reject(validate_vectors, broken, "negative regression beta")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["tolerance"] = -1.0
    expect_reject(validate_vectors, broken, "negative regression tolerance")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected_action"] = 1.0
    expect_reject(validate_vectors, broken, "identity expected-action semantic drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["expected_action"] = 11.0
    expect_reject(validate_vectors, broken, "single-link expected-action semantic drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["configuration"] = "unrelated configuration text"
    expect_reject(validate_vectors, broken, "single-link configuration semantic drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["affected_plaquettes"] = 5
    expect_reject(validate_vectors, broken, "single-link affected-plaquette drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["id"] = broken["vectors"][0]["id"]
    expect_reject(validate_vectors, broken, "duplicate Phase 0 vector id")

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
