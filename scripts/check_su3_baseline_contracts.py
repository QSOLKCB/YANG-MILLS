#!/usr/bin/env python3
"""Fail-closed checks for Phase 2C2 SU(3) published-baseline authorities."""

from __future__ import annotations

from typing import Any

from check_contracts import load

V1_CONTRACT_PATH = "contracts/YM-SU3-PLAQUETTE-BASELINE-V1.json"
V1_VECTOR_PATH = "test-vectors/su3-plaquette-baseline-v1.json"
V2_CONTRACT_PATH = "contracts/YM-SU3-PLAQUETTE-BASELINE-V2.json"
V2_VECTOR_PATH = "test-vectors/su3-plaquette-baseline-v2.json"


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def _validate_common_source_and_benchmark(payload: dict[str, Any]) -> None:
    source = payload["primary_source"]
    _require(source["doi"] == "10.1088/0305-4616/7/5/001", "Phase 2C2 source DOI drift")
    _require(source["peer_reviewed"] is True, "Phase 2C2 source must remain peer reviewed")
    _require(source["selected_surface"] == "4^4 lattice at beta=6.0", "Phase 2C2 source surface drift")
    benchmark = payload["benchmark"]
    _require(benchmark["gauge_group"] == "SU(3)" and benchmark["action"] == "Wilson", "Phase 2C2 theory surface drift")
    _require(benchmark["boundary_conditions"] == "periodic", "Phase 2C2 boundary-condition drift")
    _require(benchmark["extents"] == [4, 4, 4, 4] and benchmark["beta"] == 6.0, "Phase 2C2 geometry/coupling drift")
    _require(benchmark["source_envelope_min"] == 0.56 and benchmark["source_envelope_max"] == 0.63, "Phase 2C2 source envelope drift")
    _require(benchmark["source_envelope_is_paper_error_bar"] is False, "Phase 2C2 graphical envelope misrepresented")


def validate_v1_contract(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 V1 contract ID drift")
    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 failed status drift")
    _require(payload["depends_on"] == [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}], "Phase 2C2 V1 dependency drift")
    _validate_common_source_and_benchmark(payload)
    run = payload["reference_run"]
    _require(run["master_seed"] == "2c02198105010001", "Phase 2C2 V1 master seed drift")
    _require(run["chain_ids"] == [0, 1], "Phase 2C2 V1 chain identities drift")
    _require(run["derived_seeds"] == ["2ff3bf48550d1457", "ef7636a56e8ea29a"], "Phase 2C2 V1 derived seeds drift")
    _require(run["proposal_angle"] == 0.35, "Phase 2C2 V1 proposal angle drift")
    _require(run["thermalization_sweeps"] == 192 and run["measurements_per_chain"] == 128 and run["sweeps_between_measurements"] == 2 and run["total_sweeps_per_chain"] == 448, "Phase 2C2 V1 schedule drift")
    _require(run["minimum_ess_per_chain"] == 8.0, "Phase 2C2 V1 ESS gate drift")
    _require(run["maximum_chain_consistency_z"] == 5.0 and run["maximum_stationarity_z"] == 5.0, "Phase 2C2 V1 z-gate drift")
    _require(run["expected_metropolis_proposals_per_chain"] == 3670016, "Phase 2C2 V1 accounting drift")
    observed = payload["observed_evidence"]
    _require(observed["workflow_run_id"] == 34760576995, "Phase 2C2 V1 workflow evidence drift")
    _require(observed["evidence_head_commit"] == "0b669e9d517353fa21f7fe34f96ff630b59d8ad7", "Phase 2C2 V1 evidence commit drift")
    _require(observed["source_envelope_reproduced"] is True, "Phase 2C2 V1 source agreement erased")
    _require(observed["minimum_effective_sample_size"] == 4.51005377363487, "Phase 2C2 V1 observed ESS drift")
    _require(observed["minimum_effective_sample_size_required"] == 8.0, "Phase 2C2 V1 required ESS drift")
    _require(observed["known_failed_gate"] == "minimum_effective_sample_size_per_chain", "Phase 2C2 V1 failed-gate identity drift")
    _require(observed["parameter_retuning_after_result"] is False, "Phase 2C2 V1 post-result retuning admitted")
    gate = payload["phase2c_gate"]
    _require(gate["published_su3_baseline_reproduced"] is False, "Phase 2C2 V1 incorrectly promoted")
    _require(gate["phase2c_complete"] is False and gate["glueball_analysis_enabled"] is False, "Phase 2C2 V1 incorrectly graduated")
    _require(gate["result_status"] == "failed-predeclared-effective-sample-size-gate", "Phase 2C2 V1 failure status drift")
    _require(payload["claim_boundary"]["failed_statistical_gate_is_not_validation_success"] is True, "Phase 2C2 V1 failure boundary weakened")


def validate_v1_vectors(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 V1 vector contract drift")
    _require(payload["vector_set"] == "SU3-PLAQUETTE-BASELINE-PHASE2C2-V1", "Phase 2C2 V1 vector-set drift")
    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 vector failed status drift")
    by_id = {vector["id"]: vector for vector in payload["vectors"]}
    _require(by_id["published-source-envelope"]["expected_interval"] == [0.56, 0.63], "Phase 2C2 V1 vector source envelope drift")
    failure = by_id["observed-v1-failure"]
    _require(failure["source_envelope_reproduced"] is True, "Phase 2C2 V1 vector source agreement erased")
    _require(failure["minimum_effective_sample_size_observed"] == 4.51005377363487, "Phase 2C2 V1 vector ESS drift")
    _require(failure["minimum_effective_sample_size_required"] == 8.0, "Phase 2C2 V1 vector ESS threshold drift")
    _require(failure["v1_reclassified_as_pass"] is False and failure["parameter_retuning_after_result"] is False, "Phase 2C2 V1 vector failure rewritten")
    boundary = by_id["phase2c-v1-failure-boundary"]
    _require(boundary["published_su3_baseline_reproduced"] is False and boundary["phase2c_complete"] is False and boundary["glueball_analysis_enabled"] is False, "Phase 2C2 V1 vector boundary weakened")


def validate_v2_contract(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V2", "Phase 2C2 V2 contract ID drift")
    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 frozen status drift")
    _require(payload["depends_on"] == [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1", "required_status": "phase-2c2-v1-failed"},
    ], "Phase 2C2 V2 dependency drift")
    _validate_common_source_and_benchmark(payload)
    run = payload["reference_run"]
    _require(run["master_seed"] == "2c02198105010002", "Phase 2C2 V2 master seed drift")
    _require(run["chain_ids"] == [0, 1, 2, 3], "Phase 2C2 V2 chain identities drift")
    _require(run["derived_seeds"] == ["dfcdab179476617a", "151dd841ccef81c6", "9c28ededca3d1ae2", "17dbbf4a00214611"], "Phase 2C2 V2 derived seeds drift")
    _require(run["chain_count"] == 4, "Phase 2C2 V2 chain count drift")
    _require(run["proposal_angle"] == 0.35, "Phase 2C2 V2 proposal angle drift")
    _require(run["thermalization_sweeps"] == 192 and run["measurements_per_chain"] == 128 and run["sweeps_between_measurements"] == 2 and run["total_sweeps_per_chain"] == 448, "Phase 2C2 V2 schedule drift")
    _require(run["minimum_aggregate_effective_sample_size"] == 16.0, "Phase 2C2 V2 aggregate ESS gate drift")
    _require(run["maximum_pairwise_chain_consistency_z"] == 5.0 and run["maximum_stationarity_z"] == 5.0, "Phase 2C2 V2 z-gate drift")
    _require(run["expected_metropolis_proposals_per_chain"] == 3670016, "Phase 2C2 V2 accounting drift")
    pre = payload["predeclaration"]
    _require(pre["fresh_master_seed_and_streams"] is True, "Phase 2C2 V2 fresh-stream requirement weakened")
    _require(pre["source_surface_changed_after_v1"] is False and pre["source_envelope_changed_after_v1"] is False, "Phase 2C2 V2 source retuned after V1")
    _require(pre["sampler_algorithm_changed_after_v1"] is False and pre["proposal_angle_changed_after_v1"] is False and pre["per_chain_schedule_changed_after_v1"] is False, "Phase 2C2 V2 mechanics retuned after V1")
    _require(pre["v1_failed_result_reclassified_as_pass"] is False, "Phase 2C2 V1 failure reclassified")
    _require(pre["v2_parameters_frozen_before_v2_result_inspection"] is True, "Phase 2C2 V2 predeclaration weakened")
    observed = payload["observed_evidence"]
    _require(observed["workflow_run_id"] == 34761701867, "Phase 2C2 V2 workflow evidence drift")
    _require(observed["evidence_head_commit"] == "f5a221fdad2046779358cc59460fae488626459b", "Phase 2C2 V2 evidence commit drift")
    _require(observed["pooled_mean"] == 0.5992920042685975, "Phase 2C2 V2 pooled mean drift")
    _require(observed["aggregate_effective_sample_size"] == 78.94400476134997, "Phase 2C2 V2 aggregate ESS evidence drift")
    _require(observed["maximum_pairwise_chain_consistency_z_observed"] == 3.5504261086461475, "Phase 2C2 V2 pairwise-z evidence drift")
    _require(observed["maximum_stationarity_z_observed"] == 1.4650733209908853, "Phase 2C2 V2 stationarity evidence drift")
    _require(observed["maximum_link_unitarity_residual_observed"] == 3.1308289294429414e-14, "Phase 2C2 V2 unitarity evidence drift")
    _require(observed["maximum_link_determinant_residual_observed"] == 3.553115420753491e-14, "Phase 2C2 V2 determinant evidence drift")
    _require(observed["source_envelope_reproduced"] is True and observed["diagnostics_trustworthy"] is True, "Phase 2C2 V2 passing evidence weakened")
    _require(observed["v1_reclassified_as_pass"] is False and observed["parameter_retuning_after_v2_result"] is False, "Phase 2C2 provenance boundary weakened")
    _require(observed["aggregate_effective_sample_size"] >= run["minimum_aggregate_effective_sample_size"], "Phase 2C2 V2 aggregate ESS no longer passes")
    _require(observed["maximum_pairwise_chain_consistency_z_observed"] <= run["maximum_pairwise_chain_consistency_z"], "Phase 2C2 V2 pairwise-z no longer passes")
    _require(observed["maximum_stationarity_z_observed"] <= run["maximum_stationarity_z"], "Phase 2C2 V2 stationarity no longer passes")
    _require(observed["maximum_link_unitarity_residual_observed"] <= run["maximum_link_unitarity_residual"], "Phase 2C2 V2 unitarity no longer passes")
    _require(observed["maximum_link_determinant_residual_observed"] <= run["maximum_link_determinant_residual"], "Phase 2C2 V2 determinant no longer passes")
    gate = payload["phase2c_gate"]
    _require(gate["published_su3_baseline_reproduced"] is True and gate["phase2c_complete"] is True and gate["glueball_analysis_enabled"] is True, "Phase 2C2 V2 frozen gate weakened")
    _require(gate["result_status"] == "passed-predeclared-v2-confirmation", "Phase 2C2 V2 result status drift")
    boundary = payload["claim_boundary"]
    _require(boundary["benchmark_agreement_implies_confinement"] is False, "Phase 2C2 confinement overclaim")
    _require(boundary["benchmark_agreement_implies_continuum_qft"] is False, "Phase 2C2 continuum overclaim")
    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 mass-gap overclaim")


def validate_v2_vectors(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V2", "Phase 2C2 V2 vector contract drift")
    _require(payload["vector_set"] == "SU3-PLAQUETTE-BASELINE-PHASE2C2-V2", "Phase 2C2 V2 vector-set drift")
    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 vector frozen status drift")
    by_id = {vector["id"]: vector for vector in payload["vectors"]}
    _require(by_id["unchanged-published-source-envelope"]["expected_interval"] == [0.56, 0.63], "Phase 2C2 V2 vector source envelope drift")
    run = by_id["four-fresh-chain-confirmation"]
    _require(run["chain_ids"] == [0, 1, 2, 3], "Phase 2C2 V2 vector chain identities drift")
    _require(run["minimum_aggregate_effective_sample_size"] == 16.0, "Phase 2C2 V2 vector aggregate ESS drift")
    failure = by_id["v1-failure-preservation"]
    _require(failure["v1_result_status"] == "failed-predeclared-effective-sample-size-gate", "Phase 2C2 V2 vector V1 failure status drift")
    _require(failure["v1_reclassified_as_pass"] is False and failure["v2_uses_fresh_streams"] is True, "Phase 2C2 V2 vector independence boundary weakened")
    observed = by_id["observed-v2-confirmation"]
    _require(observed["workflow_run_id"] == 34761701867, "Phase 2C2 V2 vector workflow evidence drift")
    _require(observed["aggregate_effective_sample_size"] == 78.94400476134997, "Phase 2C2 V2 vector aggregate ESS evidence drift")
    _require(observed["published_su3_baseline_reproduced"] is True and observed["phase2c_complete"] is True, "Phase 2C2 V2 vector success evidence weakened")
    _require(observed["v1_reclassified_as_pass"] is False and observed["parameter_retuning_after_v2_result"] is False, "Phase 2C2 V2 vector provenance rewritten")
    boundary = by_id["phase2c-frozen-boundary"]
    _require(boundary["published_su3_baseline_reproduced"] is True and boundary["phase2c_complete"] is True and boundary["glueball_analysis_enabled"] is True, "Phase 2C2 frozen vector boundary weakened")
    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 frozen vector mass-gap overclaim")


def main() -> None:
    validate_v1_contract(load(V1_CONTRACT_PATH))
    validate_v1_vectors(load(V1_VECTOR_PATH))
    validate_v2_contract(load(V2_CONTRACT_PATH))
    validate_v2_vectors(load(V2_VECTOR_PATH))
    print("Phase 2C2 SU(3) V1 failure and frozen V2 validation checks: passed")


if __name__ == "__main__":
    main()
