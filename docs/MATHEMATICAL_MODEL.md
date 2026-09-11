# Phase 0 mathematical model

## Scope

Phase 0 implements classical finite-lattice `SU(2)` link configurations and the Wilson plaquette action on a periodic four-dimensional hypercubic lattice. It does not yet define a Monte Carlo path-integral estimator or continuum quantum field theory.

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

The implementation accepts only finite, non-zero component vectors for normalisation and rejects zero/invalid axes for axis-angle construction.

## Lattice

The lattice has extents

```text
L = (L0, L1, L2, L3),  Li >= 2,
```

with periodic boundary conditions.

A positively oriented link beginning at site `x` in direction `mu` is

```text
U_mu(x) in SU(2),  mu in {0,1,2,3}.
```

The Phase 0 implementation stores exactly four positive-direction links per site. Negative-direction transport is represented through the dagger of a positive-direction link at the neighbouring site.

## Plaquette

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

For `SU(2)` quaternions this becomes

```text
S_W = beta * sum_x sum_{mu<nu} (1 - scalar(P_mu_nu(x))).
```

`beta` must be finite and non-negative in the Phase 0 API.

## Local gauge transformations

Let `Omega(x) in SU(2)` be an arbitrary site field. Links transform as

```text
U_mu(x) -> Omega(x) U_mu(x) Omega(x + mu)^dagger.
```

Then plaquettes transform by conjugation at their base site:

```text
P_mu_nu(x) -> Omega(x) P_mu_nu(x) Omega(x)^dagger,
```

so the trace and Wilson action are gauge invariant.

The Phase 0 test demonstrates this numerically on a deterministic non-trivial field. A later formal phase may prove the finite-lattice identity symbolically.

## Analytic regression vector

On a periodic `3^4` lattice, set every link to identity except one positive-direction link `U_mu(x0) = Q`, where `Q` is a traceless `SU(2)` element (`Re Tr Q = 0`).

That link participates in two plaquettes for each of the other three coordinate directions, for six affected plaquettes total. Every affected plaquette has scalar component zero, so

```text
S_W = 6 beta.
```

At `beta = 2`, the expected action is `12`.

This finite-lattice combinatorial result is a regression check, not a mass-gap statement.
