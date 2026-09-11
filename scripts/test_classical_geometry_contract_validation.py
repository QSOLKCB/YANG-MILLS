#!/usr/bin/env python3
"""Negative regressions for Phase 1E classical geometry semantics."""

from __future__ import annotations

import copy

from check_contracts import validate_exact
from check_classical_geometry_contracts import (
    EXPECTED_CLASSICAL_CONTRACT,
    EXPECTED_CLASSICAL_VECTORS,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative Phase 1E contract test failed: {label} was accepted")


def main() -> None:
    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["principal_bundle"]["action"] = "free left principal action"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "principal-action convention drift",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["local_gauge_convention"]["phase1d_variable"] = "Omega=g"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "Phase 1D gauge-variable mismatch",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["curvature"]["global_type"] = "ordinary scalar 2-form"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "curvature bundle type collapse",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["su3_lie_algebra_bridge"]["anti_hermitian_basis"] = "T^a"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "Hermitian/anti-Hermitian SU(3) basis conflation",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["riemannian_hodge"]["two_form_square"] = "*^2=-1 on 2-forms in Riemannian 4D"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "Hodge-star sign drift",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["flatness_and_parallel_transport"]["flat_implies_globally_trivial_holonomy"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "flatness/global-holonomy overclaim",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["flatness_and_parallel_transport"]["phase1d_start_left_transport"] = "U_gamma=P_gamma"
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "Phase 1D/forward transport orientation collapse",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["levi_civita_firewall"]["gauge_connection_equals_levi_civita"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "gauge/Levi-Civita connection conflation",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["yang_mills_functional"]["every_critical_point_is_global_minimizer"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "critical-point/global-minimum overclaim",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["self_duality"]["quantum_mass_gap_implication"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "self-duality mass-gap overclaim",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["lattice_continuum_dictionary"]["finite_plaquette_equals_pointwise_curvature"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "finite plaquette/continuum curvature collapse",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_CONTRACT)
    broken["continuum_quantum_theory_constructed"] = True
    expect_reject(
        lambda payload: validate_exact(
            "contracts/YM-CLASSICAL-GEOMETRY-V1.json", payload, EXPECTED_CLASSICAL_CONTRACT
        ),
        broken,
        "continuum quantum construction overclaim",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_VECTORS)
    broken["vectors"][0]["expected_map"][1] = "13"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/classical-geometry-v1.json", payload, EXPECTED_CLASSICAL_VECTORS
        ),
        broken,
        "Hodge basis sign drift",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_VECTORS)
    broken["vectors"][3]["self_dual_expected"][0] = 3.0
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/classical-geometry-v1.json", payload, EXPECTED_CLASSICAL_VECTORS
        ),
        broken,
        "self-dual vector drift",
    )

    broken = copy.deepcopy(EXPECTED_CLASSICAL_VECTORS)
    broken["vectors"][5]["relation"] = "U_gamma=P_gamma"
    expect_reject(
        lambda payload: validate_exact(
            "test-vectors/classical-geometry-v1.json", payload, EXPECTED_CLASSICAL_VECTORS
        ),
        broken,
        "transport dictionary drift",
    )

    print("Negative Phase 1E classical geometry contract regression checks: passed")


if __name__ == "__main__":
    main()
