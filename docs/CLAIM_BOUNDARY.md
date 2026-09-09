# Claim boundary

Every significant result in YANG-MILLS should be classified before publication.

## C0 — software / contract result

Examples:

- a parser accepts only canonical inputs;
- CPU and GPU implementations agree within a tolerance;
- a test vector is reproduced;
- a build is deterministic under a stated environment.

These are facts about the implementation or evidence pipeline.

## C1 — finite-lattice exact result

Examples:

- the identity configuration has zero Wilson action;
- the Wilson action is invariant under local gauge transformations in exact algebra;
- a specific analytic finite-lattice configuration has a known action.

A theorem at this level concerns the discrete model only.

## C2 — finite-lattice numerical observation

Examples:

- an ensemble-average plaquette at specified `(beta, L)`;
- a measured correlator;
- a fitted finite-volume glueball mass.

Must include parameters, uncertainty/tolerance, algorithm identity, and provenance.

## C3 — physical / scaling inference

Examples:

- consistency with a published pure-gauge spectrum;
- an observed finite-volume trend;
- a continuum-extrapolation estimate under an explicitly stated scaling model.

Must state model assumptions and systematic uncertainty. Still not a continuum theorem.

## C4 — continuum mathematical theorem

A rigorously proved result about a continuum construction or limit, with hypotheses and proof artefacts sufficient for independent checking.

## C5 — Yang–Mills Millennium Problem solution claim

Reserved for a genuine proof satisfying the Clay problem statement, including existence of the required non-trivial continuum quantum Yang–Mills theory and a strictly positive mass gap.

## Forbidden equivalences

The following implications are invalid unless independently proved:

```text
positive finite-lattice mass
    => positive continuum mass gap

stable fit
    => theorem

continuum-looking extrapolation
    => existence of continuum QFT

CPU == GPU
    => physical truth

agreement with literature
    => proof

formal finite-lattice theorem
    => Clay solution
```

## Required release language

A numerical release must explicitly say that its numerical results are evidence/observations within a specified lattice model and are not a proof of the Yang–Mills existence and mass-gap problem.
