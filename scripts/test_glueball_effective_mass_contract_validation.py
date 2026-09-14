#!/usr/bin/env python3
"""Negative contract regressions for Phase 3D effective-mass analysis."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_effective_mass_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3D contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-EFFECTIVE-MASS-V1.json")
    vectors = load("test-vectors/glueball-effective-mass-phase3d-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3d-frozen"
    expect_reject(validate_contract, broken, "candidate status silently promoted")

    broken = copy.deepcopy(contract)
    broken["depends_on"][0]["required_status"] = "phase-3c-frozen"
    expect_reject(validate_contract, broken, "dependency lifecycle changed without authority update")

    broken = copy.deepcopy(contract)
    broken["input_surface"]["replicas_may_not_reuse_full_sample_vacuum_mean"] = False
    expect_reject(validate_contract, broken, "jackknife vacuum subtraction leakage")

    broken = copy.deepcopy(contract)
    broken["estimator"]["formula"] = "m_eff(tau)=log(C(tau)/C(tau+1))"
    expect_reject(validate_contract, broken, "estimator silently changed")

    broken = copy.deepcopy(contract)
    broken["bounds"]["analysis_lag_rule"] = "any integer tau"
    expect_reject(validate_contract, broken, "lag domain weakened")

    broken = copy.deepcopy(contract)
    broken["uncertainty"]["nonlinear_transform_recomputed_per_replica"] = False
    expect_reject(validate_contract, broken, "nonlinear jackknife propagation removed")

    broken = copy.deepcopy(contract)
    broken["uncertainty"]["standard_error"] = "sample standard deviation"
    expect_reject(validate_contract, broken, "jackknife standard error changed")

    broken = copy.deepcopy(contract)
    broken["canonical_fixture"]["expected_standard_error"] = 0.0
    expect_reject(validate_contract, broken, "analytic uncertainty vector drift")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["fit_window_selection_defined"] = True
    expect_reject(validate_contract, broken, "fit-window authority claimed early")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["effective_mass_point_is_fitted_glueball_mass"] = True
    expect_reject(validate_contract, broken, "effective mass promoted to fitted mass")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(validate_contract, broken, "continuum mass-gap overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected_effective_mass_each_lag"] = 0.6
    expect_reject(validate_vectors, broken, "effective-mass vector drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["expected_standard_error_each_lag"] = 0.0
    expect_reject(validate_vectors, broken, "jackknife vector weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["glueball_mass_measured"] = True
    expect_reject(validate_vectors, broken, "physical mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "boolean claim boundary replaced by integer")

    broken = copy.deepcopy(vectors)
    broken["clay_solution_proved"] = True
    expect_reject(validate_vectors, broken, "undeclared top-level overclaim field")

    print("Phase 3D effective-mass negative contract regressions: passed")


if __name__ == "__main__":
    main()
