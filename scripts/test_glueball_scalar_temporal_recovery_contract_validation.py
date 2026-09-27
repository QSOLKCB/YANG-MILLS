#!/usr/bin/env python3
"""Negative contract regressions for Phase 3I temporal-extent recovery."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_temporal_recovery_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3I contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1.json")
    vectors = load("test-vectors/glueball-scalar-temporal-recovery-phase3i-v1.json")

    broken = copy.deepcopy(contract)
    broken["depends_on"][-1]["required_status"] = "phase-3h-candidate"
    expect_reject(validate_contract, broken, "Phase 3I allowed to depend on unfrozen Phase 3H")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["master_seed_hex"] = broken["ensemble"]["phase3h_master_seed_hex"]
    expect_reject(validate_contract, broken, "Phase 3H master stream reused")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["extents"] = [6, 6, 6, 10]
    expect_reject(validate_contract, broken, "temporal extent recovery removed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["extents"] = [8, 6, 6, 16]
    expect_reject(validate_contract, broken, "spatial geometry changed under temporal-only authority")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["independent_chain_count"] = 64
    expect_reject(validate_contract, broken, "chain count escalated")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["expected_proposals_per_chain"] = 4700160
    expect_reject(validate_contract, broken, "Phase 3H proposal accounting retained")

    broken = copy.deepcopy(contract)
    broken["analysis"]["candidate_windows"] = [[2, 5]]
    expect_reject(validate_contract, broken, "fit windows retuned")

    broken = copy.deepcopy(contract)
    broken["analysis"]["maximum_pairwise_mass_z"] = 10.0
    expect_reject(validate_contract, broken, "stability threshold weakened")

    broken = copy.deepcopy(contract)
    broken["change_discipline"]["only_temporal_extent_changes"] = False
    expect_reject(validate_contract, broken, "single-variable intervention weakened")

    broken = copy.deepcopy(contract)
    broken["scientific_gate"]["scientific_gate_failure_causes_ci_failure"] = True
    expect_reject(validate_contract, broken, "scientific failure conflated with implementation failure")

    broken = copy.deepcopy(contract)
    broken["decision_boundary"]["tensor_2pp_channel_enabled"] = True
    expect_reject(validate_contract, broken, "2++ opened before scalar confirmation")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["temporal_recovery_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "temporal recovery promoted to physical mass")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["phase3h_scalar_pipeline_validated"] = True
    expect_reject(validate_vectors, broken, "Phase 3H negative baseline rewritten")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["master_seed_hex"] = "0x3f02202609150001"
    expect_reject(validate_vectors, broken, "fresh stream reused Phase 3H seed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["extents"] = [6, 6, 6, 10]
    expect_reject(validate_vectors, broken, "temporal recovery vector removed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["candidate_windows"] = [[1, 2]]
    expect_reject(validate_vectors, broken, "analysis windows changed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["only_temporal_extent_changes"] = False
    expect_reject(validate_vectors, broken, "integrity boundary weakened")

    print("Phase 3I scalar temporal recovery negative contract regressions: passed")


if __name__ == "__main__":
    main()
