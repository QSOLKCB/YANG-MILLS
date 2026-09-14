# Phase 3G — fresh-stream scalar recovery confirmation

Phase 3G follows the merged and preserved negative Phase 3F production observation. It does **not** retune `YM-GLUEBALL-SCALAR-PRODUCTION-V1`.

The new candidate authority is:

```text
YM-GLUEBALL-SCALAR-RECOVERY-V1
status = phase-3g-candidate
```

Its purpose is deliberately narrow: test whether the Phase 3F effective-mass rejection was dominated by insufficient **independent configuration count** before changing the lattice geometry, operator basis, fit windows, transformation parameters, or scientific thresholds.

## Frozen Phase 3F baseline

Phase 3F used sixteen independently seeded `4 x 4 x 4 x 10` chains at `beta=5.7`. Sampler diagnostics passed, but all six scalar surfaces were rejected by the frozen effective-mass authority and `scalar_pipeline_validated=false`.

That result is frozen historical evidence. Phase 3G may compare against it; Phase 3G may not rewrite it.

## Predeclared recovery hypothesis

The hypothesis is:

> Doubling independently seeded retained configurations from 16 to 32 at fixed geometry and frozen analysis semantics may reduce scalar-correlator noise enough for the existing effective-mass and stability gates to become admissible.

This is a hypothesis, not an expected result.

Phase 3G changes only:

```text
master stream: fresh
independent chains: 16 -> 32
```

It deliberately keeps fixed:

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

## Why the geometry is not changed yet

The Phase 3F spatial extent `4` is too small for the frozen stride-2 blocking authority, which requires a fine spatial extent of at least `6`.

Phase 3G nevertheless keeps spatial extent `4` because this experiment is intended to isolate one variable: independent statistics.

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

## Decision boundary

Phase 3G is intentionally a one-question recovery experiment.

If the gate **passes**, the next phase should not immediately open `2++` or `0-+`. It should first predeclare a larger-spatial-volume scalar confirmation whose geometry admits the frozen Phase 3B2 blocking transformation.

If the gate **fails**, the next phase should not simply increase the chain count again under this identifier. It should freeze the failure and predeclare a new recovery experiment that changes geometry and/or operator overlap while stating the new hypothesis before observing data.

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

The experiment is a finite-lattice numerical recovery test under a new, independently predeclared stream.
