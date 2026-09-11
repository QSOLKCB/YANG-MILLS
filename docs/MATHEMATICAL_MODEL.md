# Finite-lattice mathematical model

## Scope

The executable reference model is classical finite-lattice `SU(2)` gauge mechanics on a periodic four-dimensional hypercubic lattice. Phase 1A adds gauge-invariant loop observables, deterministic configuration encoding, staples, and local update mechanics. It still does not define a Monte Carlo path-integral estimator or a continuum quantum field theory.

## `SU(2)` representation

A real quadruple

```text
q = (a0, a1, a2, a3)
```

with

```text
a0^2 + a1^2 + a2^2 + a3^2 = 1
```

is mapped to

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3),
```

where `sigma_k` are the Pauli matrices.

Under this convention:

- quaternion multiplication is matrix multiplication;
- quaternion conjugation is Hermitian conjugation;
- `det U = ||q||^2 = 1` for a unit quaternion;
- `Re Tr U = 2 a0`.

Floating-point multiplication is re-normalized onto the unit-quaternion manifold after every product so the public `Su2` type preserves its defining group invariant.

## Lattice

The lattice has extents

```text
L = (L0, L1, L2, L3),  Li >= 2,
```

with periodic boundary conditions. The bounded reference implementation stores at most `2^20` positive-direction links and rejects size arithmetic overflow before allocation.

A positively oriented link beginning at site `x` in direction `mu` is

```text
U_mu(x) in SU(2),  mu in {0,1,2,3}.
```

Only positive-direction links are stored. Negative-direction transport from `x` in direction `-mu` is

```text
U_mu(x - mu)^dagger,
```

after first moving to the periodic neighbour `x - mu`.

## Plaquette and Wilson action

For `mu != nu`, the oriented plaquette is

```text
P_mu_nu(x)
  = U_mu(x)
    U_nu(x + mu)
    U_mu(x + nu)^dagger
    U_nu(x)^dagger.
```

The Wilson action sums each unoriented coordinate plane once using `mu < nu`:

```text
S_W = beta * sum_x sum_{mu<nu}
      (1 - 1/2 Re Tr P_mu_nu(x)).
```

`beta` must be finite and non-negative. A non-finite floating-point result is rejected rather than emitted as numerical evidence.

## Oriented paths and holonomy

A lattice path is a finite ordered sequence of forward/backward coordinate steps. For traversal-order segments `V_1, ..., V_n`, the reference holonomy is

```text
H(C) = V_1 V_2 ... V_n.
```

The path API returns both `H(C)` and the final periodic lattice coordinate. Open paths are valid transport paths. A Wilson loop requires the final coordinate to equal the starting coordinate.

For a closed path `C`, the normalized `SU(2)` Wilson observable is

```text
W(C) = 1/2 Re Tr H(C).
```

The implementation regression surface checks reverse-link cancellation, path closure, cyclic invariance of the closed-loop trace, local gauge invariance, identity-lattice `W(C)=1`, and rectangular `R x T` paths.

## Polyakov loops

For a chosen periodic direction `mu`, a Polyakov loop is the ordered product of positive links wrapping that direction exactly once. Phase 1A exposes the normalized trace as an observable and checks gauge invariance.

No confinement/deconfinement claim follows merely from defining or evaluating this observable on a finite deterministic configuration.

## Staples and local action mechanics

For a link `U_mu(x)`, there are two adjacent plaquettes in each of the other three coordinate directions, giving six staple terms in four dimensions. The implementation constructs their quaternion-component sum as a non-group-valued `Staple` object.

If `S_mu(x)` denotes that six-term staple sum, the local six-plaquette action is evaluated through the scalar part of `U_mu(x) S_mu(x)`. For a candidate replacement `U'_mu(x)`, `local_action_difference` returns

```text
Delta S = S_local(U') - S_local(U).
```

This is checked against the difference of two complete Wilson-action evaluations. These deterministic primitives are intended for later heat-bath/over-relaxation algorithms; Phase 1A does not implement stochastic sampling.

## Local gauge transformations

Let `Omega(x) in SU(2)` be an arbitrary site field. Links transform as

```text
U_mu(x) -> Omega(x) U_mu(x) Omega(x + mu)^dagger.
```

Plaquettes transform by conjugation at their base site, and closed path holonomies transform by conjugation at their basepoint. Their traces, the Wilson action, Wilson loops, and Polyakov loops are therefore gauge invariant.

The reference implementation caches one `Omega(x)` value per site before transforming links so a stateful callable cannot represent inconsistent site fields.

## Deterministic configuration serialization

`YM-SU2-CONFIG-V1` serializes links in canonical site-major order, then direction order `0,1,2,3`. Each quaternion component is represented by its exact IEEE-754 binary64 bit pattern as 16 lowercase hexadecimal digits.

Parsing is fail-closed:

- the header and extents must be canonical;
- declared and computed link counts must agree;
- each component token must be exactly 16 lowercase hexadecimal digits;
- each decoded quaternion must be finite and unit within the declared serialization tolerance;
- re-serialization must reproduce the input text exactly.

This encoding establishes deterministic identity/provenance for finite configurations; it does not make floating-point outputs physically exact.

## Analytic regression configuration

On a periodic `3^4` lattice, set every link to identity except one positive-direction link `U_0(0) = Q`, where `Q=(0,1,0,0)` is traceless.

That link participates in six affected plaquettes, so

```text
S_W = 6 beta.
```

At `beta = 2`, `S_W = 12`.

The same configuration provides additional Phase 1A vectors:

- a `2 x 1` rectangle in the `(0,1)` plane has `W=0` when it contains the traceless link once;
- the direction-0 Polyakov loop through that link has normalized trace `0`;
- replacing the traceless link by identity has local action difference `-12` at `beta=2`.

These are finite-lattice regression statements only.

## Lattice-to-continuum boundary

Link and plaquette holonomies are discrete gauge objects. In a controlled small-spacing expansion, plaquette holonomy contains information about continuum field strength `F_mu_nu`. A finite-spacing plaquette is not literally the continuum curvature tensor, and finite loop observables do not by themselves establish continuum confinement, continuum quantum-field-theory existence, or a mass gap.
