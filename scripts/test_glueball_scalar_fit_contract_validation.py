#!/usr/bin/env python3
"""Negative contract regressions for Phase 3E scalar correlated fitting."""

from __future__ import annotations

import copy
from typing import Any, Callable

from check_contracts import load
from check_glueball_scalar_fit_contracts import validate_contract, validate_vectors


def expect_reject(
    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str
) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 3E contract test failed: {label} was accepted")


def main() -> None:
    contract = load("contracts/YM-GLUEBALL-SCALAR-FIT-V1.json")
    vectors = load("test-vectors/glueball-scalar-fit-phase3e-v1.json")

    broken = copy.deepcopy(contract)
    broken["status"] = "phase-3e-frozen"
    expect_reject(validate_contract, broken, "candidate status silently promoted")

    broken = copy.deepcopy(contract)
    broken["depends_on"][0]["required_status"] = "phase-3d-candidate"
    expect_reject(validate_contract, broken, "effective-mass dependency freeze weakened")

    broken = copy.deepcopy(contract)
    broken["input_surface"]["automatic_data_dependent_candidate_generation"] = True
    expect_reject(validate_contract, broken, "data-dependent window generation silently enabled")

    broken = copy.deepcopy(contract)
    broken["plateau_selection"]["maximum_standardized_deviation"] = 10.0
    expect_reject(validate_contract, broken, "plateau threshold weakened")

    broken = copy.deepcopy(contract)
    broken["plateau_selection"]["tie_break_rule"] = "first window in input order"
    expect_reject(validate_contract, broken, "window selection made input-order dependent")

    broken = copy.deepcopy(contract)
    broken["covariance"]["complete_matrix_used"] = False
    expect_reject(validate_contract, broken, "full covariance disabled")

    broken = copy.deepcopy(contract)
    broken["covariance"]["diagonal_only_fallback"] = True
    expect_reject(validate_contract, broken, "diagonal covariance fallback enabled")

    broken = copy.deepcopy(contract)
    broken["spectral_fit"]["model"] = "C(tau)=A*exp(-m*tau)"
    expect_reject(validate_contract, broken, "periodic backward propagator removed")

    broken = copy.deepcopy(contract)
    broken["bounds"]["mass_grid_step"] = 0.01
    expect_reject(validate_contract, broken, "mass-grid resolution changed")

    broken = copy.deepcopy(contract)
    broken["parameter_uncertainty"]["replica_fit_rule"] = "reuse full-sample fitted parameters"
    expect_reject(validate_contract, broken, "jackknife parameter refit removed")

    broken = copy.deepcopy(contract)
    broken["future_surfaces"]["production_scalar_ensemble_analysis_defined"] = True
    expect_reject(validate_contract, broken, "production scalar analysis claimed early")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["correlated_fit_parameter_is_physical_glueball_mass"] = True
    expect_reject(validate_contract, broken, "fit parameter promoted to physical glueball mass")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["positive_continuum_mass_gap_proved"] = True
    expect_reject(validate_contract, broken, "continuum mass-gap overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["expected_selected_window"] = [2, 5]
    expect_reject(validate_vectors, broken, "canonical selected window drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][1]["diagonal_only_fallback"] = True
    expect_reject(validate_vectors, broken, "vector diagonal fallback enabled")

    broken = copy.deepcopy(vectors)
    broken["vectors"][2]["expected_mass_standard_error"] = 0.0
    expect_reject(validate_vectors, broken, "jackknife mass uncertainty weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["physical_glueball_mass_reported"] = True
    expect_reject(validate_vectors, broken, "physical mass overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][8]["clay_solution_claimed"] = 0
    expect_reject(validate_vectors, broken, "claim-boundary boolean replaced by integer")

    print("Phase 3E scalar correlated-fit negative contract regressions: passed")


if __name__ == "__main__":
    main()
