# Phase 1C — `C3`, `Z3`, and the genuine `SU(3)` center

## Scope

Phase 1C separates three objects that are abstractly isomorphic but concretely different:

```text
abstract cyclic group C3 ~= Z3
non-central order-three subgroup <Omega> subset SU(3)
genuine center Z(SU(3)) = {I, omega I, omega^2 I}
```

“Same abstract group” is **not** treated as “same embedded subgroup” or “same action.”

This phase adds finite-dimensional group regressions and a bounded finite-lattice center transformation. It does not claim confinement, deconfinement, an area law, a glueball spectrum, continuum existence, or a mass gap.

## Abstract `C3`

The project uses

```text
C3 = <g | g^3 = e>
```

with elements represented by exponents modulo three. Composition is addition modulo three, so the abstract group is isomorphic to additive `Z3`.

This statement alone says nothing about how a copy of `C3` is embedded in `SU(3)`.

## Non-central cycle embedding

Phase 1C fixes the forward basis-cycle convention

```text
Omega |k> = |k+1 mod 3>
```

with matrix

```text
Omega = [[0,0,1],
         [1,0,0],
         [0,1,0]].
```

It has exact binary64 entries in `{0,1}`, satisfies

```text
Omega^3 = I,
det(Omega) = +1,
Omega^dagger Omega = I,
```

and therefore generates an order-three subgroup of `SU(3)`.

It is **not central**. The exact `SU(3)` matrix

```text
D = diag(-1,-1,1)
```

is used as a group-valued witness, with

```text
Omega D != D Omega.
```

The reference max-entry commutator residual is exactly `2`.

The Phase 1B three-cycle regression fixture used the inverse orientation. Both matrices generate the same non-central order-three subgroup, but Phase 1C freezes the orientation above for `Omega`.

## Genuine `SU(3)` center

Let

```text
omega = exp(2 pi i / 3).
```

Then

```text
Z(SU(3)) = {I, omega I, omega^2 I}.
```

Each element is represented as a scalar `3 x 3` matrix and is checked against the frozen Phase 1B `Su3` acceptance rule. Numerically,

```text
omega = -1/2 + i sqrt(3)/2.
```

The center representation is tested for:

- unitarity;
- determinant one;
- order three;
- multiplication matching exponent addition modulo three;
- commutation with independently generated `SU(3)` matrices.

Thus

```text
Z(SU(3)) ~= C3 ~= Z3
```

as abstract groups, while

```text
Z(SU(3)) != <Omega>
```

as concrete subgroups/actions.

## Polyakov center transformation

For a periodic lattice direction `mu`, choose one coordinate slice `x_mu = s` and a genuine center element `z`. Phase 1C defines the finite-lattice transformation

```text
U_mu(x) -> z U_mu(x)    when x_mu = s,
```

with all other links unchanged.

Because `z` is central, the two transformed links entering any affected plaquette contribute cancelling `z` and `z^dagger` factors. The Wilson action is therefore invariant, up to the declared floating-point tolerance of the reference implementation.

A Polyakov holonomy wrapping direction `mu` crosses the selected slice exactly once, so

```text
L_mu -> z L_mu
```

and its normalized trace transforms as

```text
(1/3) Tr L_mu -> z (1/3) Tr L_mu.
```

The reference surface checks this directly on the identity lattice and also checks Wilson-action invariance on the nontrivial Phase 1B single-cycle-link configuration.

## Claim boundary

The transformation law above is a finite-lattice symmetry fact/observation. Phase 1C does **not** infer from it that a sampled ensemble is confined or deconfined, does not report spontaneous center-symmetry breaking, and does not treat a single deterministic Polyakov loop as a thermodynamic order parameter measurement.

Those claims require a statistical ensemble, volume/temperature interpretation, uncertainty treatment, and later evidence contracts.
