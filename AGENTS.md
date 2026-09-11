# YANG-MILLS agent contract

This repository intentionally separates software correctness, numerical evidence, physical inference, classical geometry, continuum quantum construction, and mathematical proof. Coding and research agents must preserve that separation.

## Non-negotiable claim hierarchy

Use these classes:

1. **implementation fact** — a property established about code/tests/contracts;
2. **finite-lattice mathematical fact** — an exact or proved statement about the discrete model;
3. **numerical observation** — a computed result with declared parameters and tolerances/uncertainties;
4. **physical inference** — an interpretation supported by a specified model and evidence;
5. **continuum theorem** — a mathematically proved statement about the continuum theory;
6. **Clay solution claim** — only permissible if the full official existence-and-mass-gap problem is actually proved.

A correct classical bundle/connection identity is not automatically a construction of the quantum theory. Standalone finite-dimensional algebra, Hodge-star, center, path-covariance, or gauge-invariance regressions should be published conservatively as implementation/contract facts unless the claim taxonomy is explicitly extended.

Never promote a result to a stronger class because a simulation is large, a GPU agrees with a CPU, an extrapolation looks stable, or a classical equation is standard.

## Required semantic refusals

An agent must refuse to write or approve language asserting that this repository has solved the Yang–Mills Millennium Problem merely because it has any of the following:

- a positive finite-lattice energy or mass estimate;
- a glueball mass estimate;
- an apparently positive continuum extrapolation;
- agreement with lattice literature;
- gauge-invariant numerical observables;
- a formal proof about the finite lattice;
- a classical Yang–Mills connection or self-dual solution;
- a complete principal-bundle/curvature dictionary;
- cross-runtime or CPU/GPU agreement.

## Mathematical authority

Phase 0 / Phase 1A freeze the `SU(2)` finite-lattice authority. Do not silently change the sign convention, plaquette orientation, direction ordering, boundary conditions, lattice dimension, Wilson normalisation, path orientation, or frozen contract identifiers.

The `SU(2)` convention is

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

with Wilson action

```text
S_W = beta * sum_{x, mu<nu} (1 - 1/2 Re Tr U_mu_nu(x)).
```

Phase 1B freezes a separate `SU(3)` authority. The algebra convention is

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

Phase 1C freezes a separate center authority. Keep these meanings distinct:

```text
abstract C3 ~= Z3
non-central <Omega> subset SU(3)
genuine Z(SU(3)) = {I, omega I, omega^2 I}.
```

A finite-lattice center transformation is not by itself evidence of confinement, deconfinement, spontaneous symmetry breaking, or a thermodynamic order parameter.

### Phase 1D path/transport authority

Phase 1D preserves the path ordering already frozen by `YM-WILSON-LOOP-V1`:

```text
+mu : use U_mu(x), then advance x->x+mu
-mu : first advance x->x-mu, then use U_mu(x-mu)^dagger
path product : multiply segments left-to-right in traversal order.
```

The matching continuum convention is

```text
dU_gamma/dt = U_gamma a_gamma(t)
A^Omega = Omega A Omega^-1 - dOmega Omega^-1
U_gamma^Omega = Omega(start) U_gamma Omega(end)^-1.
```

If `gamma2 o gamma1` means traverse `gamma1` first and `gamma2` second, then

```text
U_{gamma2 o gamma1} = U_gamma1 U_gamma2.
```

Open transport is endpoint-covariant, not gauge invariant. Closed normalized traces are gauge invariant under ordinary periodic local gauge transformations. Phase 1C center transformations remain distinct.

The small-loop relation is a limiting continuum statement. Never equate a finite loop or finite plaquette directly with pointwise gauge curvature. Likewise Frenet curvature, torsion, and osculating geometry are not gauge curvature `F_A`.

### Phase 1E classical geometry authority

Phase 1E freezes `YM-CLASSICAL-GEOMETRY-V1`. It is a **classical dictionary**, not a quantum construction.

The principal bundle convention is

```text
pi:P->M
right action: p -> p g
connection form: omega_P in Omega^1(P,g)
R_g^* omega_P = Ad_(g^-1) omega_P.
```

For a local section `sigma:U->P`,

```text
A = sigma^* omega_P.
```

If `sigma'=sigma g`, then

```text
A' = g^-1 A g + g^-1 d g.
```

The already-frozen Phase 1D variable is

```text
Omega = g^-1,
```

so this must reproduce exactly

```text
A^Omega = Omega A Omega^-1 - dOmega Omega^-1.
```

Do not change one of these formulas without changing the others under a new contract version.

For an associated bundle

```text
E = P x_rho V
(p,v) ~ (p g, rho(g^-1)v),
```

local fields transform as `psi^Omega=rho(Omega)psi` and the local covariant derivative is

```text
D_A = d + rho_*(A).
```

The global curvature type is

```text
F_A in Omega^2(M, ad P),
```

with local matrix convention

```text
F_A = dA + A wedge A
```

and Bianchi identity

```text
D_A F_A = 0.
```

Do not collapse `F_A` into an ordinary scalar 2-form or into Riemann/Frenet/Gaussian curvature.

The mathematical Lie algebra `su(3)` is traceless anti-Hermitian. Phase 1B's `T^a=lambda_a/2` are Hermitian generators, so the corresponding anti-Hermitian basis is `i T^a`. This bridge is frozen.

For the oriented Riemannian four-dimensional convention,

```text
*^2 = +1 on 2-forms
D_A^* F_A = - * D_A * F_A
YM(A) = (1/2) integral_M |F_A|^2 dvol
Euler-Lagrange: D_A^* F_A = 0.
```

A Yang–Mills connection is a critical point, not automatically a global minimizer. Self-duality/anti-self-duality plus Bianchi implies the **classical** Yang–Mills equation; it does not imply a quantum mass gap.

The Levi-Civita connection is the unique torsion-free metric-compatible connection on `TM`; it is not the gauge connection. Its Riemann curvature is not `F_A`.

Phase 1E freezes the transport dictionary

```text
U_gamma = P_gamma^-1,
```

where `P_gamma` is forward associated-fiber transport and `U_gamma` is the Phase 1D start-left transporter. Multi-chart transport must use path subdivision and transition-function coordinate changes; no local potential may be silently extended outside its trivialization.

The finite-dimensional Rust Hodge fixture checks only sign/algebra conventions in oriented Euclidean `R^4`. It is not a continuum PDE/manifold solver.

Do not import triadic ontology, Q/K/V-to-colour semantics, or other cross-domain interpretations into these authorities.

## Donor boundary

GLUBALL `v1.0.0` is an engineering donor only. The recorded tag target is:

```text
80941183d14531093117e122da0fc32c13d2464b
```

Allowed donor ideas include provenance sealing, contract/version discipline, deterministic test vectors, reference-vs-accelerator separation, residual acceptance, and bounded stress/testing patterns.

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
cargo run --quiet --locked --bin ym-path-holonomy
cargo run --quiet --locked --bin ym-classical-geometry
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative. Adding a new contract surface without registering it in those commands is a readiness failure.

Tests must fail closed on invalid lattice extents, invalid group construction, invalid/oversized paths, non-finite inputs, group-product acceptance failures, broken contract structure, or weakened claim boundaries. Public `Result` APIs may not turn an admissible numerical or allocation failure into a panic merely because an internal helper expected success.

Floating-point equality is not assumed. Every numerical comparison must have an explicit tolerance and must not be advertised as bitwise cross-platform identity unless separately established.

## Formalisation

Do not add a theorem prover merely for prestige. Formalise only stable definitions and theorem surfaces. A finite-lattice, finite-dimensional group/Hodge, or classical bundle/connection proof is valuable but must not be described as the continuum quantum Yang–Mills existence/mass-gap proof.
