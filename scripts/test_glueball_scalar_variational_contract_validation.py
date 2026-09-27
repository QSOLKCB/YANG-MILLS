#!/usr/bin/env python3
"""Negative contract regressions for Phase 3J scalar variational mechanics."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_variational_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3J contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-VARIATIONAL-BASIS-V1.json")
    vectors = load("test-vectors/glueball-scalar-variational-phase3j-v1.json")

    broken = copy.deepcopy(contract)
    broken["lifecycle"]["phase3i_outcome_assumed"] = True
    expect_reject(validate_contract, broken, "unobserved Phase 3I outcome assumed")

    broken = copy.deepcopy(contract)
    broken["lifecycle"]["production_recovery_activated"] = True
    expect_reject(validate_contract, broken, "production recovery activated")

    broken = copy.deepcopy(contract)
    broken["basis"]["dimension"] = 1
    expect_reject(validate_contract, broken, "variational basis collapsed")

    broken = copy.deepcopy(contract)
    broken["correlator_matrix"]["diagonal_entries_must_replay_phase3c"] = False
    expect_reject(validate_contract, broken, "Phase 3C diagonal replay removed")

    broken = copy.deepcopy(contract)
    broken["correlator_matrix"]["matrix_symmetry_tolerance"] = 1e-4
    expect_reject(validate_contract, broken, "matrix symmetry tolerance weakened")

    broken = copy.deepcopy(contract)
    broken["generalized_eigenproblem"]["reference_time"] = 2
    expect_reject(validate_contract, broken, "reference time changed")

    broken = copy.deepcopy(contract)
    broken["generalized_eigenproblem"]["reference_matrix_must_be_positive_definite"] = False
    expect_reject(validate_contract, broken, "positive-definite gate removed")

    broken = copy.deepcopy(contract)
    broken["generalized_eigenproblem"]["normalization"] = "arbitrary"
    expect_reject(validate_contract, broken, "GEVP normalization changed")

    broken = copy.deepcopy(contract)
    broken["invariants"]["basis_rescaling_invariance_checked"] = False
    expect_reject(validate_contract, broken, "basis rescaling invariant removed")

    broken = copy.deepcopy(contract)
    broken["decision_boundary"]["tensor_2pp_channel_enabled"] = True
    expect_reject(validate_contract, broken, "2++ opened")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["variational_fixture_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "fixture promoted to physical mass")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["expected_principal_eigenvalue"] = 1.0
    expect_reject(validate_vectors, broken, "canonical eigenvalue changed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["reference_matrix_positive_definite_required"] = False
    expect_reject(validate_vectors, broken, "vector positive-definite gate removed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["production_recovery_activated"] = True
    expect_reject(validate_vectors, broken, "activation boundary weakened")

    print("Phase 3J scalar variational mechanics negative contract regressions: passed")


if __name__ == "__main__":
    main()
