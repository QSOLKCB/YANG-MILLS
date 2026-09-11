#!/usr/bin/env python3
"""Negative regressions for Phase 2A SU(2) ensemble semantics."""

from __future__ import annotations

import copy

from check_su2_ensemble_contracts import (
    validate_ensemble_contract,
    validate_ensemble_vectors,
)


def expect_reject(callback, payload, label: str) -> None:
    try:
        callback(payload)
    except SystemExit:
        return
    raise SystemExit(f"negative Phase 2A contract test failed: {label} was accepted")


def main() -> None:
    from check_contracts import load

    contract = load("contracts/YM-SU2-ENSEMBLE-V1.json")
    vectors = load("test-vectors/su2-ensemble-v1.json")

    broken = copy.deepcopy(contract)
    broken["rng"]["seed_is_physical_parameter"] = True
    expect_reject(validate_ensemble_contract, broken, "seed promoted to physical parameter")

    broken = copy.deepcopy(contract)
    broken["chain_identity"] = {}
    expect_reject(validate_ensemble_contract, broken, "chain identity structure erased")

    broken = copy.deepcopy(contract)
    broken["schedule"]["observable"] = "unrelated observable"
    expect_reject(validate_ensemble_contract, broken, "schedule observable drift")

    broken = copy.deepcopy(contract)
    broken["reproducibility"]["raw_measurement_series_retained_in_reference_report"] = False
    expect_reject(validate_ensemble_contract, broken, "raw measurement retention weakened")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["total_complete_markov_sweeps_must_be_at_most_max_schedule_sweeps"] = False
    expect_reject(validate_ensemble_contract, broken, "total schedule bound weakened")

    broken = copy.deepcopy(contract)
    broken["su2_heatbath"]["conditional_density"] = "uniform Haar only"
    expect_reject(validate_ensemble_contract, broken, "heat-bath conditional density drift")

    broken = copy.deepcopy(contract)
    broken["su2_heatbath"]["alpha_zero_limit"] = 1e-100
    expect_reject(validate_ensemble_contract, broken, "tiny-alpha numerical threshold drift")

    broken = copy.deepcopy(contract)
    broken["su2_heatbath"]["haar_jacobian_acceptance"] = "always accept"
    expect_reject(validate_ensemble_contract, broken, "Haar Jacobian rejection removed")

    broken = copy.deepcopy(contract)
    broken["su2_overrelaxation"]["local_wilson_action_preserved"] = False
    expect_reject(validate_ensemble_contract, broken, "microcanonical invariant weakened")

    broken = copy.deepcopy(contract)
    broken["autocorrelation_diagnostics"]["effective_sample_size"] = "N"
    expect_reject(validate_ensemble_contract, broken, "ESS autocorrelation correction removed")

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["published_plaquette_baseline_reproduced"] = True
    expect_reject(validate_ensemble_contract, broken, "published baseline claimed prematurely")

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["phase2_graduation_gate_satisfied"] = True
    expect_reject(validate_ensemble_contract, broken, "Phase 2 graduated prematurely")

    broken = copy.deepcopy(contract)
    broken["phase2_gate"]["su3_ensemble_enabled"] = True
    expect_reject(validate_ensemble_contract, broken, "SU(3) enabled before SU(2) graduation")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["single_seed_is_physical_truth"] = True
    expect_reject(validate_ensemble_contract, broken, "single-seed physical-truth overclaim")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["finite_lattice_ensemble_implies_positive_mass_gap"] = True
    expect_reject(validate_ensemble_contract, broken, "finite ensemble mass-gap overclaim")

    broken = copy.deepcopy(vectors)
    broken["vectors"][0]["outputs_hex"][0] = "0000000000000000"
    expect_reject(validate_ensemble_vectors, broken, "RNG golden stream drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][3]["absolute_mean_max"] = 0.8
    expect_reject(validate_ensemble_vectors, broken, "beta=0 sanity band weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][4]["expected_max"] = 2e-4
    expect_reject(validate_ensemble_vectors, broken, "over-relaxation tolerance weakened")

    print("Negative Phase 2A SU(2) ensemble contract regression checks: passed")


if __name__ == "__main__":
    main()
