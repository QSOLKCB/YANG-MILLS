# YANG-MILLS

**A deterministic, provenance-sealed computational and theorem-facing laboratory for pure Yang–Mills theory.**

YANG-MILLS studies lattice gauge dynamics, gauge-invariant observables, glueball spectra, continuum extrapolation, and the mathematical boundary surrounding the Yang–Mills existence and mass-gap problem.

> Numerical evidence is not a proof of the Millennium Problem.

## Current implementation surface

Phase 0 was merged in PR #1. It established the project constitution, claim boundary, GLUBALL engineering-donor boundary, machine-readable contracts, and a minimal four-dimensional periodic `SU(2)` Wilson-lattice reference core.

Phase 1A was merged in PR #2 and froze the finite-lattice `SU(2)` observable/mechanics surface, including deterministic configuration identity, oriented paths, Wilson loops, Polyakov loops, staples, and local update mechanics.

Phase 1B was merged in PR #3 and froze `SU(3)` in two ordered gates:

1. `YM-SU3-ALGEBRA-V1` — canonical Gell-Mann generators, `f^{abc}`/`d^{abc}`, quadratic/cubic Casimir regressions, bounded `U(theta)=exp(i theta_a T^a)`, and fail-closed unitarity/determinant acceptance;
2. `YM-SU3-LATTICE-V1` — bounded periodic four-dimensional `SU(3)` links, plaquettes, Wilson action, and local gauge transformations, introduced only after the algebra authority was review-clean and frozen.

Phase 1C is implemented, CI-green, review-clean, and frozen in PR #4 pending merge. It deliberately distinguishes:

```text
abstract C3 ~= Z3
        !=
non-central <Omega> subset SU(3)
        !=
genuine Z(SU(3)) = {I, omega I, omega^2 I}.
```

It also adds `SU(3)` Polyakov holonomy/trace accessors and a finite-lattice center transformation that leaves the Wilson action invariant while rotating a Polyakov loop by the chosen center element. This is a transformation law only; no confinement/deconfinement inference is made.

See [`docs/SU3_ALGEBRA.md`](docs/SU3_ALGEBRA.md), [`docs/SU3_LATTICE.md`](docs/SU3_LATTICE.md), and [`docs/SU3_CENTER.md`](docs/SU3_CENTER.md).

No current phase performs Monte Carlo ensemble generation, glueball extraction, continuum extrapolation, or any proof-facing continuum construction.

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

The code uses the standard unit-quaternion representation of `SU(2)`:

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

with Wilson action

```text
S_W = beta * sum_p (1 - 1/2 Re Tr U_p).
```

## `SU(3)` reference surface

The frozen fundamental-representation convention is

```text
T^a = lambda_a / 2
Tr(lambda_a lambda_b) = 2 delta_ab
U(theta) = exp(i sum_a theta_a T^a).
```

The checked `Su3` type uses dagger-symmetric unitarity and determinant residuals. No projection-to-`SU(3)` step hides acceptance failures.

The minimal Phase 1B lattice action is

```text
S_W = beta * sum_p (1 - 1/3 Re Tr U_p).
```

## `C3`, `Z3`, and the `SU(3)` center

Phase 1C freezes the orientation

```text
Omega |k> = |k+1 mod 3>
```

for a non-central order-three permutation subgroup. An exact `SU(3)` witness `diag(-1,-1,1)` does not commute with `Omega`.

The genuine center is separately represented as

```text
Z(SU(3)) = {I, omega I, omega^2 I}
omega = exp(2 pi i / 3).
```

These two concrete subgroups are abstractly isomorphic to `C3 ~= Z3`, but they are not the same embedding or action.

For a periodic direction `mu`, multiplying all `U_mu(x)` on one `x_mu = slice` by a genuine center element `z` leaves every plaquette unchanged while

```text
L_mu -> z L_mu
(1/3) Tr L_mu -> z (1/3) Tr L_mu.
```

No area law, confinement, deconfinement, or thermodynamic order-parameter conclusion follows merely from this finite-lattice transformation rule.

## Deterministic configuration encoding

`YM-SU2-CONFIG-V1` serializes a lattice in canonical site-major/link-direction order using exact IEEE-754 binary64 bit patterns. This is an implementation/provenance surface, not a physical observable.

## Run the reference checks

Requires the pinned Rust toolchain in [`rust-toolchain.toml`](rust-toolchain.toml).

```bash
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo run --quiet --locked --bin ym-su2-reference
cargo run --quiet --locked --bin ym-su3-algebra
cargo run --quiet --locked --bin ym-su3-lattice
cargo run --quiet --locked --bin ym-su3-center
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative for every currently advertised contract surface.

## Repository map

```text
contracts/             machine-readable contract surfaces
docs/                  constitution, mathematical dictionaries, evidence rules
reference/rust/        deterministic reference implementations
test-vectors/          human- and machine-readable regression vectors
tests/                 integration tests
scripts/               fail-closed contract checks
.github/workflows/      CI gates
```

## GLUBALL donor boundary

[`QSOLKCB/GLUBALL`](https://github.com/QSOLKCB/GLUBALL) `v1.0.0` is an **engineering donor**, not a physics donor. See [`docs/DONOR_BOUNDARY.md`](docs/DONOR_BOUNDARY.md).

## Roadmap

The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 1C freezes the abstract/concrete center distinction; Phase 1D next develops the path/parallel-transport/holonomy dictionary.

## Licence

Mozilla Public License 2.0.
