# YANG-MILLS

**A deterministic, provenance-sealed computational and theorem-facing laboratory for pure Yang–Mills theory.**

YANG-MILLS studies lattice gauge dynamics, gauge-invariant observables, glueball spectra, continuum extrapolation, and the mathematical boundary surrounding the Yang–Mills existence and mass-gap problem.

> Numerical evidence is not a proof of the Millennium Problem.

## Current implementation surface

Phase 0 was merged in PR #1. It established the project constitution, claim boundary, GLUBALL engineering-donor boundary, machine-readable contracts, and a minimal four-dimensional periodic `SU(2)` Wilson-lattice reference core.

Phase 1A extends that core with finite-lattice observables and local mechanics:

- bounded periodic `SU(2)` reference lattices;
- exact deterministic `YM-SU2-CONFIG-V1` configuration serialization;
- arbitrary oriented lattice paths with forward and reverse-link transport;
- Wilson loops and rectangular `R x T` loops;
- Polyakov loops as gauge-invariant observables, without a confinement claim;
- six-term staple diagnostics plus authoritative normalized-plaquette local action and action differences;
- deterministic left-link update primitives for later ensemble algorithms;
- analytic/gauge-invariance/cyclic-trace regression tests;
- frozen contracts `YM-SU2-LATTICE-V1` and `YM-WILSON-LOOP-V1`.

Phase 1A still does **not** perform Monte Carlo ensemble generation, glueball extraction, continuum extrapolation, or any proof-facing continuum construction.

## The target

The Clay Mathematics Institute problem asks, in substance, for a non-trivial quantum Yang–Mills theory on four-dimensional Euclidean space for any compact simple gauge group, together with a strictly positive mass gap. See [`docs/CLAY_PROBLEM.md`](docs/CLAY_PROBLEM.md).

The repository therefore maintains a strict implication boundary:

```text
finite lattice calculation
        !=
continuum Yang-Mills construction
        !=
proof of a positive continuum mass gap
```

A finite-lattice spectral estimate may be scientifically useful evidence. It is not interchangeable with the theorem Clay asks for.

## `SU(2)` reference surface

The code uses the standard unit-quaternion representation of `SU(2)`. A unit quaternion

```text
q = (a0, a1, a2, a3)
```

represents

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3).
```

For a four-dimensional periodic lattice, links are `U_mu(x) in SU(2)` and the positively oriented plaquette is

```text
U_mu_nu(x) = U_mu(x)
             U_nu(x + mu)
             U_mu(x + nu)^dagger
             U_nu(x)^dagger.
```

The Wilson action is

```text
S_W = beta * sum_p (1 - 1/2 Re Tr U_p).
```

For the quaternion representation, `1/2 Re Tr U` is the scalar quaternion component before floating-point rounding.

## Oriented paths and Wilson loops

A forward path step `+mu` uses `U_mu(x)` and advances to `x + mu`. A backward step `-mu` first moves to `x - mu` and then uses `U_mu(x - mu)^dagger`.

For a closed path `C`, Phase 1A defines

```text
W(C) = 1/2 Re Tr product_C U
```

with factors multiplied in traversal order. Open paths are valid transport paths but are rejected by the `wilson_loop` API unless they close on the periodic lattice.

Polyakov loops wrap one complete periodic lattice extent. Their presence in the reference surface is an observable definition only; no confinement/deconfinement inference is made in Phase 1A.

## Deterministic configuration encoding

`YM-SU2-CONFIG-V1` serializes a lattice in a canonical site-major/link-direction order. Each quaternion component is stored as its exact IEEE-754 binary64 bit pattern using 16 lowercase hexadecimal digits. Round-trip parsing must reproduce the canonical text exactly and must preserve the unit-quaternion invariant. Parsing is incremental and enforces the bounded reference link count before processing the configuration body.

This is an implementation/provenance surface, not a physical observable.

## Run the reference checks

Requires the pinned Rust toolchain in [`rust-toolchain.toml`](rust-toolchain.toml).

```bash
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo run --quiet --locked --bin ym-su2-reference
python3 scripts/check_contracts.py
python3 scripts/test_contract_validation.py
```

The reference executable checks the Phase 0 action vectors plus Phase 1A rectangle, Polyakov-loop, local-action-difference, and canonical-serialization invariants.

## Repository map

```text
contracts/             machine-readable contract surfaces
docs/                  constitution, donor boundary, model, evidence rules
reference/rust/        canonical finite-lattice reference implementation
test-vectors/          human- and machine-readable regression vectors
tests/                 integration tests
scripts/               fail-closed contract checks
.github/workflows/      CI gates
```

## GLUBALL donor boundary

[`QSOLKCB/GLUBALL`](https://github.com/QSOLKCB/GLUBALL) `v1.0.0` is an **engineering donor**, not a physics donor. YANG-MILLS may reuse or reimplement its deterministic evidence patterns, provenance discipline, reference-vs-accelerator separation, and test-vector strategy. GLUBALL knot geometry, topology language, ternary/triality metadata, phi-Weyl sampling, sonification, and any physical interpretation are outside the Yang–Mills model unless independently justified in a future proposal.

See [`docs/DONOR_BOUNDARY.md`](docs/DONOR_BOUNDARY.md).

## Roadmap

The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 1A establishes the frozen finite `SU(2)` observable/mechanics surface. Later Phase 1 work adds `SU(3)` algebra, center symmetry, path/continuum dictionaries, then subsequent phases add ensemble generation, glueball observables, published-baseline reproduction, HPC acceleration, continuum scaling, constructive analysis, and formalization of stable theorem surfaces.

## Licence

Mozilla Public License 2.0.