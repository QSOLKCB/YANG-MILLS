#!/usr/bin/env python3
"""One-shot status/readiness synchronization for PR #13."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text(encoding="utf-8")
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected patch block not found in {path_str}: {old[:180]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


replace_once(
    "README.md",
    "Phase 3A was merged in PR #11 and is frozen as `YM-GLUEBALL-OPERATORS-V1`: an unsmeared, gauge-invariant spatial Wilson-loop basis on cubic periodic `SU(3)` spatial slices, with finite-lattice `A1++` classification and continuum `0++` channel intent. Phase 3B1 is introduced in PR #12 as the candidate `YM-GLUEBALL-SPATIAL-STOUT-V1`, a deterministic spatial-only stout-smearing transformation with temporal links unchanged. Spatial blocking, connected correlators, effective masses, and glueball spectra remain unimplemented. No current phase performs a continuum/infinite-volume limit, constructs the interacting quantum theory, or proves a mass gap. See [`docs/GLUEBALL_OPERATORS.md`](docs/GLUEBALL_OPERATORS.md) and [`docs/GLUEBALL_SMEARING.md`](docs/GLUEBALL_SMEARING.md).\n",
    "Phase 3A was merged in PR #11 and is frozen as `YM-GLUEBALL-OPERATORS-V1`. Phase 3B1 was merged in PR #12 and is frozen as `YM-GLUEBALL-SPATIAL-STOUT-V1`, a deterministic spatial-only stout-smearing transformation. Phase 3B2 is introduced in PR #13 as candidate `YM-GLUEBALL-SPATIAL-BLOCK-V1`, a parameter-free stride-2 spatial coarse-link map that halves even cubic spatial extents while preserving Euclidean time extent. Connected correlators, effective masses, and glueball spectra remain unimplemented. No current phase performs a continuum/infinite-volume limit, constructs the interacting quantum theory, or proves a mass gap. See [`docs/GLUEBALL_OPERATORS.md`](docs/GLUEBALL_OPERATORS.md), [`docs/GLUEBALL_SMEARING.md`](docs/GLUEBALL_SMEARING.md), and [`docs/GLUEBALL_BLOCKING.md`](docs/GLUEBALL_BLOCKING.md).\n",
)
replace_once(
    "README.md",
    "cargo run --quiet --locked --bin ym-glueball-smearing\npython3 scripts/check_all_contracts.py\n",
    "cargo run --quiet --locked --bin ym-glueball-smearing\ncargo run --quiet --locked --bin ym-glueball-blocking\npython3 scripts/check_all_contracts.py\n",
)

replace_once(
    "ROADMAP.md",
    "Phase 3A was merged in PR #11 and is frozen. Phase 3B1 begins the transformation layer with a separately versioned spatial stout-smearing candidate; spatial blocking and the spectroscopy stages remain unopened.\n\n- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, frozen).\n- [ ] Add smearing/blocking as separately versioned transformations.\n  - [x] Phase 3B1 spatial stout smearing (`YM-GLUEBALL-SPATIAL-STOUT-V1`, candidate).\n  - [ ] Spatial blocking remains separately versioned and unimplemented.\n",
    "Phase 3A was merged in PR #11 and is frozen. Phase 3B1 was merged in PR #12 and is frozen. Phase 3B2 introduces the separately versioned stride-2 spatial blocking candidate; connected correlators and later spectroscopy stages remain unopened.\n\n- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, frozen).\n- [ ] Add smearing/blocking as separately versioned transformations.\n  - [x] Phase 3B1 spatial stout smearing (`YM-GLUEBALL-SPATIAL-STOUT-V1`, frozen).\n  - [x] Phase 3B2 stride-2 spatial blocking (`YM-GLUEBALL-SPATIAL-BLOCK-V1`, candidate).\n",
)

replace_once(
    "AGENTS.md",
    "### Phase 3B1 spatial stout-smearing candidate authority\n\nPhase 3B1 introduces `YM-GLUEBALL-SPATIAL-STOUT-V1` as a deterministic transformation of the frozen Phase 3A operator input. It follows the stout-link construction with a project-specific spatial-only specialization: directions `[0,1,2]` are smeared synchronously, direction `3` is copied exactly, spatial extents must be equal and at least `3`, `0 < rho <= 0.25`, and at most `8` levels are admitted. The canonical regression uses `rho=0.10` and one level.\n\nThe stout parameter is an analysis/transformation parameter, not a physical observable. Phase 3B1 does not establish improved overlap, define spatial blocking, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.\n",
    "### Phase 3B1 frozen spatial stout-smearing authority\n\nPhase 3B1 freezes `YM-GLUEBALL-SPATIAL-STOUT-V1` after PR #12 merged. It is a deterministic transformation of the frozen Phase 3A operator input. Directions `[0,1,2]` are smeared synchronously, direction `3` is copied exactly, spatial extents must be equal and at least `3`, `0 < rho <= 0.25`, and at most `8` levels are admitted. The canonical regression uses `rho=0.10` and one level.\n\nThe stout parameter is an analysis/transformation parameter, not a physical observable. Phase 3B1 does not establish improved overlap, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.\n\n### Phase 3B2 spatial blocking candidate authority\n\nPhase 3B2 introduces `YM-GLUEBALL-SPATIAL-BLOCK-V1` as a deterministic parameter-free stride-2 spatial coarse-link map. Fine spatial extents must be equal, even, and at least `6`; the output extents are `[Lx/2,Ly/2,Lz/2,Nt]`. A coarse site `y` embeds as `(2y0,2y1,2y2,y3)`. Spatial coarse links are the ordered product of the two fine spatial links along the blocked edge; temporal links are sampled exactly at the embedded fine site. No projection, averaging, or smearing is part of this blocking authority.\n\nThe gauge-covariance regression uses the restricted coarse gauge field `G_c(y)=G_f(2y0,2y1,2y2,y3)`. Phase 3B2 does not establish improved overlap, validate combined smear/block tuning, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.\n",
)
replace_once(
    "AGENTS.md",
    "cargo run --quiet --locked --bin ym-glueball-smearing\npython3 scripts/check_all_contracts.py\n",
    "cargo run --quiet --locked --bin ym-glueball-smearing\ncargo run --quiet --locked --bin ym-glueball-blocking\npython3 scripts/check_all_contracts.py\n",
)

print("PR #13 status/readiness surfaces synchronized")
