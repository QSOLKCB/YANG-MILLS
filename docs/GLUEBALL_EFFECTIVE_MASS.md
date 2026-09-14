# Phase 3D — scalar effective mass with jackknife uncertainty

Phase 3D adds a separately versioned analysis surface on top of the Phase 3C connected Euclidean correlator authority. It is deliberately limited to **effective-mass mechanics plus uncertainty propagation** for the scalar `A1++` channel, whose continuum target is `0++`.

PR #15 merged this authority and froze it as:

```text
YM-GLUEBALL-EFFECTIVE-MASS-V1
status = phase-3d-frozen
```

This phase does **not** select a fit window, perform a correlated spectral fit, solve a variational GEVP, report a physical glueball mass, establish a finite-volume mass gap, establish confinement, construct continuum quantum Yang–Mills theory, or prove a positive continuum mass gap. Those remain separately versioned later authorities; Phase 3E now owns plateau/window selection and correlated scalar-fit mechanics.

## Periodic cosh effective mass

For a periodic connected correlator `C(tau)` with Euclidean time extent `N_t`, Phase 3D uses the nearest-neighbour cosh estimator

```text
m_eff(tau) = acosh(
    (C(tau-1) + C(tau+1)) / (2 C(tau))
)
```

on predeclared integer lags satisfying

```text
1 <= tau < N_t / 2.
```

The bounded reference surface admits

```text
4 <= N_t <= 64.
```

No hidden time-reflection symmetrization is applied. The estimator fails closed when `C(tau) <= 0`, when the `acosh` argument is below `1`, when values are non-finite, when shapes differ, or when the requested lag set is empty, unordered, duplicated, or reaches the temporal midpoint.

## Delete-one-configuration jackknife

For `N` original configurations, uncertainty propagation is defined by `N` delete-one replicas. Replica `i` omits configuration `i` and **recomputes the complete Phase 3C connected correlator**, including its vacuum mean and vacuum subtraction, from only the retained configurations.

The full-sample effective mass is the reported point estimate. Each jackknife replica is passed through the same nonlinear cosh transform:

```text
m_i(tau) = m_eff[C_i](tau).
```

The jackknife mean is

```text
m_bar(tau) = (1/N) sum_i m_i(tau)
```

and the standard error is

```text
sigma_JK(tau) = sqrt(
    (N-1)/N * sum_i (m_i(tau) - m_bar(tau))^2
).
```

The reference requires

```text
3 <= N <= 64
```

jackknife replicas. Reusing the full-sample vacuum mean inside the replicas is outside the contract because it would leak omitted-sample information back into the resample.

## Analytic mechanics fixture

The canonical Phase 3D fixture is not a Monte Carlo ensemble. It is a deterministic single-state periodic correlator family

```text
C(tau) = A [ exp(-m tau) + exp(-m (N_t - tau)) ]
```

with

```text
N_t = 16
A = 1
m_full = 0.5
```

and analysis lags `1..7`. For this exact form, the cosh identity returns

```text
m_eff(tau) = 0.5
```

at every admitted lag up to floating replay tolerance `2e-12`.

Five deterministic synthetic jackknife correlators use masses

```text
0.48, 0.49, 0.50, 0.51, 0.52.
```

Their jackknife mean is `0.5`, and the exact reference standard error is

```text
sqrt(0.0008) = 0.0282842712474619.
```

This fixture validates the nonlinear transform and jackknife algebra only. It is **not** evidence for a physical glueball mass.

## Why the cosh estimator

A zero-momentum state on a periodic Euclidean lattice contributes both forward and backward propagation across the temporal boundary. The single-state periodic form therefore has a cosh structure. The nearest-neighbour ratio above exactly recovers the mass parameter for that idealized single-state correlator.

Real lattice data can contain excited-state contamination, autocorrelation, finite-volume effects, discretisation effects, operator-overlap dependence, and covariance between time separations. Phase 3D does not pretend that a local effective-mass point resolves those issues.

## Claim boundary

A finite-lattice effective-mass point with a jackknife error bar is not automatically a fitted glueball mass. In particular, Phase 3D does not establish:

- a statistically justified plateau or fit window;
- a correlated constant or multi-exponential spectral fit;
- a variational basis or generalized eigenvalue problem;
- the `2++` or `0-+` channels;
- a finite-volume glueball spectrum;
- an infinite-volume or continuum extrapolation;
- confinement;
- continuum quantum Yang–Mills existence;
- a positive continuum mass gap;
- a Clay Millennium Problem solution.

Phase 3E is separately versioned specifically so those later mechanics are not back-propagated into the frozen Phase 3D authority.
