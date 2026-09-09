# GLUBALL donor boundary

## Recorded donor

Repository: `QSOLKCB/GLUBALL`

Release: `v1.0.0`

Verified tag target at Phase 0 bootstrap:

```text
80941183d14531093117e122da0fc32c13d2464b
```

Licence: Mozilla Public License 2.0.

## Why use GLUBALL as an engineering donor?

GLUBALL developed useful engineering practices for deterministic scientific software:

- explicit versioned contracts;
- stable reference implementations;
- sealed regression vectors;
- canonical evidence/provenance records;
- separation of mathematical authority from accelerator execution;
- residual checking of high-performance sidecars;
- release manifests and claim quarantine.

Those patterns are relevant to YANG-MILLS because lattice calculations can become computationally large while still requiring a small, inspectable reference authority.

## What Phase 0 actually copies

PR #1 independently implements its `SU(2)` mathematics and lattice code. It does **not** copy GLUBALL geometry code into the Yang–Mills reference core.

The donor relationship in Phase 0 is architectural and procedural: contract discipline, evidence boundaries, provenance thinking, and the future reference/accelerator split.

If a future change copies or modifies a GLUBALL source file, that file must retain applicable MPL-2.0 notices and the PR must identify the donor path and source revision.

## Quarantined donor concepts

The following GLUBALL concepts have no Yang–Mills authority merely because they exist in the donor repository:

- `(2,3)` torus-knot geometry;
- `GLUBALL-KNOT-V1`;
- knot or tube topology;
- phi-Weyl sampling;
- ternary/triality metadata;
- sonification;
- RSH geometry or formalisation;
- visual cavity/shape interpretations.

In particular:

```text
GLUBALL name/geometry
    != glueball state
    != gluon field
    != Yang-Mills gauge configuration
```

Any future connection must arrive as an independently motivated hypothesis, with a new contract and a claim classification. It must never enter through donor code by implication.

## Accelerator rule inherited in spirit

Future GPU or multi-device code may accelerate a specified Yang–Mills computation. It may not silently become the authority that defines that computation. A bounded CPU/reference path and independent acceptance surface must exist first.
