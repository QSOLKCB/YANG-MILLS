# Phase 3B1 spatial stout smearing

Phase 3B1 adds one separately versioned deterministic transformation surface for the Phase 3A glueball operators: **spatial stout-link smearing**.

It is intentionally narrower than a spectroscopy pipeline. This phase does **not** define spatial blocking, connected correlators, vacuum subtraction, effective masses, fit windows, a glueball spectrum, confinement, a continuum quantum theory, or a positive continuum mass gap.

## Source and specialization

The method follows the stout-link construction of Colin Morningstar and Mike Peardon, *Physical Review D* **69**, 054501 (2004), DOI `10.1103/PhysRevD.69.054501`.

The repository freezes a project-specific spatial specialization:

- Euclidean time direction: `3`;
- smeared directions: `[0,1,2]` only;
- temporal links are copied exactly and are excluded from the stout staples;
- the spatial lattice must be cubic with every spatial extent at least `3`, matching the Phase 3A `A1++` admission surface;
- every level is synchronous: all level-`n+1` spatial links are computed only from level `n`.

This spatial-only restriction is a YANG-MILLS implementation choice. It is not represented as a claim that the cited paper uniquely requires this specialization.

## Frozen candidate formulas

For spatial `mu`, define the spatial staple sum

```text
C_mu(x) = rho * sum_(nu in {0,1,2}, nu != mu) [
    U_nu(x) U_mu(x+nu) U_nu(x+mu)^dagger
  + U_nu(x-nu)^dagger U_mu(x-nu) U_nu(x-nu+mu)
].
```

Then

```text
Omega_mu(x) = C_mu(x) U_mu(x)^dagger
Q_mu(x) = (i/2)(Omega_mu^dagger - Omega_mu)
          - (i/6) Tr(Omega_mu^dagger - Omega_mu) I
U'_mu(x) = exp(i Q_mu(x)) U_mu(x).
```

Under the frozen Gell-Mann normalization, `Q=sum_a q_a T^a` is recovered with

```text
q_a = Tr(Q lambda_a).
```

The update therefore uses the already-frozen `YM-SU3-ALGEBRA-V1` exponential rather than adding a new SU(3) projection algorithm.

Temporal links satisfy

```text
U'_3(x) = U_3(x)
```

exactly.

## Bounded reference parameters

The candidate reference surface freezes:

```text
canonical rho = 0.10
0 < rho <= 0.25
canonical levels = 1
maximum levels = 8
```

`levels=0` is an exact no-op. The smearing parameter is an analysis/transformation parameter, not a physical observable.

## Deterministic regression surface

The reference implementation requires:

- the identity lattice is an exact fixed point;
- zero levels are an exact no-op;
- temporal links are exactly unchanged;
- the canonical nontrivial fixture changes at least one spatial link;
- local gauge transformation commutes with smearing within the declared numerical tolerance;
- the stout Hermitian/traceless generator residual remains bounded;
- all smeared links remain inside the frozen SU(3) unitarity and determinant-one tolerances through the maximum admitted level count;
- invalid rho, excessive levels, too-small spatial geometry, and anisotropic spatial geometry fail closed.

## Claim boundary

A smoother operator field is not itself evidence that ground-state overlap improved. Any such statement requires later ensemble/correlator evidence.

Phase 3B1 therefore does **not** establish:

- a connected Euclidean correlator;
- an effective mass;
- a glueball mass or spectrum;
- a finite-volume mass gap;
- confinement;
- a continuum or infinite-volume extrapolation;
- construction of continuum quantum Yang-Mills theory;
- a positive continuum mass gap;
- a Clay solution.

Spatial blocking remains a separately versioned future transformation.
