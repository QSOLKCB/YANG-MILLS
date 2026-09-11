# Phase 1B — standalone `SU(3)` algebra reference

## Scope

Phase 1B introduces `SU(3)` first as a finite-dimensional algebra/group reference surface. This layer is deliberately isolated from lattice dynamics until its conventions and invariants have survived CI and review.

It does **not** claim that triadic semantics, Q/K/V assignments, or other cross-domain constructions are QCD colour physics.

## Fundamental representation

The canonical Gell-Mann matrices `lambda_a`, `a=1,...,8`, are used in the fundamental three-dimensional representation. The Hermitian generators are

```text
T^a = lambda_a / 2
```

with normalization

```text
Tr(lambda_a lambda_b) = 2 delta_ab
Tr(T^a T^b) = (1/2) delta_ab.
```

Every `lambda_a` is checked to be Hermitian and traceless.

## Structure constants

The antisymmetric and symmetric structure constants are fixed by

```text
[T^a,T^b] = i f^{abc} T^c
```

and

```text
{T^a,T^b} = (1/3) delta_ab I + d^{abc} T^c.
```

The reference implementation extracts them through trace identities and then reconstructs every pairwise commutator and anticommutator from the resulting coefficients. Selected canonical values are additionally sealed as independent golden regressions, including

```text
f^123 = 1
f^147 = 1/2
f^156 = -1/2
f^458 = sqrt(3)/2

d^118 = 1/sqrt(3)
d^888 = -1/sqrt(3).
```

Indices in prose above are one-based; Rust APIs use zero-based indices `0..8`.

## Casimir regressions

With the normalization above, the fundamental quadratic Casimir regression is

```text
sum_a T^a T^a = (4/3) I.
```

Before any cubic invariant is used elsewhere in the project, Phase 1B separately seals the cubic operator regression

```text
sum_{a,b,c} d^{abc} T^a T^b T^c = (10/9) I.
```

These are finite-dimensional representation identities. They are not mass-gap statements.

## Bounded exponentiation into `SU(3)`

For finite real coefficients `theta_a`, the reference map is

```text
H(theta) = sum_a theta_a T^a
U(theta) = exp(i H(theta)).
```

`H(theta)` is Hermitian and traceless. Therefore the exact mathematical exponential is unitary with determinant one.

The dependency-free reference implementation uses bounded scaling-and-squaring with a 64-term Taylor series. The accepted coefficient domain is

```text
abs(theta_a) <= 64
```

for every component. The implementation does **not** project an arbitrary output back onto `SU(3)`. Instead, it computes the exponential and then fails closed unless both

```text
max |U^dagger U - I| <= 5e-12
abs(det(U) - 1) <= 5e-12.
```

This makes the residual checks meaningful rather than hiding an unstable exponential behind a projection step.

A diagonal golden vector uses only `T^3`:

```text
theta_3 = 0.7
U = diag(exp(i*0.35), exp(-i*0.35), 1).
```

## Lattice boundary

`YM-SU3-ALGEBRA-V1` initially sets

```text
lattice_links_enabled = false.
```

That is intentional. `SU(3)` lattice links may be added only after this standalone algebra/group surface is CI-green and reviewed. The link layer must depend on these sealed conventions rather than redefining them.

## Claim boundary

This phase establishes algebra and numerical-reference facts only. It does not establish confinement, a glueball spectrum, continuum existence, or a positive Yang-Mills mass gap.
