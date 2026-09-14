# Phase 3G — fresh-stream scalar recovery confirmation

Phase 3G follows the merged and preserved negative Phase 3F production observation. It does **not** retune `YM-GLUEBALL-SCALAR-PRODUCTION-V1`.

The authority is now frozen after PR #18 merged:

```text
YM-GLUEBALL-SCALAR-RECOVERY-V1
status = phase-3g-frozen
```

Its purpose was deliberately narrow: test whether the Phase 3F effective-mass rejection was dominated by insufficient **independent configuration count** before changing the lattice geometry, operator basis, fit windows, transformation parameters, or scientific thresholds.

## Frozen Phase 3F baseline

Phase 3F used sixteen independently seeded `4 x 4 x 4 x 10` chains at `beta=5.7`. Sampler diagnostics passed, but all six scalar surfaces were rejected by the frozen effective-mass authority and `scalar_pipeline_validated=false`.

That result is frozen historical evidence. Phase 3G may compare against it; Phase 3G may not rewrite it.

## Predeclared recovery hypothesis

The hypothesis was:

> Doubling independently seeded retained configurations from 16 to 32 at fixed geometry and frozen analysis semantics may reduce scalar-correlator noise enough for the existing effective-mass and stability gates to become admissible.

This was a hypothesis, not an expected result.

Phase 3G changed only:

```text
master stream: fresh
independent chains: 16 -> 32
```

It deliberately kept fixed:

```text
lattice                     = 4 x 4 x 4 x 10
beta                        = 5.7
cold start per chain        = true
thermalization sweeps       = 64
post-thermalization sweeps  = 4
proposal angle              = 0.35
retained configurations     = 1 per independent chain
operators                   = plaquette-1x1, rectangle-2x1
transformations             = raw, stout-1, stout-2
stout rho                   = 0.10
candidate windows           = [1,3], [2,4], [1,4]
primary surface             = stout-2 / plaquette-1x1
maximum pairwise mass z     = 5
correlator replay tolerance = 2e-12
```

The new master seed is:

```text
0x3f01202609150001
```

The Phase 3F master seed was:

```text
0x3f00202609150001
```

They are intentionally distinct.

## Why the geometry was not changed

The Phase 3F spatial extent `4` is too small for the frozen stride-2 blocking authority, which requires a fine spatial extent of at least `6`.

Phase 3G nevertheless kept spatial extent `4` because this experiment was intended to isolate one variable: independent statistics.

If geometry and statistics were changed simultaneously, a recovered signal could not be cleanly attributed to either change.

Therefore Phase 3G still does not claim blocking stability.

## Independent jackknife units

Exactly one configuration is retained from each of the 32 independently seeded Markov chains.

Delete-one jackknife units therefore remain independent-chain configurations rather than sequential measurements from one chain.

Every delete-one replica recomputes the complete connected correlator and vacuum subtraction under the frozen Phase 3C semantics.

## Fit and failure semantics

The Phase 3D/3E mechanics remain unchanged.

Data-dependent admissibility failures such as a non-positive central correlator or failed `acosh` domain remain scientific rejection evidence.

Implementation failures remain implementation failures. This includes:

- malformed or non-finite generated data;
- SU(3) link residual violations;
- proposal-accounting drift;
- allocation failure;
- non-finite fit arithmetic;
- contract or shape failures;
- broken Phase 3C correlator replay.

A negative scalar scientific gate does not by itself make CI fail.

## Stability gate

The aggregate gate remains the Phase 3F gate:

1. sampler acceptance diagnostics pass;
2. the primary `stout-2 / plaquette-1x1` surface fits;
3. at least two primary candidate windows fit and every successful pair is scorable with maximum pairwise `z <= 5`;
4. stout-2 plaquette and rectangle fits both succeed and agree within `z <= 5`;
5. at least two plaquette transformation surfaces fit and every successful pair agrees within `z <= 5`.

No threshold is relaxed for Phase 3G.

## First Phase 3G observation

The predeclared fresh-stream observation ran on source head

```text
65dacc9f03cb0fd74826cd211ee7bd586668e08a
```

in GitHub Actions workflow run

```text
34875522823
```

The 32 independent chains consumed exactly `44,564,480` proposals in total. Every chain completed with finite observables, acceptance inside the frozen `[0.10,0.95]` interval, and final unitarity/determinant residuals far below `5e-12`. Sampler diagnostics therefore passed.

The scalar spectroscopy outcome was unchanged from Phase 3F:

```text
raw     / plaquette-1x1 -> effective-mass-rejected
raw     / rectangle-2x1 -> effective-mass-rejected
stout-1 / plaquette-1x1 -> effective-mass-rejected
stout-1 / rectangle-2x1 -> effective-mass-rejected
stout-2 / plaquette-1x1 -> effective-mass-rejected
stout-2 / rectangle-2x1 -> effective-mass-rejected
```

The preserved summary is:

```text
sampler_diagnostics_passed              = true
primary_window_successes                = 0
primary_window_max_z                    = unavailable
operator_stability_z                    = unavailable
transformation_stability_successes      = 0
transformation_stability_max_z          = unavailable
scalar_pipeline_validated               = false
physical_glueball_mass_claimed          = false
```

This is a retained negative scientific result. It shows that **doubling the number of independent retained configurations from 16 to 32, by itself, was not sufficient to recover effective-mass admissibility on the fixed Phase 3F geometry and operator/transformation surface**.

The result does not prove that statistics are irrelevant in general. It rejects only this specific predeclared recovery test.

## Reproducibility receipt

The machine-readable Phase 3G vectors pin the observation head, workflow run, six surface outcomes, zero successful primary windows, zero successful transformation fits, unavailable stability statistics, `sampler_diagnostics_passed=true`, and `scalar_pipeline_validated=false`.

The reference executable also replay-checks that qualitative receipt. A future deterministic replay that drifts from the pinned historical result is an implementation/reproducibility failure. The fact that the pinned receipt itself is scientifically negative remains valid and does not become a CI failure merely because `scalar_pipeline_validated=false`.

## Decision boundary and Phase 3H handoff

The predeclared failure branch applies and Phase 3G is now frozen.

The next authority is separately versioned as `YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1` (Phase 3H). It is predeclared at `6 x 6 x 6 x 10` with 32 fresh independent chains and adds the already-frozen Phase 3B2 stride-2 `block-1` transformation, producing a `3 x 3 x 3 x 10` coarse spatial surface. Phase 3H changes geometry/operator overlap under a fresh stream; it does not modify this Phase 3G authority.

The project must still not open `2++` or `0-+` merely to bypass the unresolved scalar gate.

## Claim boundary

Phase 3G does not establish:

- a universally accepted physical `0++` glueball mass;
- blocking stability;
- a `2++` or `0-+` spectrum;
- finite-volume scaling;
- an infinite-volume limit;
- a continuum extrapolation;
- confinement;
- continuum quantum Yang–Mills existence;
- a positive continuum Yang–Mills mass gap;
- a Clay Millennium Problem solution.

The experiment is a finite-lattice numerical recovery test under a frozen, independently predeclared stream.
