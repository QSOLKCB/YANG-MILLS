#!/usr/bin/env python3
"""Negative regressions for the Phase 2C1 SU(3) ensemble authority."""

from __future__ import annotations

import copy
from collections.abc import Callable

from check_contracts import load
from check_su3_ensemble_contracts import (
    CONTRACT_PATH,
    VECTOR_PATH,
    validate_su3_ensemble_contract,
    validate_su3_ensemble_vectors,
)


def expect_reject(fn: Callable[[dict], None], payload: dict, label: str) -> None:
    try:
        fn(payload)
    except (KeyError, TypeError, ValueError):
        return
    raise AssertionError(f"negative SU(3) ensemble regression was accepted: {label}")


def main() -> None:
    contract = load(CONTRACT_PATH)
    vectors = load(VECTOR_PATH)

    broken = copy.deepcopy(contract)
    broken["identities"]["sampler_id"] = "YM-SU3-ENSEMBLE-V2"
    expect_reject(validate_su3_ensemble_contract, broken, "sampler identity drift")

    broken = copy.deepcopy(contract)
    broken["stream"]["domain_separator_hex"] = "0000000000000000"
    expect_reject(validate_su3_ensemble_contract, broken, "stream domain drift")

    broken = copy.deepcopy(contract)
    broken["proposal"]["generator_order"] = "implementation-defined"
    expect_reject(validate_su3_ensemble_contract, broken, "generator order weakened")

    broken = copy.deepcopy(contract)
    broken["proposal"]["acceptance_draw_rule"] = "draw only for uphill proposals"
    expect_reject(validate_su3_ensemble_contract, broken, "RNG consumption drift")

    broken = copy.deepcopy(contract)
    broken["local_action"]["affected_plaquettes"] = 4
    expect_reject(validate_su3_ensemble_contract, broken, "local action topology drift")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["link_unitarity_residual_max"] = 1.0
    expect_reject(validate_su3_ensemble_contract, broken, "unitarity gate weakened")

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["published_su3_baseline_reproduced"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "premature external benchmark claim")

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["glueball_analysis_enabled"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "premature glueball analysis")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["finite_lattice_ensemble_implies_positive_mass_gap"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "mass-gap overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["derived_seeds_hex"][0] = "0000000000000000"
    expect_reject(validate_su3_ensemble_vectors, broken, "golden stream drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["expected_accepts_per_sweep"] = 511
    expect_reject(validate_su3_ensemble_vectors, broken, "beta-zero acceptance drift")

    print("Negative Phase 2C1 SU(3) ensemble contract regression checks: passed")


if __name__ == "__main__":
    main()
