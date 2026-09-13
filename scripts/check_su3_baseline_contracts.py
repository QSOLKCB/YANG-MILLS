#!/usr/bin/env python3
"""Fail-closed checks for Phase 2C2 SU(3) published-baseline authorities."""

from __future__ import annotations

from typing import Any

from check_contracts import exact_typed_equal, load

V1_CONTRACT_PATH = "contracts/YM-SU3-PLAQUETTE-BASELINE-V1.json"
V1_VECTOR_PATH = "test-vectors/su3-plaquette-baseline-v1.json"
V2_CONTRACT_PATH = "contracts/YM-SU3-PLAQUETTE-BASELINE-V2.json"
V2_VECTOR_PATH = "test-vectors/su3-plaquette-baseline-v2.json"

EXPECTED_OBSERVABLE = "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/3)ReTr P_mu_nu(x)"
EXPECTED_NUMERIC_EXTRACTION = {
    "YM-SU3-PLAQUETTE-BASELINE-V1": "conservative graphical source-resolution envelope for the equivalent normalized average plaquette; not a quoted tabulated statistical error",
    "YM-SU3-PLAQUETTE-BASELINE-V2": "the same conservative graphical source-resolution envelope frozen before the V1 result was inspected",
}
EXPECTED_SCOPES = {
    "YM-SU3-PLAQUETTE-BASELINE-V1": "first predeclared independent published finite-lattice validation attempt of the frozen Phase 2C1 pure-SU(3) sampler",
    "YM-SU3-PLAQUETTE-BASELINE-V2": "fresh-stream confirmatory published finite-lattice validation after the V1 attempt reproduced the source envelope but failed its predeclared per-chain ESS gate",
}
EXPECTED_PRIMARY_SOURCES = {
    "YM-SU3-PLAQUETTE-BASELINE-V1": {
        "title": "The specific heat of SU(3) lattice gauge theory",
        "authors": ["R. C. Edgar", "L. McCrossen", "K. J. M. Moriarty"],
        "journal": "Journal of Physics G: Nuclear Physics 7 (1981) L85-L88",
        "doi": "10.1088/0305-4616/7/5/001",
        "peer_reviewed": True,
        "source_surface": "four-dimensional pure SU(3) average action/plaquette beta scan on 4^4 and 5^4 lattices; the 4^4 calculation averages over 400 iterations through the lattice",
        "selected_surface": "4^4 lattice at beta=6.0",
        "numeric_extraction": EXPECTED_NUMERIC_EXTRACTION["YM-SU3-PLAQUETTE-BASELINE-V1"],
    },
    "YM-SU3-PLAQUETTE-BASELINE-V2": {
        "title": "The specific heat of SU(3) lattice gauge theory",
        "authors": ["R. C. Edgar", "L. McCrossen", "K. J. M. Moriarty"],
        "journal": "Journal of Physics G: Nuclear Physics 7 (1981) L85-L88",
        "doi": "10.1088/0305-4616/7/5/001",
        "peer_reviewed": True,
        "selected_surface": "4^4 lattice at beta=6.0",
        "numeric_extraction": EXPECTED_NUMERIC_EXTRACTION["YM-SU3-PLAQUETTE-BASELINE-V2"],
    },
}
EXPECTED_RUNTIME_IDENTITY = {
    "sampler_id": "YM-SU3-ENSEMBLE-V1",
    "proposal_id": "YM-SU3-GENERATOR-METROPOLIS-V1",
    "rng_id": "SPLITMIX64-V1",
    "stream_id": "YM-SU3-STREAM-V1",
}
EXPECTED_V1_REPLAY_TOLERANCE = {
    "summary_absolute": 1e-12,
    "summary_relative": 1e-10,
}
EXPECTED_V2_REPLAY_TOLERANCE = {
    "summary_absolute": 1e-12,
    "summary_relative": 1e-10,
    "residual_absolute": 1e-13,
}
EXPECTED_DIAGNOSTIC_PARAMETERS = {
    "autocorrelation_max_lag": 24,
    "stationarity_half_measurements": 64,
    "stationarity_autocorrelation_max_lag": 12,
    "minimum_metropolis_acceptance": 0.1,
    "maximum_metropolis_acceptance": 0.95,
    "expected_metropolis_proposals_per_chain": 3670016,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
}
V1_COMPARISON_RULE = {
    "pooled_mean_must_lie_inside_source_envelope": True,
    "independent_chain_identities_required": True,
    "raw_measurements_retained": True,
    "diagnostics_recomputed_from_raw_measurements": True,
    "within_chain_stationarity_required": True,
    "stationarity_rule": "for each chain compare the first 64 and last 64 post-thermalization measurements using the frozen Phase 2A diagnostic arithmetic with max_lag=12; require |mean_first-mean_second|/sqrt(SE_first^2+SE_second^2) <= 5; reject a constant half or zero combined SE scale",
    "metropolis_accounting_must_match_frozen_schedule": True,
    "acceptance_window_is_diagnostic_not_source_claim": True,
    "source_envelope_is_paper_error_bar": False,
}
V2_COMPARISON_RULE = {
    "pooled_mean_must_lie_inside_source_envelope": True,
    "all_four_chain_identities_must_be_distinct": True,
    "all_raw_measurement_series_must_be_distinct": True,
    "raw_measurements_retained": True,
    "diagnostics_recomputed_from_raw_measurements": True,
    "each_chain_ess_must_be_finite_and_positive": True,
    "aggregate_ess_must_meet_frozen_minimum": True,
    "all_pairwise_chain_consistency_z_must_meet_frozen_maximum": True,
    "within_chain_stationarity_required": True,
    "metropolis_accounting_must_match_frozen_schedule": True,
    "acceptance_window_is_diagnostic_not_source_claim": True,
    "source_envelope_is_paper_error_bar": False,
}
V1_PREDECLARATION = {
    "selected_after_phase2c1_freeze": True,
    "benchmark_parameters_frozen_before_reference_result_inspection": True,
    "phase2c1_sampler_contract_mutated": False,
}
V1_OBSERVED_EVIDENCE = {
    "workflow_run_id": 34760576995,
    "evidence_head_commit": "0b669e9d517353fa21f7fe34f96ff630b59d8ad7",
    "pooled_mean": 0.6018083560823128,
    "source_envelope_reproduced": True,
    "minimum_effective_sample_size": 4.51005377363487,
    "minimum_effective_sample_size_required": 8.0,
    "chain_consistency_z": 0.4307895037511404,
    "maximum_chain_consistency_z": 5.0,
    "maximum_stationarity_z_observed": 3.7834285113688733,
    "maximum_stationarity_z_allowed": 5.0,
    "chain_acceptances": [0.867445537022182, 0.8678583417619977],
    "known_failed_gate": "minimum_effective_sample_size_per_chain",
    "parameter_retuning_after_result": False,
}
V1_GATE = {
    "published_su3_baseline_reproduced": False,
    "phase2c_complete": False,
    "glueball_analysis_enabled": False,
    "result_status": "failed-predeclared-effective-sample-size-gate",
}
V1_CLAIM_BOUNDARY = {
    "benchmark_agreement_is_finite_lattice_validation": True,
    "failed_statistical_gate_is_not_validation_success": True,
    "benchmark_agreement_implies_confinement": False,
    "benchmark_agreement_implies_continuum_qft": False,
    "benchmark_agreement_implies_positive_mass_gap": False,
    "glueball_analysis_authorization_is_not_a_glueball_result": True,
}
V2_PREDECLARATION = {
    "designed_after_v1_statistical_power_failure": True,
    "fresh_master_seed_and_streams": True,
    "source_surface_changed_after_v1": False,
    "source_envelope_changed_after_v1": False,
    "sampler_algorithm_changed_after_v1": False,
    "proposal_angle_changed_after_v1": False,
    "per_chain_schedule_changed_after_v1": False,
    "v1_failed_result_reclassified_as_pass": False,
    "v2_parameters_frozen_before_v2_result_inspection": True,
    "phase2c1_sampler_contract_mutated": False,
}
V2_GATE = {
    "published_su3_baseline_reproduced": True,
    "phase2c_complete": True,
    "glueball_analysis_enabled": True,
    "result_status": "passed-predeclared-v2-confirmation",
}
V2_CLAIM_BOUNDARY = {
    "successful_v2_is_finite_lattice_validation": True,
    "v1_source_agreement_overrides_v1_ess_failure": False,
    "benchmark_agreement_implies_confinement": False,
    "benchmark_agreement_implies_continuum_qft": False,
    "benchmark_agreement_implies_positive_mass_gap": False,
    "glueball_analysis_authorization_is_not_a_glueball_result": True,
}
EXPECTED_BENCHMARK = {
    "gauge_group": "SU(3)",
    "action": "Wilson",
    "boundary_conditions": "periodic",
    "extents": [4, 4, 4, 4],
    "beta": 6.0,
    "observable": EXPECTED_OBSERVABLE,
    "source_envelope_min": 0.56,
    "source_envelope_max": 0.63,
    "source_envelope_is_paper_error_bar": False,
}
V1_REFERENCE_RUN = {
    **EXPECTED_RUNTIME_IDENTITY,
    "master_seed": "2c02198105010001",
    "chain_ids": [0, 1],
    "derived_seeds": ["2ff3bf48550d1457", "ef7636a56e8ea29a"],
    "proposal_angle": 0.35,
    "thermalization_sweeps": 192,
    "measurements_per_chain": 128,
    "sweeps_between_measurements": 2,
    "total_sweeps_per_chain": 448,
    "autocorrelation_max_lag": 24,
    "minimum_ess_per_chain": 8.0,
    "maximum_chain_consistency_z": 5.0,
    "stationarity_half_measurements": 64,
    "stationarity_autocorrelation_max_lag": 12,
    "maximum_stationarity_z": 5.0,
    "minimum_metropolis_acceptance": 0.1,
    "maximum_metropolis_acceptance": 0.95,
    "expected_metropolis_proposals_per_chain": 3670016,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
}
V2_REFERENCE_RUN = {
    **EXPECTED_RUNTIME_IDENTITY,
    "master_seed": "2c02198105010002",
    "chain_ids": [0, 1, 2, 3],
    "derived_seeds": [
        "dfcdab179476617a",
        "151dd841ccef81c6",
        "9c28ededca3d1ae2",
        "17dbbf4a00214611",
    ],
    "proposal_angle": 0.35,
    "thermalization_sweeps": 192,
    "measurements_per_chain": 128,
    "sweeps_between_measurements": 2,
    "total_sweeps_per_chain": 448,
    "autocorrelation_max_lag": 24,
    "chain_count": 4,
    "minimum_aggregate_effective_sample_size": 16.0,
    "aggregate_ess_rationale": "preserves the V1 design's intended total minimum statistical information of two chains times ESS 8 while adding fresh independent streams rather than lowering the failed V1 threshold",
    "maximum_pairwise_chain_consistency_z": 5.0,
    "stationarity_half_measurements": 64,
    "stationarity_autocorrelation_max_lag": 12,
    "maximum_stationarity_z": 5.0,
    "minimum_metropolis_acceptance": 0.1,
    "maximum_metropolis_acceptance": 0.95,
    "expected_metropolis_proposals_per_chain": 3670016,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
}
V2_OBSERVED_EVIDENCE = {
    "workflow_run_id": 34761701867,
    "evidence_head_commit": "f5a221fdad2046779358cc59460fae488626459b",
    "pooled_mean": 0.5992920042685975,
    "pooled_standard_error": 0.0008268453534905086,
    "chain_means": [
        0.5951228942049566,
        0.6010578916054883,
        0.6006811934765629,
        0.6003060377873822,
    ],
    "chain_standard_errors": [
        0.00073731922680896,
        0.0017334647942562633,
        0.0013810304643338124,
        0.002341577852401104,
    ],
    "chain_effective_sample_sizes": [
        37.108298427827265,
        12.12458490228564,
        19.780522239783117,
        9.930599191453949,
    ],
    "aggregate_effective_sample_size": 78.94400476134997,
    "chain_acceptances": [
        0.8675000326974052,
        0.8676090240478516,
        0.867549078805106,
        0.867567607334682,
    ],
    "stationarity_z": [
        1.4650733209908853,
        1.2503134066297152,
        0.6586970772179463,
        0.6522302609275793,
    ],
    "maximum_pairwise_chain_consistency_z_observed": 3.5504261086461475,
    "maximum_stationarity_z_observed": 1.4650733209908853,
    "maximum_link_unitarity_residual_observed": 3.1308289294429414e-14,
    "maximum_link_determinant_residual_observed": 3.553115420753491e-14,
    "source_envelope_reproduced": True,
    "diagnostics_trustworthy": True,
    "v1_reclassified_as_pass": False,
    "parameter_retuning_after_v2_result": False,
}
EXPECTED_V1_CONTRACT = {
    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1",
    "status": "phase-2c2-v1-failed",
    "scope": EXPECTED_SCOPES["YM-SU3-PLAQUETTE-BASELINE-V1"],
    "depends_on": [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}],
    "primary_source": EXPECTED_PRIMARY_SOURCES["YM-SU3-PLAQUETTE-BASELINE-V1"],
    "benchmark": EXPECTED_BENCHMARK,
    "reference_run": V1_REFERENCE_RUN,
    "comparison_rule": V1_COMPARISON_RULE,
    "predeclaration": V1_PREDECLARATION,
    "observed_evidence": V1_OBSERVED_EVIDENCE,
    "phase2c_gate": V1_GATE,
    "claim_boundary": V1_CLAIM_BOUNDARY,
    "replay_tolerance": EXPECTED_V1_REPLAY_TOLERANCE,
}
EXPECTED_V2_CONTRACT = {
    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2",
    "status": "phase-2c2-frozen",
    "scope": EXPECTED_SCOPES["YM-SU3-PLAQUETTE-BASELINE-V2"],
    "depends_on": [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1", "required_status": "phase-2c2-v1-failed"},
    ],
    "primary_source": EXPECTED_PRIMARY_SOURCES["YM-SU3-PLAQUETTE-BASELINE-V2"],
    "benchmark": EXPECTED_BENCHMARK,
    "reference_run": V2_REFERENCE_RUN,
    "comparison_rule": V2_COMPARISON_RULE,
    "predeclaration": V2_PREDECLARATION,
    "observed_evidence": V2_OBSERVED_EVIDENCE,
    "phase2c_gate": V2_GATE,
    "claim_boundary": V2_CLAIM_BOUNDARY,
    "replay_tolerance": EXPECTED_V2_REPLAY_TOLERANCE,
}
V1_VECTOR_SOURCE = {
    "id": "published-source-envelope",
    "doi": "10.1088/0305-4616/7/5/001",
    "source_lattice": [4, 4, 4, 4],
    "source_iterations": 400,
    "beta": 6.0,
    "observable": EXPECTED_OBSERVABLE,
    "expected_interval": [0.56, 0.63],
    "interval_semantics": "conservative graphical source-resolution envelope for the equivalent normalized plaquette, not a quoted paper error bar",
    "claim_class": "C0",
}
V1_VECTOR_PREDECLARATION = {
    "id": "two-independent-chain-predeclaration",
    "master_seed": "2c02198105010001",
    "chain_ids": [0, 1],
    "derived_seeds": ["2ff3bf48550d1457", "ef7636a56e8ea29a"],
    "proposal_angle": 0.35,
    "thermalization_sweeps": 192,
    "measurements_per_chain": 128,
    "sweeps_between_measurements": 2,
    "total_sweeps_per_chain": 448,
    "expected_metropolis_proposals_per_chain": 3670016,
    "minimum_ess_per_chain": 8.0,
    "maximum_chain_consistency_z": 5.0,
    "minimum_metropolis_acceptance": 0.1,
    "maximum_metropolis_acceptance": 0.95,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
    "claim_class": "C0",
}
V1_VECTOR_OBSERVED_EVIDENCE = {
    "id": "observed-v1-failure",
    "workflow_run_id": 34760576995,
    "evidence_head_commit": "0b669e9d517353fa21f7fe34f96ff630b59d8ad7",
    "pooled_mean": 0.6018083560823128,
    "source_envelope_reproduced": True,
    "minimum_effective_sample_size_observed": 4.51005377363487,
    "minimum_effective_sample_size_required": 8.0,
    "chain_consistency_z": 0.4307895037511404,
    "maximum_stationarity_z_observed": 3.7834285113688733,
    "chain_acceptances": [0.867445537022182, 0.8678583417619977],
    "failed_gate": "minimum_effective_sample_size_per_chain",
    "parameter_retuning_after_result": False,
    "v1_reclassified_as_pass": False,
    "claim_class": "C2",
    "replay_tolerance": EXPECTED_V1_REPLAY_TOLERANCE,
}
V1_VECTOR_BOUNDARY = {
    "id": "phase2c-v1-failure-boundary",
    "published_su3_baseline_reproduced": False,
    "phase2c_complete": False,
    "glueball_analysis_enabled": False,
    "benchmark_agreement_implies_confinement": False,
    "benchmark_agreement_implies_continuum_qft": False,
    "benchmark_agreement_implies_positive_mass_gap": False,
    "claim_class": "C0",
}
V2_VECTOR_SOURCE = {
    "id": "unchanged-published-source-envelope",
    "doi": "10.1088/0305-4616/7/5/001",
    "source_lattice": [4, 4, 4, 4],
    "beta": 6.0,
    "expected_interval": [0.56, 0.63],
    "source_envelope_changed_after_v1": False,
    "claim_class": "C0",
}
V2_VECTOR_CONFIRMATION = {
    "id": "four-fresh-chain-confirmation",
    "master_seed": "2c02198105010002",
    "chain_ids": [0, 1, 2, 3],
    "derived_seeds": [
        "dfcdab179476617a",
        "151dd841ccef81c6",
        "9c28ededca3d1ae2",
        "17dbbf4a00214611",
    ],
    "proposal_angle": 0.35,
    "thermalization_sweeps": 192,
    "measurements_per_chain": 128,
    "sweeps_between_measurements": 2,
    "total_sweeps_per_chain": 448,
    "expected_metropolis_proposals_per_chain": 3670016,
    "minimum_aggregate_effective_sample_size": 16.0,
    "maximum_pairwise_chain_consistency_z": 5.0,
    "maximum_stationarity_z": 5.0,
    "minimum_metropolis_acceptance": 0.1,
    "maximum_metropolis_acceptance": 0.95,
    "maximum_link_unitarity_residual": 5e-12,
    "maximum_link_determinant_residual": 5e-12,
    "expected": "all four fresh chains pass accounting, group-residual, acceptance, stationarity and pairwise consistency gates; aggregate ESS is at least 16; pooled mean lies inside the unchanged published envelope",
    "claim_class": "C2",
    "replay_tolerance": EXPECTED_V2_REPLAY_TOLERANCE,
}
V2_VECTOR_V1_PRESERVATION = {
    "id": "v1-failure-preservation",
    "v1_result_status": "failed-predeclared-effective-sample-size-gate",
    "v1_minimum_effective_sample_size_observed": 4.51005377363487,
    "v1_minimum_effective_sample_size_required": 8.0,
    "v1_reclassified_as_pass": False,
    "v2_uses_fresh_streams": True,
    "v2_preserves_original_intended_total_ess_target": 16.0,
    "claim_class": "C0",
}
V2_VECTOR_OBSERVED_EVIDENCE = {
    "id": "observed-v2-confirmation",
    "workflow_run_id": 34761701867,
    "evidence_head_commit": "f5a221fdad2046779358cc59460fae488626459b",
    "pooled_mean": 0.5992920042685975,
    "pooled_standard_error": 0.0008268453534905086,
    "chain_means": [
        0.5951228942049566,
        0.6010578916054883,
        0.6006811934765629,
        0.6003060377873822,
    ],
    "chain_effective_sample_sizes": [
        37.108298427827265,
        12.12458490228564,
        19.780522239783117,
        9.930599191453949,
    ],
    "aggregate_effective_sample_size": 78.94400476134997,
    "maximum_pairwise_chain_consistency_z_observed": 3.5504261086461475,
    "maximum_stationarity_z_observed": 1.4650733209908853,
    "source_envelope_reproduced": True,
    "diagnostics_trustworthy": True,
    "published_su3_baseline_reproduced": True,
    "phase2c_complete": True,
    "v1_reclassified_as_pass": False,
    "parameter_retuning_after_v2_result": False,
    "claim_class": "C2",
}
V2_VECTOR_BOUNDARY = {
    "id": "phase2c-frozen-boundary",
    "published_su3_baseline_reproduced": True,
    "phase2c_complete": True,
    "glueball_analysis_enabled": True,
    "benchmark_agreement_implies_confinement": False,
    "benchmark_agreement_implies_continuum_qft": False,
    "benchmark_agreement_implies_positive_mass_gap": False,
    "claim_class": "C0",
}
EXPECTED_V1_VECTOR_PAYLOAD = {
    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1",
    "vector_set": "SU3-PLAQUETTE-BASELINE-PHASE2C2-V1",
    "status": "phase-2c2-v1-failed",
    "vectors": [
        V1_VECTOR_SOURCE,
        V1_VECTOR_PREDECLARATION,
        V1_VECTOR_OBSERVED_EVIDENCE,
        V1_VECTOR_BOUNDARY,
    ],
}
EXPECTED_V2_VECTOR_PAYLOAD = {
    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2",
    "vector_set": "SU3-PLAQUETTE-BASELINE-PHASE2C2-V2",
    "status": "phase-2c2-frozen",
    "vectors": [
        V2_VECTOR_SOURCE,
        V2_VECTOR_CONFIRMATION,
        V2_VECTOR_V1_PRESERVATION,
        V2_VECTOR_OBSERVED_EVIDENCE,
        V2_VECTOR_BOUNDARY,
    ],
}


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def _validate_common_source_and_benchmark(payload: dict[str, Any]) -> None:
    source = payload["primary_source"]
    expected_source = EXPECTED_PRIMARY_SOURCES[payload["contract_id"]]
    _require(source == expected_source, "Phase 2C2 complete primary-source provenance drift")
    benchmark = payload["benchmark"]
    _require(benchmark["gauge_group"] == "SU(3)" and benchmark["action"] == "Wilson", "Phase 2C2 theory surface drift")
    _require(benchmark["boundary_conditions"] == "periodic", "Phase 2C2 boundary-condition drift")
    _require(benchmark["extents"] == [4, 4, 4, 4] and benchmark["beta"] == 6.0, "Phase 2C2 geometry/coupling drift")
    _require(benchmark["observable"] == EXPECTED_OBSERVABLE, "Phase 2C2 observable/normalization drift")
    _require(benchmark["source_envelope_min"] == 0.56 and benchmark["source_envelope_max"] == 0.63, "Phase 2C2 source envelope drift")
    _require(benchmark["source_envelope_is_paper_error_bar"] is False, "Phase 2C2 graphical envelope misrepresented")


def _validate_runtime_identity(run: dict[str, Any], label: str) -> None:
    for field, expected in EXPECTED_RUNTIME_IDENTITY.items():
        _require(run[field] == expected, f"{label} {field} drift")


def _validate_diagnostic_parameters(run: dict[str, Any], label: str) -> None:
    for field, expected in EXPECTED_DIAGNOSTIC_PARAMETERS.items():
        _require(run[field] == expected, f"{label} {field} drift")


def validate_v1_contract(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 V1 contract ID drift")
    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 failed status drift")
    _require(payload["scope"] == EXPECTED_SCOPES[payload["contract_id"]], "Phase 2C2 V1 scope/claim drift")
    _require(payload["depends_on"] == [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}], "Phase 2C2 V1 dependency drift")
    _validate_common_source_and_benchmark(payload)
    run = payload["reference_run"]
    _validate_runtime_identity(run, "Phase 2C2 V1 runtime")
    _validate_diagnostic_parameters(run, "Phase 2C2 V1 runtime")
    _require(run["master_seed"] == "2c02198105010001", "Phase 2C2 V1 master seed drift")
    _require(run["chain_ids"] == [0, 1], "Phase 2C2 V1 chain identities drift")
    _require(run["derived_seeds"] == ["2ff3bf48550d1457", "ef7636a56e8ea29a"], "Phase 2C2 V1 derived seeds drift")
    _require(run["proposal_angle"] == 0.35, "Phase 2C2 V1 proposal angle drift")
    _require(run["thermalization_sweeps"] == 192 and run["measurements_per_chain"] == 128 and run["sweeps_between_measurements"] == 2 and run["total_sweeps_per_chain"] == 448, "Phase 2C2 V1 schedule drift")
    _require(run["minimum_ess_per_chain"] == 8.0, "Phase 2C2 V1 ESS gate drift")
    _require(run["maximum_chain_consistency_z"] == 5.0 and run["maximum_stationarity_z"] == 5.0, "Phase 2C2 V1 z-gate drift")
    _require(payload["comparison_rule"] == V1_COMPARISON_RULE, "Phase 2C2 V1 comparison-rule drift")
    _require(payload["replay_tolerance"] == EXPECTED_V1_REPLAY_TOLERANCE, "Phase 2C2 V1 replay-tolerance drift")
    _require(payload["predeclaration"] == V1_PREDECLARATION, "Phase 2C2 V1 predeclaration drift")
    _require(payload["observed_evidence"] == V1_OBSERVED_EVIDENCE, "Phase 2C2 V1 observed evidence drift")
    _require(payload["phase2c_gate"] == V1_GATE, "Phase 2C2 V1 gate drift")
    _require(payload["claim_boundary"] == V1_CLAIM_BOUNDARY, "Phase 2C2 V1 claim boundary drift")
    _require(exact_typed_equal(payload, EXPECTED_V1_CONTRACT), "Phase 2C2 V1 closed typed-schema drift")


def validate_v1_vectors(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V1", "Phase 2C2 V1 vector contract drift")
    _require(payload["vector_set"] == "SU3-PLAQUETTE-BASELINE-PHASE2C2-V1", "Phase 2C2 V1 vector-set drift")
    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 vector failed status drift")
    vectors = payload["vectors"]
    by_id = {vector["id"]: vector for vector in vectors}
    _require(len(vectors) == 4 and len(by_id) == 4, "Phase 2C2 V1 vector cardinality/identity drift")
    _require(set(by_id) == {"published-source-envelope", "two-independent-chain-predeclaration", "observed-v1-failure", "phase2c-v1-failure-boundary"}, "Phase 2C2 V1 vector IDs drift")
    _require(by_id["published-source-envelope"] == V1_VECTOR_SOURCE, "Phase 2C2 V1 source vector drift")
    _require(by_id["two-independent-chain-predeclaration"] == V1_VECTOR_PREDECLARATION, "Phase 2C2 V1 predeclaration vector drift")
    _require(by_id["observed-v1-failure"] == V1_VECTOR_OBSERVED_EVIDENCE, "Phase 2C2 V1 complete observed-evidence vector drift")
    _require(by_id["phase2c-v1-failure-boundary"] == V1_VECTOR_BOUNDARY, "Phase 2C2 V1 claim-boundary vector drift")
    _require(exact_typed_equal(payload, EXPECTED_V1_VECTOR_PAYLOAD), "Phase 2C2 V1 vector closed typed-schema drift")


def validate_v2_contract(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V2", "Phase 2C2 V2 contract ID drift")
    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 frozen status drift")
    _require(payload["scope"] == EXPECTED_SCOPES[payload["contract_id"]], "Phase 2C2 V2 scope/claim drift")
    _require(payload["depends_on"] == [
        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},
        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1", "required_status": "phase-2c2-v1-failed"},
    ], "Phase 2C2 V2 dependency drift")
    _validate_common_source_and_benchmark(payload)
    run = payload["reference_run"]
    _validate_runtime_identity(run, "Phase 2C2 V2 runtime")
    _validate_diagnostic_parameters(run, "Phase 2C2 V2 runtime")
    _require(run["master_seed"] == "2c02198105010002", "Phase 2C2 V2 master seed drift")
    _require(run["chain_ids"] == [0, 1, 2, 3], "Phase 2C2 V2 chain identities drift")
    _require(run["derived_seeds"] == ["dfcdab179476617a", "151dd841ccef81c6", "9c28ededca3d1ae2", "17dbbf4a00214611"], "Phase 2C2 V2 derived seeds drift")
    _require(run["chain_count"] == 4, "Phase 2C2 V2 chain count drift")
    _require(run["proposal_angle"] == 0.35, "Phase 2C2 V2 proposal angle drift")
    _require(run["thermalization_sweeps"] == 192 and run["measurements_per_chain"] == 128 and run["sweeps_between_measurements"] == 2 and run["total_sweeps_per_chain"] == 448, "Phase 2C2 V2 schedule drift")
    _require(run["minimum_aggregate_effective_sample_size"] == 16.0, "Phase 2C2 V2 aggregate ESS gate drift")
    _require(run["maximum_pairwise_chain_consistency_z"] == 5.0 and run["maximum_stationarity_z"] == 5.0, "Phase 2C2 V2 z-gate drift")
    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")
    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")
    _require(payload["predeclaration"] == V2_PREDECLARATION, "Phase 2C2 V2 predeclaration drift")
    observed = payload["observed_evidence"]
    _require(observed["workflow_run_id"] == 34761701867, "Phase 2C2 V2 workflow evidence drift")
    _require(observed["evidence_head_commit"] == "f5a221fdad2046779358cc59460fae488626459b", "Phase 2C2 V2 evidence commit drift")
    _require(observed["pooled_mean"] == 0.5992920042685975, "Phase 2C2 V2 pooled mean drift")
    _require(observed["pooled_standard_error"] == 0.0008268453534905086, "Phase 2C2 V2 pooled standard-error drift")
    expected_arrays = {
        "chain_means": [0.5951228942049566, 0.6010578916054883, 0.6006811934765629, 0.6003060377873822],
        "chain_standard_errors": [0.00073731922680896, 0.0017334647942562633, 0.0013810304643338124, 0.0023415778524011042],
        "chain_effective_sample_sizes": [37.108298427827265, 12.12458490228564, 19.780522239783117, 9.930599191453949],
        "chain_acceptances": [0.8675000326974052, 0.8676090240478516, 0.867549078805106, 0.867567607334682],
        "stationarity_z": [1.4650733209908853, 1.2503134066297152, 0.6586970772179463, 0.6522302609275793],
    }
    for field, expected in expected_arrays.items():
        value = observed[field]
        _require(isinstance(value, list) and len(value) == 4 and value == expected, f"Phase 2C2 V2 {field} evidence drift")
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
    _require(payload["phase2c_gate"] == V2_GATE, "Phase 2C2 V2 frozen gate drift")
    _require(payload["claim_boundary"] == V2_CLAIM_BOUNDARY, "Phase 2C2 V2 claim boundary drift")
    _require(exact_typed_equal(payload, EXPECTED_V2_CONTRACT), "Phase 2C2 V2 closed typed-schema drift")


def validate_v2_vectors(payload: dict[str, Any]) -> None:
    _require(payload["contract_id"] == "YM-SU3-PLAQUETTE-BASELINE-V2", "Phase 2C2 V2 vector contract drift")
    _require(payload["vector_set"] == "SU3-PLAQUETTE-BASELINE-PHASE2C2-V2", "Phase 2C2 V2 vector-set drift")
    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 vector frozen status drift")
    vectors = payload["vectors"]
    by_id = {vector["id"]: vector for vector in vectors}
    _require(len(vectors) == 5 and len(by_id) == 5, "Phase 2C2 V2 vector cardinality/identity drift")
    _require(set(by_id) == {"unchanged-published-source-envelope", "four-fresh-chain-confirmation", "v1-failure-preservation", "observed-v2-confirmation", "phase2c-frozen-boundary"}, "Phase 2C2 V2 vector IDs drift")
    _require(by_id["unchanged-published-source-envelope"] == V2_VECTOR_SOURCE, "Phase 2C2 V2 source vector drift")
    _require(by_id["four-fresh-chain-confirmation"] == V2_VECTOR_CONFIRMATION, "Phase 2C2 V2 complete confirmation vector drift")
    _require(by_id["v1-failure-preservation"] == V2_VECTOR_V1_PRESERVATION, "Phase 2C2 V2 V1-preservation vector drift")
    _require(by_id["observed-v2-confirmation"] == V2_VECTOR_OBSERVED_EVIDENCE, "Phase 2C2 V2 complete observed-evidence vector drift")
    _require(by_id["phase2c-frozen-boundary"] == V2_VECTOR_BOUNDARY, "Phase 2C2 V2 claim-boundary vector drift")
    _require(exact_typed_equal(payload, EXPECTED_V2_VECTOR_PAYLOAD), "Phase 2C2 V2 vector closed typed-schema drift")


def main() -> None:
    validate_v1_contract(load(V1_CONTRACT_PATH))
    validate_v1_vectors(load(V1_VECTOR_PATH))
    validate_v2_contract(load(V2_CONTRACT_PATH))
    validate_v2_vectors(load(V2_VECTOR_PATH))
    print("Phase 2C2 SU(3) V1 failure and frozen V2 validation checks: passed")


if __name__ == "__main__":
    main()
