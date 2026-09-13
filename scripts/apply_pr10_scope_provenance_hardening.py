#!/usr/bin/env python3
"""One-shot patch for Phase 2C2 contract scope and publication provenance hardening."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:180]!r}")
    path.write_text(text.replace(old, new, 1))


# Freeze the contract scopes and complete primary-source provenance objects.
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''EXPECTED_NUMERIC_EXTRACTION = {\n    "YM-SU3-PLAQUETTE-BASELINE-V1": "conservative graphical source-resolution envelope for the equivalent normalized average plaquette; not a quoted tabulated statistical error",\n    "YM-SU3-PLAQUETTE-BASELINE-V2": "the same conservative graphical source-resolution envelope frozen before the V1 result was inspected",\n}\n''',
    '''EXPECTED_NUMERIC_EXTRACTION = {\n    "YM-SU3-PLAQUETTE-BASELINE-V1": "conservative graphical source-resolution envelope for the equivalent normalized average plaquette; not a quoted tabulated statistical error",\n    "YM-SU3-PLAQUETTE-BASELINE-V2": "the same conservative graphical source-resolution envelope frozen before the V1 result was inspected",\n}\nEXPECTED_SCOPES = {\n    "YM-SU3-PLAQUETTE-BASELINE-V1": "first predeclared independent published finite-lattice validation attempt of the frozen Phase 2C1 pure-SU(3) sampler",\n    "YM-SU3-PLAQUETTE-BASELINE-V2": "fresh-stream confirmatory published finite-lattice validation after the V1 attempt reproduced the source envelope but failed its predeclared per-chain ESS gate",\n}\nEXPECTED_PRIMARY_SOURCES = {\n    "YM-SU3-PLAQUETTE-BASELINE-V1": {\n        "title": "The specific heat of SU(3) lattice gauge theory",\n        "authors": ["R. C. Edgar", "L. McCrossen", "K. J. M. Moriarty"],\n        "journal": "Journal of Physics G: Nuclear Physics 7 (1981) L85-L88",\n        "doi": "10.1088/0305-4616/7/5/001",\n        "peer_reviewed": True,\n        "source_surface": "four-dimensional pure SU(3) average action/plaquette beta scan on 4^4 and 5^4 lattices; the 4^4 calculation averages over 400 iterations through the lattice",\n        "selected_surface": "4^4 lattice at beta=6.0",\n        "numeric_extraction": EXPECTED_NUMERIC_EXTRACTION["YM-SU3-PLAQUETTE-BASELINE-V1"],\n    },\n    "YM-SU3-PLAQUETTE-BASELINE-V2": {\n        "title": "The specific heat of SU(3) lattice gauge theory",\n        "authors": ["R. C. Edgar", "L. McCrossen", "K. J. M. Moriarty"],\n        "journal": "Journal of Physics G: Nuclear Physics 7 (1981) L85-L88",\n        "doi": "10.1088/0305-4616/7/5/001",\n        "peer_reviewed": True,\n        "selected_surface": "4^4 lattice at beta=6.0",\n        "numeric_extraction": EXPECTED_NUMERIC_EXTRACTION["YM-SU3-PLAQUETTE-BASELINE-V2"],\n    },\n}\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''def _validate_common_source_and_benchmark(payload: dict[str, Any]) -> None:\n    source = payload["primary_source"]\n    _require(source["doi"] == "10.1088/0305-4616/7/5/001", "Phase 2C2 source DOI drift")\n    _require(source["peer_reviewed"] is True, "Phase 2C2 source must remain peer reviewed")\n    _require(source["selected_surface"] == "4^4 lattice at beta=6.0", "Phase 2C2 source surface drift")\n    expected_extraction = EXPECTED_NUMERIC_EXTRACTION[payload["contract_id"]]\n    _require(source["numeric_extraction"] == expected_extraction, "Phase 2C2 source numeric-extraction semantics drift")\n    benchmark = payload["benchmark"]\n''',
    '''def _validate_common_source_and_benchmark(payload: dict[str, Any]) -> None:\n    source = payload["primary_source"]\n    expected_source = EXPECTED_PRIMARY_SOURCES[payload["contract_id"]]\n    _require(source == expected_source, "Phase 2C2 complete primary-source provenance drift")\n    benchmark = payload["benchmark"]\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 failed status drift")\n    _require(payload["depends_on"] == [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}], "Phase 2C2 V1 dependency drift")\n''',
    '''    _require(payload["status"] == "phase-2c2-v1-failed", "Phase 2C2 V1 failed status drift")\n    _require(payload["scope"] == EXPECTED_SCOPES[payload["contract_id"]], "Phase 2C2 V1 scope/claim drift")\n    _require(payload["depends_on"] == [{"contract_id": "YM-SU3-ENSEMBLE-V1", "required_status": "phase-2c1-frozen"}], "Phase 2C2 V1 dependency drift")\n''',
)
replace_once(
    "scripts/check_su3_baseline_contracts.py",
    '''    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 frozen status drift")\n    _require(payload["depends_on"] == [\n''',
    '''    _require(payload["status"] == "phase-2c2-frozen", "Phase 2C2 V2 frozen status drift")\n    _require(payload["scope"] == EXPECTED_SCOPES[payload["contract_id"]], "Phase 2C2 V2 scope/claim drift")\n    _require(payload["depends_on"] == [\n''',
)

# Add explicit negative regressions for scope promotion and provenance rewriting.
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["status"] = "phase-2c2-frozen"\n    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["status"] = "phase-2c2-frozen"\n    expect_reject(validate_v1_contract, broken, "V1 failure reclassified as frozen success")\n\n    broken = copy.deepcopy(v1)\n    broken["scope"] = "continuum construction and Clay Yang-Mills mass-gap solution"\n    expect_reject(validate_v1_contract, broken, "V1 scope promoted beyond finite-lattice validation")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v1)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v1_contract, broken, "V1 graphical source extraction rewritten")\n\n''',
    '''    broken = copy.deepcopy(v1)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v1_contract, broken, "V1 graphical source extraction rewritten")\n\n    broken = copy.deepcopy(v1)\n    broken["primary_source"]["title"] = "Rewritten benchmark title"\n    expect_reject(validate_v1_contract, broken, "V1 publication title provenance rewritten")\n\n    broken = copy.deepcopy(v1)\n    del broken["primary_source"]["source_surface"]\n    expect_reject(validate_v1_contract, broken, "V1 retained source-surface provenance deleted")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["benchmark"]["source_envelope_max"] = 0.75\n    expect_reject(validate_v2_contract, broken, "V2 source envelope retuned")\n\n    broken = copy.deepcopy(v2)\n    broken["scope"] = "continuum quantum Yang-Mills construction and positive mass-gap proof"\n    expect_reject(validate_v2_contract, broken, "V2 scope promoted beyond finite-lattice validation")\n\n''',
)
replace_once(
    "scripts/test_su3_baseline_contract_validation.py",
    '''    broken = copy.deepcopy(v2)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v2_contract, broken, "V2 graphical source extraction rewritten")\n\n''',
    '''    broken = copy.deepcopy(v2)\n    broken["primary_source"]["numeric_extraction"] = "quoted paper statistical error bar"\n    expect_reject(validate_v2_contract, broken, "V2 graphical source extraction rewritten")\n\n    broken = copy.deepcopy(v2)\n    broken["primary_source"]["authors"][0] = "Different Author"\n    expect_reject(validate_v2_contract, broken, "V2 publication author provenance rewritten")\n\n    broken = copy.deepcopy(v2)\n    broken["primary_source"]["journal"] = "Different Journal"\n    expect_reject(validate_v2_contract, broken, "V2 publication journal provenance rewritten")\n\n    broken = copy.deepcopy(v2)\n    broken["primary_source"]["source_surface"] = "V1-only provenance field injected into V2"\n    expect_reject(validate_v2_contract, broken, "V2 primary-source provenance gained an unfrozen field")\n\n''',
)
