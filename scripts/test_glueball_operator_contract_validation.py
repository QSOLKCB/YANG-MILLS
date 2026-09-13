#!/usr/bin/env python3
"""Negative contract regressions for Phase 3A glueball operators."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_operator_contracts import (
    EXPECTED_CONTRACT,
    EXPECTED_VECTORS,
)


def expect_reject(actual: dict[str, Any], expected: dict[str, Any], label: str) -> None:
    from check_contracts import exact_typed_equal

    if exact_typed_equal(actual, expected):
        raise SystemExit(f"negative Phase 3A contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-OPERATORS-V1.json")
    vectors = load("test-vectors/glueball-operators-phase3a-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3a-frozen"
    expect_reject(broken, EXPECTED_CONTRACT, "candidate status silently promoted")

    broken = copy.deepcopy(contract)
    broken["geometry"]["euclidean_time_direction"] = 0
    expect_reject(broken, EXPECTED_CONTRACT, "time direction drift")

    broken = copy.deepcopy(contract)
    broken["basis"]["smearing"] = "APE"
    expect_reject(broken, EXPECTED_CONTRACT, "undeclared smearing injected")

    broken = copy.deepcopy(contract)
    broken["channel_semantics"]["continuum_spin_assignment_exact_at_finite_spacing"] = True
    expect_reject(broken, EXPECTED_CONTRACT, "finite lattice irrep promoted to exact continuum spin")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["effective_mass_extracted"] = True
    expect_reject(broken, EXPECTED_CONTRACT, "mass extraction claimed before correlator phase")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(broken, EXPECTED_CONTRACT, "continuum mass-gap overclaim")

    broken = copy.deepcopy(contract)
    broken["clay_solution_proved"] = True
    expect_reject(broken, EXPECTED_CONTRACT, "undeclared top-level overclaim field")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["connected_correlator_constructed"] = 0
    expect_reject(broken, EXPECTED_CONTRACT, "boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected"]["rectangle_loop_count"] = 161
    expect_reject(broken, EXPECTED_VECTORS, "rectangle orbit cardinality drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["expected_max"] = 1.0
    expect_reject(broken, EXPECTED_VECTORS, "gauge invariance tolerance weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][5]["effective_mass_extracted"] = True
    expect_reject(broken, EXPECTED_VECTORS, "vector mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][5]["clay_solution_claimed"] = 0
    expect_reject(broken, EXPECTED_VECTORS, "vector boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["clay_solution_proved"] = True
    expect_reject(broken, EXPECTED_VECTORS, "vector top-level overclaim field")

    print("Phase 3A glueball operator negative contract regressions: passed")


if __name__ == "__main__":
    main()
