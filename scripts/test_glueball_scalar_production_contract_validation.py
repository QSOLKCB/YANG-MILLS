#!/usr/bin/env python3
"""Negative contract regressions for frozen Phase 3F production scalar analysis."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_production_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3F contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-PRODUCTION-V1.json")
    vectors = load("test-vectors/glueball-scalar-production-phase3f-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3f-candidate"
    expect_reject(validate_contract, broken, "frozen lifecycle rolled back to candidate")

    broken = copy.deepcopy(vectors)
    broken["status"] = "phase-3f-candidate"
    expect_reject(validate_vectors, broken, "frozen vector lifecycle rolled back to candidate")

    broken = copy.deepcopy(contract)
    del broken["change_discipline"]["freeze_classification"]
    expect_reject(validate_contract, broken, "freeze provenance classification removed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["master_seed_hex"] = "0x3f00202609150002"
    expect_reject(validate_contract, broken, "production stream changed after predeclaration")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["thermalization_sweeps"] = 32
    expect_reject(validate_contract, broken, "thermalization schedule changed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["independent_chain_count"] = 8
    expect_reject(validate_contract, broken, "independent chain count weakened")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["sequential_configurations_pooled_within_chain"] = True
    expect_reject(validate_contract, broken, "sequential within-chain configurations pooled")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["group_residual_violation_is_implementation_failure"] = False
    expect_reject(validate_contract, broken, "group residual failure downgraded to scientific outcome")

    broken = copy.deepcopy(contract)
    broken["fit"]["candidate_windows"] = [[2, 4]]
    expect_reject(validate_contract, broken, "fit windows changed after predeclaration")

    broken = copy.deepcopy(contract)
    broken["fit"]["computational_fit_failure_is_implementation_failure"] = False
    expect_reject(validate_contract, broken, "computational fit failure downgraded to scientific rejection")

    broken = copy.deepcopy(contract)
    broken["stability"]["maximum_pairwise_mass_z"] = 10.0
    expect_reject(validate_contract, broken, "stability threshold weakened")

    broken = copy.deepcopy(contract)
    broken["stability"]["all_successful_fit_pairs_must_be_scorable"] = False
    expect_reject(validate_contract, broken, "unscorable successful-fit pairs allowed")

    broken = copy.deepcopy(contract)
    broken["scientific_gate"]["scientific_gate_failure_causes_ci_failure"] = True
    expect_reject(validate_contract, broken, "scientific outcome conflated with implementation CI")

    broken = copy.deepcopy(contract)
    broken["change_discipline"]["failed_scientific_gate_must_be_preserved"] = False
    expect_reject(validate_contract, broken, "failed scientific result allowed to disappear")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["production_numerical_observation_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "production fit promoted to physical mass")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(validate_contract, broken, "continuum mass-gap overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["chain_count"] = 8
    expect_reject(validate_vectors, broken, "predeclared chain-count vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["maximum_pairwise_mass_z"] = 20.0
    expect_reject(validate_vectors, broken, "stability vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["physical_glueball_mass_claimed"] = True
    expect_reject(validate_vectors, broken, "vector physical mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "claim boundary boolean replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["scalar_pipeline_validated"] = True
    expect_reject(validate_vectors, broken, "observed negative receipt rewritten as validated")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["surface_outcomes"][0]["reason"] = "no-accepted-plateau-window"
    expect_reject(validate_vectors, broken, "observed surface rejection reason drifted")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["primary_window_successes"] = 1
    expect_reject(validate_vectors, broken, "observed window-success count drifted")

    print("Phase 3F production scalar negative contract regressions: passed")


if __name__ == "__main__":
    main()
