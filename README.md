# YANG-MILLS

**A deterministic, provenance-sealed computational and theorem-facing laboratory for pure Yang–Mills theory.**

YANG-MILLS studies lattice gauge dynamics, gauge-invariant observables, glueball spectra, continuum extrapolation, and the mathematical boundary surrounding the Yang–Mills existence and mass-gap problem.

> Numerical evidence is not a proof of the Millennium Problem.

## Phase 0 bootstrap

PR #1 establishes the project constitution and a deliberately small reference implementation:

- the Clay target and claim boundary;
- the GLUBALL engineering-donor boundary;
- mathematical and evidence contracts;
- a deterministic four-dimensional periodic `SU(2)` lattice core in Rust;
- Wilson plaquettes and the Wilson action;
- analytic and gauge-invariance regression tests;
- CI gates for contracts and the reference core.

The Phase 0 reference core does **not** perform Monte Carlo ensemble generation, glueball extraction, continuum extrapolation, or any proof-facing continuum construction. Those are later phases.

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

The initial code uses the standard quaternion representation of `SU(2)`. A unit quaternion

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

The Phase 0 Wilson action is

```text
S_W = beta * sum_p (1 - 1/2 Re Tr U_p).
```

For the quaternion representation, `1/2 Re Tr U_p` is exactly the scalar quaternion component before floating-point rounding.

## Run the reference checks

Requires the pinned Rust toolchain in [`rust-toolchain.toml`](rust-toolchain.toml).

```bash
cargo test --all-targets
cargo run --quiet --bin ym-su2-reference
python3 scripts/check_contracts.py
```

The executable reports two deliberately simple reference cases:

1. the identity lattice, whose Wilson action is zero;
2. a `3^4` lattice with one link set to a traceless `SU(2)` element, whose action is analytically `6 beta`.

## Repository map

```text
contracts/             machine-readable Phase 0 contract surfaces
docs/                  constitution, donor boundary, model, evidence rules
reference/rust/        canonical finite-lattice reference implementation
test-vectors/          human- and machine-readable regression vectors
tests/                 integration tests
scripts/               contract checks
.github/workflows/      CI gates
```

## GLUBALL donor boundary

[`QSOLKCB/GLUBALL`](https://github.com/QSOLKCB/GLUBALL) `v1.0.0` is an **engineering donor**, not a physics donor. YANG-MILLS may reuse or reimplement its deterministic evidence patterns, provenance discipline, reference-vs-accelerator separation, and test-vector strategy. GLUBALL knot geometry, topology language, ternary/triality metadata, phi-Weyl sampling, sonification, and any physical interpretation are outside the Yang–Mills model unless independently justified in a future proposal.

See [`docs/DONOR_BOUNDARY.md`](docs/DONOR_BOUNDARY.md).

## Roadmap

The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 1 expands the finite lattice core; later phases add ensemble generation, glueball observables, published-baseline reproduction, HPC acceleration, continuum scaling, constructive analysis, and finally formalization of stable theorem surfaces.

## Licence

Mozilla Public License 2.0.
