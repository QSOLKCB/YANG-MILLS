# Phase 3J — scalar variational-basis mechanics

Phase 3J introduces a separately versioned **mechanics-only** authority for a two-operator scalar variational analysis.

It deliberately does **not** assume whether the still-running Phase 3I temporal-extent observation passes or fails. It also does not activate a new production recovery experiment.

The authority is:

```text
YM-GLUEBALL-SCALAR-VARIATIONAL-BASIS-V1
status = phase-3j-candidate
```

## Why this phase exists

The frozen scalar basis already contains two independent finite-lattice `A1++` operators:

- spatial plaquette `1x1`;
- spatial rectangle `2x1`.

Earlier production/recovery phases fit those observables separately. Phase 3J adds the missing **joint** analysis mechanics: the connected cross-correlator matrix and a bounded two-dimensional generalized eigenvalue problem (GEVP).

This is useful whether Phase 3I eventually passes or fails:

- if Phase 3I passes, the temporal surface still requires an independent confirmation before any production variational reinterpretation;
- if Phase 3I fails, Phase 3J may become a prerequisite for a new, separately predeclared variational scalar recovery authority.

## Connected cross-correlator matrix

For the ordered basis `O_0 = plaquette-1x1`, `O_1 = rectangle-2x1`, Phase 3J freezes

```text
C_ij(tau)
  = 1/2 [ <O_i(t) O_j(t+tau)> + <O_j(t) O_i(t+tau)> ]
    - <O_i><O_j>.
```

Every supplied configuration and every periodic Euclidean time origin receives equal arithmetic weight.

The explicit forward/reverse average makes the finite-sample matrix symmetric by construction. This is a new variational mechanics authority, not a retroactive change to frozen Phase 3C.

The diagonal entries must replay the frozen Phase 3C connected correlators within `2e-12`.

## Generalized eigenvalue problem

The frozen candidate reference times are

```text
t0 = 1
t1 = 2
```

and the mechanics solve

```text
C(t1) v = lambda C(t0) v.
```

The reference requires `C(t0)` to be positive definite. It performs a Cholesky whitening

```text
C(t0) = L L^T
B = L^-1 C(t1) L^-T
```

and selects the largest eigenvalue of the symmetric `2 x 2` matrix `B`.

The returned basis coefficients are normalized by

```text
v^T C(t0) v = 1
```

with a canonical sign convention: the first non-negligible coefficient is positive.

The projected correlator is then

```text
C_var(tau) = v^T C(tau) v.
```

Phase 3J does not yet feed this projected series into a production mass fit.

## Canonical fixture

The deterministic mechanics fixture contains two periodic states on `N_t = 16`:

```text
ground mass       = 0.4
excited mass      = 0.9
ground amplitude  = 1.2
excited amplitude = 0.8
ground coupling   = [1.0, 0.5]
excited coupling  = [0.25, 1.0]
```

At `t0=1`, `t1=2`, the principal generalized eigenvalue is frozen as

```text
0.673346666250657
```

and the projected reference-time correlator is normalized to exactly one within tolerance.

This fixture is deterministic mechanics evidence only. It is not a Monte Carlo ensemble and the masses inside it are not glueball measurements.

## Adversarial invariants

The reference checks that:

- the matrix diagonal replays frozen Phase 3C;
- the cross-correlator matrix is gauge invariant;
- basis permutation does not change the principal eigenvalue or projected correlator;
- independent rescaling of the two basis elements does not change the principal eigenvalue or projected correlator;
- asymmetric matrices fail closed;
- a non-positive-definite reference matrix fails closed;
- invalid reference times fail closed;
- non-finite inputs fail closed.

## Activation boundary

Phase 3J is not a production recovery phase.

```text
phase3i_outcome_assumed       = false
production_recovery_activated = false
```

A later production use requires a new versioned authority after the Phase 3I observation has been frozen.

## Claim boundary

Phase 3J does not establish:

- a physical `0++` glueball mass;
- a finite-volume mass gap;
- confinement;
- continuum Yang-Mills construction;
- a positive continuum mass gap;
- a Clay Millennium Prize solution.
