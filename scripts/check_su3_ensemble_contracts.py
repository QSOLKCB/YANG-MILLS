#!/usr/bin/env python3
"""Fail-closed exact checks for the Phase 2C1 SU(3) ensemble surface."""

from __future__ import annotations

import hashlib
import json
from typing import Any

from check_contracts import load

CONTRACT_PATH = "contracts/YM-SU3-ENSEMBLE-V1.json"
VECTOR_PATH = "test-vectors/su3-ensemble-v1.json"
CONTRACT_SHA256 = "f0107ca34c7706495e4df34e5a5f0860d07093570639008a6bde994cdd003d66"
VECTOR_SHA256 = "b741ba27bd7740abf7654a5719fb86a0a33e4251238f70d5924eebfa113a9b07"


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
    if payload["status"] != "phase-2c1-candidate":
        raise ValueError(f"{CONTRACT_PATH}: candidate status drift")
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


def main() -> None:
    validate_su3_ensemble_contract(load(CONTRACT_PATH))
    validate_su3_ensemble_vectors(load(VECTOR_PATH))
    print("Phase 2C1 SU(3) ensemble contract checks: passed")


if __name__ == "__main__":
    main()
