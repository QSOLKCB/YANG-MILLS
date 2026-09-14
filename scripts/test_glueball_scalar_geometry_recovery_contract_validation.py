#!/usr/bin/env python3
"""Negative contract regressions for Phase 3H blocking-capable geometry recovery."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_geometry_recovery_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3H contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1.json")
    vectors = load("test-vectors/glueball-scalar-geometry-recovery-phase3h-v1.json")

    broken = copy.deepcopy(contract)
    broken["depends_on"][-1]["required_status"] = "phase-3g-candidate"
    expect_reject(validate_contract, broken, "Phase 3H allowed to depend on unfrozen Phase 3G")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["master_seed_hex"] = broken["ensemble"]["phase3g_master_seed_hex"]
    expect_reject(validate_contract, broken, "Phase 3G master stream reused")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["extents"] = [4, 4, 4, 10]
    expect_reject(validate_contract, broken, "blocking-capable geometry removed")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["independent_chain_count"] = 16
    expect_reject(validate_contract, broken, "chain count changed while testing geometry")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["expected_proposals_per_chain"] = 1392640
    expect_reject(validate_contract, broken, "proposal accounting retained from old geometry")

    broken = copy.deepcopy(contract)
    broken["ensemble"]["proposal_angle"] = 0.25
    expect_reject(validate_contract, broken, "proposal angle retuned")

    broken = copy.deepcopy(contract)
    broken["analysis"]["transformations"] = broken["analysis"]["transformations"][:-1]
    expect_reject(validate_contract, broken, "blocking surface removed")

    broken = copy.deepcopy(contract)
    broken["analysis"]["primary_surface"] = {"transformation": "stout-2", "operator": "plaquette-1x1"}
    expect_reject(validate_contract, broken, "predeclared primary blocked surface changed")

    broken = copy.deepcopy(contract)
    broken["analysis"]["candidate_windows"] = [[2, 4]]
    expect_reject(validate_contract, broken, "fit windows retuned")

    broken = copy.deepcopy(contract)
    broken["analysis"]["maximum_pairwise_mass_z"] = 10.0
    expect_reject(validate_contract, broken, "stability threshold weakened")

    broken = copy.deepcopy(contract)
    broken["change_discipline"]["no_outcome_by_construction"] = False
    expect_reject(validate_contract, broken, "scientific-integrity invariant weakened")

    broken = copy.deepcopy(contract)
    broken["scientific_gate"]["scientific_gate_failure_causes_ci_failure"] = True
    expect_reject(validate_contract, broken, "scientific failure conflated with implementation failure")

    broken = copy.deepcopy(contract)
    broken["decision_boundary"]["tensor_2pp_channel_enabled"] = True
    expect_reject(validate_contract, broken, "2++ opened before scalar gate")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["geometry_recovery_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "geometry recovery promoted to physical mass")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["phase3g_scalar_pipeline_validated"] = True
    expect_reject(validate_vectors, broken, "Phase 3G negative baseline rewritten")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["master_seed_hex"] = "0x3f01202609150001"
    expect_reject(validate_vectors, broken, "fresh stream reused Phase 3G seed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["extents"] = [4, 4, 4, 10]
    expect_reject(validate_vectors, broken, "geometry recovery vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["transformations"] = ["raw", "stout-1", "stout-2"]
    expect_reject(validate_vectors, broken, "blocking vector removed")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["no_outcome_by_construction"] = False
    expect_reject(validate_vectors, broken, "integrity boundary weakened")

    print("Phase 3H scalar geometry recovery negative contract regressions: passed")


if __name__ == "__main__":
    main()
