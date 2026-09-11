#!/usr/bin/env python3
"""Negative regressions for Phase 1D path/holonomy contract semantics."""

from __future__ import annotations

import copy

from check_contracts import validate_exact
from check_path_contracts import EXPECTED_PATH_CONTRACT, EXPECTED_PATH_VECTORS


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative Phase 1D contract test failed: {label} was accepted")


def main() -> None:
    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["continuum_transport_convention"]["concatenation"] = (
        "if gamma2 o gamma1 means traverse gamma1 then gamma2, U_{gamma2 o gamma1}=U_gamma2 U_gamma1"
    )
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "reversed continuum composition order",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["continuum_transport_convention"]["endpoint_covariance"] = (
        "U_gamma^Omega=Omega(gamma(1)) U_gamma Omega(gamma(0))^-1"
    )
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "swapped endpoint covariance",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["lattice_discretization"]["product_order"] = (
        "segments-multiplied-in-reverse-traversal-order"
    )
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "lattice product-order drift",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["observables"]["open_wilson_line"] = "gauge invariant by itself"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "open-line gauge-invariance overclaim",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["small_loop_bridge"]["finite_loop_equals_pointwise_curvature"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "finite-loop curvature collapse",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["curve_geometry_firewall"]["frenet_curvature_is_gauge_curvature"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "Frenet/gauge curvature conflation",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["static_potential_future_surface"]["confinement_claimed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "premature confinement claim",
    )

    broken = copy.deepcopy(EXPECTED_PATH_CONTRACT)
    broken["continuum_parallel_transport_numerically_solved"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-PATH-HOLONOMY-V1.json", payload, EXPECTED_PATH_CONTRACT
        ),
        broken,
        "undeclared continuum numerical solver",
    )

    broken = copy.deepcopy(EXPECTED_PATH_VECTORS)
    broken["vectors"][0]["expected_max"] = 5e-6
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/path-holonomy-v1.json", payload, EXPECTED_PATH_VECTORS
        ),
        broken,
        "weakened composition tolerance",
    )

    broken = copy.deepcopy(EXPECTED_PATH_VECTORS)
    broken["vectors"][2]["claim_class"] = "C1"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/path-holonomy-v1.json", payload, EXPECTED_PATH_VECTORS
        ),
        broken,
        "numerical endpoint covariance overclassification",
    )

    broken = copy.deepcopy(EXPECTED_PATH_VECTORS)
    broken["vectors"][5]["expected"] = [1.0, 0.0]
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/path-holonomy-v1.json", payload, EXPECTED_PATH_VECTORS
        ),
        broken,
        "nonplanar exact-vector drift",
    )

    broken = copy.deepcopy(EXPECTED_PATH_VECTORS)
    broken["vectors"][7]["expected"] = "different-semantics"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/path-holonomy-v1.json", payload, EXPECTED_PATH_VECTORS
        ),
        broken,
        "SU(2) compatibility drift",
    )

    print("Negative Phase 1D path/holonomy contract regression checks: passed")


if __name__ == "__main__":
    main()
