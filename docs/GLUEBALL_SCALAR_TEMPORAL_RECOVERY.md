# Phase 3I — scalar temporal-extent recovery

Phase 3I follows the frozen Phase 3H blocking-capable geometry recovery. Phase 3H completed correctly on source head `c241991e58860795d247f70d7f92b9e984c96a64` in workflow `34901768744`: all 32 sampler chains passed diagnostics, but all eight raw/stout/block scalar surfaces were `effective-mass-rejected`, so `scalar_pipeline_validated=false`.

Phase 3I does **not** retune Phase 3H.

## Scientific question

The predeclared hypothesis is:

> Does increasing only the Euclidean temporal extent from 10 to 16, while preserving the Phase 3H spatial geometry, blocking surface, chain count, operator basis, fit windows, fit mechanics, and stability thresholds, improve scalar effective-mass admissibility?

This is intentionally a one-variable intervention. It tests temporal extent / finite-periodic wraparound as a possible contributor before introducing a new variational operator-basis analysis.

## What changes from Phase 3H

```text
fine geometry    : 6 x 6 x 6 x 10 -> 6 x 6 x 6 x 16
blocked geometry : 3 x 3 x 3 x 10 -> 3 x 3 x 3 x 16
master stream    : 0x3f02202609150001 -> 0x3f03202609270001
```

## What remains fixed

```text
beta                         = 5.7
independent chains           = 32
cold start                   = true
thermalization sweeps        = 64
post-thermalization sweeps   = 4
total sweeps / chain         = 68
proposal angle               = 0.35
retained configurations      = 1 per independent chain
operators                    = plaquette-1x1, rectangle-2x1
transformations              = raw, stout-1, stout-2, block-1
stout rho                    = 0.10
stout levels                 = 1, 2
candidate windows            = [1,3], [2,4], [1,4]
primary surface              = block-1 / plaquette-1x1
maximum pairwise mass z      = 5
correlator replay tolerance  = 2e-12
```

The exact proposal workload is:

```text
fine sites                   = 3456
positive-direction links     = 13824
proposals / link / sweep     = 8
proposals / chain            = 7,520,256
proposals across 32 chains   = 240,648,192
```

## Scientific gate

The Phase 3H gate is retained unchanged. The scalar pipeline validates only if sampler diagnostics pass, the primary blocked fit succeeds, at least two primary windows succeed and are stable at `z <= 5`, blocked plaquette/rectangle stability is scorable and passes, at least two plaquette transformations succeed, and all successful transformation pairs are scorable and stable at `z <= 5`.

A data-admissibility rejection remains a scientific result. Accounting drift, non-finite arithmetic, invalid SU(3) links, broken blocking geometry, correlator replay mismatch, or impossible fit state remains an implementation failure.

## Decision boundary

If Phase 3I passes, freeze it and run a fresh-stream independent confirmation of the same `6 x 6 x 6 x 16` surface before opening `2++` or `0-+`.

If Phase 3I fails, freeze the negative result and predeclare a **variational scalar operator-basis** authority. Do not retune Phase 3I windows, thresholds, chain count, or stream after observation.

## Claim boundary

Phase 3I is finite-lattice numerical evidence only. It does not establish a physical continuum `0++` glueball mass, confinement, continuum quantum Yang-Mills construction, a positive continuum mass gap, or a Clay solution.
