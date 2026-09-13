#!/usr/bin/env python3
"""Fail-closed checks for the predeclared Phase 2C2 published SU(3) baseline."""

from __future__ import annotations

from typing import Any

from check_contracts import load

CONTRACT_PATH = "contracts/YM-SU3-PLAQUETTE-BASELINE-V1.json"
VECTOR_PATH = "test-vectors/su3-plaquette-baseline-v1.json"


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def validate_su3_baseline_contract(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 contract ID drift")
    _require(payload["status"] == "phase-2c2-candidate", "Phase 2C2 candidate status drift")
    _require(
        payload["depends_on"] == [
            {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}
        ],
        "Phase 2C2 dependency drift",
    )
    source = payload["primary_source"]
    _require(source["doi"] == "10.1088/0305-4616/7/5/001", "Phase 2C2 source DOI drift")
    _require(source["peer_reviewed"] is True, "Phase 2C2 source must remain peer reviewed")
    _require(source["selected_surface"] == "4^4 lattice at beta=6.0", "Phase 2C2 source surface drift")
    benchmark = payload["benchmark"]
    _require(benchmark["gauge_group"] == "SU(3)" and benchmark["action"] == "Wilson", "Phase 2C2 theory surface drift")
    _require(benchmark["boundary_conditions"] == "periodic", "Phase 2C2 boundary-condition drift")
    _require(benchmark["extents"] == [4, 4, 4, 4] and benchmark["beta"] == 6.0, "Phase 2C2 benchmark geometry/coupling drift")
    _require(
        benchmark["source_envelope_min"] == 0.56 and benchmark["source_envelope_max"] == 0.63,
        "Phase 2C2 graphical source envelope drift",
    )
    _require(benchmark["source_envelope_is_paper_error_bar"] is False, "Phase 2C2 graphical envelope misrepresented")
    run = payload["reference_run"]
    _require(run["sampler_id"] == "YM-SU3-ENSEMBLE-V1", "Phase 2C2 sampler identity drift")
    _require(run["proposal_id"] == "YM-SU3-GENERATOR-METROPOLIS-V1", "Phase 2C2 proposal identity drift")
    _require(run["rng_id"] == "SPLITMIX64-V1" and run["stream_id"] == "YM-SU3-STREAM-V1", "Phase 2C2 RNG/stream identity drift")
    _require(run["master_seed"] == "2c02198105010001", "Phase 2C2 master seed drift")
    _require(run["chain_ids"] == [0, 1], "Phase 2C2 independent chain identities drift")
    _require(run["derived_seeds"] == ["2ff3bf48550d1457", "ef7636a56e8ea29a"], "Phase 2C2 derived seed drift")
    _require(run["proposal_angle"] == 0.35, "Phase 2C2 proposal angle drift")
    _require(
        run["thermalization_sweeps"] == 192
        and run["measurements_per_chain"] == 128
        and run["sweeps_between_measurements"] == 2
        and run["total_sweeps_per_chain"] == 448,
        "Phase 2C2 schedule drift",
    )
    _require(run["total_sweeps_per_chain"] <= 512, "Phase 2C2 schedule escaped frozen Phase 2C1 bound")
    _require(run["expected_metropolis_proposals_per_chain"] == 3670016, "Phase 2C2 proposal accounting drift")
    _require(run["minimum_ess_per_chain"] == 8.0, "Phase 2C2 ESS gate drift")
    _require(run["maximum_chain_consistency_z"] == 5.0, "Phase 2C2 chain-consistency gate drift")
    _require(run["stationarity_half_measurements"] == 64, "Phase 2C2 stationarity split drift")
    _require(run["stationarity_autocorrelation_max_lag"] == 12, "Phase 2C2 stationarity autocorrelation drift")
    _require(run["maximum_stationarity_z"] == 5.0, "Phase 2C2 stationarity threshold drift")
    _require(run["minimum_metropolis_acceptance"] == 0.1 and run["maximum_metropolis_acceptance"] == 0.95, "Phase 2C2 acceptance gate drift")
    _require(run["maximum_link_unitarity_residual"] == 5e-12, "Phase 2C2 unitarity gate drift")
    _require(run["maximum_link_determinant_residual"] == 5e-12, "Phase 2C2 determinant gate drift")
    comparison = payload["comparison_rule"]
    _require(comparison["pooled_mean_must_lie_inside_source_envelope"] is True, "Phase 2C2 published comparison weakened")
    _require(comparison["independent_chain_identities_required"] is True, "Phase 2C2 independence gate weakened")
    _require(comparison["raw_measurements_retained"] is True, "Phase 2C2 raw evidence gate weakened")
    _require(comparison["diagnostics_recomputed_from_raw_measurements"] is True, "Phase 2C2 diagnostic authority weakened")
    _require(comparison["within_chain_stationarity_required"] is True, "Phase 2C2 stationarity gate weakened")
    _require(comparison["metropolis_accounting_must_match_frozen_schedule"] is True, "Phase 2C2 accounting gate weakened")
    predeclaration = payload["predeclaration"]
    _require(predeclaration["selected_after_phase2c1_freeze"] is True, "Phase 2C2 selection ordering weakened")
    _require(predeclaration["benchmark_parameters_frozen_before_reference_result_inspection"] is True, "Phase 2C2 predeclaration weakened")
    _require(predeclaration["phase2c1_sampler_contract_mutated"] is False, "Phase 2C1 frozen sampler mutation admitted")
    gate = payload["phase2c_gate"]
    _require(gate["published_su3_baseline_reproduced"] is False, "Phase 2C2 result claimed before executable evidence")
    _require(gate["phase2c_complete"] is False, "Phase 2C completed before executable evidence")
    _require(gate["glueball_analysis_enabled"] is False, "glueball analysis enabled before Phase 2C2 evidence")
    _require(gate["result_status"] == "pending-executable-evidence", "Phase 2C2 pending result status drift")
    boundary = payload["claim_boundary"]
    _require(boundary["benchmark_agreement_implies_confinement"] is False, "Phase 2C2 confinement overclaim")
    _require(boundary["benchmark_agreement_implies_continuum_qft"] is False, "Phase 2C2 continuum overclaim")
    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 mass-gap overclaim")


def validate_su3_baseline_vectors(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 vector contract ID drift")
    _require(payload["vector_set"] == "SU3-PLAQUETTE-BASELINE-PHASE2C2-V1", "Phase 2C2 vector-set ID drift")
    _require(payload["status"] == "phase-2c2-candidate", "Phase 2C2 vector status drift")
    by_id = {vector["id"]: vector for vector in payload["vectors"]}
    source = by_id["published-source-envelope"]
    _require(source["doi"] == "10.1088/0305-4616/7/5/001", "Phase 2C2 source vector DOI drift")
    _require(source["source_lattice"] == [4, 4, 4, 4] and source["source_iterations"] == 400, "Phase 2C2 source vector geometry drift")
    _require(source["beta"] == 6.0 and source["expected_interval"] == [0.56, 0.63], "Phase 2C2 source vector numeric drift")
    run = by_id["two-independent-chain-predeclaration"]
    _require(run["chain_ids"] == [0, 1], "Phase 2C2 vector chain identity drift")
    _require(run["derived_seeds"] == ["2ff3bf48550d1457", "ef7636a56e8ea29a"], "Phase 2C2 vector derived seed drift")
    _require(run["total_sweeps_per_chain"] == 448 and run["expected_metropolis_proposals_per_chain"] == 3670016, "Phase 2C2 vector schedule/accounting drift")
    stationarity = by_id["post-thermalization-stationarity"]
    _require(stationarity["half_measurements"] == 64 and stationarity["autocorrelation_max_lag_per_half"] == 12, "Phase 2C2 stationarity vector drift")
    _require(stationarity["constant_half_rejected"] is True, "Phase 2C2 constant-series refusal weakened")
    boundary = by_id["phase2c-predeclaration-boundary"]
    _require(boundary["published_su3_baseline_reproduced"] is False, "Phase 2C2 vector result claimed prematurely")
    _require(boundary["phase2c_complete"] is False and boundary["glueball_analysis_enabled"] is False, "Phase 2C2 vector boundary weakened")
    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 vector mass-gap overclaim")


def main() -> None:
    validate_su3_baseline_contract(load(CONTRACT_PATH))
    validate_su3_baseline_vectors(load(VECTOR_PATH))
    print("Phase 2C2 SU(3) published baseline predeclaration checks: passed")


if __name__ == "__main__":
    main()
