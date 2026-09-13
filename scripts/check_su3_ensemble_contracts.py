#!/usr/bin/env python3
"""Fail-closed exact checks for the frozen Phase 2C1 SU(3) ensemble surface."""

from __future__ import annotations

import hashlib
import json
from typing import Any

from check_contracts import load

CONTRACT_PATH = "contracts/YM-SU3-ENSEMBLE-V1.json"
VECTOR_PATH = "test-vectors/su3-ensemble-v1.json"
CONTRACT_SHA256 = "493f243d9243c009a26915f42cb0172c6eb6771bd99e57629d2fb7aedfc8f173"
VECTOR_SHA256 = "52047cc6756a9a7bc65645a858015d787bfb2eea3ebc9304e4e423b4bc6e7982"


def canonical_sha256(payload: dict[str, Any]) -> str:
    encoded = json.dumps(
        payload,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def validate_su3_ensemble_contract(payload: dict[str, Any]) -> None:
    if canonical_sha256(payload) != CONTRACT_SHA256:
        raise ValueError(f"{CONTRACT_PATH}: exact contract drift")
    if payload["contract_id"] != "YM-SU3-ENSEMBLE-V1":
        raise ValueError(f"{CONTRACT_PATH}: contract ID drift")
    if payload["status"] != "phase-2c1-frozen":
        raise ValueError(f"{CONTRACT_PATH}: frozen status drift")
    if payload["reference_bounds"]["max_schedule_sweeps"] != 512:
        raise ValueError(f"{CONTRACT_PATH}: executable schedule bound drift")
    angle_grid = payload["proposal"]["angle_grid"]
    if angle_grid["sign_bit"] != 63 or angle_grid["magnitude_bits"] != 52:
        raise ValueError(f"{CONTRACT_PATH}: paired proposal grid drift")
    if angle_grid["endpoints_excluded"] is not True or angle_grid["equal_multiplicity"] is not True:
        raise ValueError(f"{CONTRACT_PATH}: proposal symmetry weakened")
    if payload["reproducibility"]["literal_golden_reference_chain_required"] is not True:
        raise ValueError(f"{CONTRACT_PATH}: literal golden chain requirement weakened")
    if payload["reproducibility"]["complete_reference_runtime_evidence_required"] is not True:
        raise ValueError(f"{CONTRACT_PATH}: complete runtime evidence requirement weakened")
    if payload["phase2c_gate"]["published_su3_baseline_reproduced"] is not False:
        raise ValueError(f"{CONTRACT_PATH}: premature SU(3) benchmark claim")
    if payload["phase2c_gate"]["glueball_analysis_enabled"] is not False:
        raise ValueError(f"{CONTRACT_PATH}: premature glueball enablement")
    if payload["phase2c_gate"]["phase2c_complete"] is not False:
        raise ValueError(f"{CONTRACT_PATH}: premature Phase 2C completion")


def validate_su3_ensemble_vectors(payload: dict[str, Any]) -> None:
    if canonical_sha256(payload) != VECTOR_SHA256:
        raise ValueError(f"{VECTOR_PATH}: exact vector drift")
    if payload["contract_id"] != "YM-SU3-ENSEMBLE-V1":
        raise ValueError(f"{VECTOR_PATH}: contract ID drift")
    if payload["vector_set"] != "SU3-ENSEMBLE-PHASE2C1-V1":
        raise ValueError(f"{VECTOR_PATH}: vector-set ID drift")
    if payload["status"] != "phase-2c1-frozen":
        raise ValueError(f"{VECTOR_PATH}: frozen status drift")
    by_id = {vector["id"]: vector for vector in payload["vectors"]}
    if by_id["paired-angle-grid"]["exact_inverse_pairing"] is not True:
        raise ValueError(f"{VECTOR_PATH}: exact inverse pairing weakened")
    if by_id["maximum-schedule-group-stress"]["maximum_total_sweeps"] != 512:
        raise ValueError(f"{VECTOR_PATH}: maximum schedule stress drift")
    golden = by_id["frozen-reference-chain-golden-evidence"]
    if golden["metropolis_proposals"] != 10240 or golden["metropolis_accepts"] != 10240:
        raise ValueError(f"{VECTOR_PATH}: golden proposal accounting drift")
    if len(golden["raw_measurements"]) != 16:
        raise ValueError(f"{VECTOR_PATH}: golden raw evidence length drift")


def main() -> None:
    validate_su3_ensemble_contract(load(CONTRACT_PATH))
    validate_su3_ensemble_vectors(load(VECTOR_PATH))
    print("Phase 2C1 SU(3) ensemble contract checks: passed")


if __name__ == "__main__":
    main()
