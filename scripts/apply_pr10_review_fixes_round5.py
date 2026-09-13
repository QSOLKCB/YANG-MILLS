#!/usr/bin/env python3
"""One-shot idempotent patch for the fifth PR #10 Codex hardening pass."""

from __future__ import annotations

import json
from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected review-fix block not found in {path_str}")
    path.write_text(text.replace(old, new, 1))


def update_json(path_str: str, mutate) -> None:
    path = Path(path_str)
    payload = json.loads(path.read_text())
    mutate(payload)
    path.write_text(json.dumps(payload, indent=2) + "\n")


# 1. Pin primary-source graphical extraction semantics.
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''EXPECTED_OBSERVABLE = "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/3)ReTr P_mu_nu(x)"\n''',
    '''EXPECTED_OBSERVABLE = "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/3)ReTr P_mu_nu(x)"\nEXPECTED_NUMERIC_EXTRACTION = {\n    "YM-SU3-PLAQUETTE-BASELINE-V1": "conservative graphical source-resolution envelope for the equivalent normalized average plaquette; not a quoted tabulated statistical error",\n    "YM-SU3-PLAQUETTE-BASELINE-V2": "the same conservative graphical source-resolution envelope frozen before the V1 result was inspected",\n}\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(source["selected_surface"] == "4^4 lattice at beta=6.0", "Phase 2C2 source surface drift")\n    benchmark = payload["benchmark"]\n''',
    '''    _require(source["selected_surface"] == "4^4 lattice at beta=6.0", "Phase 2C2 source surface drift")\n    expected_extraction = EXPECTED_NUMERIC_EXTRACTION[payload["contract_id"]]\n    _require(source["numeric_extraction"] == expected_extraction, "Phase 2C2 source numeric-extraction semantics drift")\n    benchmark = payload["benchmark"]\n''',
)

# 2. Pin replay tolerances in the contract checker.
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''EXPECTED_DIAGNOSTIC_PARAMETERS = {\n''',
    '''EXPECTED_V1_REPLAY_TOLERANCE = {\n    "summary_absolute": 1e-12,\n    "summary_relative": 1e-10,\n}\nEXPECTED_V2_REPLAY_TOLERANCE = {\n    "summary_absolute": 1e-12,\n    "summary_relative": 1e-10,\n    "residual_absolute": 1e-13,\n}\nEXPECTED_DIAGNOSTIC_PARAMETERS = {\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["comparison_rule"] == V1_COMPARISON_RULE, "Phase 2C2 V1 comparison-rule drift")\n    _require(payload["predeclaration"] == V1_PREDECLARATION, "Phase 2C2 V1 predeclaration drift")\n''',
    '''    _require(payload["comparison_rule"] == V1_COMPARISON_RULE, "Phase 2C2 V1 comparison-rule drift")\n    _require(payload["replay_tolerance"] == EXPECTED_V1_REPLAY_TOLERANCE, "Phase 2C2 V1 replay-tolerance drift")\n    _require(payload["predeclaration"] == V1_PREDECLARATION, "Phase 2C2 V1 predeclaration drift")\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    pre = payload["predeclaration"]\n''',
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")\n    pre = payload["predeclaration"]\n''',
)

# 3. Pin vector claim classes and replay tolerance semantics.
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(by_id["published-source-envelope"]["expected_interval"] == [0.56, 0.63], "Phase 2C2 V1 vector source envelope drift")\n    failure = by_id["observed-v1-failure"]\n''',
    '''    source = by_id["published-source-envelope"]\n    _require(source["expected_interval"] == [0.56, 0.63], "Phase 2C2 V1 vector source envelope drift")\n    _require(source["interval_semantics"] == "conservative graphical source-resolution envelope for the equivalent normalized plaquette, not a quoted paper error bar", "Phase 2C2 V1 vector interval semantics drift")\n    _require(source["claim_class"] == "C0", "Phase 2C2 V1 source claim class drift")\n    predeclared = by_id["two-independent-chain-predeclaration"]\n    _require(predeclared["claim_class"] == "C0", "Phase 2C2 V1 predeclaration claim class drift")\n    failure = by_id["observed-v1-failure"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(failure["v1_reclassified_as_pass"] is False and failure["parameter_retuning_after_result"] is False, "Phase 2C2 V1 vector failure rewritten")\n    boundary = by_id["phase2c-v1-failure-boundary"]\n''',
    '''    _require(failure["v1_reclassified_as_pass"] is False and failure["parameter_retuning_after_result"] is False, "Phase 2C2 V1 vector failure rewritten")\n    _require(failure["claim_class"] == "C2", "Phase 2C2 V1 observed-evidence claim class drift")\n    _require(failure["replay_tolerance"] == EXPECTED_V1_REPLAY_TOLERANCE, "Phase 2C2 V1 vector replay-tolerance drift")\n    boundary = by_id["phase2c-v1-failure-boundary"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(boundary["published_su3_baseline_reproduced"] is False and boundary["phase2c_complete"] is False and boundary["glueball_analysis_enabled"] is False, "Phase 2C2 V1 vector boundary weakened")\n\n\ndef validate_v2_contract''',
    '''    _require(boundary["published_su3_baseline_reproduced"] is False and boundary["phase2c_complete"] is False and boundary["glueball_analysis_enabled"] is False, "Phase 2C2 V1 vector boundary weakened")\n    _require(boundary["claim_class"] == "C0", "Phase 2C2 V1 boundary claim class drift")\n\n\ndef validate_v2_contract''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(by_id["unchanged-published-source-envelope"]["expected_interval"] == [0.56, 0.63], "Phase 2C2 V2 vector source envelope drift")\n    run = by_id["four-fresh-chain-confirmation"]\n''',
    '''    source = by_id["unchanged-published-source-envelope"]\n    _require(source["expected_interval"] == [0.56, 0.63], "Phase 2C2 V2 vector source envelope drift")\n    _require(source["claim_class"] == "C0", "Phase 2C2 V2 source claim class drift")\n    run = by_id["four-fresh-chain-confirmation"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(run["minimum_aggregate_effective_sample_size"] == 16.0, "Phase 2C2 V2 vector aggregate ESS drift")\n    failure = by_id["v1-failure-preservation"]\n''',
    '''    _require(run["minimum_aggregate_effective_sample_size"] == 16.0, "Phase 2C2 V2 vector aggregate ESS drift")\n    _require(run["claim_class"] == "C2", "Phase 2C2 V2 confirmation claim class drift")\n    _require(run["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 vector replay-tolerance drift")\n    failure = by_id["v1-failure-preservation"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(failure["v1_reclassified_as_pass"] is False and failure["v2_uses_fresh_streams"] is True, "Phase 2C2 V2 vector independence boundary weakened")\n    observed = by_id["observed-v2-confirmation"]\n''',
    '''    _require(failure["v1_reclassified_as_pass"] is False and failure["v2_uses_fresh_streams"] is True, "Phase 2C2 V2 vector independence boundary weakened")\n    _require(failure["claim_class"] == "C0", "Phase 2C2 V2 V1-preservation claim class drift")\n    observed = by_id["observed-v2-confirmation"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(observed["v1_reclassified_as_pass"] is False and observed["parameter_retuning_after_v2_result"] is False, "Phase 2C2 V2 vector provenance rewritten")\n    boundary = by_id["phase2c-frozen-boundary"]\n''',
    '''    _require(observed["v1_reclassified_as_pass"] is False and observed["parameter_retuning_after_v2_result"] is False, "Phase 2C2 V2 vector provenance rewritten")\n    _require(observed["claim_class"] == "C2", "Phase 2C2 V2 observed-evidence claim class drift")\n    boundary = by_id["phase2c-frozen-boundary"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 frozen vector mass-gap overclaim")\n''',
    '''    _require(boundary["benchmark_agreement_implies_positive_mass_gap"] is False, "Phase 2C2 frozen vector mass-gap overclaim")\n    _require(boundary["claim_class"] == "C0", "Phase 2C2 frozen boundary claim class drift")\n''',
)

# 4. Add negative regressions for all new checker surfaces.
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["benchmark"]["observable"] = "unrelated observable"\n    expect_reject(validate_v1_contract, broken, "V1 observable normalization drift")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["benchmark"]["observable"] = "unrelated observable"\n    expect_reject(validate_v1_contract, broken, "V1 observable normalization drift")\n\n    broken = copy.deepcopy(v1)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v1_contract, broken, "V1 graphical source extraction rewritten")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["observed_evidence"]["minimum_effective_sample_size"] = 8.1\n    expect_reject(validate_v1_contract, broken, "V1 failed ESS evidence rewritten")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["observed_evidence"]["minimum_effective_sample_size"] = 8.1\n    expect_reject(validate_v1_contract, broken, "V1 failed ESS evidence rewritten")\n\n    broken = copy.deepcopy(v1)\n    broken["replay_tolerance"]["summary_relative"] = 1.0\n    expect_reject(validate_v1_contract, broken, "V1 replay tolerance weakened")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][2]["v1_reclassified_as_pass"] = True\n    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")\n\n''',
    '''    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][2]["v1_reclassified_as_pass"] = True\n    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")\n\n    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][2]["claim_class"] = "C5"\n    expect_reject(validate_v1_vectors, broken, "V1 observed evidence promoted to C5")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n    broken = copy.deepcopy(v2)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v2_contract, broken, "V2 graphical source extraction rewritten")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["comparison_rule"]["raw_measurements_retained"] = False\n    expect_reject(validate_v2_contract, broken, "V2 raw-evidence comparison rule weakened")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["comparison_rule"]["raw_measurements_retained"] = False\n    expect_reject(validate_v2_contract, broken, "V2 raw-evidence comparison rule weakened")\n\n    broken = copy.deepcopy(v2)\n    broken["replay_tolerance"]["summary_absolute"] = 1.0\n    expect_reject(validate_v2_contract, broken, "V2 replay tolerance weakened")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][3]["aggregate_effective_sample_size"] = 15.0\n    expect_reject(validate_v2_vectors, broken, "V2 vector passing evidence weakened")\n\n''',
    '''    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][3]["aggregate_effective_sample_size"] = 15.0\n    expect_reject(validate_v2_vectors, broken, "V2 vector passing evidence weakened")\n\n    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][3]["claim_class"] = "C5"\n    expect_reject(validate_v2_vectors, broken, "V2 observed evidence promoted to C5")\n\n''',
)

# 5. Reject NaN in reported acceptance evidence, with regression.
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    '''    if !acceptance.is_finite()\n        || (report.mean_metropolis_acceptance - acceptance).abs() > ACCEPTANCE_EVIDENCE_TOLERANCE\n''',
    '''    if !acceptance.is_finite()\n        || !report.mean_metropolis_acceptance.is_finite()\n        || (report.mean_metropolis_acceptance - acceptance).abs() > ACCEPTANCE_EVIDENCE_TOLERANCE\n''',
)
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    '''        let mut broken = first.clone();\n        broken.mean_metropolis_acceptance += ACCEPTANCE_EVIDENCE_TOLERANCE * 2.0;\n''',
    '''        let mut non_finite = first.clone();\n        non_finite.mean_metropolis_acceptance = f64::NAN;\n        assert_eq!(\n            summarize_two_chains(&non_finite, &second),\n            Err(BenchmarkError::InvalidAcceptanceEvidence)\n        );\n\n        let mut broken = first.clone();\n        broken.mean_metropolis_acceptance += ACCEPTANCE_EVIDENCE_TOLERANCE * 2.0;\n''',
)

# 6. Bind the V1 replay to archived numerical evidence with frozen tolerances.
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    '''use su3_ensemble::{run_chain, Su3EnsembleConfig};\n\nfn config''',
    '''use su3_ensemble::{run_chain, Su3EnsembleConfig};\n\nconst FROZEN_SUMMARY_ABS_TOLERANCE: f64 = 1.0e-12;\nconst FROZEN_SUMMARY_REL_TOLERANCE: f64 = 1.0e-10;\nconst FROZEN_POOLED_MEAN: f64 = 0.6018083560823128;\nconst FROZEN_MIN_EFFECTIVE_SAMPLE_SIZE: f64 = 4.51005377363487;\nconst FROZEN_CHAIN_CONSISTENCY_Z: f64 = 0.4307895037511404;\nconst FROZEN_MAX_STATIONARITY_Z: f64 = 3.7834285113688733;\nconst FROZEN_CHAIN_ACCEPTANCES: [f64; 2] = [0.867445537022182, 0.8678583417619977];\n\nfn evidence_close(actual: f64, expected: f64) -> bool {\n    (actual - expected).abs()\n        <= FROZEN_SUMMARY_ABS_TOLERANCE + FROZEN_SUMMARY_REL_TOLERANCE * expected.abs()\n}\n\nfn archived_v1_evidence_matches(summary: &su3_benchmark::BenchmarkSummary) -> bool {\n    evidence_close(summary.pooled_mean, FROZEN_POOLED_MEAN)\n        && evidence_close(\n            summary.min_effective_sample_size,\n            FROZEN_MIN_EFFECTIVE_SAMPLE_SIZE,\n        )\n        && evidence_close(summary.chain_consistency_z, FROZEN_CHAIN_CONSISTENCY_Z)\n        && evidence_close(summary.max_stationarity_z, FROZEN_MAX_STATIONARITY_Z)\n        && summary\n            .chain_acceptances\n            .iter()\n            .zip(FROZEN_CHAIN_ACCEPTANCES)\n            .all(|(actual, expected)| evidence_close(*actual, expected))\n}\n\nfn config''',
)
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    '''        && summary.max_stationarity_z <= MAX_STATIONARITY_Z\n        && summary.max_link_unitarity_residual <= MAX_LINK_UNITARITY_RESIDUAL\n        && summary.max_link_determinant_residual <= MAX_LINK_DETERMINANT_RESIDUAL;\n''',
    '''        && summary.max_stationarity_z <= MAX_STATIONARITY_Z\n        && summary.max_link_unitarity_residual <= MAX_LINK_UNITARITY_RESIDUAL\n        && summary.max_link_determinant_residual <= MAX_LINK_DETERMINANT_RESIDUAL\n        && archived_v1_evidence_matches(&summary);\n''',
)
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    '''    println!("result_role=archived_v1_failed_attempt");\n''',
    '''    println!("result_role=archived_v1_failed_attempt");\n    println!("archived_evidence_reproduced=true");\n    println!("frozen_summary_abs_tolerance={FROZEN_SUMMARY_ABS_TOLERANCE:.17e}");\n    println!("frozen_summary_rel_tolerance={FROZEN_SUMMARY_REL_TOLERANCE:.17e}");\n''',
)
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    '''    println!("status=archived_failed_gate_reproduced");\n    Ok(())\n}\n''',
    '''    println!("status=archived_failed_gate_reproduced");\n    Ok(())\n}\n\n#[cfg(test)]\nmod replay_tolerance_tests {\n    use super::*;\n\n    #[test]\n    fn archived_v1_replay_tolerances_are_frozen() {\n        assert_eq!(FROZEN_SUMMARY_ABS_TOLERANCE, 1.0e-12);\n        assert_eq!(FROZEN_SUMMARY_REL_TOLERANCE, 1.0e-10);\n    }\n}\n''',
)

# 7. Pin the V2 private replay tolerances with an executable unit test.
replace_once(
    "reference/rust/src/bin/su3_confirmation_reference.rs",
    '''    println!("status=passed");\n    Ok(())\n}\n''',
    '''    println!("status=passed");\n    Ok(())\n}\n\n#[cfg(test)]\nmod replay_tolerance_tests {\n    use super::*;\n\n    #[test]\n    fn frozen_v2_replay_tolerances_are_frozen() {\n        assert_eq!(FROZEN_SUMMARY_ABS_TOLERANCE, 1.0e-12);\n        assert_eq!(FROZEN_SUMMARY_REL_TOLERANCE, 1.0e-10);\n        assert_eq!(FROZEN_RESIDUAL_ABS_TOLERANCE, 1.0e-13);\n    }\n}\n''',
)

# 8. Record replay-tolerance semantics in contracts and vectors.
def mutate_v1_contract(payload):
    payload["replay_tolerance"] = {
        "summary_absolute": 1e-12,
        "summary_relative": 1e-10,
    }


def mutate_v2_contract(payload):
    payload["replay_tolerance"] = {
        "summary_absolute": 1e-12,
        "summary_relative": 1e-10,
        "residual_absolute": 1e-13,
    }


def mutate_v1_vectors(payload):
    by_id = {item["id"]: item for item in payload["vectors"]}
    by_id["observed-v1-failure"]["replay_tolerance"] = {
        "summary_absolute": 1e-12,
        "summary_relative": 1e-10,
    }


def mutate_v2_vectors(payload):
    by_id = {item["id"]: item for item in payload["vectors"]}
    by_id["four-fresh-chain-confirmation"]["replay_tolerance"] = {
        "summary_absolute": 1e-12,
        "summary_relative": 1e-10,
        "residual_absolute": 1e-13,
    }


update_json("contracts/YM-SU3-PLAQUETTE-BASELINE-V1.json", mutate_v1_contract)
update_json("contracts/YM-SU3-PLAQUETTE-BASELINE-V2.json", mutate_v2_contract)
update_json("test-vectors/su3-plaquette-baseline-v1.json", mutate_v1_vectors)
update_json("test-vectors/su3-plaquette-baseline-v2.json", mutate_v2_vectors)

print("PR #10 fifth review hardening patch applied")
