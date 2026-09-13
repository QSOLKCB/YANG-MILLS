#!/usr/bin/env python3
"""One-shot patch for closed typed Phase 2C2 contract validation."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:180]!r}")
    path.write_text(text.replace(old, new, 1))


replace_once(
    "scripts/check_su3_baseline_contracts.py",
    "from check_contracts import load\n",
    "from check_contracts import exact_typed_equal, load\n",
)

replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''V2_CLAIM_BOUNDARY = {\n    "successful_v2_is_finite_lattice_validation": True,\n    "v1_source_agreement_overrides_v1_ess_failure": False,\n    "benchmark_agreement_implies_confinement": False,\n    "benchmark_agreement_implies_continuum_qft": False,\n    "benchmark_agreement_implies_positive_mass_gap": False,\n    "glueball_analysis_authorization_is_not_a_glueball_result": True,\n}\nV1_VECTOR_SOURCE = {\n''',
    '''V2_CLAIM_BOUNDARY = {\n    "successful_v2_is_finite_lattice_validation": True,\n    "v1_source_agreement_overrides_v1_ess_failure": False,\n    "benchmark_agreement_implies_confinement": False,\n    "benchmark_agreement_implies_continuum_qft": False,\n    "benchmark_agreement_implies_positive_mass_gap": False,\n    "glueball_analysis_authorization_is_not_a_glueball_result": True,\n}\nEXPECTED_BENCHMARK = {\n    "gauge_group": "SU(3)",\n    "action": "Wilson",\n    "boundary_conditions": "periodic",\n    "extents": [4, 4, 4, 4],\n    "beta": 6.0,\n    "observable": EXPECTED_OBSERVABLE,\n    "source_envelope_min": 0.56,\n    "source_envelope_max": 0.63,\n    "source_envelope_is_paper_error_bar": False,\n}\nV1_REFERENCE_RUN = {\n    **EXPECTED_RUNTIME_IDENTITY,\n    "master_seed": "2c02198105010001",\n    "chain_ids": [0, 1],\n    "derived_seeds": ["2ff3bf48550d1457", "ef7636a56e8ea29a"],\n    "proposal_angle": 0.35,\n    "thermalization_sweeps": 192,\n    "measurements_per_chain": 128,\n    "sweeps_between_measurements": 2,\n    "total_sweeps_per_chain": 448,\n    "autocorrelation_max_lag": 24,\n    "minimum_ess_per_chain": 8.0,\n    "maximum_chain_consistency_z": 5.0,\n    "stationarity_half_measurements": 64,\n    "stationarity_autocorrelation_max_lag": 12,\n    "maximum_stationarity_z": 5.0,\n    "minimum_metropolis_acceptance": 0.1,\n    "maximum_metropolis_acceptance": 0.95,\n    "expected_metropolis_proposals_per_chain": 3670016,\n    "maximum_link_unitarity_residual": 5e-12,\n    "maximum_link_determinant_residual": 5e-12,\n}\nV2_REFERENCE_RUN = {\n    **EXPECTED_RUNTIME_IDENTITY,\n    "master_seed": "2c02198105010002",\n    "chain_ids": [0, 1, 2, 3],\n    "derived_seeds": [\n        "dfcdab179476617a",\n        "151dd841ccef81c6",\n        "9c28ededca3d1ae2",\n        "17dbbf4a00214611",\n    ],\n    "proposal_angle": 0.35,\n    "thermalization_sweeps": 192,\n    "measurements_per_chain": 128,\n    "sweeps_between_measurements": 2,\n    "total_sweeps_per_chain": 448,\n    "autocorrelation_max_lag": 24,\n    "chain_count": 4,\n    "minimum_aggregate_effective_sample_size": 16.0,\n    "aggregate_ess_rationale": "preserves the V1 design's intended total minimum statistical information of two chains times ESS 8 while adding fresh independent streams rather than lowering the failed V1 threshold",\n    "maximum_pairwise_chain_consistency_z": 5.0,\n    "stationarity_half_measurements": 64,\n    "stationarity_autocorrelation_max_lag": 12,\n    "maximum_stationarity_z": 5.0,\n    "minimum_metropolis_acceptance": 0.1,\n    "maximum_metropolis_acceptance": 0.95,\n    "expected_metropolis_proposals_per_chain": 3670016,\n    "maximum_link_unitarity_residual": 5e-12,\n    "maximum_link_determinant_residual": 5e-12,\n}\nV2_OBSERVED_EVIDENCE = {\n    "workflow_run_id": 34761701867,\n    "evidence_head_commit": "f5a221fdad2046779358cc59460fae488626459b",\n    "pooled_mean": 0.5992920042685975,\n    "pooled_standard_error": 0.0008268453534905086,\n    "chain_means": [\n        0.5951228942049566,\n        0.6010578916054883,\n        0.6006811934765629,\n        0.6003060377873822,\n    ],\n    "chain_standard_errors": [\n        0.00073731922680896,\n        0.0017334647942562633,\n        0.0013810304643338124,\n        0.002341577852401104,\n    ],\n    "chain_effective_sample_sizes": [\n        37.108298427827265,\n        12.12458490228564,\n        19.780522239783117,\n        9.930599191453949,\n    ],\n    "aggregate_effective_sample_size": 78.94400476134997,\n    "chain_acceptances": [\n        0.8675000326974052,\n        0.8676090240478516,\n        0.867549078805106,\n        0.867567607334682,\n    ],\n    "stationarity_z": [\n        1.4650733209908853,\n        1.2503134066297152,\n        0.6586970772179463,\n        0.6522302609275793,\n    ],\n    "maximum_pairwise_chain_consistency_z_observed": 3.5504261086461475,\n    "maximum_stationarity_z_observed": 1.4650733209908853,\n    "maximum_link_unitarity_residual_observed": 3.1308289294429414e-14,\n    "maximum_link_determinant_residual_observed": 3.553115420753491e-14,\n    "source_envelope_reproduced": True,\n    "diagnostics_trustworthy": True,\n    "v1_reclassified_as_pass": False,\n    "parameter_retuning_after_v2_result": False,\n}\nEXPECTED_V1_CONTRACT = {\n    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1",\n    "status": "phase-2c2-v1-failed",\n    "scope": EXPECTED_SCOPES["YM-SU3-PLAQUETTE-BASELINE-V1"],\n    "depends_on": [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}],\n    "primary_source": EXPECTED_PRIMARY_SOURCES["YM-SU3-PLAQUETTE-BASELINE-V1"],\n    "benchmark": EXPECTED_BENCHMARK,\n    "reference_run": V1_REFERENCE_RUN,\n    "comparison_rule": V1_COMPARISON_RULE,\n    "predeclaration": V1_PREDECLARATION,\n    "observed_evidence": V1_OBSERVED_EVIDENCE,\n    "phase2c_gate": V1_GATE,\n    "claim_boundary": V1_CLAIM_BOUNDARY,\n    "replay_tolerance": EXPECTED_V1_REPLAY_TOLERANCE,\n}\nEXPECTED_V2_CONTRACT = {\n    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2",\n    "status": "phase-2c2-frozen",\n    "scope": EXPECTED_SCOPES["YM-SU3-PLAQUETTE-BASELINE-V2"],\n    "depends_on": [\n        {"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"},\n        {"contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1", "required_status": "phase-2c2-v1-failed"},\n    ],\n    "primary_source": EXPECTED_PRIMARY_SOURCES["YM-SU3-PLAQUETTE-BASELINE-V2"],\n    "benchmark": EXPECTED_BENCHMARK,\n    "reference_run": V2_REFERENCE_RUN,\n    "comparison_rule": V2_COMPARISON_RULE,\n    "predeclaration": V2_PREDECLARATION,\n    "observed_evidence": V2_OBSERVED_EVIDENCE,\n    "phase2c_gate": V2_GATE,\n    "claim_boundary": V2_CLAIM_BOUNDARY,\n    "replay_tolerance": EXPECTED_V2_REPLAY_TOLERANCE,\n}\nV1_VECTOR_SOURCE = {\n''',
)

replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["claim_boundary"] == V1_CLAIM_BOUNDARY, "Phase 2C2 V1 claim boundary drift")\n\n\ndef validate_v1_vectors''',
    '''    _require(payload["claim_boundary"] == V1_CLAIM_BOUNDARY, "Phase 2C2 V1 claim boundary drift")\n    _require(exact_typed_equal(payload, EXPECTED_V1_CONTRACT), "Phase 2C2 V1 closed typed-schema drift")\n\n\ndef validate_v1_vectors''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["claim_boundary"] == V2_CLAIM_BOUNDARY, "Phase 2C2 V2 claim boundary drift")\n\n\ndef validate_v2_vectors''',
    '''    _require(payload["claim_boundary"] == V2_CLAIM_BOUNDARY, "Phase 2C2 V2 claim boundary drift")\n    _require(exact_typed_equal(payload, EXPECTED_V2_CONTRACT), "Phase 2C2 V2 closed typed-schema drift")\n\n\ndef validate_v2_vectors''',
)

replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["status"] = "phase-2c2-frozen"\n    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["status"] = "phase-2c2-frozen"\n    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")\n\n    broken = copy.deepcopy(v1)\n    broken["clay_solution_proved"] = True\n    expect_reject(validate_v1_contract, broken, "V1 undeclared top-level overclaim field accepted")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["comparison_rule"]["within_chain_stationarity_required"] = False\n    expect_reject(validate_v1_contract, broken, "V1 stationarity comparison rule weakened")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["comparison_rule"]["within_chain_stationarity_required"] = False\n    expect_reject(validate_v1_contract, broken, "V1 stationarity comparison rule weakened")\n\n    broken = copy.deepcopy(v1)\n    broken["comparison_rule"]["raw_measurements_retained"] = 1\n    expect_reject(validate_v1_contract, broken, "V1 boolean replaced by integer")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n    broken = copy.deepcopy(v2)\n    broken["clay_solution_proved"] = True\n    expect_reject(validate_v2_contract, broken, "V2 undeclared top-level overclaim field accepted")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["comparison_rule"]["raw_measurements_retained"] = False\n    expect_reject(validate_v2_contract, broken, "V2 raw-evidence comparison rule weakened")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["comparison_rule"]["raw_measurements_retained"] = False\n    expect_reject(validate_v2_contract, broken, "V2 raw-evidence comparison rule weakened")\n\n    broken = copy.deepcopy(v2)\n    broken["comparison_rule"]["raw_measurements_retained"] = 1\n    expect_reject(validate_v2_contract, broken, "V2 boolean replaced by integer")\n\n''',
)
