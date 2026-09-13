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
    broken["stream"]["rng_outputs_per_generator_proposal"] = 1
    expect_reject(validate_su3_ensemble_contract, broken, "RNG consumption weakened")

    broken = copy.deepcopy(contract)
    broken["proposal"]["generator_order"] = "implementation-defined"
    expect_reject(validate_su3_ensemble_contract, broken, "generator order weakened")

    broken = copy.deepcopy(contract)
    broken["proposal"]["acceptance_draw_rule"] = "draw only for uphill proposals"
    expect_reject(validate_su3_ensemble_contract, broken, "acceptance RNG consumption drift")

    broken = copy.deepcopy(contract)
    broken["proposal"]["angle_grid"]["angle_rule"] = "theta=proposal_angle*(2u-1)"
    expect_reject(validate_su3_ensemble_contract, broken, "half-open proposal grid restored")

    broken = copy.deepcopy(contract)
    broken["proposal"]["angle_grid"]["equal_multiplicity"] = False
    expect_reject(validate_su3_ensemble_contract, broken, "proposal inverse multiplicity weakened")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["proposal_angle_min"] = 0.0
    expect_reject(validate_su3_ensemble_contract, broken, "proposal-angle floor removed")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["max_schedule_sweeps"] = 262144
    expect_reject(validate_su3_ensemble_contract, broken, "unverified schedule bound restored")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["maximum_schedule_group_stress_required"] = False
    expect_reject(validate_su3_ensemble_contract, broken, "maximum schedule stress removed")

    broken = copy.deepcopy(contract)
    broken["local_action"]["affected_plaquettes"] = 4
    expect_reject(validate_su3_ensemble_contract, broken, "local action topology drift")

    broken = copy.deepcopy(contract)
    broken["local_action"]["candidate_evaluation"] = "clone the complete lattice for every proposal"
    expect_reject(validate_su3_ensemble_contract, broken, "allocation-free local delta removed")

    broken = copy.deepcopy(contract)
    broken["reference_bounds"]["link_unitarity_residual_max"] = 1.0
    expect_reject(validate_su3_ensemble_contract, broken, "unitarity gate weakened")

    broken = copy.deepcopy(contract)
    broken["reproducibility"]["literal_golden_reference_chain_required"] = False
    expect_reject(validate_su3_ensemble_contract, broken, "literal golden chain removed")

    broken = copy.deepcopy(contract)
    broken["reproducibility"]["complete_reference_runtime_evidence_required"] = False
    expect_reject(validate_su3_ensemble_contract, broken, "complete runtime evidence weakened")

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["published_su3_baseline_reproduced"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "premature external benchmark claim")

    broken = copy.deepcopy(contract)
    broken["phase2c_gate"]["glueball_analysis_enabled"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "premature glueball analysis")

    broken = copy.deepcopy(contract)
    broken["claim_boundary"]["finite_lattice_ensemble_implies_positive_mass_gap"] = True
    expect_reject(validate_su3_ensemble_contract, broken, "mass-gap overclaim")

    by_id = {vector["id"]: index for index, vector in enumerate(vectors["vectors"])}

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["su3-stream-golden-seeds"]]["derived_seeds_hex"][0] = "0000000000000000"
    expect_reject(validate_su3_ensemble_vectors, broken, "golden stream drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["beta-zero-metropolis-all-accept"]]["expected_accepts_per_sweep"] = 511
    expect_reject(validate_su3_ensemble_vectors, broken, "beta-zero acceptance drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["proposal-angle-resolution-floor"]]["proposal_angle_min"] = 0.0
    expect_reject(validate_su3_ensemble_vectors, broken, "proposal-resolution vector drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["paired-angle-grid"]]["exact_inverse_pairing"] = False
    expect_reject(validate_su3_ensemble_vectors, broken, "paired-grid inverse weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["maximum-schedule-group-stress"]]["maximum_total_sweeps"] = 262144
    expect_reject(validate_su3_ensemble_vectors, broken, "maximum schedule stress weakened")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["frozen-reference-chain-golden-evidence"]]["raw_measurements"][0] = 0.9
    expect_reject(validate_su3_ensemble_vectors, broken, "golden raw chain drift")

    broken = copy.deepcopy(vectors)
    broken["vectors"][by_id["frozen-reference-chain-golden-evidence"]]["diagnostics"]["mean"] = 0.9
    expect_reject(validate_su3_ensemble_vectors, broken, "golden diagnostic drift")

    print("Negative Phase 2C1 SU(3) ensemble contract regression checks: passed")


if __name__ == "__main__":
    main()
