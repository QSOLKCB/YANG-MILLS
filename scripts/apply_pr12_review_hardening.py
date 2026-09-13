#!/usr/bin/env python3
"""One-shot hardening for PR #12 review findings."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text(encoding="utf-8")
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:200]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


# Make every Phase 3B1 lattice copy allocation-fallible without changing the
# frozen SU(3) lattice authority itself.
replace_once(
    "reference/rust/src/glueball_smearing.rs",
    '''fn smear_one_level(lattice: &Su3Lattice4, rho: f64) -> Result<Su3Lattice4, GlueballSmearingError> {\n    let extents = lattice.extents();\n    let mut out = lattice.clone();\n''',
    '''fn fallible_copy_lattice(\n    lattice: &Su3Lattice4,\n) -> Result<Su3Lattice4, GlueballSmearingError> {\n    let extents = lattice.extents();\n    let mut out = Su3Lattice4::new_identity(extents)?;\n    for x0 in 0..extents[0] {\n        for x1 in 0..extents[1] {\n            for x2 in 0..extents[2] {\n                for x3 in 0..extents[3] {\n                    let x = [x0, x1, x2, x3];\n                    for mu in 0..DIM {\n                        out.set_link(x, mu, lattice.link(x, mu)?)?;\n                    }\n                }\n            }\n        }\n    }\n    Ok(out)\n}\n\nfn smear_one_level(lattice: &Su3Lattice4, rho: f64) -> Result<Su3Lattice4, GlueballSmearingError> {\n    let extents = lattice.extents();\n    let mut out = fallible_copy_lattice(lattice)?;\n''',
)
replace_once(
    "reference/rust/src/glueball_smearing.rs",
    '''    validate_parameters(lattice, rho, levels)?;\n    let mut current = lattice.clone();\n''',
    '''    validate_parameters(lattice, rho, levels)?;\n    let mut current = fallible_copy_lattice(lattice)?;\n''',
)

# Freeze canonical nontrivial outputs so algorithmic semantic drift cannot
# hide behind generic group/gauge invariants.
replace_once(
    "reference/rust/src/glueball_smearing.rs",
    '''pub const GAUGE_COVARIANCE_TOLERANCE: f64 = 5.0e-11;\n''',
    '''pub const GAUGE_COVARIANCE_TOLERANCE: f64 = 5.0e-11;\npub const FROZEN_OUTPUT_ABS_TOLERANCE: f64 = 1.0e-12;\npub const FROZEN_SPATIAL_CHANGE: f64 = 0.2158247298307305;\npub const FROZEN_SMEARED_PLAQUETTE_1X1: f64 = 0.9693015447000209;\npub const FROZEN_SMEARED_RECTANGLE_2X1: f64 = 0.9511047606560571;\n''',
)
replace_once(
    "reference/rust/src/glueball_smearing.rs",
    '''    #[test]\n    fn smeared_links_remain_in_su3() {\n''',
    '''    #[test]\n    fn canonical_nontrivial_output_is_frozen() {\n        let lattice = single_traceless_su3_link_lattice().unwrap();\n        let smeared = spatial_stout_smear(&lattice, CANONICAL_RHO, 1).unwrap();\n        let change = max_link_residual(&lattice, &smeared).unwrap();\n        let basis = scalar_a1pp_operator_basis(&smeared, 0).unwrap();\n        assert!((change - FROZEN_SPATIAL_CHANGE).abs() <= FROZEN_OUTPUT_ABS_TOLERANCE);\n        assert!(\n            (basis.spatial_plaquette_1x1 - FROZEN_SMEARED_PLAQUETTE_1X1).abs()\n                <= FROZEN_OUTPUT_ABS_TOLERANCE\n        );\n        assert!(\n            (basis.spatial_rectangle_2x1 - FROZEN_SMEARED_RECTANGLE_2X1).abs()\n                <= FROZEN_OUTPUT_ABS_TOLERANCE\n        );\n    }\n\n    #[test]\n    fn smeared_links_remain_in_su3() {\n''',
)

# Runtime independently enforces the literal output authority.
replace_once(
    "reference/rust/src/bin/glueball_smearing_reference.rs",
    '''use glueball_smearing::{\n    max_link_residual, spatial_stout_smear, CANONICAL_RHO, GAUGE_COVARIANCE_TOLERANCE,\n    MAX_LEVELS, SMEARING_CONTRACT_ID, TRANSFORMATION_ID,\n};\n''',
    '''use glueball_smearing::{\n    max_link_residual, spatial_stout_smear, CANONICAL_RHO, FROZEN_OUTPUT_ABS_TOLERANCE,\n    FROZEN_SMEARED_PLAQUETTE_1X1, FROZEN_SMEARED_RECTANGLE_2X1,\n    FROZEN_SPATIAL_CHANGE, GAUGE_COVARIANCE_TOLERANCE, MAX_LEVELS, SMEARING_CONTRACT_ID,\n    TRANSFORMATION_ID,\n};\n''',
)
replace_once(
    "reference/rust/src/bin/glueball_smearing_reference.rs",
    '''    if !spatial_change.is_finite() || spatial_change <= 0.0 {\n        return Err("Phase 3B1 deterministic nontrivial field was not changed".into());\n    }\n''',
    '''    if !spatial_change.is_finite()\n        || (spatial_change - FROZEN_SPATIAL_CHANGE).abs() > FROZEN_OUTPUT_ABS_TOLERANCE\n        || (basis_after.spatial_plaquette_1x1 - FROZEN_SMEARED_PLAQUETTE_1X1).abs()\n            > FROZEN_OUTPUT_ABS_TOLERANCE\n        || (basis_after.spatial_rectangle_2x1 - FROZEN_SMEARED_RECTANGLE_2X1).abs()\n            > FROZEN_OUTPUT_ABS_TOLERANCE\n    {\n        return Err("Phase 3B1 frozen nontrivial output regression mismatch".into());\n    }\n''',
)

# Machine-readable contract: make allocation behavior and literal replay part
# of the authority surface.
replace_once(
    "contracts/YM-GLUEBALL-SPATIAL-STOUT-V1.json",
    '''    "blocking_step_used": false\n  },\n''',
    '''    "blocking_step_used": false,\n    "lattice_copy_allocation_failure_rule": "propagate via Result; no infallible Vec clone in the public smearing path"\n  },\n''',
)
replace_once(
    "contracts/YM-GLUEBALL-SPATIAL-STOUT-V1.json",
    '''    "canonical_nontrivial_fixture_must_change_spatial_links": true,\n    "non_finite_intermediate_rule": "reject"\n''',
    '''    "canonical_nontrivial_fixture_must_change_spatial_links": true,\n    "frozen_output_abs_tolerance": 1e-12,\n    "frozen_spatial_change": 0.2158247298307305,\n    "frozen_smeared_plaquette_1x1": 0.9693015447000209,\n    "frozen_smeared_rectangle_2x1": 0.9511047606560571,\n    "non_finite_intermediate_rule": "reject"\n''',
)

# Vector: replace the weak merely-positive assertion with literal outputs.
replace_once(
    "test-vectors/glueball-spatial-stout-phase3b1-v1.json",
    '''      "observable": "maximum all-link residual before/after smearing",\n      "expected": "finite-and-strictly-positive",\n      "claim_class": "C2"\n''',
    '''      "observable": "canonical one-level stout output on the deterministic nontrivial fixture",\n      "expected": {\n        "maximum_all_link_residual": 0.2158247298307305,\n        "spatial_plaquette_1x1": 0.9693015447000209,\n        "spatial_rectangle_2x1": 0.9511047606560571\n      },\n      "absolute_tolerance": 1e-12,\n      "claim_class": "C2"\n''',
)

# Python typed authority mirrors the contract/vector additions.
replace_once(
    "scripts/check_glueball_smearing_contracts.py",
    '''        "blocking_step_used": False,\n    },\n''',
    '''        "blocking_step_used": False,\n        "lattice_copy_allocation_failure_rule": "propagate via Result; no infallible Vec clone in the public smearing path",\n    },\n''',
)
replace_once(
    "scripts/check_glueball_smearing_contracts.py",
    '''        "canonical_nontrivial_fixture_must_change_spatial_links": True,\n        "non_finite_intermediate_rule": "reject",\n''',
    '''        "canonical_nontrivial_fixture_must_change_spatial_links": True,\n        "frozen_output_abs_tolerance": 1e-12,\n        "frozen_spatial_change": 0.2158247298307305,\n        "frozen_smeared_plaquette_1x1": 0.9693015447000209,\n        "frozen_smeared_rectangle_2x1": 0.9511047606560571,\n        "non_finite_intermediate_rule": "reject",\n''',
)
replace_once(
    "scripts/check_glueball_smearing_contracts.py",
    '''            "observable": "maximum all-link residual before/after smearing",\n            "expected": "finite-and-strictly-positive",\n            "claim_class": "C2",\n''',
    '''            "observable": "canonical one-level stout output on the deterministic nontrivial fixture",\n            "expected": {\n                "maximum_all_link_residual": 0.2158247298307305,\n                "spatial_plaquette_1x1": 0.9693015447000209,\n                "spatial_rectangle_2x1": 0.9511047606560571,\n            },\n            "absolute_tolerance": 1e-12,\n            "claim_class": "C2",\n''',
)

# Negative regressions protect both hardenings.
replace_once(
    "scripts/test_glueball_smearing_contract_validation.py",
    '''    broken = copy.deepcopy(contract)\n    broken["algorithm"]["blocking_step_used"] = True\n    expect_reject(validate_contract, broken, "undeclared blocking injected")\n\n''',
    '''    broken = copy.deepcopy(contract)\n    broken["algorithm"]["blocking_step_used"] = True\n    expect_reject(validate_contract, broken, "undeclared blocking injected")\n\n    broken = copy.deepcopy(contract)\n    broken["algorithm"]["lattice_copy_allocation_failure_rule"] = "abort on allocation failure"\n    expect_reject(validate_contract, broken, "allocation failure propagation weakened")\n\n    broken = copy.deepcopy(contract)\n    broken["validation"]["frozen_spatial_change"] = 0.0\n    expect_reject(validate_contract, broken, "frozen stout output drift")\n\n''',
)
replace_once(
    "scripts/test_glueball_smearing_contract_validation.py",
    '''    broken = copy.deepcopy(vectors)\n    broken["vectors"][3]["expected_max"] = 1.0\n''',
    '''    broken = copy.deepcopy(vectors)\n    broken["vectors"][5]["expected"]["spatial_plaquette_1x1"] = 0.0\n    expect_reject(validate_vectors, broken, "literal stout output weakened")\n\n    broken = copy.deepcopy(vectors)\n    broken["vectors"][3]["expected_max"] = 1.0\n''',
)

# Documentation records the strengthened reproducibility/allocation contract.
replace_once(
    "docs/GLUEBALL_SMEARING.md",
    '''- the canonical nontrivial fixture changes at least one spatial link;\n- local gauge transformation commutes with smearing within the declared numerical tolerance;\n''',
    '''- the canonical nontrivial fixture is pinned to literal one-level link/operator outputs with an absolute `1e-12` replay tolerance;\n- local gauge transformation commutes with smearing within the declared numerical tolerance;\n''',
)
replace_once(
    "docs/GLUEBALL_SMEARING.md",
    '''- invalid rho, excessive levels, too-small spatial geometry, and anisotropic spatial geometry fail closed.\n''',
    '''- invalid rho, excessive levels, too-small spatial geometry, and anisotropic spatial geometry fail closed;\n- lattice-copy allocation failures propagate through `Result`; the public smearing path does not use an infallible full-lattice clone.\n''',
)

print("PR #12 review hardening applied")
