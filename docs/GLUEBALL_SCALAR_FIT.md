# Phase 3E — scalar plateau selection and correlated one-state fitting

Phase 3E is the frozen scalar spectroscopy-analysis mechanics authority after Phase 3D effective-mass mechanics. PR #16 merged this surface without changing its mathematical/statistical semantics during the lifecycle handoff.

The frozen authority is:

```text
YM-GLUEBALL-SCALAR-FIT-V1
status = phase-3e-frozen
```

The candidate→frozen transition is classified as **implementation-only lifecycle/provenance**. Window-selection, covariance, spectral-fit, uncertainty, canonical-fixture, and claim semantics are unchanged.

It is deliberately limited to **predeclared plateau-window selection plus correlated periodic one-state fitting** for the finite-lattice scalar `A1++` channel, whose continuum target is `0++`.

Phase 3E itself does **not** analyze a production Monte Carlo glueball ensemble, establish a physical glueball mass, validate operator-basis stability, solve a GEVP, establish a finite-volume mass gap, establish confinement, construct continuum quantum Yang–Mills theory, or prove a positive continuum mass gap. Production scalar evidence is owned by the separately versioned Phase 3F authority.

## 1. Predeclared candidate windows

Candidate windows are supplied before the fit. Phase 3E does not generate candidate windows from the observed data.

Each window is a contiguous integer range

```text
[t_min, t_max]
```

with

```text
1 <= t_min <= t_max < N_t/2
3 <= number of points <= 8.
```

At most 16 candidates are admitted by the bounded reference implementation.

For each candidate, the frozen Phase 3D periodic cosh effective mass and delete-one jackknife errors are evaluated. The inverse-variance weighted mean is

```text
m_bar =
  sum_t m_eff(t) / sigma(t)^2
  -------------------------
       sum_t 1 / sigma(t)^2
```

and the candidate passes when

```text
max_t |m_eff(t)-m_bar| / sigma(t) <= 2.5.
```

Positive finite jackknife errors are required. Among passing windows, selection is deterministic:

1. choose the window with the largest point count;
2. if tied, choose the later starting Euclidean time.

Input order is not a selection rule.

This is a frozen project-defined reference criterion. It is not represented as a universal physical definition of a plateau.

## 2. Full jackknife covariance

On the selected window, the delete-one jackknife covariance is

```text
Cov_ab = (N-1)/N * sum_i
         (C_i(a)-C_bar(a))
         (C_i(b)-C_bar(b)).
```

The complete covariance matrix is used. There is **no diagonal-only fallback**.

The reference requires more jackknife replicas than fitted time points and requires the covariance to be positive definite. Matrix inversion fails closed when the relative pivot falls below `1e-12` of the covariance scale.

The same covariance authority is used for the full-sample fit and the jackknife parameter refits. Phase 3E does not silently recompute or regularize a different covariance for each replica.

## 3. Correlated periodic one-state model

The fitted model is

```text
C(tau) = A [ exp(-m tau) + exp(-m (N_t-tau)) ].
```

For a fixed mass `m`, write the unit-amplitude model vector as `f(m)`. The amplitude is profiled analytically:

```text
A(m) = (f^T Cov^-1 C) / (f^T Cov^-1 f).
```

The correlated objective is

```text
chi^2(m) =
(C - A(m) f(m))^T Cov^-1 (C - A(m) f(m)).
```

The deterministic reference scans the frozen mass grid

```text
0.05 <= m <= 2.0
step = 0.001
1951 grid points.
```

The minimum finite chi-square is selected; an exact tie chooses the lower mass. Positive amplitude is required.

The degrees of freedom are

```text
N_window - 2
```

for the mass and amplitude parameters.

The grid is an explicit bounded reference algorithm, not an assertion that `0.001` is a universally adequate production fit resolution.

## 4. Parameter jackknife

After the full-sample window has been selected, every delete-one jackknife correlator is refit on that same window with the same covariance authority.

For either fitted parameter `theta` (`m` or `A`), Phase 3E reports

```text
theta_bar = (1/N) sum_i theta_i

sigma_JK(theta) = sqrt(
  (N-1)/N * sum_i (theta_i-theta_bar)^2
).
```

This preserves the nonlinear fit inside the resampling loop instead of propagating an error bar through the fit afterward.

## 5. Canonical mechanics fixture

The canonical fixture is **not a Monte Carlo ensemble** and is **not physical glueball evidence**.

The full correlator is the exact periodic one-state curve

```text
N_t = 16
m = 0.5
A = 1.0.
```

Eight jackknife-shaped synthetic replicas apply balanced Hadamard-derived perturbations on `tau=1..7` with relative scale `0.005` and a shared correlation component `0.25`. This deliberately creates a positive-definite covariance with non-zero off-diagonal structure.

The predeclared candidate windows are

```text
[1,3]
[2,5]
[3,6].
```

The two four-point windows pass the exact single-state plateau criterion, so the later-time tie break selects

```text
[3,6].
```

The fixture freezes the following mechanics regressions:

```text
plateau weighted mass = 0.5
fitted mass           = 0.5
fitted amplitude      = 1.0
chi-square             = 0
DoF                    = 2
jackknife mass mean    = 0.5
jackknife mass SE      = 0.005916079783099616
max off-diagonal corr  = 1/17
```

The maximum off-diagonal correlation is intentionally non-zero so the canonical covariance surface is genuinely correlated rather than accidentally diagonal.

## 6. Fail-closed boundaries

The reference rejects or declines:

- empty, duplicate, oversized, midpoint-crossing, or otherwise invalid candidate windows;
- candidate windows with degenerate effective-mass uncertainty;
- a candidate set with no passing plateau window;
- selected windows with too few jackknife replicas for covariance rank;
- non-positive-definite or numerically singular covariance;
- non-finite arithmetic;
- a mass grid with no admissible finite positive-amplitude fit;
- bounded allocation failure through `Result`.

No covariance diagonalization shortcut or hidden regularizer is introduced.

## Claim boundary

A correlated one-state fit parameter from a deterministic mechanics fixture is not a physical glueball mass.

Phase 3E does **not** establish:

- production scalar glueball spectroscopy;
- fit-window stability on sealed Monte Carlo evidence;
- operator-basis or smearing/blocking stability;
- a variational/GEVP spectrum;
- the `2++` or `0-+` channels;
- a finite-volume glueball spectrum;
- an infinite-volume or continuum extrapolation;
- confinement;
- continuum quantum Yang–Mills existence;
- a positive continuum mass gap;
- a Clay Millennium Problem solution.

Phase 3F is the separately versioned authority that applies these frozen mechanics to **predeclared production ensemble evidence with fit-window and operator/transformation stability scans**.
