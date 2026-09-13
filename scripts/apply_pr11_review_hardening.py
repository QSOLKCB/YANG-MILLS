#!/usr/bin/env python3
"""One-shot hardening for PR #11 review findings."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text(encoding="utf-8")
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:200]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


# Rust: A1++ is only admitted on a cubic spatial lattice.
replace_once(
    "reference/rust/src/glueball_operators.rs",
    "    SpatialExtentTooSmall,\n    CountOverflow,\n",
    "    SpatialExtentTooSmall,\n    SpatialExtentsNotCubic,\n    CountOverflow,\n",
)
replace_once(
    "reference/rust/src/glueball_operators.rs",
    '''            Self::SpatialExtentTooSmall => {\n                "Phase 3A 2x1 spatial operator basis requires every spatial extent to be at least 3"\n            }\n            Self::CountOverflow => "Phase 3A spatial loop count overflowed",\n''',
    '''            Self::SpatialExtentTooSmall => {\n                "Phase 3A 2x1 spatial operator basis requires every spatial extent to be at least 3"\n            }\n            Self::SpatialExtentsNotCubic => {\n                "Phase 3A A1++ operator basis requires equal spatial lattice extents"\n            }\n            Self::CountOverflow => "Phase 3A spatial loop count overflowed",\n''',
)
replace_once(
    "reference/rust/src/glueball_operators.rs",
    '''    if SPATIAL_DIRECTIONS\n        .iter()\n        .any(|&direction| extents[direction] < MINIMUM_SPATIAL_EXTENT)\n    {\n        return Err(GlueballOperatorError::SpatialExtentTooSmall);\n    }\n    Ok(())\n''',
    '''    if SPATIAL_DIRECTIONS\n        .iter()\n        .any(|&direction| extents[direction] < MINIMUM_SPATIAL_EXTENT)\n    {\n        return Err(GlueballOperatorError::SpatialExtentTooSmall);\n    }\n    if extents[0] != extents[1] || extents[1] != extents[2] {\n        return Err(GlueballOperatorError::SpatialExtentsNotCubic);\n    }\n    Ok(())\n''',
)
replace_once(
    "reference/rust/src/glueball_operators.rs",
    '''    #[test]\n    fn spatial_extent_two_is_not_admitted_for_the_2x1_basis() {\n        let lattice = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();\n        assert_eq!(\n            scalar_a1pp_operator_basis(&lattice, 0),\n            Err(GlueballOperatorError::SpatialExtentTooSmall)\n        );\n    }\n}\n''',
    '''    #[test]\n    fn spatial_extent_two_is_not_admitted_for_the_2x1_basis() {\n        let lattice = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();\n        assert_eq!(\n            scalar_a1pp_operator_basis(&lattice, 0),\n            Err(GlueballOperatorError::SpatialExtentTooSmall)\n        );\n    }\n\n    #[test]\n    fn anisotropic_spatial_lattice_is_not_labeled_a1pp() {\n        let lattice = Su3Lattice4::new_identity([3, 4, 5, 3]).unwrap();\n        assert_eq!(\n            scalar_a1pp_operator_basis(&lattice, 0),\n            Err(GlueballOperatorError::SpatialExtentsNotCubic)\n        );\n    }\n}\n''',
)

# Contract and vectors: close the cubic-geometry semantic surface.
replace_once(
    "contracts/YM-GLUEBALL-OPERATORS-V1.json",
    '    "minimum_spatial_extent": 3,\n    "time_slice_rule": "0 <= t < extent[3]",\n',
    '    "minimum_spatial_extent": 3,\n    "spatial_extents_equal": true,\n    "time_slice_rule": "0 <= t < extent[3]",\n',
)
replace_once(
    "test-vectors/glueball-operators-phase3a-v1.json",
    '''    {\n      "id": "claim-boundary",\n''',
    '''    {\n      "id": "anisotropic-spatial-lattice-rejected",\n      "extents": [3, 4, 5, 3],\n      "time_slice": 0,\n      "expected": "SpatialExtentsNotCubic",\n      "claim_class": "C0"\n    },\n    {\n      "id": "claim-boundary",\n''',
)

# Python authority: exact contract/vector schemas plus callable validators.
replace_once(
    "scripts/check_glueball_operator_contracts.py",
    '        "minimum_spatial_extent": 3,\n        "time_slice_rule": "0 <= t < extent[3]",\n',
    '        "minimum_spatial_extent": 3,\n        "spatial_extents_equal": True,\n        "time_slice_rule": "0 <= t < extent[3]",\n',
)
replace_once(
    "scripts/check_glueball_operator_contracts.py",
    '''        {\n            "id": "claim-boundary",\n''',
    '''        {\n            "id": "anisotropic-spatial-lattice-rejected",\n            "extents": [3, 4, 5, 3],\n            "time_slice": 0,\n            "expected": "SpatialExtentsNotCubic",\n            "claim_class": "C0",\n        },\n        {\n            "id": "claim-boundary",\n''',
)
replace_once(
    "scripts/check_glueball_operator_contracts.py",
    '''def main() -> None:\n    validate_exact(\n        "contracts/YM-GLUEBALL-OPERATORS-V1.json",\n        load("contracts/YM-GLUEBALL-OPERATORS-V1.json"),\n        EXPECTED_CONTRACT,\n    )\n    validate_exact(\n        "test-vectors/glueball-operators-phase3a-v1.json",\n        load("test-vectors/glueball-operators-phase3a-v1.json"),\n        EXPECTED_VECTORS,\n    )\n    print("Phase 3A glueball operator contract checks: passed")\n''',
    '''def validate_contract(payload: dict[str, Any]) -> None:\n    validate_exact(\n        "contracts/YM-GLUEBALL-OPERATORS-V1.json",\n        payload,\n        EXPECTED_CONTRACT,\n    )\n\n\ndef validate_vectors(payload: dict[str, Any]) -> None:\n    validate_exact(\n        "test-vectors/glueball-operators-phase3a-v1.json",\n        payload,\n        EXPECTED_VECTORS,\n    )\n\n\ndef main() -> None:\n    validate_contract(load("contracts/YM-GLUEBALL-OPERATORS-V1.json"))\n    validate_vectors(load("test-vectors/glueball-operators-phase3a-v1.json"))\n    print("Phase 3A glueball operator contract checks: passed")\n''',
)

# Negative regressions must exercise the actual advertised validators.
replace_once(
    "scripts/test_glueball_operator_contract_validation.py",
    '''from typing import Any, Callable\n\nfrom check_contracts import load\nfrom check_glueball_operator_contracts import (\n    EXPECTED_CONTRACT,\n    EXPECTED_VECTORS,\n)\n\n\ndef expect_reject(actual: dict[str, Any], expected: dict[str, Any], label: str) -> None:\n    from check_contracts import exact_typed_equal\n\n    if exact_typed_equal(actual, expected):\n        raise SystemExit(f"negative Phase 3A contract test failed: {label} was accepted")\n''',
    '''from typing import Any, Callable\n\nfrom check_contracts import load\nfrom check_glueball_operator_contracts import validate_contract, validate_vectors\n\n\ndef expect_reject(\n    callback: Callable[[dict[str, Any]], None], payload: dict[str, Any], label: str\n) -> None:\n    try:\n        callback(payload)\n    except (SystemExit, KeyError, TypeError, ValueError):\n        return\n    raise SystemExit(f"negative Phase 3A contract test failed: {label} was accepted")\n''',
)
replace_once(
    "scripts/test_glueball_operator_contract_validation.py",
    '    expect_reject(broken, EXPECTED_CONTRACT, "candidate status silently promoted")\n',
    '    expect_reject(validate_contract, broken, "candidate status silently promoted")\n',
)
# Remaining contract call sites.
path = Path("scripts/test_glueball_operator_contract_validation.py")
text = path.read_text(encoding="utf-8")
text = text.replace("expect_reject(broken, EXPECTED_CONTRACT, ", "expect_reject(validate_contract, broken, ")
text = text.replace("expect_reject(broken, EXPECTED_VECTORS, ", "expect_reject(validate_vectors, broken, ")
# Claim-boundary vector moved from index 5 to 6.
text = text.replace('broken["vectors"][5]["effective_mass_extracted"]', 'broken["vectors"][6]["effective_mass_extracted"]')
text = text.replace('broken["vectors"][5]["clay_solution_claimed"]', 'broken["vectors"][6]["clay_solution_claimed"]')
# Explicit regression for the cubic-admission contract itself.
needle = '''    broken = copy.deepcopy(contract)\n    broken["geometry"]["euclidean_time_direction"] = 0\n    expect_reject(validate_contract, broken, "time direction drift")\n\n'''
insert = needle + '''    broken = copy.deepcopy(contract)\n    broken["geometry"]["spatial_extents_equal"] = False\n    expect_reject(validate_contract, broken, "cubic spatial admission weakened")\n\n'''
if insert not in text:
    if needle not in text:
        raise SystemExit("expected negative-test insertion point missing")
    text = text.replace(needle, insert, 1)
path.write_text(text, encoding="utf-8")

# Documentation and status surfaces.
replace_once(
    "docs/GLUEBALL_OPERATORS.md",
    '''Every operator is built entirely from closed spatial loops on one fixed Euclidean time slice. The reference basis requires every spatial extent to be at least 3 so the `2x1` rectangle is not admitted only through immediate periodic wrapping.\n''',
    '''Every operator is built entirely from closed spatial loops on one fixed Euclidean time slice. The reference basis requires every spatial extent to be at least 3 so the `2x1` rectangle is not admitted only through immediate periodic wrapping. It also requires `Lx = Ly = Lz`; the `A1++` label is only used on a cubic periodic spatial lattice where axis permutations belong to the spatial symmetry group.\n''',
)
replace_once(
    "README.md",
    '''Phase 3 glueball analysis is now authorized but has not yet been implemented. No current phase performs glueball extraction, a continuum/infinite-volume limit, construction of the interacting quantum theory, or a mass-gap proof.\n''',
    '''Phase 3A is implemented as a candidate authority in PR #11: `YM-GLUEBALL-OPERATORS-V1` defines an unsmeared, gauge-invariant spatial Wilson-loop basis on cubic periodic `SU(3)` spatial slices, with finite-lattice `A1++` classification and continuum `0++` channel intent. It does **not** yet construct connected correlators, apply smearing/blocking, extract an effective mass, or produce a glueball spectrum. No current phase performs a continuum/infinite-volume limit, constructs the interacting quantum theory, or proves a mass gap. See [`docs/GLUEBALL_OPERATORS.md`](docs/GLUEBALL_OPERATORS.md).\n''',
)
replace_once(
    "README.md",
    '''cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2\npython3 scripts/check_all_contracts.py\n''',
    '''cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2\ncargo run --quiet --locked --bin ym-glueball-operators\npython3 scripts/check_all_contracts.py\n''',
)
replace_once(
    "ROADMAP.md",
    '''## Phase 3 — glueball laboratory\n\n- [ ] Define gauge-invariant spatial loop operator bases.\n''',
    '''## Phase 3 — glueball laboratory\n\nPhase 3A is implemented as a candidate in PR #11; review/freeze is still pending and the later spectroscopy stages remain unopened.\n\n- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, candidate).\n''',
)
replace_once(
    "AGENTS.md",
    '''cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2\npython3 scripts/check_all_contracts.py\n''',
    '''cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2\ncargo run --quiet --locked --bin ym-glueball-operators\npython3 scripts/check_all_contracts.py\n''',
)
replace_once(
    "AGENTS.md",
    '''Do not import triadic ontology, Q/K/V-to-colour semantics, or other cross-domain interpretations into these authorities.\n\n## Donor boundary\n''',
    '''Do not import triadic ontology, Q/K/V-to-colour semantics, or other cross-domain interpretations into these authorities.\n\n### Phase 3A glueball-operator candidate authority\n\nPhase 3A introduces the candidate `YM-GLUEBALL-OPERATORS-V1`. It defines unsmeared closed spatial Wilson-loop operators only. The reference uses Euclidean time direction `3`, spatial directions `[0,1,2]`, requires every spatial extent to be at least `3`, and requires equal spatial extents before using the cubic-lattice `A1++` label. Its continuum channel intent is `0++`; that is not an exact finite-spacing spin assignment.\n\nThe candidate does not authorize smearing/blocking, connected correlators, effective masses, a glueball spectrum, a finite-volume mass-gap claim, confinement, continuum QFT construction, or a Clay solution. Those require separately versioned later Phase 3 authorities.\n\n## Donor boundary\n''',
)

print("PR #11 review hardening applied")
