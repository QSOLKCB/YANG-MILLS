#!/usr/bin/env python3
"""One-shot patch for closed typed Phase 2C2 vector validation."""

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
    '''V2_VECTOR_BOUNDARY = {\n    "id": "phase2c-frozen-boundary",\n    "published_su3_baseline_reproduced": True,\n    "phase2c_complete": True,\n    "glueball_analysis_enabled": True,\n    "benchmark_agreement_implies_confinement": False,\n    "benchmark_agreement_implies_continuum_qft": False,\n    "benchmark_agreement_implies_positive_mass_gap": False,\n    "claim_class": "C0",\n}\n\n\ndef _require''',
    '''V2_VECTOR_BOUNDARY = {\n    "id": "phase2c-frozen-boundary",\n    "published_su3_baseline_reproduced": True,\n    "phase2c_complete": True,\n    "glueball_analysis_enabled": True,\n    "benchmark_agreement_implies_confinement": False,\n    "benchmark_agreement_implies_continuum_qft": False,\n    "benchmark_agreement_implies_positive_mass_gap": False,\n    "claim_class": "C0",\n}\nEXPECTED_V1_VECTOR_PAYLOAD = {\n    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V1",\n    "vector_set": "SU3-PLAQUETTE-BASELINE-PHASE2C2-V1",\n    "status": "phase-2c2-v1-failed",\n    "vectors": [\n        V1_VECTOR_SOURCE,\n        V1_VECTOR_PREDECLARATION,\n        V1_VECTOR_OBSERVED_EVIDENCE,\n        V1_VECTOR_BOUNDARY,\n    ],\n}\nEXPECTED_V2_VECTOR_PAYLOAD = {\n    "contract_id": "YM-SU3-PLAQUETTE-BASELINE-V2",\n    "vector_set": "SU3-PLAQUETTE-BASELINE-PHASE2C2-V2",\n    "status": "phase-2c2-frozen",\n    "vectors": [\n        V2_VECTOR_SOURCE,\n        V2_VECTOR_CONFIRMATION,\n        V2_VECTOR_V1_PRESERVATION,\n        V2_VECTOR_OBSERVED_EVIDENCE,\n        V2_VECTOR_BOUNDARY,\n    ],\n}\n\n\ndef _require''',
)

replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(by_id["phase2c-v1-failure-boundary"] == V1_VECTOR_BOUNDARY, "Phase 2C2 V1 claim-boundary vector drift")\n\n\ndef validate_v2_contract''',
    '''    _require(by_id["phase2c-v1-failure-boundary"] == V1_VECTOR_BOUNDARY, "Phase 2C2 V1 claim-boundary vector drift")\n    _require(exact_typed_equal(payload, EXPECTED_V1_VECTOR_PAYLOAD), "Phase 2C2 V1 vector closed typed-schema drift")\n\n\ndef validate_v2_contract''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(by_id["phase2c-frozen-boundary"] == V2_VECTOR_BOUNDARY, "Phase 2C2 V2 claim-boundary vector drift")\n\n\ndef main''',
    '''    _require(by_id["phase2c-frozen-boundary"] == V2_VECTOR_BOUNDARY, "Phase 2C2 V2 claim-boundary vector drift")\n    _require(exact_typed_equal(payload, EXPECTED_V2_VECTOR_PAYLOAD), "Phase 2C2 V2 vector closed typed-schema drift")\n\n\ndef main''',
)

replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][2]["v1_reclassified_as_pass"] = True\n    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")\n\n''',
    '''    broken = copy.deepcopy(v1_vectors)\n    broken["clay_solution_proved"] = True\n    expect_reject(validate_v1_vectors, broken, "V1 vector undeclared top-level overclaim field accepted")\n\n    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][3]["published_su3_baseline_reproduced"] = 0\n    expect_reject(validate_v1_vectors, broken, "V1 vector boolean replaced by integer")\n\n    broken = copy.deepcopy(v1_vectors)\n    broken["vectors"][2]["v1_reclassified_as_pass"] = True\n    expect_reject(validate_v1_vectors, broken, "V1 vector failure reclassified")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][1]["chain_ids"] = [0, 1, 2, 2]\n    expect_reject(validate_v2_vectors, broken, "V2 vector independent streams collapsed")\n\n''',
    '''    broken = copy.deepcopy(v2_vectors)\n    broken["clay_solution_proved"] = True\n    expect_reject(validate_v2_vectors, broken, "V2 vector undeclared top-level overclaim field accepted")\n\n    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][4]["published_su3_baseline_reproduced"] = 1\n    expect_reject(validate_v2_vectors, broken, "V2 vector boolean replaced by integer")\n\n    broken = copy.deepcopy(v2_vectors)\n    broken["vectors"][1]["chain_ids"] = [0, 1, 2, 2]\n    expect_reject(validate_v2_vectors, broken, "V2 vector independent streams collapsed")\n\n''',
)
