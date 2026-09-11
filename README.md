# YANG-MILLS

**A deterministic, provenance-sealed computational and theorem-facing laboratory for pure Yang–Mills theory.**

YANG-MILLS studies lattice gauge dynamics, gauge-invariant observables, glueball spectra, continuum extrapolation, and the mathematical boundary surrounding the Yang–Mills existence and mass-gap problem.

> Numerical evidence is not a proof of the Millennium Problem.

## Current implementation surface

Phase 0 was merged in PR #1. It established the project constitution, claim boundary, GLUBALL engineering-donor boundary, machine-readable contracts, and a minimal four-dimensional periodic `SU(2)` Wilson-lattice reference core.

Phase 1A was merged in PR #2 and froze the finite-lattice `SU(2)` observable/mechanics surface, including deterministic configuration identity, oriented paths, Wilson loops, Polyakov loops, staples, and local update mechanics.

Phase 1B was merged in PR #3 and froze `SU(3)` in two ordered gates:

1. `YM-SU3-ALGEBRA-V1` — canonical Gell-Mann generators, `f^{abc}`/`d^{abc}`, quadratic/cubic Casimir regressions, bounded `U(theta)=exp(i theta_a T^a)`, and fail-closed unitarity/determinant acceptance;
2. `YM-SU3-LATTICE-V1` — bounded periodic four-dimensional `SU(3)` links, plaquettes, Wilson action, and local gauge transformations.

Phase 1C was merged in PR #4 and froze the distinction

```text
abstract C3 ~= Z3
        !=
non-central <Omega> subset SU(3)
        !=
genuine Z(SU(3)) = {I, omega I, omega^2 I}.
```

Phase 1D was merged in PR #5 and froze `YM-PATH-HOLONOMY-V1`: arbitrary bounded `SU(3)` open/closed lattice transport, Wilson/Polyakov holonomy, endpoint covariance, reversal/concatenation, and the continuum path-ordering convention matched to the already-frozen lattice product order.

Phase 1E is implemented, CI-green, review-clean, and frozen in PR #6 pending merge. `YM-CLASSICAL-GEOMETRY-V1` freezes the classical bundle/connection dictionary: base geometry, principal/associated bundles, sections, covariant derivative, `ad P`-valued curvature, Hodge-star conventions, Bianchi identity, Yang–Mills functional/equation, self-duality, and the lattice-to-continuum dictionary. See [`docs/CLASSICAL_CONTINUUM.md`](docs/CLASSICAL_CONTINUUM.md).

No current phase performs Monte Carlo ensemble generation, glueball extraction, a continuum/infinite-volume limit, construction of the interacting quantum theory, or a mass-gap proof.

## The target

The Clay Mathematics Institute problem asks, in substance, for a non-trivial quantum Yang–Mills theory on four-dimensional Euclidean space for any compact simple gauge group, together with a strictly positive mass gap. See [`docs/CLAY_PROBLEM.md`](docs/CLAY_PROBLEM.md).

The repository therefore maintains a strict implication boundary:

```text
finite lattice calculation
        !=
classical continuum dictionary
        !=
continuum quantum Yang-Mills construction
        !=
proof of a positive continuum mass gap
```

## Frozen lattice/group conventions

The `SU(2)` reference uses

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

and

```text
S_W = beta * sum_p (1 - 1/2 Re Tr U_p).
```

The frozen `SU(3)` fundamental-representation convention is

```text
T^a = lambda_a / 2
Tr(lambda_a lambda_b) = 2 delta_ab
U(theta) = exp(i sum_a theta_a T^a),
```

with minimal lattice action

```text
S_W = beta * sum_p (1 - 1/3 Re Tr U_p).
```

The mathematical Lie algebra `su(3)` consists of traceless anti-Hermitian matrices. Phase 1E therefore freezes the bridge

```text
Phase 1B Hermitian basis : T^a
anti-Hermitian su(3) basis: i T^a.
```

## Paths and forward transport

Phase 1D freezes the start-left transporter

```text
dU_gamma/dt = U_gamma a_gamma(t)
U_gamma^Omega = Omega(start) U_gamma Omega(end)^-1.
```

Phase 1E introduces the conventional forward associated-fiber transport `P_gamma` and freezes

```text
U_gamma = P_gamma^-1.
```

Thus

```text
P_gamma^Omega
  = rho(Omega(end))
    P_gamma
    rho(Omega(start))^-1.
```

This makes the principal-bundle, associated-bundle, and lattice path conventions one consistent system rather than competing sign conventions.

## Classical continuum geometry

Phase 1E uses a principal bundle

```text
pi:P->M
```

with free **right** `G` action. A principal connection one-form `omega_P` yields a local gauge potential

```text
A = sigma^* omega_P
```

for a local section `sigma:U->P`. If `sigma'=sigma g`, then

```text
A' = g^-1 A g + g^-1 d g.
```

With the frozen Phase 1D variable `Omega=g^-1`, this becomes

```text
A^Omega = Omega A Omega^-1 - dOmega Omega^-1.
```

The global curvature type is

```text
F_A in Omega^2(M, ad P),
```

with local matrix formula

```text
F_A = dA + A wedge A
```

and Bianchi identity

```text
D_A F_A = 0.
```

Gauge curvature is kept distinct from Riemann curvature, Gaussian/surface curvature, and Frenet/path curvature.

For an oriented Riemannian four-manifold, Phase 1E freezes

```text
*^2 = +1 on 2-forms
D_A^* F_A = - * D_A * F_A
YM(A) = (1/2) integral_M |F_A|^2 dvol
Euler-Lagrange: D_A^* F_A = 0.
```

A Yang–Mills connection is a **critical point** of this functional; it is not automatically a global minimizer.

Self-duality/anti-self-duality are

```text
F_A = +/- *F_A.
```

Together with the Bianchi identity they imply the classical Yang–Mills equation. They do not imply a quantum mass gap.

## Executable Hodge convention fixture

The Phase 1E Rust reference is intentionally only a finite-dimensional sign/convention check in oriented Euclidean `R^4`. In the ordered 2-form basis

```text
01, 02, 03, 12, 13, 23
```

it freezes

```text
*01=23  *02=-13  *03=12
*12=03  *13=-02  *23=01.
```

It verifies `*^2=+1`, the wedge/Hodge inner-product identity, and self-dual/anti-self-dual decomposition. It is **not** a continuum manifold or PDE solver.

## Lattice-to-continuum boundary

A lattice link approximates the frozen start-left transport along an edge. A plaquette is a finite small-loop holonomy whose smooth small-spacing expansion contains `F_mu_nu`. A finite plaquette is not equal to pointwise continuum curvature, and the finite-spacing Wilson action is not exactly the continuum functional.

Likewise, no finite-lattice observable becomes a continuum quantum theorem merely because the classical dictionary is now explicit.

## Run the reference checks

Requires the pinned Rust toolchain in [`rust-toolchain.toml`](rust-toolchain.toml).

```bash
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo run --quiet --locked --bin ym-su2-reference
cargo run --quiet --locked --bin ym-su3-algebra
cargo run --quiet --locked --bin ym-su3-lattice
cargo run --quiet --locked --bin ym-su3-center
cargo run --quiet --locked --bin ym-path-holonomy
cargo run --quiet --locked --bin ym-classical-geometry
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

The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 1E closes the classical continuum dictionary before Phase 2 introduces statistical ensemble generation.

## Licence

Mozilla Public License 2.0.
