#!/usr/bin/env python3
"""Negative contract regressions for Phase 3G fresh-stream scalar recovery."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_recovery_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3G contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-RECOVERY-V1.json")
    vectors = load("test-vectors/glueball-scalar-recovery-phase3g-v1.json")

    broken = copy.deepcopy(contract)
    broken["depends_on"][-1]["required_status"] = "phase-3f-candidate"
    expect_reject(validate_contract, broken, "Phase 3G allowed to depend on unfrozen Phase 3F")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["master_seed_hex"] = broken["ensemble"]["phase3f_master_seed_hex"]
    expect_reject(validate_contract, broken, "Phase 3F master stream reused")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["independent_chain_count"] = 16
    expect_reject(validate_contract, broken, "independent-statistics recovery removed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["extents"] = [6, 6, 6, 10]
    expect_reject(validate_contract, broken, "geometry changed inside fixed-geometry recovery")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["thermalization_sweeps"] = 96
    expect_reject(validate_contract, broken, "sweep schedule changed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["proposal_angle"] = 0.25
    expect_reject(validate_contract, broken, "proposal angle retuned")

    broken = copy.deepcopy(contract)
    broken["analysis_inheritance"]["candidate_windows"] = [[2, 4]]
    expect_reject(validate_contract, broken, "fit windows retuned")

    broken = copy.deepcopy(contract)
    broken["analysis_inheritance"]["maximum_pairwise_mass_z"] = 10.0
    expect_reject(validate_contract, broken, "stability threshold weakened")

    broken = copy.deepcopy(contract)
    broken["analysis_inheritance"]["blocking_scan_defined"] = True
    expect_reject(validate_contract, broken, "blocking silently added without geometry change")

    broken = copy.deepcopy(contract)
    broken["scientific_gate"]["scientific_gate_failure_causes_ci_failure"] = True
    expect_reject(validate_contract, broken, "scientific failure conflated with implementation failure")

    broken = copy.deepcopy(contract)
    broken["decision_boundary"]["tensor_2pp_channel_enabled"] = True
    expect_reject(validate_contract, broken, "2++ opened before scalar recovery gate")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["recovery_confirmation_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "recovery promoted to physical mass")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["phase3f_scalar_pipeline_validated"] = True
    expect_reject(validate_vectors, broken, "Phase 3F negative baseline rewritten")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["master_seed_hex"] = "0x3f00202609150001"
    expect_reject(validate_vectors, broken, "fresh-stream vector reused Phase 3F seed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["chain_count"] = 16
    expect_reject(validate_vectors, broken, "doubled-statistics vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["candidate_windows"] = [[1, 3]]
    expect_reject(validate_vectors, broken, "analysis vector drifted")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "claim boundary boolean replaced by integer")

    print("Phase 3G scalar recovery negative contract regressions: passed")


if __name__ == "__main__":
    main()
