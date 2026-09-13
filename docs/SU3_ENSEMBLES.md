# Phase 2C1 — bounded `SU(3)` ensemble reference

Phase 2C1 introduces a separately versioned stochastic surface for pure `SU(3)` Wilson lattice gauge theory. It does not extend `YM-SU2-ENSEMBLE-V1`, and it does not inherit the external `SU(2)` benchmark result from Phase 2B.

The frozen authority is:

```text
YM-SU3-ENSEMBLE-V1
SU3-ENSEMBLE-PHASE2C1-V1
YM-SU3-GENERATOR-METROPOLIS-V1
YM-SU3-STREAM-V1
```

## Why a simple Metropolis reference first

The classic `SU(3)` updating literature includes Cabibbo and Marinari's `SU(2)`-subgroup method and the later Kennedy-Pendleton improved `SU(2)` heatbath used inside `SU(3)` quasi-heatbath calculations:

- N. Cabibbo and E. Marinari, *A new method for updating SU(N) matrices in computer simulations of gauge theories*, Phys. Lett. B **119**, 387–390 (1982), DOI `10.1016/0370-2693(82)90696-7`.
- A. D. Kennedy and B. J. Pendleton, *Improved heatbath method for Monte Carlo calculations in lattice gauge theories*, Phys. Lett. B **156**, 393–399 (1985), DOI `10.1016/0370-2693(85)91632-6`.

Phase 2C1 does **not** claim to implement either algorithm. Instead it freezes a correctness-first symmetric Metropolis kernel over the already-sealed Gell-Mann generator basis. This avoids introducing a second layer of subgroup-factorization and conditional-density conventions before the basic `SU(3)` stochastic evidence path is independently validated.

Metropolis acceptance is the standard symmetric-proposal rule associated with N. Metropolis et al., *Equation of State Calculations by Fast Computing Machines*, J. Chem. Phys. **21**, 1087 (1953), DOI `10.1063/1.1699114`.

## Frozen proposal

For every lattice link `U_mu(x)` and every generator

```text
T^a = lambda_a / 2,  a = 0,...,7,
```

the reference consumes one raw `SplitMix64` `u64` for the angle. Bit 63 selects the sign and the low 52 bits select a magnitude index

```text
m = raw & (2^52 - 1)
|theta| = proposal_angle * (2m + 1) / 2^53.
```

The resulting finite binary64 support is strictly inside `(-proposal_angle,+proposal_angle)`. Toggling only bit 63 while preserving the low 52 bits produces the exact floating inverse angle `-theta`; the ignored raw bits `52..62` give equal multiplicity to every pair. This closes the half-open-endpoint asymmetry that would occur with a direct `2u-1` map from `u in [0,1)`.

The group proposal remains

```text
R = exp(i theta T^a)
U_candidate = R U_old.
```

The bounded binary64 reference requires

```text
1e-8 <= proposal_angle <= 1.
```

Positive values below `1e-8`, including subnormal values such as `f64::MIN_POSITIVE`, are rejected as numerically inert for this reference surface rather than being accepted as nominally nonzero updates. The lower bound is an implementation-resolution guard, not a physical scale.

Because the implemented finite proposal grid has exact equal-probability `theta/-theta` pairs and `R(-theta)=R(theta)^-1`, the symmetric-proposal Metropolis rule is

```text
A = min(1, exp(-DeltaS)).
```

The reference consumes a second PRNG output for the acceptance draw for **every** generator proposal, including downhill or zero-action proposals. The frozen stream therefore consumes exactly two PRNG outputs per proposal.

At `beta=0`, `DeltaS=0` exactly under the Wilson action, so every valid proposal must be accepted. This is an executable transition-kernel sanity check only. A short random walk from the cold lattice is not declared to be an exact Haar draw and no `beta=0` equilibrium expectation is frozen in Phase 2C1.

## Frozen local action difference

A four-dimensional link belongs to six elementary plaquettes. For each `nu != mu`, the local action includes the plaquette based at `x` and the corresponding plaquette based at `x-nu`:

```text
S_local(U_mu(x))
  = beta * sum_(nu != mu) [
      1 - (1/3) ReTr P_mu,nu(x)
    + 1 - (1/3) ReTr P_mu,nu(x-nu)
  ].
```

The local difference is

```text
DeltaS = S_local(candidate) - S_local(old).
```

Candidate evaluation is allocation-free. The implementation substitutes `U_candidate` directly into the forward plaquette as the first factor and into the `x-nu` plaquette as the daggered third factor, evaluating only the six affected plaquettes. It does **not** clone the lattice or allocate a temporary full configuration for every proposal.

A deterministic regression compares this local result against a complete Wilson-action recomputation after a one-link replacement. Local bookkeeping is therefore tested against the already-frozen global action rather than accepted by definition.

## Frozen update order and bounded schedule

One complete Markov sweep is:

1. sites in lexicographic `x0,x1,x2,x3` order;
2. directions `mu=0,1,2,3` at each site;
3. generators `a=0,...,7` at each link.

There are exactly

```text
8 * link_count
```

Metropolis proposals per sweep.

The Phase 2C1 binary64 reference admits at most

```text
512 total Markov sweeps
512 retained measurements
```

with the usual checked schedule

```text
thermalization_sweeps + measurements*sweeps_between_measurements.
```

The old exploratory `262144`-sweep bound is not part of the frozen authority. Review found deterministic determinant drift on a much longer beta-zero trajectory, so the admitted V1 bound was reduced and the **maximum accepted 512-sweep schedule is executed in CI** at `2^4`, `beta=0`, chain `0`, and `proposal_angle=1.0`, requiring the final unitarity and determinant residuals to remain below `5e-12`.

No gauge fixing is performed. Gauge-dependent link state is implementation state; the recorded observable is the normalized gauge-invariant average plaquette.

## Independent stream identity

The PRNG core remains the frozen `SPLITMIX64-V1` numerical utility, but Phase 2C1 defines a new stream domain:

```text
stream_id = YM-SU3-STREAM-V1
domain     = 0x535533454e535631
```

and

```text
derived_seed = mix64(
    master_seed
    XOR domain
    XOR ((chain_id+1) * 0x9e3779b97f4a7c15 mod 2^64)
).
```

For master seed `0x3c00198212230101`, the golden chain seeds are:

```text
chain 0: 94c715630ee8e90a
chain 1: 8aba19af6781dc09
```

The seed is reproducibility metadata, not a physical parameter.

## Measurements, diagnostics, and literal golden chain

The recorded observable is

```text
Pbar = (1/(6V)) sum_x sum_(mu<nu) (1/3) ReTr P_mu,nu(x).
```

For exact `SU(3)` matrices its normalized real trace lies in `[-1/2,1]`; the floating reference permits only the separately declared numerical tolerance around that interval.

Phase 2C1 reuses the already-frozen Phase 2A arithmetic definitions for mean, variance, autocorrelation sequence, positive-window integrated autocorrelation time, ESS, and diagnostic standard error. Reusing those numerical utilities does not make the `SU(3)` sampler part of the `SU(2)` contract.

The canonical beta-zero reference chain is frozen at:

```text
lattice = 2^4
master seed = 3c00198212230101
chain ID = 0
derived seed = 94c715630ee8e90a
thermalisation sweeps = 4
measurements = 16
stride = 1
proposal angle = 0.35
autocorrelation max lag = 4
```

It executes `20` sweeps, `10240` proposals, and `10240` accepts. Its review-hardened literal regression freezes the complete 16-value raw plaquette series plus the mean, variance, lag-0…4 autocorrelation sequence, positive-window lag, `tau_int`, ESS, standard error, and final group residuals with an absolute floating tolerance of `1e-12`. This is intentionally stronger than simply running the same implementation twice and comparing the two runs.

The canonical runtime emits into the commit-tied Actions log:

- sampler/proposal/RNG/stream identities;
- master seed, chain ID, and derived seed;
- lattice, beta, proposal angle, finite-grid identity, and maximum reference sweep bound;
- thermalisation, measurement count, cadence, and autocorrelation lag;
- total proposal/accept accounting and acceptance fraction;
- the complete raw plaquette series;
- mean, variance, full requested autocorrelation sequence, positive-window lag, `tau_int`, ESS, and standard error;
- final maximum link unitarity and determinant-one residuals;
- explicit Phase 2C1 claim-boundary flags.

## What Phase 2C1 validates

The frozen executable surface requires:

- literal identity and golden seed checks;
- exact paired finite proposal-angle support;
- exactly two PRNG outputs consumed per proposal;
- exact beta-zero all-accept accounting;
- local-vs-global Wilson-action difference agreement;
- allocation-free candidate evaluation of the six affected plaquettes;
- an executable `proposal_angle >= 1e-8` resolution floor;
- literal golden reference-chain measurements and diagnostics;
- distinct chain IDs producing distinct evidence;
- maximum admitted schedule group-residual stress;
- bounded/fail-closed configuration validation;
- generated links remaining inside the frozen `SU(3)` group acceptance tolerance.

These are finite-dimensional software and finite-lattice numerical checks. They do not by themselves establish that a finite nonzero-beta chain reproduces a published equilibrium observable.

## Phase 2C2 remains deliberately open

With the sampler mechanics frozen, Phase 2C2 may now select a peer-reviewed pure-`SU(3)` Wilson-action benchmark, predeclare its volume/coupling/observable/statistical gates, and run independent chains against it. The benchmark target and gates must be selected before its result is inspected.

Until that gate is passed:

```text
published_su3_baseline_reproduced=false
glueball_analysis_enabled=false
phase2c_complete=false
```

Agreement with a future finite-lattice benchmark will still not imply confinement, continuum quantum Yang-Mills existence, or a positive continuum mass gap.
