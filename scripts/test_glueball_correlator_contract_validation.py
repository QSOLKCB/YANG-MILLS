#!/usr/bin/env python3
"""Negative contract regressions for Phase 3C connected correlators."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_correlator_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3C contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-CONNECTED-CORRELATOR-V1.json")
    vectors = load("test-vectors/glueball-connected-correlator-phase3c-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3c-candidate"
    expect_reject(validate_contract, broken, "frozen status regressed to candidate")

    broken = copy.deepcopy(contract)
    broken["depends_on"][2]["required_status"] = "phase-3b2-candidate"
    expect_reject(validate_contract, broken, "blocking dependency freeze weakened")

    broken = copy.deepcopy(contract)
    broken["bounds"]["maximum_configurations"] = 640
    expect_reject(validate_contract, broken, "configuration bound weakened")

    broken = copy.deepcopy(contract)
    broken["estimator"]["vacuum_subtraction_uses_same_samples"] = False
    expect_reject(validate_contract, broken, "vacuum subtraction sample identity changed")

    broken = copy.deepcopy(contract)
    broken["estimator"]["connected_two_point"] = "C_O(tau)=G_O(tau)"
    expect_reject(validate_contract, broken, "vacuum subtraction removed")

    broken = copy.deepcopy(contract)
    broken["estimator"]["time_reflection_symmetrization_applied"] = True
    expect_reject(validate_contract, broken, "undeclared time symmetrization added")

    broken = copy.deepcopy(contract)
    broken["estimator"]["uncertainty_estimated"] = True
    expect_reject(validate_contract, broken, "uncertainty claimed before next phase")

    broken = copy.deepcopy(contract)
    broken["canonical_fixture"]["plaquette_connected"][0] = 0.0
    expect_reject(validate_contract, broken, "analytic correlator drift")

    broken = copy.deepcopy(contract)
    broken["method_origin"]["canonical_fixture_is_physical_ensemble"] = True
    expect_reject(validate_contract, broken, "deterministic fixture promoted to physical ensemble")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["effective_mass_extracted"] = True
    expect_reject(validate_contract, broken, "effective mass claimed inside frozen Phase 3C")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(validate_contract, broken, "continuum mass-gap overclaim")

    broken = copy.deepcopy(contract)
    broken["clay_solution_proved"] = True
    expect_reject(validate_contract, broken, "undeclared top-level overclaim field")

    broken = copy.deepcopy(vectors)
    broken["status"] = "phase-3c-candidate"
    expect_reject(validate_vectors, broken, "vector-set frozen status regressed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["expected"]["rectangle_connected"][1] = 0.0
    expect_reject(validate_vectors, broken, "analytic vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["expected_max"] = 1.0
    expect_reject(validate_vectors, broken, "gauge-invariance tolerance weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][7]["canonical_fixture_is_physical_ensemble"] = True
    expect_reject(validate_vectors, broken, "fixture physicality overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][7]["effective_mass_extracted"] = True
    expect_reject(validate_vectors, broken, "vector effective-mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][7]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "vector boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["clay_solution_proved"] = True
    expect_reject(validate_vectors, broken, "vector top-level overclaim field")

    print("Phase 3C connected-correlator negative contract regressions: passed")


if __name__ == "__main__":
    main()
