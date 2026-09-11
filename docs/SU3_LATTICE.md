# Phase 1B — minimal `SU(3)` lattice reference

## Dependency boundary

`YM-SU3-LATTICE-V1` depends on the already frozen `YM-SU3-ALGEBRA-V1` authority. The lattice layer may use the canonical Gell-Mann basis, `T^a=lambda_a/2`, bounded exponentiation, checked `Su3` type, and frozen residual tolerances, but it may not redefine them.

## Lattice

The reference lattice is a bounded periodic four-dimensional hypercubic lattice with positive-direction links

```text
U_mu(x) in SU(3), mu in {0,1,2,3}.
```

At most `262144` positive-direction links are admitted by the Phase 1B reference implementation. Size arithmetic overflow and oversized allocations fail before allocation.

Negative-direction transport follows the usual convention

```text
U_mu(x-mu)^dagger.
```

## Plaquette and Wilson action

The positively oriented plaquette is

```text
P_mu_nu(x)
  = U_mu(x)
    U_nu(x+mu)
    U_mu(x+nu)^dagger
    U_nu(x)^dagger.
```

The Wilson action is

```text
S_W = beta * sum_x sum_{mu<nu}
      (1 - (1/3) Re Tr P_mu_nu(x)).
```

`beta` must be finite and non-negative. Group-product failures and non-finite accumulated/final action values return errors; the reference action path does not panic or project matrices back onto `SU(3)`.

## Local gauge transformations

A site field `Omega(x) in SU(3)` acts by

```text
U_mu(x) -> Omega(x) U_mu(x) Omega(x+mu)^dagger.
```

The reference transformation samples `Omega(x)` exactly once per lattice site and caches those values before transforming links. This prevents a stateful callable from representing inconsistent values for the same site.

The deterministic regression helper uses only the frozen `T^3` exponential and reduces coordinate codes modulo a fixed bound before exponentiation, so it stays inside the declared `abs(theta_a)<=64` algebra domain for every `usize` coordinate.

## Exact single-link fixture

Phase 1B uses the three-cycle permutation matrix

```text
Omega_cycle =
[0 1 0]
[0 0 1]
[1 0 0]
```

only as an exact floating-point `SU(3)` regression fixture. It has

```text
Tr(Omega_cycle) = 0
det(Omega_cycle) = 1
Omega_cycle^dagger Omega_cycle = I.
```

On a periodic `3^4` lattice with all links identity except one such positive-direction link, six plaquettes are affected. Each contributes one unit to the Wilson action sum, so

```text
S_W = 6 beta,
```

and `S_W=12` at `beta=2`.

This use of a three-cycle matrix makes **no center-symmetry claim**. The distinction between a non-central cyclic subgroup and the genuine center `Z(SU(3))` is reserved for Phase 1C.

## Claim boundary

The exact identity/single-cycle finite-lattice regression statements are C1. Floating local-gauge-invariance residuals are C2 numerical observations with explicit tolerances. Neither class establishes confinement, continuum existence, or a Yang–Mills mass gap.
