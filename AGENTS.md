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

Standalone finite-dimensional algebra/group regressions that are not finite-lattice theorems should be published conservatively as implementation/contract facts unless the claim taxonomy is explicitly extended. Floating-point matrix-exponential or center-element residual vectors are not `C1` finite-lattice exact results.

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

## Mathematical authority

Phase 0 / Phase 1A freeze the `SU(2)` finite-lattice authority under `reference/rust/` and its frozen contracts. Do not silently change the sign convention, plaquette orientation, direction ordering, boundary conditions, lattice dimension, Wilson normalisation, or frozen contract identifiers.

The `SU(2)` convention is

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

with Wilson action

```text
S_W = beta * sum_{x, mu<nu} (1 - 1/2 Re Tr U_mu_nu(x)).
```

Phase 1B freezes a separate `SU(3)` authority in two ordered contracts. The algebra convention is

```text
T^a = lambda_a / 2
Tr(lambda_a lambda_b) = 2 delta_ab
U(theta) = exp(i sum_a theta_a T^a).
```

`YM-SU3-ALGEBRA-V1` is frozen first and retains `lattice_links_enabled=false` because that contract itself does not define lattice semantics. The dependent `YM-SU3-LATTICE-V1` may use but may not redefine the frozen algebra/group conventions, residuals, or tolerances.

The minimal `SU(3)` Wilson action is

```text
S_W = beta * sum_{x, mu<nu} (1 - 1/3 Re Tr P_mu_nu(x)).
```

Phase 1C introduces a **new** contract rather than mutating either frozen Phase 1B identifier. Agents must keep these three meanings separate:

```text
abstract C3 ~= Z3
non-central <Omega> subset SU(3)
genuine Z(SU(3)) = {I, omega I, omega^2 I}.
```

The Phase 1C orientation is `Omega|k>=|k+1 mod 3>`. An exact group-valued witness `diag(-1,-1,1)` proves that this permutation embedding is not central. The scalar center embedding must commute with tested `SU(3)` matrices and must never be conflated with `<Omega>` merely because both are abstractly `C3`.

A finite-lattice center transformation may rotate an `SU(3)` Polyakov loop by `z in Z(SU(3))` while leaving the Wilson action invariant. This transformation law is **not by itself** evidence of confinement, deconfinement, spontaneous symmetry breaking, or a thermodynamic order parameter. Those inferences require later ensemble/statistical contracts.

Do not import triadic ontology, Q/K/V-to-colour semantics, or other cross-domain interpretations into these authorities.

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

Before declaring a PR ready, run the complete currently advertised surface:

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

The unified contract entry points are authoritative. Adding a new contract surface without registering it in those commands is a readiness failure.

Tests must fail closed on invalid lattice extents, invalid group construction, non-finite inputs, group-product acceptance failures, broken contract structure, or weakened claim boundaries. Public `Result` APIs may not turn an admissible numerical failure into a panic merely because an internal helper expected success.

Floating-point equality is not assumed. Every numerical comparison must have an explicit tolerance and must not be advertised as bitwise cross-platform identity unless separately established.

## Formalisation

Do not add a theorem prover merely for prestige. Formalise only stable definitions and theorem surfaces. A finite-lattice or finite-dimensional algebra/group proof is valuable but must not be described as the continuum Yang–Mills existence/mass-gap proof.
