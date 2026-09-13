#!/usr/bin/env python3
"""Negative regressions for Phase 2C2 V1/V2 published SU(3) semantics."""

from __future__ import annotations

import copy
from collections.abc import Callable
from typing import Any

from check_contracts import load
from check_su3_baseline_contracts import (
    V1_CONTRACT_PATH,
    V1_VECTOR_PATH,
    V2_CONTRACT_PATH,
    V2_VECTOR_PATH,
    validate_v1_contract,
    validate_v1_vectors,
    validate_v2_contract,
    validate_v2_vectors,
)


def expect_reject(callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 2C2 contract test failed: {label} was accepted")


def main() -> None:
    v1 = load(V1_CONTRACT_PATH)
    v1_vectors = load(V1_VECTOR_PATH)
    v2 = load(V2_CONTRACT_PATH)
    v2_vectors = load(V2_VECTOR_PATH)

    broken = copy.deepcopy(v1)
    broken["status"] = "phase-2c2-frozen"
    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["minimum_effective_sample_size_observed"] = 8.1
    expect_reject(validate_v1_contract, broken, "V1 failed ESS evidence rewritten")

    broken = copy.deepcopy(v1)
    broken["phase2c_gate"]["glueball_analysis_enabled"] = True
    expect_reject(validate_v1_contract, broken, "V1 failure enabled glueball analysis")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["v1_reclassified_as_pass"] = True
    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")

    broken = copy.deepcopy(v2)
    broken["benchmark"]["source_envelope_max"] = 0.75
    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["master_seed"] = "2c02198105010001"
    expect_reject(validate_v2_contract, broken, "V2 reused V1 master seed")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["minimum_aggregate_effective_sample_size"] = 8.0
    expect_reject(validate_v2_contract, broken, "V2 aggregate ESS target weakened")

    broken = copy.deepcopy(v2)
    broken["predeclaration"]["proposal_angle_changed_after_v1"] = True
    expect_reject(validate_v2_contract, broken, "V2 proposal retuning admitted")

    broken = copy.deepcopy(v2)
    broken["phase2c_gate"]["published_su3_baseline_reproduced"] = True
    expect_reject(validate_v2_contract, broken, "V2 benchmark claimed before run")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["chain_ids"] = [0, 1, 2, 2]
    expect_reject(validate_v2_vectors, broken, "V2 vector independent streams collapsed")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][2]["v1_reclassified_as_pass"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector rewrote V1 failure")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["phase2c_complete"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector completed prematurely")

    print("Negative Phase 2C2 SU(3) V1/V2 contract regressions: passed")


if __name__ == "__main__":
    main()
