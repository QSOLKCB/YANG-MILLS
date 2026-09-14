# Phase 3H — blocking-capable scalar geometry recovery

Phase 3H follows the merged Phase 3G fresh-stream statistics recovery, which preserved a second negative scalar result. Phase 3H does **not** retune Phase 3F or Phase 3G.

## Scientific question

The predeclared hypothesis is:

> Does moving to the minimum spatial geometry admitted by the already-frozen stride-2 blocking authority, while keeping the Phase 3G independent-chain count and the frozen scalar-analysis semantics, improve scalar operator overlap enough for the effective-mass and stability gates to become admissible?

This is a composite geometry/operator-overlap intervention. If it succeeds, the run does not by itself identify whether the improvement came from larger spatial extent, blocking, or their interaction. A successful result therefore requires a fresh-stream confirmation before causal attribution or channel expansion.

## What changes from Phase 3G

```text
geometry      : 4 x 4 x 4 x 10 -> 6 x 6 x 6 x 10
master stream : 0x3f01202609150001 -> 0x3f02202609150001
transform set : raw, stout-1, stout-2
                -> raw, stout-1, stout-2, block-1
primary       : stout-2 / plaquette-1x1
                -> block-1 / plaquette-1x1
```

The stride-2 blocked geometry is therefore:

```text
3 x 3 x 3 x 10
```

The blocking transformation is exactly `YM-GLUEBALL-SPATIAL-BLOCK-V1`; Phase 3H does not change its stride, link composition rule, or geometry contract.

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
stout rho                    = 0.10
stout levels                 = 1, 2
candidate windows            = [1,3], [2,4], [1,4]
maximum pairwise mass z      = 5
correlator replay tolerance  = 2e-12
```

The larger geometry implies exact proposal accounting of:

```text
sites                        = 2160
positive-direction links     = 8640
proposals / link / sweep     = 8
proposals / chain            = 4,700,160
proposals across 32 chains   = 150,405,120
```

## Analysis surface

Eight scalar surfaces are evaluated:

```text
raw     / plaquette-1x1
raw     / rectangle-2x1
stout-1 / plaquette-1x1
stout-1 / rectangle-2x1
stout-2 / plaquette-1x1
stout-2 / rectangle-2x1
block-1 / plaquette-1x1
block-1 / rectangle-2x1
```

The primary surface is predeclared as:

```text
block-1 / plaquette-1x1
```

Full and delete-one correlators retain the frozen Phase 3C vacuum-subtraction semantics. The complete correlator reconstructed from operator samples is replay-checked against the frozen Phase 3C implementation.

## Scientific gate

The scalar pipeline validates only if all of the following hold:

- sampler diagnostics pass;
- the blocked primary fit succeeds;
- at least two individual primary candidate windows succeed;
- the primary-window maximum pairwise z is at most 5;
- blocked plaquette/rectangle operator stability is scorable and at most 5;
- at least two plaquette transformation fits succeed across raw/stout/block surfaces;
- every successful transformation pair is scorable;
- the maximum transformation pairwise z is at most 5.

Data-admissibility rejection is a scientific result. Allocation failure, non-finite arithmetic, sampler/accounting drift, invalid SU(3) links, blocking failure, broken correlator replay, or impossible fit states are implementation failures.

## Scientific-integrity invariants

Phase 3H explicitly adopts [`SCIENTIFIC_INTEGRITY_INVARIANTS.md`](SCIENTIFIC_INTEGRITY_INVARIANTS.md):

- no outcome by construction;
- preserve failed gates;
- no post-observation threshold/seed/window retuning;
- every claimed invariant receives adversarial regression coverage;
- regulator/geometry dependence remains explicit;
- success of one mechanism is not treated as proof of necessity;
- finite-lattice evidence remains below physical/theorem claim classes.

## Decision boundary

If Phase 3H passes, freeze it and run a fresh-stream independent confirmation of the **same** blocking-capable geometry before opening `2++` or `0-+`.

If Phase 3H fails, freeze the negative result and predeclare a new scalar authority that changes time extent and/or the variational operator basis. Do not retune this contract and do not return to repeated chain-count escalation until it passes.

## Claim boundary

Phase 3H is finite-lattice numerical evidence only. It does not establish:

- a physical continuum `0++` glueball mass;
- a finite-volume mass gap theorem;
- confinement;
- existence of continuum four-dimensional quantum Yang-Mills theory;
- a positive continuum Yang-Mills mass gap;
- a Clay Millennium Prize solution.
