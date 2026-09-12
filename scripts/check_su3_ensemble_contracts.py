#!/usr/bin/env python3
"""Fail-closed exact checks for the frozen Phase 2C1 SU(3) ensemble surface."""

from __future__ import annotations

import hashlib
import json
from typing import Any

from check_contracts import load

CONTRACT_PATH = "contracts/YM-SU3-ENSEMBLE-V1.json"
VECTOR_PATH = "test-vectors/su3-ensemble-v1.json"
CONTRACT_SHA256 = "77656622eb8452268fc21ffd13836e3e1704ce0ae5655ad96cf9e5c613af7622"
VECTOR_SHA256 = "9c41d1073de078c04979562b56d28c18d695159f9645e2eab0618a9f7b18ddd6"


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


def main() -> None:
    validate_su3_ensemble_contract(load(CONTRACT_PATH))
    validate_su3_ensemble_vectors(load(VECTOR_PATH))
    print("Phase 2C1 SU(3) ensemble contract checks: passed")


if __name__ == "__main__":
    main()
