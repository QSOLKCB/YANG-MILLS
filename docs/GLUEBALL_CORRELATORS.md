# Phase 3C — connected Euclidean correlators

Phase 3C introduces the first time-separated observable-analysis surface in the glueball laboratory. It defines **periodic connected Euclidean two-point estimators with vacuum subtraction** for the two frozen Phase 3A scalar `A1++` basis elements.

The frozen authority is:

```text
YM-GLUEBALL-CONNECTED-CORRELATOR-V1
status = phase-3c-frozen
```

Phase 3C was frozen after PR #14 merged. It freezes correlator mechanics only. It does **not** estimate statistical uncertainty, extract an effective mass, select a fit window, solve a variational generalized eigenvalue problem, or claim a glueball spectrum.

## Input surface

For every supplied periodic `SU(3)` lattice, Phase 3A produces the two zero-spatial-momentum scalar operator time series

```text
O_P(t) = spatial-plaquette-1x1 A1++ operator
O_R(t) = spatial-rectangle-2x1 A1++ operator.
```

All configurations must have identical lattice extents. The bounded reference implementation admits

```text
1 <= N_cfg <= 64
2 <= N_t   <= 64.
```

Raw unsmeared lattices, stout-smeared lattices, or stride-2 blocked lattices may be supplied as long as they satisfy the frozen operator geometry. Phase 3C itself does not tune or compare those transformation choices.

## Estimator

For either scalar basis element `O`, define the finite-sample vacuum mean using exactly the supplied configurations and all periodic Euclidean time origins:

```text
O_bar = (1 / (N_cfg N_t)) sum_(c,t) O_c(t).
```

The raw periodic two-point estimator is

```text
G_O(tau) = (1 / (N_cfg N_t))
           sum_(c,t) O_c(t) O_c((t + tau) mod N_t).
```

The connected correlator is

```text
C_O(tau) = G_O(tau) - O_bar^2.
```

Every configuration and every time origin has equal arithmetic weight. No time-reflection symmetrization is silently applied, and no uncertainty estimate is attached in this phase.

## Canonical analytic fixture

The deterministic regression fixture contains four `3x3x3x4` lattices. Configuration `c` is identity except for one exact traceless-cycle spatial link at

```text
[0, 0, 0, c]
```

in spatial direction `0`.

This fixture is **not** a Monte Carlo ensemble and is **not** a physical correlator measurement. It exists to make the estimator analytically checkable.

For the plaquette operator the single-defect slice value is `77/81`, while the other three slices are `1`. Therefore

```text
O_bar = 80/81
G(0)  = 6403/6561
G(1) = G(2) = G(3) = 79/81
C(0)  = 1/2187
C(1) = C(2) = C(3) = -1/6561.
```

For the rectangle operator the corresponding single-defect value is `25/27`, giving

```text
O_bar = 53/54
G(0)  = 703/729
G(1) = G(2) = G(3) = 26/27
C(0)  = 1/972
C(1) = C(2) = C(3) = -1/2916.
```

The executable reference replays those analytic values with absolute tolerance `2e-12`.

## Gauge invariance

Each configuration is independently transformed with the frozen deterministic local `SU(3)` gauge field. The complete output surface—both vacuum means, both raw correlator vectors, and both connected correlator vectors—must agree with the untransformed result within maximum residual `5e-11`.

## Fail-closed boundaries

The reference rejects:

- an empty configuration set;
- more than `64` configurations;
- Euclidean time extent above `64`;
- mixed lattice extents;
- non-finite operator samples or correlator arithmetic;
- allocation failures, which propagate through `Result`.

The identity configuration gives an exact zero connected correlator after vacuum subtraction.

## Claim boundary

A connected correlator is not a glueball mass. Phase 3C does not establish:

- statistical uncertainty on the correlator;
- an effective mass;
- a fit window;
- a variational/GEVP spectrum;
- a finite-volume mass gap;
- confinement;
- continuum quantum Yang–Mills construction;
- a positive continuum mass gap;
- a Clay Millennium Problem solution.

Phase 3D owns effective-mass analysis **with uncertainty propagation** under the separately versioned `YM-GLUEBALL-EFFECTIVE-MASS-V1` authority. Fit-window selection and spectral fitting remain later surfaces.
