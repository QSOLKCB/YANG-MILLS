#!/usr/bin/env python3
"""Negative contract regressions for Phase 3B1 spatial stout smearing."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_smearing_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3B1 contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SPATIAL-STOUT-V1.json")
    vectors = load("test-vectors/glueball-spatial-stout-phase3b1-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3b1-candidate"
    expect_reject(validate_contract, broken, "frozen status reopened")

    broken = copy.deepcopy(contract)
    broken["geometry"]["temporal_links_modified"] = True
    expect_reject(validate_contract, broken, "temporal-link mutation admitted")

    broken = copy.deepcopy(contract)
    broken["geometry"]["spatial_extents_equal"] = False
    expect_reject(validate_contract, broken, "cubic spatial admission weakened")

    broken = copy.deepcopy(contract)
    broken["parameters"]["maximum_rho"] = 2.5
    expect_reject(validate_contract, broken, "rho bound weakened")

    broken = copy.deepcopy(contract)
    broken["parameters"]["maximum_levels"] = 80
    expect_reject(validate_contract, broken, "level bound weakened")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["level_semantics"] = "in-place Gauss-Seidel update within one level"
    expect_reject(validate_contract, broken, "synchronous update semantics changed")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["blocking_step_used"] = True
    expect_reject(validate_contract, broken, "undeclared blocking injected")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["lattice_copy_allocation_failure_rule"] = "abort on allocation failure"
    expect_reject(validate_contract, broken, "allocation failure propagation weakened")

    broken = copy.deepcopy(contract)
    broken["validation"]["frozen_spatial_change"] = 0.0
    expect_reject(validate_contract, broken, "frozen stout output drift")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["effective_mass_extracted"] = True
    expect_reject(validate_contract, broken, "mass extraction claimed before correlator phase")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(validate_contract, broken, "continuum mass-gap overclaim")

    broken = copy.deepcopy(contract)
    broken["clay_solution_proved"] = True
    expect_reject(validate_contract, broken, "undeclared top-level overclaim field")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["connected_correlator_constructed"] = 0
    expect_reject(validate_contract, broken, "boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["status"] = "phase-3b1-candidate"
    expect_reject(validate_vectors, broken, "vector frozen status reopened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][5]["expected"]["spatial_plaquette_1x1"] = 0.0
    expect_reject(validate_vectors, broken, "literal stout output weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["expected_max"] = 1.0
    expect_reject(validate_vectors, broken, "gauge covariance tolerance weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["unitarity_residual_max"] = 1.0
    expect_reject(validate_vectors, broken, "group residual gate weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][9]["effective_mass_extracted"] = True
    expect_reject(validate_vectors, broken, "vector mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][9]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "vector boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["clay_solution_proved"] = True
    expect_reject(validate_vectors, broken, "vector top-level overclaim field")

    print("Phase 3B1 spatial stout-smearing negative contract regressions: passed")


if __name__ == "__main__":
    main()
