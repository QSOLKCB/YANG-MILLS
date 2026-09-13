#!/usr/bin/env python3
"""One-shot status/readiness synchronization for PR #12."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text(encoding="utf-8")
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:160]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


replace_once(
    "README.md",
    "Phase 3A is implemented as a candidate authority in PR #11: `YM-GLUEBALL-OPERATORS-V1` defines an unsmeared, gauge-invariant spatial Wilson-loop basis on cubic periodic `SU(3)` spatial slices, with finite-lattice `A1++` classification and continuum `0++` channel intent. It does **not** yet construct connected correlators, apply smearing/blocking, extract an effective mass, or produce a glueball spectrum. No current phase performs a continuum/infinite-volume limit, constructs the interacting quantum theory, or proves a mass gap. See [`docs/GLUEBALL_OPERATORS.md`](docs/GLUEBALL_OPERATORS.md).\n",
    "Phase 3A was merged in PR #11 and is frozen as `YM-GLUEBALL-OPERATORS-V1`: an unsmeared, gauge-invariant spatial Wilson-loop basis on cubic periodic `SU(3)` spatial slices, with finite-lattice `A1++` classification and continuum `0++` channel intent. Phase 3B1 is introduced in PR #12 as the candidate `YM-GLUEBALL-SPATIAL-STOUT-V1`, a deterministic spatial-only stout-smearing transformation with temporal links unchanged. Spatial blocking, connected correlators, effective masses, and glueball spectra remain unimplemented. No current phase performs a continuum/infinite-volume limit, constructs the interacting quantum theory, or proves a mass gap. See [`docs/GLUEBALL_OPERATORS.md`](docs/GLUEBALL_OPERATORS.md) and [`docs/GLUEBALL_SMEARING.md`](docs/GLUEBALL_SMEARING.md).\n",
)
replace_once(
    "README.md",
    "cargo run --quiet --locked --bin ym-glueball-operators\npython3 scripts/check_all_contracts.py\n",
    "cargo run --quiet --locked --bin ym-glueball-operators\ncargo run --quiet --locked --bin ym-glueball-smearing\npython3 scripts/check_all_contracts.py\n",
)

replace_once(
    "ROADMAP.md",
    "Phase 3A is implemented as a candidate in PR #11; review/freeze is still pending and the later spectroscopy stages remain unopened.\n\n- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, candidate).\n- [ ] Add smearing/blocking as separately versioned transformations.\n",
    "Phase 3A was merged in PR #11 and is frozen. Phase 3B1 begins the transformation layer with a separately versioned spatial stout-smearing candidate; spatial blocking and the spectroscopy stages remain unopened.\n\n- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, frozen).\n- [ ] Add smearing/blocking as separately versioned transformations.\n  - [x] Phase 3B1 spatial stout smearing (`YM-GLUEBALL-SPATIAL-STOUT-V1`, candidate).\n  - [ ] Spatial blocking remains separately versioned and unimplemented.\n",
)

replace_once(
    "AGENTS.md",
    "### Phase 3A glueball-operator candidate authority\n\nPhase 3A introduces the candidate `YM-GLUEBALL-OPERATORS-V1`. It defines unsmeared closed spatial Wilson-loop operators only. The reference uses Euclidean time direction `3`, spatial directions `[0,1,2]`, requires every spatial extent to be at least `3`, and requires equal spatial extents before using the cubic-lattice `A1++` label. Its continuum channel intent is `0++`; that is not an exact finite-spacing spin assignment.\n\nThe candidate does not authorize smearing/blocking, connected correlators, effective masses, a glueball spectrum, a finite-volume mass-gap claim, confinement, continuum QFT construction, or a Clay solution. Those require separately versioned later Phase 3 authorities.\n",
    "### Phase 3A frozen glueball-operator authority\n\nPhase 3A freezes `YM-GLUEBALL-OPERATORS-V1` after PR #11 merged. It defines unsmeared closed spatial Wilson-loop operators only. The reference uses Euclidean time direction `3`, spatial directions `[0,1,2]`, requires every spatial extent to be at least `3`, and requires equal spatial extents before using the cubic-lattice `A1++` label. Its continuum channel intent is `0++`; that is not an exact finite-spacing spin assignment.\n\nPhase 3A itself does not authorize connected correlators, effective masses, a glueball spectrum, a finite-volume mass-gap claim, confinement, continuum QFT construction, or a Clay solution. Later surfaces require separate contract identifiers.\n\n### Phase 3B1 spatial stout-smearing candidate authority\n\nPhase 3B1 introduces `YM-GLUEBALL-SPATIAL-STOUT-V1` as a deterministic transformation of the frozen Phase 3A operator input. It follows the stout-link construction with a project-specific spatial-only specialization: directions `[0,1,2]` are smeared synchronously, direction `3` is copied exactly, spatial extents must be equal and at least `3`, `0 < rho <= 0.25`, and at most `8` levels are admitted. The canonical regression uses `rho=0.10` and one level.\n\nThe stout parameter is an analysis/transformation parameter, not a physical observable. Phase 3B1 does not establish improved overlap, define spatial blocking, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.\n",
)
replace_once(
    "AGENTS.md",
    "cargo run --quiet --locked --bin ym-glueball-operators\npython3 scripts/check_all_contracts.py\n",
    "cargo run --quiet --locked --bin ym-glueball-operators\ncargo run --quiet --locked --bin ym-glueball-smearing\npython3 scripts/check_all_contracts.py\n",
)

replace_once(
    "reference/rust/src/glueball_smearing.rs",
    '''        assert_eq!(\n            spatial_stout_smear(&lattice, f64::NAN, 1),\n            Err(GlueballSmearingError::NonFiniteRho)\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, 0.0, 1),\n            Err(GlueballSmearingError::RhoOutOfRange)\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, MAX_RHO + 1.0e-6, 1),\n            Err(GlueballSmearingError::RhoOutOfRange)\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, CANONICAL_RHO, MAX_LEVELS + 1),\n            Err(GlueballSmearingError::TooManyLevels)\n        );\n\n        let too_small = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();\n        assert_eq!(\n            spatial_stout_smear(&too_small, CANONICAL_RHO, 1),\n            Err(GlueballSmearingError::SpatialExtentTooSmall)\n        );\n\n        let anisotropic = Su3Lattice4::new_identity([3, 4, 5, 3]).unwrap();\n        assert_eq!(\n            spatial_stout_smear(&anisotropic, CANONICAL_RHO, 1),\n            Err(GlueballSmearingError::SpatialExtentsNotCubic)\n        );\n''',
    '''        assert_eq!(\n            spatial_stout_smear(&lattice, f64::NAN, 1).unwrap_err(),\n            GlueballSmearingError::NonFiniteRho\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, 0.0, 1).unwrap_err(),\n            GlueballSmearingError::RhoOutOfRange\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, MAX_RHO + 1.0e-6, 1).unwrap_err(),\n            GlueballSmearingError::RhoOutOfRange\n        );\n        assert_eq!(\n            spatial_stout_smear(&lattice, CANONICAL_RHO, MAX_LEVELS + 1).unwrap_err(),\n            GlueballSmearingError::TooManyLevels\n        );\n\n        let too_small = Su3Lattice4::new_identity([2, 3, 3, 3]).unwrap();\n        assert_eq!(\n            spatial_stout_smear(&too_small, CANONICAL_RHO, 1).unwrap_err(),\n            GlueballSmearingError::SpatialExtentTooSmall\n        );\n\n        let anisotropic = Su3Lattice4::new_identity([3, 4, 5, 3]).unwrap();\n        assert_eq!(\n            spatial_stout_smear(&anisotropic, CANONICAL_RHO, 1).unwrap_err(),\n            GlueballSmearingError::SpatialExtentsNotCubic\n        );\n''',
)

print("PR #12 status/readiness surfaces synchronized")
