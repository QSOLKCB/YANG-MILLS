#!/usr/bin/env python3
"""Negative regressions for Phase 2B published SU(2) plaquette baseline semantics."""

from __future__ import annotations

import copy

from check_contracts import load, validate_exact
from check_su2_baseline_contracts import (
    CONTRACT_PATH,
    VECTOR_PATH,
    EXPECTED_BASELINE_CONTRACT,
    EXPECTED_BASELINE_VECTORS,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative Phase 2B contract test failed: {label} was accepted")


def main() -> None:
    contract = load(CONTRACT_PATH)
    vectors = load(VECTOR_PATH)

    broken = copy.deepcopy(contract)
    broken["primary_source"]["peer_reviewed"] = False
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "peer-reviewed source weakened",
    )

    broken = copy.deepcopy(contract)
    broken["benchmark"]["extents"] = [8, 8, 8, 8]
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "benchmark volume drift",
    )

    broken = copy.deepcopy(contract)
    broken["benchmark"]["source_envelope_max"] = 0.70
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "source envelope weakened",
    )

    broken = copy.deepcopy(contract)
    broken["comparison_rule"]["source_envelope_is_paper_error_bar"] = True
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "graphical envelope misrepresented as paper error bar",
    )

    broken = copy.deepcopy(contract)
    broken["reference_run"]["chain_ids"] = [0, 0]
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "independent chain identity collapsed",
    )

    broken = copy.deepcopy(contract)
    broken["comparison_rule"]["within_chain_stationarity_required"] = False
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "within-chain stationarity gate removed",
    )

    broken = copy.deepcopy(contract)
    broken["reference_run"]["maximum_stationarity_z"] = 50.0
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "stationarity drift threshold weakened",
    )

    broken = copy.deepcopy(contract)
    broken["reference_run"]["stationarity_autocorrelation_max_lag"] = 0
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "stationarity autocorrelation rule drift",
    )

    broken = copy.deepcopy(contract)
    broken["comparison_rule"]["zero_variance_stationarity_rejected"] = False
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "zero-variance stationarity refusal removed",
    )

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["published_plaquette_baseline_reproduced"] = False
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "frozen published baseline result erased",
    )

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["phase2_graduation_gate_satisfied"] = False
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "frozen Phase 2 graduation erased",
    )

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["su3_ensemble_enabled"] = True
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "SU(3) ensemble enabled by SU(2) benchmark",
    )

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["benchmark_agreement_implies_positive_mass_gap"] = True
    expect_reject(
        lambda payload: validate_exact(CONTRACT_PATH, payload, EXPECTED_BASELINE_CONTRACT),
        broken,
        "benchmark mass-gap overclaim",
    )

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected_interval"] = [0.5, 0.7]
    expect_reject(
        lambda payload: validate_exact(VECTOR_PATH, payload, EXPECTED_BASELINE_VECTORS),
        broken,
        "published envelope vector weakened",
    )

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["maximum_half_drift_z"] = 50.0
    expect_reject(
        lambda payload: validate_exact(VECTOR_PATH, payload, EXPECTED_BASELINE_VECTORS),
        broken,
        "stationarity vector weakened",
    )

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["zero_variance_half_rejected"] = False
    expect_reject(
        lambda payload: validate_exact(VECTOR_PATH, payload, EXPECTED_BASELINE_VECTORS),
        broken,
        "zero-variance stationarity vector weakened",
    )

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["phase2_graduation_gate_satisfied"] = False
    expect_reject(
        lambda payload: validate_exact(VECTOR_PATH, payload, EXPECTED_BASELINE_VECTORS),
        broken,
        "graduation vector erased",
    )

    print("Negative Phase 2B published baseline contract regression checks: passed")


if __name__ == "__main__":
    main()
