#!/usr/bin/env python3
"""Negative regressions for Phase 2C2 V1/V2 published SU(3) semantics."""

from __future__ import annotations

import copy
from collections.abc import Callable
from typing import Any

from check_contracts import load
from check_su3_baseline_contracts import (
    V1_CONTRACT_PATH,
    V1_VECTOR_PATH,
    V2_CONTRACT_PATH,
    V2_VECTOR_PATH,
    validate_v1_contract,
    validate_v1_vectors,
    validate_v2_contract,
    validate_v2_vectors,
)


def expect_reject(callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str) -> None:
    try:
        callback(payload)
    except (SystemExit, KeyError, TypeError, ValueError):
        return
    raise SystemExit(f"negative Phase 2C2 contract test failed: {label} was accepted")


def main() -> None:
    v1 = load(V1_CONTRACT_PATH)
    v1_vectors = load(V1_VECTOR_PATH)
    v2 = load(V2_CONTRACT_PATH)
    v2_vectors = load(V2_VECTOR_PATH)

    broken = copy.deepcopy(v1)
    broken["status"] = "phase-2c2-frozen"
    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")

    broken = copy.deepcopy(v1)
    broken["clay_solution_proved"] = True
    expect_reject(validate_v1_contract, broken, "V1 undeclared top-level overclaim field accepted")

    broken = copy.deepcopy(v1)
    broken["scope"] = "continuum construction and Clay Yang-Mills mass-gap solution"
    expect_reject(validate_v1_contract, broken, "V1 scope promoted beyond finite-lattice validation")

    broken = copy.deepcopy(v1)
    broken["benchmark"]["observable"] = "unrelated observable"
    expect_reject(validate_v1_contract, broken, "V1 observable normalization drift")

    broken = copy.deepcopy(v1)
    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"
    expect_reject(validate_v1_contract, broken, "V1 graphical source extraction rewritten")

    broken = copy.deepcopy(v1)
    broken["primary_source"]["title"] = "Rewritten benchmark title"
    expect_reject(validate_v1_contract, broken, "V1 publication title provenance rewritten")

    broken = copy.deepcopy(v1)
    del broken["primary_source"]["source_surface"]
    expect_reject(validate_v1_contract, broken, "V1 retained source-surface provenance deleted")

    broken = copy.deepcopy(v1)
    broken["reference_run"]["sampler_id"] = "YM-SU3-ENSEMBLE-V2"
    expect_reject(validate_v1_contract, broken, "V1 sampler identity drift")

    broken = copy.deepcopy(v1)
    broken["reference_run"]["autocorrelation_max_lag"] = 0
    expect_reject(validate_v1_contract, broken, "V1 autocorrelation gate weakened")

    broken = copy.deepcopy(v1)
    broken["reference_run"]["maximum_link_unitarity_residual"] = 1.0
    expect_reject(validate_v1_contract, broken, "V1 residual gate weakened")

    broken = copy.deepcopy(v1)
    broken["comparison_rule"]["within_chain_stationarity_required"] = False
    expect_reject(validate_v1_contract, broken, "V1 stationarity comparison rule weakened")

    broken = copy.deepcopy(v1)
    broken["comparison_rule"]["raw_measurements_retained"] = 1
    expect_reject(validate_v1_contract, broken, "V1 boolean replaced by integer")

    broken = copy.deepcopy(v1)
    broken["predeclaration"]["selected_after_phase2c1_freeze"] = False
    expect_reject(validate_v1_contract, broken, "V1 predeclaration ordering weakened")

    broken = copy.deepcopy(v1)
    broken["predeclaration"]["phase2c1_sampler_contract_mutated"] = True
    expect_reject(validate_v1_contract, broken, "V1 sampler-mutation boundary weakened")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["minimum_effective_sample_size"] = 8.1
    expect_reject(validate_v1_contract, broken, "V1 failed ESS evidence rewritten")

    broken = copy.deepcopy(v1)
    broken["replay_tolerance"]["summary_relative"] = 1.0
    expect_reject(validate_v1_contract, broken, "V1 replay tolerance weakened")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["pooled_mean"] += 0.01
    expect_reject(validate_v1_contract, broken, "V1 pooled mean evidence corrupted")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["chain_consistency_z"] += 1.0
    expect_reject(validate_v1_contract, broken, "V1 chain consistency evidence corrupted")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["maximum_stationarity_z_observed"] += 1.0
    expect_reject(validate_v1_contract, broken, "V1 stationarity evidence corrupted")

    broken = copy.deepcopy(v1)
    broken["observed_evidence"]["chain_acceptances"][0] = 0.5
    expect_reject(validate_v1_contract, broken, "V1 acceptance evidence corrupted")

    broken = copy.deepcopy(v1)
    broken["phase2c_gate"]["glueball_analysis_enabled"] = True
    expect_reject(validate_v1_contract, broken, "V1 failure enabled glueball analysis")

    broken = copy.deepcopy(v1)
    broken["claim_boundary"]["benchmark_agreement_implies_positive_mass_gap"] = True
    expect_reject(validate_v1_contract, broken, "V1 mass-gap claim boundary weakened")

    broken = copy.deepcopy(v1)
    broken["claim_boundary"]["benchmark_agreement_implies_confinement"] = True
    expect_reject(validate_v1_contract, broken, "V1 confinement claim boundary weakened")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["v1_reclassified_as_pass"] = True
    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["claim_class"] = "C5"
    expect_reject(validate_v1_vectors, broken, "V1 observed evidence promoted to C5")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["workflow_run_id"] = 0
    expect_reject(validate_v1_vectors, broken, "V1 vector workflow evidence corrupted")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["evidence_head_commit"] = "0000000000000000000000000000000000000000"
    expect_reject(validate_v1_vectors, broken, "V1 vector evidence commit corrupted")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["pooled_mean"] = 0.0
    expect_reject(validate_v1_vectors, broken, "V1 vector pooled mean corrupted")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["chain_acceptances"][0] = 0.5
    expect_reject(validate_v1_vectors, broken, "V1 vector acceptance evidence corrupted")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][2]["failed_gate"] = "none"
    expect_reject(validate_v1_vectors, broken, "V1 vector failed-gate label corrupted")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][3]["benchmark_agreement_implies_confinement"] = True
    expect_reject(validate_v1_vectors, broken, "V1 vector confinement overclaim")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][3]["benchmark_agreement_implies_continuum_qft"] = True
    expect_reject(validate_v1_vectors, broken, "V1 vector continuum-QFT overclaim")

    broken = copy.deepcopy(v1_vectors)
    broken["vectors"][3]["benchmark_agreement_implies_positive_mass_gap"] = True
    expect_reject(validate_v1_vectors, broken, "V1 vector mass-gap overclaim")

    broken = copy.deepcopy(v2)
    broken["benchmark"]["source_envelope_max"] = 0.75
    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")

    broken = copy.deepcopy(v2)
    broken["clay_solution_proved"] = True
    expect_reject(validate_v2_contract, broken, "V2 undeclared top-level overclaim field accepted")

    broken = copy.deepcopy(v2)
    broken["scope"] = "continuum quantum Yang-Mills construction and positive mass-gap proof"
    expect_reject(validate_v2_contract, broken, "V2 scope promoted beyond finite-lattice validation")

    broken = copy.deepcopy(v2)
    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"
    expect_reject(validate_v2_contract, broken, "V2 graphical source extraction rewritten")

    broken = copy.deepcopy(v2)
    broken["primary_source"]["authors"][0] = "Different Author"
    expect_reject(validate_v2_contract, broken, "V2 publication author provenance rewritten")

    broken = copy.deepcopy(v2)
    broken["primary_source"]["journal"] = "Different Journal"
    expect_reject(validate_v2_contract, broken, "V2 publication journal provenance rewritten")

    broken = copy.deepcopy(v2)
    broken["primary_source"]["source_surface"] = "V1-only provenance field injected into V2"
    expect_reject(validate_v2_contract, broken, "V2 primary-source provenance gained an unfrozen field")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["stream_id"] = "YM-SU3-STREAM-V2"
    expect_reject(validate_v2_contract, broken, "V2 stream identity drift")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["master_seed"] = "2c02198105010001"
    expect_reject(validate_v2_contract, broken, "V2 reused V1 master seed")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["minimum_aggregate_effective_sample_size"] = 8.0
    expect_reject(validate_v2_contract, broken, "V2 aggregate ESS target weakened")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["stationarity_half_measurements"] = 32
    expect_reject(validate_v2_contract, broken, "V2 stationarity sizing weakened")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["minimum_metropolis_acceptance"] = 0.0
    expect_reject(validate_v2_contract, broken, "V2 acceptance gate weakened")

    broken = copy.deepcopy(v2)
    broken["reference_run"]["maximum_link_determinant_residual"] = 1.0
    expect_reject(validate_v2_contract, broken, "V2 determinant residual gate weakened")

    broken = copy.deepcopy(v2)
    broken["comparison_rule"]["raw_measurements_retained"] = False
    expect_reject(validate_v2_contract, broken, "V2 raw-evidence comparison rule weakened")

    broken = copy.deepcopy(v2)
    broken["comparison_rule"]["raw_measurements_retained"] = 1
    expect_reject(validate_v2_contract, broken, "V2 boolean replaced by integer")

    broken = copy.deepcopy(v2)
    broken["replay_tolerance"]["summary_absolute"] = 1.0
    expect_reject(validate_v2_contract, broken, "V2 replay tolerance weakened")

    broken = copy.deepcopy(v2)
    broken["predeclaration"]["proposal_angle_changed_after_v1"] = True
    expect_reject(validate_v2_contract, broken, "V2 proposal retuning admitted")

    broken = copy.deepcopy(v2)
    broken["predeclaration"]["phase2c1_sampler_contract_mutated"] = True
    expect_reject(validate_v2_contract, broken, "V2 sampler-mutation boundary weakened")

    broken = copy.deepcopy(v2)
    broken["observed_evidence"]["aggregate_effective_sample_size"] = 15.0
    expect_reject(validate_v2_contract, broken, "V2 passing ESS evidence rewritten below gate")

    broken = copy.deepcopy(v2)
    broken["observed_evidence"]["chain_means"][0] += 0.01
    expect_reject(validate_v2_contract, broken, "V2 per-chain evidence corrupted")

    broken = copy.deepcopy(v2)
    del broken["observed_evidence"]["chain_standard_errors"]
    expect_reject(validate_v2_contract, broken, "V2 per-chain evidence deleted")

    broken = copy.deepcopy(v2)
    broken["phase2c_gate"]["published_su3_baseline_reproduced"] = False
    expect_reject(validate_v2_contract, broken, "V2 frozen success gate downgraded")

    broken = copy.deepcopy(v2)
    broken["claim_boundary"]["benchmark_agreement_implies_continuum_qft"] = True
    expect_reject(validate_v2_contract, broken, "V2 continuum claim boundary weakened")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["chain_ids"] = [0, 1, 2, 2]
    expect_reject(validate_v2_vectors, broken, "V2 vector independent streams collapsed")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["master_seed"] = "2c02198105010001"
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector master seed drift")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["derived_seeds"][0] = "0000000000000000"
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector derived seed drift")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["proposal_angle"] = 0.5
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector proposal drift")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["thermalization_sweeps"] = 191
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector schedule drift")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["expected_metropolis_proposals_per_chain"] -= 1
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector proposal accounting drift")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["minimum_metropolis_acceptance"] = 0.0
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector acceptance gate weakened")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["maximum_pairwise_chain_consistency_z"] = 50.0
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector consistency gate weakened")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][1]["maximum_link_unitarity_residual"] = 1.0
    expect_reject(validate_v2_vectors, broken, "V2 confirmation vector residual gate weakened")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][2]["v1_reclassified_as_pass"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector rewrote V1 failure")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["aggregate_effective_sample_size"] = 15.0
    expect_reject(validate_v2_vectors, broken, "V2 vector passing evidence weakened")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["evidence_head_commit"] = "0000000000000000000000000000000000000000"
    expect_reject(validate_v2_vectors, broken, "V2 vector evidence commit corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["pooled_mean"] = 0.0
    expect_reject(validate_v2_vectors, broken, "V2 vector pooled mean corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["pooled_standard_error"] = 1.0
    expect_reject(validate_v2_vectors, broken, "V2 vector pooled standard error corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["chain_means"][0] = 0.0
    expect_reject(validate_v2_vectors, broken, "V2 vector per-chain mean corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["chain_effective_sample_sizes"][0] = 1.0
    expect_reject(validate_v2_vectors, broken, "V2 vector per-chain ESS corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["maximum_pairwise_chain_consistency_z_observed"] = 0.0
    expect_reject(validate_v2_vectors, broken, "V2 vector pairwise-z evidence corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["maximum_stationarity_z_observed"] = 0.0
    expect_reject(validate_v2_vectors, broken, "V2 vector stationarity evidence corrupted")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["source_envelope_reproduced"] = False
    expect_reject(validate_v2_vectors, broken, "V2 vector source agreement erased")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["diagnostics_trustworthy"] = False
    expect_reject(validate_v2_vectors, broken, "V2 vector diagnostic trust erased")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][3]["claim_class"] = "C5"
    expect_reject(validate_v2_vectors, broken, "V2 observed evidence promoted to C5")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][4]["phase2c_complete"] = False
    expect_reject(validate_v2_vectors, broken, "V2 frozen boundary downgraded")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][4]["benchmark_agreement_implies_confinement"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector confinement overclaim")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][4]["benchmark_agreement_implies_continuum_qft"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector continuum-QFT overclaim")

    broken = copy.deepcopy(v2_vectors)
    broken["vectors"][4]["benchmark_agreement_implies_positive_mass_gap"] = True
    expect_reject(validate_v2_vectors, broken, "V2 vector mass-gap overclaim")

    print("Negative Phase 2C2 SU(3) V1/V2 contract regressions: passed")


if __name__ == "__main__":
    main()
