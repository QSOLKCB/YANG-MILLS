#!/usr/bin/env python3
"""Negative regressions for Phase 2C2 published SU(3) benchmark semantics."""

from __future__ import annotations

import copy
from collections.abc import Callable
from typing import Any

from check_contracts import load
from check_su3_baseline_contracts import (
    CONTRACT_PATH,
    VECTOR_PATH,
    validate_su3_baseline_contract,
    validate_su3_baseline_vectors,
)


def expect_reject(
    callback: Callable[[dict[str, Any]], None],
    payload: dict[str, Any],
    label: str,
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 2C2 contract test failed: {label} was accepted")


def main() -> None:
    contract = load(CONTRACT_PATH)
    vectors = load(VECTOR_PATH)
    contract_cases: list[tuple[str, dict[str, Any]]] = []

    broken = copy.deepcopy(contract)
    broken["primary_source"]["peer_reviewed"] = False
    contract_cases.append(("peer-reviewed source weakened", broken))

    broken = copy.deepcopy(contract)
    broken["benchmark"]["extents"] = [6, 6, 6, 6]
    contract_cases.append(("benchmark volume drift", broken))

    broken = copy.deepcopy(contract)
    broken["benchmark"]["source_envelope_max"] = 0.75
    contract_cases.append(("source envelope weakened", broken))

    broken = copy.deepcopy(contract)
    broken["benchmark"]["source_envelope_is_paper_error_bar"] = True
    contract_cases.append(("graphical envelope misrepresented", broken))

    broken = copy.deepcopy(contract)
    broken["reference_run"]["chain_ids"] = [0, 0]
    contract_cases.append(("independent chain identity collapsed", broken))

    broken = copy.deepcopy(contract)
    broken["reference_run"]["total_sweeps_per_chain"] = 513
    contract_cases.append(("frozen Phase 2C1 schedule escaped", broken))

    broken = copy.deepcopy(contract)
    broken["comparison_rule"]["within_chain_stationarity_required"] = False
    contract_cases.append(("stationarity gate removed", broken))

    broken = copy.deepcopy(contract)
    broken["predeclaration"]["benchmark_parameters_frozen_before_reference_result_inspection"] = False
    contract_cases.append(("predeclaration weakened", broken))

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["published_su3_baseline_reproduced"] = True
    contract_cases.append(("benchmark claimed before executable evidence", broken))

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["glueball_analysis_enabled"] = True
    contract_cases.append(("glueball enabled before benchmark evidence", broken))

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["benchmark_agreement_implies_positive_mass_gap"] = True
    contract_cases.append(("mass-gap overclaim", broken))

    for label, payload in contract_cases:
        expect_reject(validate_su3_baseline_contract, payload, label)

    broken_vectors = copy.deepcopy(vectors)
    broken_vectors["vectors"][0]["expected_interval"] = [0.4, 0.8]
    expect_reject(
        validate_su3_baseline_vectors,
        broken_vectors,
        "source vector envelope weakened",
    )

    broken_vectors = copy.deepcopy(vectors)
    broken_vectors["vectors"][1]["chain_ids"] = [0, 0]
    expect_reject(
        validate_su3_baseline_vectors,
        broken_vectors,
        "vector chain identity collapsed",
    )

    broken_vectors = copy.deepcopy(vectors)
    broken_vectors["vectors"][2]["constant_half_rejected"] = False
    expect_reject(
        validate_su3_baseline_vectors,
        broken_vectors,
        "constant-series refusal removed",
    )

    broken_vectors = copy.deepcopy(vectors)
    broken_vectors["vectors"][3]["phase2c_complete"] = True
    expect_reject(
        validate_su3_baseline_vectors,
        broken_vectors,
        "Phase 2C vector completed prematurely",
    )

    print("Negative Phase 2C2 SU(3) published baseline contract regressions: passed")


if __name__ == "__main__":
    main()
