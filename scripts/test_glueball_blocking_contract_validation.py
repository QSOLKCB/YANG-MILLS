#!/usr/bin/env python3
"""Negative contract regressions for Phase 3B2 spatial blocking."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_blocking_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3B2 contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SPATIAL-BLOCK-V1.json")
    vectors = load("test-vectors/glueball-spatial-block-phase3b2-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3b2-frozen"
    expect_reject(validate_contract, broken, "candidate status silently promoted")

    broken = copy.deepcopy(contract)
    broken["geometry"]["block_factor"] = 3
    expect_reject(validate_contract, broken, "block factor drift")

    broken = copy.deepcopy(contract)
    broken["geometry"]["fine_spatial_extents_even"] = False
    expect_reject(validate_contract, broken, "even spatial admission weakened")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["spatial_link_rule"] = "V_mu(y)=U_mu(x+mu) U_mu(x)"
    expect_reject(validate_contract, broken, "spatial product order reversed")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["temporal_link_rule"] = "multiply two temporal links"
    expect_reject(validate_contract, broken, "temporal sampling semantics changed")

    broken = copy.deepcopy(contract)
    broken["algorithm"]["projection_step_used"] = True
    expect_reject(validate_contract, broken, "undeclared projection injected")

    broken = copy.deepcopy(contract)
    broken["validation"]["frozen_coarse_plaquette_1x1"] = 0.0
    expect_reject(validate_contract, broken, "frozen coarse output drift")

    broken = copy.deepcopy(contract)
    broken["composition_boundary"]["combined_smear_block_tuning_validated"] = True
    expect_reject(validate_contract, broken, "undeclared combined tuning claim")

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
    broken["vectors"][1]["expected"]["spatial_plaquette_1x1"] = 0.0
    expect_reject(validate_vectors, broken, "literal blocked operator output weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["expected_max"] = 1.0
    expect_reject(validate_vectors, broken, "gauge covariance tolerance weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["unitarity_residual_max"] = 1.0
    expect_reject(validate_vectors, broken, "group residual gate weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["effective_mass_extracted"] = True
    expect_reject(validate_vectors, broken, "vector mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "vector boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["clay_solution_proved"] = True
    expect_reject(validate_vectors, broken, "vector top-level overclaim field")

    print("Phase 3B2 spatial blocking negative contract regressions: passed")


if __name__ == "__main__":
    main()
