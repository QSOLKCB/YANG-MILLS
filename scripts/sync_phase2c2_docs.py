#!/usr/bin/env python3
"""One-shot idempotent synchronization of Phase 2C2 repository-facing docs."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected Phase 2C2 documentation block not found in {path_str}")
    path.write_text(text.replace(old, new, 1))


replace_once(
    "ROADMAP.md",
    """### Phase 2C2 — independent published `SU(3)` validation

Phase 2C1 does **not** authorize later `SU(3)` physics analysis by itself.

- [ ] Select a peer-reviewed pure-`SU(3)` Wilson-action finite-lattice benchmark only after the Phase 2C1 sampler mechanics are frozen.
- [ ] Predeclare its action normalization, lattice geometry/volume, coupling, observable, source treatment, chain identities, thermalisation, cadence, and statistical gates before inspecting the result.
- [ ] Reproduce the benchmark with independent chains and retained raw evidence under a separately versioned validation authority.
- [ ] Keep glueball analysis disabled until the independent `SU(3)` benchmark gate passes.
""",
    """### Phase 2C2 — independent published `SU(3)` validation

**Status: completed on PR #10 branch, V2 validation authority frozen pending review/merge.**

Phase 2C1 does **not** authorize later `SU(3)` physics analysis by itself. Phase 2C2 preserves the failed first attempt rather than rewriting it: `YM-SU3-PLAQUETTE-BASELINE-V1` reproduced the published plaquette envelope but failed its predeclared per-chain ESS floor (`4.510 < 8`). A separately predeclared fresh-stream `YM-SU3-PLAQUETTE-BASELINE-V2` then retained the same source surface, sampler, proposal scale, per-chain schedule, and diagnostic thresholds while using four fresh independent chains and an aggregate ESS floor of 16.

- [x] Select a peer-reviewed pure-`SU(3)` Wilson-action finite-lattice benchmark only after the Phase 2C1 sampler mechanics are frozen.
- [x] Predeclare its action normalization, lattice geometry/volume, coupling, observable, source treatment, chain identities, thermalisation, cadence, and statistical gates before inspecting the result.
- [x] Preserve the failed V1 attempt without lowering its threshold after inspection.
- [x] Reproduce the benchmark with fresh independent chains and retained raw evidence under the separately versioned V2 validation authority.
- [x] Keep glueball analysis disabled until the independent `SU(3)` benchmark gate passes.

Primary source: Edgar, McCrossen & Moriarty, *Journal of Physics G: Nuclear Physics* **7** (1981) L85–L88, DOI `10.1088/0305-4616/7/5/001`. The frozen comparison is pure `SU(3)` Wilson action on `4^4` at `beta=6.0`, with conservative graphical normalized-plaquette envelope `[0.56,0.63]`.

The successful V2 confirmation (Actions run `34761701867`, evidence head `f5a221fdad2046779358cc59460fae488626459b`) produced pooled mean `0.5992920043`, aggregate ESS `78.944`, maximum pairwise chain-consistency `z ~= 3.550`, and maximum stationarity `z ~= 1.465`. All predeclared acceptance, accounting, source-envelope, and group-residual gates passed.

**Phase 2C2 freeze gate:** `YM-SU3-PLAQUETTE-BASELINE-V2` and `SU3-PLAQUETTE-BASELINE-PHASE2C2-V2` are sealed as `phase-2c2-frozen`; `published_su3_baseline_reproduced=true`, `phase2c_complete=true`, and `glueball_analysis_enabled=true`. V1 remains `phase-2c2-v1-failed`. This authorizes Phase 3 work but is not itself a glueball result, confinement claim, continuum construction, or mass-gap proof.
""",
)

replace_once(
    "README.md",
    """Phase 2C1 is frozen in PR #9. It introduces the separately versioned `SU(3)` ensemble authority `YM-SU3-ENSEMBLE-V1`, using a correctness-first symmetric single-generator Metropolis kernel over the frozen Gell-Mann basis, a domain-separated `YM-SU3-STREAM-V1` chain identity, an exactly paired finite binary64 proposal-angle grid, allocation-free six-plaquette local action differences, a stress-tested 512-sweep reference bound, complete raw runtime evidence, and literal golden-chain regression vectors. See [`docs/SU3_ENSEMBLES.md`](docs/SU3_ENSEMBLES.md). The external peer-reviewed `SU(3)` benchmark remains deliberately open for Phase 2C2, so `published_su3_baseline_reproduced=false`, `glueball_analysis_enabled=false`, and `phase2c_complete=false`.

No current phase performs glueball extraction, a continuum/infinite-volume limit, construction of the interacting quantum theory, or a mass-gap proof.
""",
    """Phase 2C1 was merged in PR #9. It froze the separately versioned `SU(3)` ensemble authority `YM-SU3-ENSEMBLE-V1`, using a correctness-first symmetric single-generator Metropolis kernel over the frozen Gell-Mann basis, a domain-separated `YM-SU3-STREAM-V1` chain identity, an exactly paired finite binary64 proposal-angle grid, allocation-free six-plaquette local action differences, a stress-tested 512-sweep reference bound, complete raw runtime evidence, and literal golden-chain regression vectors. See [`docs/SU3_ENSEMBLES.md`](docs/SU3_ENSEMBLES.md).

Phase 2C2 is implemented in PR #10. The first predeclared authority, `YM-SU3-PLAQUETTE-BASELINE-V1`, reproduced the peer-reviewed `4^4`, `beta=6.0` plaquette surface but failed its predeclared per-chain ESS floor, so it remains explicitly failed. The separately predeclared `YM-SU3-PLAQUETTE-BASELINE-V2` used four fresh independent streams without changing the external target, sampler, proposal scale, or per-chain schedule. It passed with pooled mean `0.5992920043` inside the frozen `[0.56,0.63]` source envelope, aggregate ESS `78.944`, maximum pairwise chain-consistency `z ~= 3.550`, and maximum stationarity `z ~= 1.465`. The V2 authority is frozen with `published_su3_baseline_reproduced=true`, `phase2c_complete=true`, and `glueball_analysis_enabled=true`. See [`docs/PUBLISHED_SU3_PLAQUETTE_BASELINE.md`](docs/PUBLISHED_SU3_PLAQUETTE_BASELINE.md).

Phase 3 glueball analysis is now authorized but has not yet been implemented. No current phase performs glueball extraction, a continuum/infinite-volume limit, construction of the interacting quantum theory, or a mass-gap proof.
""",
)

replace_once(
    "README.md",
    """The sampler mechanics are frozen before selecting an external `SU(3)` finite-lattice benchmark. Phase 2C2 therefore retains

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```
""",
    """The Phase 2C1 sampler contract remains mechanically scoped and therefore retains

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

External validation is owned by the separately versioned Phase 2C2 V2 authority, which is frozen with all three corresponding graduation flags true. Those validation flags must not be back-propagated into `YM-SU3-ENSEMBLE-V1`.
""",
)

replace_once(
    "README.md",
    """cargo run --quiet --locked --bin ym-su3-ensemble
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
""",
    """cargo run --quiet --locked --bin ym-su3-ensemble
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
""",
)

replace_once(
    "README.md",
    """The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 2B closes the published `SU(2)` plaquette validation gate. Phase 2C1 freezes a separately versioned `SU(3)` sampler; Phase 2C2 must independently validate it against a predeclared published `SU(3)` finite-lattice benchmark before Phase 3 may consume `SU(3)` ensemble evidence.
""",
    """The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 2B closes the published `SU(2)` plaquette validation gate. Phase 2C1 freezes a separately versioned `SU(3)` sampler, and Phase 2C2 now freezes its independent published finite-lattice validation after preserving the failed V1 attempt and passing the fresh-stream V2 confirmation. Phase 3 may now begin under separately versioned glueball-analysis authorities.
""",
)

replace_once(
    "AGENTS.md",
    """Phase 2C1 must retain

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

until a separately predeclared Phase 2C2 peer-reviewed `SU(3)` benchmark is actually reproduced. Do not use the Phase 2B `SU(2)` result as external validation for `SU(3)`. Incompatible sampler, proposal, stream, update-order, local-action, diagnostic, evidence, or admitted-schedule semantics require a new contract identifier rather than mutation of the frozen V1 surface.
""",
    """The frozen Phase 2C1 sampler authority itself must retain

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

because external validation does not belong to the sampler contract. Phase 2C2 is now carried by the separately versioned `YM-SU3-PLAQUETTE-BASELINE-V2` authority, which records `published_su3_baseline_reproduced=true`, `glueball_analysis_enabled=true`, and `phase2c_complete=true` after the predeclared fresh-stream V2 confirmation passed. Do **not** back-propagate those true values into `YM-SU3-ENSEMBLE-V1`, and do not reclassify the preserved failed V1 validation attempt as a pass. Do not use the Phase 2B `SU(2)` result as external validation for `SU(3)`. Incompatible sampler, proposal, stream, update-order, local-action, diagnostic, evidence, or admitted-schedule semantics require a new contract identifier rather than mutation of the frozen V1 sampler surface.
""",
)

replace_once(
    "AGENTS.md",
    """cargo run --quiet --locked --bin ym-su3-ensemble
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
""",
    """cargo run --quiet --locked --bin ym-su3-ensemble
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
""",
)

print("Phase 2C2 repository-facing documentation synchronized")
