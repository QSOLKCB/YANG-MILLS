# Evidence contract

`YM-EVIDENCE-V1` is a Phase 0 draft provenance surface. It describes what later computational evidence must identify before the project treats it as reproducible scientific material.

## Required identity classes

A result record should bind at minimum:

- source repository and exact commit;
- release/contract identifiers;
- gauge group and representation contract;
- lattice dimensions and boundary conditions;
- action/observable definitions;
- algorithm identity;
- all physical/numerical parameters;
- pseudorandom seed/stream identity where applicable;
- toolchain/runtime identity;
- hardware identity where materially relevant;
- precision mode and numerical tolerances;
- raw measurement or content-addressed evidence references;
- analysis version;
- claim classification from `CLAIM_BOUNDARY.md`.

## Determinism versus statistics

A deterministic program can still implement a statistical experiment. Replaying the same pseudorandom seed demonstrates reproducibility of that execution path; it does not turn Monte Carlo samples into exact physical truth.

Future ensemble evidence must distinguish:

```text
same seed reproducibility
independent-chain reproducibility
statistical uncertainty
systematic uncertainty
```

## Hashes

Content hashes may establish identity/integrity of evidence bytes. A SHA-256 receipt is not evidence that the physical interpretation of those bytes is correct.

## Accelerator evidence

Future GPU output must include enough metadata to identify the accelerator execution and must be checked against a stable reference surface according to an explicit acceptance contract.

Passing residual checks means the accelerator agrees with the reference within the stated acceptance domain. It does not establish the continuum theory or mass gap.
