# YANG-MILLS agent contract

This repository intentionally separates software correctness, numerical evidence, physical inference, and mathematical proof. Coding and research agents must preserve that separation.

## Non-negotiable claim hierarchy

Use these classes:

1. **implementation fact** — a property established about code/tests/contracts;
2. **finite-lattice mathematical fact** — an exact or proved statement about the discrete model;
3. **numerical observation** — a computed result with declared parameters and tolerances/uncertainties;
4. **physical inference** — an interpretation supported by a specified model and evidence;
5. **continuum theorem** — a mathematically proved statement about the continuum theory;
6. **Clay solution claim** — only permissible if the full official existence-and-mass-gap problem is actually proved.

Never promote a result to a stronger class because a simulation is large, a GPU agrees with a CPU, an extrapolation looks stable, or a plot is persuasive.

## Required semantic refusals

An agent must refuse to write or approve language asserting that this repository has solved the Yang–Mills Millennium Problem merely because it has any of the following:

- a positive finite-lattice energy or mass estimate;
- a glueball mass estimate;
- an apparently positive continuum extrapolation;
- agreement with lattice literature;
- gauge-invariant numerical observables;
- a formal proof about the finite lattice;
- cross-runtime or CPU/GPU agreement.

## Phase 0 mathematical authority

For PR #1, the authoritative executable surface is the dependency-free Rust reference implementation under `reference/rust/` plus the contracts in `contracts/`.

The Phase 0 reference uses:

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

for unit quaternion `q`, quaternion multiplication for group multiplication, quaternion conjugation for Hermitian conjugation, and

```text
S_W = beta * sum_{x, mu<nu} (1 - 1/2 Re Tr U_mu_nu(x)).
```

Do not silently change the sign convention, plaquette orientation, direction ordering, boundary conditions, lattice dimension, or Wilson normalisation.

## Donor boundary

GLUBALL `v1.0.0` is an engineering donor only. The recorded tag target is:

```text
80941183d14531093117e122da0fc32c13d2464b
```

Allowed donor ideas include:

- provenance sealing;
- contract/version discipline;
- deterministic test vectors;
- reference-vs-accelerator separation;
- residual acceptance;
- bounded stress/testing patterns.

Not imported as Yang–Mills physics:

- `GLUBALL-KNOT-V1`;
- `(2,3)` torus-knot geometry;
- topology claims;
- phi-Weyl sampling;
- ternary/triality channels;
- sonification;
- RSH geometry;
- any implication that GLUBALL geometry represents gluons, glueballs, gauge fields, or the mass gap.

Any future proposal connecting those concepts to Yang–Mills must be introduced as a new hypothesis with independent motivation and tests.

## Change rules

When modifying an existing mathematical contract:

- state whether the change is editorial, implementation-only, or semantic;
- add/update regression vectors for semantic changes;
- do not reuse a frozen contract identifier for incompatible semantics;
- keep historical provenance intact;
- update `ROADMAP.md` if a phase gate changes.

## Testing rules

Before declaring a PR ready:

```bash
cargo test --all-targets
cargo run --quiet --bin ym-su2-reference
python3 scripts/check_contracts.py
```

Tests must fail closed on invalid lattice extents, invalid `SU(2)` construction, non-finite beta, or broken contract structure.

Floating-point equality is not assumed. Every numerical comparison must have an explicit tolerance and must not be advertised as bitwise cross-platform identity unless separately established.

## Formalisation

Do not add a theorem prover merely for prestige. Formalise only stable definitions and theorem surfaces. A finite-lattice proof is valuable but must not be described as the continuum Yang–Mills existence/mass-gap proof.
