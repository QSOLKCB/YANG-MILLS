# Phase 3F — production scalar evidence and stability scans

Phase 3F is the first Phase 3 surface that executes the frozen scalar spectroscopy pipeline on **production Monte Carlo configurations** rather than an analytic mechanics fixture.

The candidate authority is:

```text
YM-GLUEBALL-SCALAR-PRODUCTION-V1
status = phase-3f-candidate
```

Its evidence class is a **finite-lattice production numerical observation**. That is materially stronger evidence than the synthetic mechanics fixtures used to validate Phases 3C–3E, but it is still not a continuum glueball mass, a confinement theorem, a construction of continuum quantum Yang–Mills theory, or a proof of a positive mass gap.

## Predeclaration discipline

The production schedule, stream identity, analysis windows, transformations, operators, and stability thresholds are machine-readable and frozen before the first production observation is inspected. If a scientific gate fails, the result must be retained. Thresholds may not be weakened and seeds may not be replaced after seeing the outcome under the same contract identifier.

A fresh stream or incompatible analysis semantics require a new contract identifier.

## Production ensemble

Phase 3F reuses the frozen Phase 2C1 `SU(3)` transition kernel:

```text
sampler  = YM-SU3-ENSEMBLE-V1
proposal = YM-SU3-GENERATOR-METROPOLIS-V1
rng      = SPLITMIX64-V1
stream   = YM-SU3-STREAM-V1
```

The predeclared schedule is:

```text
lattice                     = 4 x 4 x 4 x 10
beta                        = 5.7
master seed                 = 0x3f00202609150001
independent chains          = 16
chain IDs                   = 0..15
cold start per chain        = true
thermalization sweeps       = 64
post-thermalization sweeps  = 4
proposal angle              = 0.35
retained configurations     = 1 per chain
```

Each chain consumes exactly the frozen proposal accounting implied by 68 complete sweeps. The retained configurations therefore come from sixteen independently seeded Markov chains. Phase 3F does **not** pool sequential configurations from a single chain and pretend they are independent delete-one jackknife units.

Per-chain evidence includes the derived seed, proposal/accept counts, acceptance fraction, retained average plaquette, and final link unitarity/determinant residuals.

## Scalar operator surfaces

Every retained configuration is analyzed with the two frozen Phase 3A scalar `A1++` basis elements:

```text
plaquette-1x1
rectangle-2x1
```

The production operator surface is evaluated on three predeclared transformation states:

```text
raw
stout-1 : rho=0.10, levels=1
stout-2 : rho=0.10, levels=2
```

The frozen stride-2 blocking surface is intentionally **not** included in this Phase 3F scan. Its fine spatial extent requirement is at least `6`, while this predeclared production run uses spatial extent `4`. Phase 3F records that limitation rather than silently changing the production geometry after inspection.

## Connected correlators and jackknife units

For every operator/transformation surface, Phase 3F evaluates the frozen Phase 3C connected correlator. It also independently reconstructs the full correlator from the retained operator time series and requires agreement with the Phase 3C authority to absolute tolerance `2e-12`.

For each delete-one replica, the omitted unit is **one retained independent-chain configuration**. The vacuum mean and vacuum subtraction are recomputed from the retained configurations in that replica. The full-sample vacuum mean is never leaked into a delete-one resample.

## Frozen Phase 3E fit

Phase 3F uses the frozen `YM-GLUEBALL-SCALAR-FIT-V1` mechanics without changing them. Candidate windows are predeclared as:

```text
[1,3]
[2,4]
[1,4]
```

The primary production surface is:

```text
transformation = stout-2
operator       = plaquette-1x1
```

A fit rejection is a scientific observation. It is recorded with a stable reason code and does not become an implementation failure merely because a predeclared production correlator does not satisfy the frozen Phase 3E assumptions.

## Stability scans

Successful finite-lattice mass fits are compared with

```text
z = |m_a - m_b| / sqrt(sigma_a^2 + sigma_b^2).
```

The predeclared stability gate is `z <= 5`.

Three stability surfaces are evaluated:

1. **Fit-window stability** — on the primary `stout-2 / plaquette-1x1` surface, at least two of the three individual candidate-window fits must succeed and their maximum pairwise mass `z` must not exceed 5.
2. **Operator stability** — on `stout-2`, both plaquette and rectangle scalar fits must succeed and agree within `z <= 5`.
3. **Transformation stability** — for `plaquette-1x1`, at least two of `raw`, `stout-1`, and `stout-2` must fit and their maximum pairwise mass `z` must not exceed 5.

The aggregate `scalar_pipeline_validated` flag is true only if these stability conditions and the predeclared sampler diagnostics all pass.

## First predeclared production outcome

The first completed production receipt preserved the frozen schedule exactly. All sixteen independent chains completed with valid accounting, finite observables, acceptance fractions around `0.872`, and link residuals far below the frozen `5e-12` limit. The sampler diagnostics therefore passed.

The spectroscopy gate did **not** pass. All six predeclared scalar surfaces were rejected by the frozen effective-mass authority:

```text
raw     / plaquette-1x1  -> effective-mass-rejected
raw     / rectangle-2x1  -> effective-mass-rejected
stout-1 / plaquette-1x1  -> effective-mass-rejected
stout-1 / rectangle-2x1  -> effective-mass-rejected
stout-2 / plaquette-1x1  -> effective-mass-rejected
stout-2 / rectangle-2x1  -> effective-mass-rejected
```

Consequently:

```text
sampler_diagnostics_passed              = true
primary_window_successes                = 0
primary_window_max_z                    = unavailable
operator_stability_z                    = unavailable
transformation_stability_successes      = 0
transformation_stability_max_z          = unavailable
scalar_pipeline_validated               = false
physical_glueball_mass_claimed          = false
finite_volume_mass_gap_established      = false
```

This is a **retained negative scientific result**, not an implementation failure. The Phase 3F workflow itself passed because the predeclared experiment executed correctly and reported the negative spectroscopy outcome without weakening any gate.

Phase 3F must not be “repaired” by changing these seeds, windows, thresholds, or transformation choices under the same identifier. Any follow-up experiment must use a separately versioned, predeclared authority and should address the observed effective-mass rejection directly — for example with stronger independently predeclared ensemble evidence and/or a geometry/operator surface designed before inspection to improve the signal while preserving the existing Phase 3F record.

## Scientific outcome versus CI correctness

The production runtime distinguishes two questions:

- **Did the implementation execute the predeclared experiment correctly?** This is a CI/software question. Invalid accounting, malformed configurations, group-residual violations in required operations, broken Phase 3C replay, non-finite arithmetic, or contract drift are implementation failures.
- **Did the predeclared finite-lattice data satisfy every spectroscopy stability gate?** This is a scientific observation. A negative answer is preserved as `scalar_pipeline_validated=false` and does not make CI fail.

This separation prevents CI pressure from becoming an incentive to tune scientific thresholds after observing the result.

## Claim boundary

Phase 3F does not by itself establish:

- a universally accepted physical `0++` glueball mass;
- fit stability beyond the declared finite-lattice run;
- blocking stability on this `4^3` spatial geometry;
- a `2++` or `0-+` spectrum;
- finite-volume scaling;
- an infinite-volume limit;
- a continuum extrapolation;
- confinement;
- continuum quantum Yang–Mills existence;
- a positive continuum Yang–Mills mass gap;
- a Clay Millennium Problem solution.

Because the first Phase 3F production gate is negative, the next scientific phase should **not** advance immediately to additional irreducible channels. It should open a separately versioned, independently predeclared scalar confirmation/recovery experiment that addresses the effective-mass rejection while preserving Phase 3F unchanged as evidence.
