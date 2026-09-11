# Phase 2A — SU(2) statistical ensemble generation

## Scope

Phase 2A is the first stochastic layer in YANG-MILLS. It introduces a bounded, reproducible Markov-chain reference implementation for the frozen four-dimensional periodic `SU(2)` Wilson action.

It does **not** yet graduate all of Phase 2. The repository's Phase 2 graduation gate additionally requires reproduction of a selected published plaquette benchmark in a matching action, volume, and parameter regime. That comparison is deliberately left false in `YM-SU2-ENSEMBLE-V1` until it has actually been performed and reviewed.

`SU(3)` ensemble generation is likewise disabled until the `SU(2)` sampler and diagnostics are trustworthy.

## Literature basis

The implementation is independently derived under the repository's frozen quaternion/staple conventions, with the update strategy grounded in standard lattice-gauge Monte Carlo literature:

- Michael Creutz, *Monte Carlo study of quantized SU(2) gauge theory*, **Phys. Rev. D 21, 2308–2315 (1980)**, DOI `10.1103/PhysRevD.21.2308`.
- Frank R. Brown and Thomas J. Woch, *Overrelaxed heat-bath and Metropolis algorithms for accelerating pure gauge Monte Carlo calculations*, **Phys. Rev. Lett. 58, 2394–2396 (1987)**, DOI `10.1103/PhysRevLett.58.2394`.

These papers motivate the algorithm family. The machine-readable contract, not an implicit appeal to the literature, is the authority for the exact implementation conventions in this repository.

## Frozen local Wilson weight

For one link `U=U_mu(x)`, the six plaquettes containing that link can be written under the frozen Phase 1A orientation as

```text
exp[ beta * (1/2) ReTr(U V) ],
```

up to factors independent of `U`, where `V` is the sum of the six oriented staple terms returned by the frozen `Lattice4::staple()` implementation.

For nonzero staple norm,

```text
V = k Vhat,
k = sqrt(v0^2+v1^2+v2^2+v3^2),
Vhat in SU(2).
```

Define

```text
R = U Vhat.
```

Then the local conditional measure is

```text
dP(R) proportional to exp(alpha r0) d_mu_Haar(R),
alpha = beta k,
```

where `r0=(1/2)ReTr(R)`.

## Bounded heat-bath draw

Writing a unit quaternion as

```text
R = (r0, sqrt(1-r0^2) n_hat),
```

with `n_hat` uniform on the unit two-sphere, Haar measure gives

```text
p(r0) proportional to sqrt(1-r0^2) exp(alpha r0),
-1 <= r0 <= 1.
```

For ordinary positive `alpha`, the Phase 2A reference sampler:

1. proposes `r0` from the truncated exponential density proportional to `exp(alpha r0)` on `[-1,1]`;
2. accepts that proposal with probability `sqrt(1-r0^2)`;
3. samples the vector direction uniformly on `S^2`;
4. reconstructs `U_new=R Vhat^dagger`.

### Tiny-alpha numerical branch

The direct inverse CDF contains a division by `alpha`. For subnormal positive `alpha`, evaluating that expression in binary64 collapses the proposal onto a tiny set of values even though the target distribution is indistinguishable from its Haar limit at machine resolution.

The frozen numerical reference therefore uses

```text
ALPHA_ZERO_LIMIT = 1e-300
```

and, for

```text
0 <= alpha <= ALPHA_ZERO_LIMIT,
```

uses the exact `alpha -> 0` proposal limit

```text
r0 = 2u - 1.
```

The usual Haar-Jacobian rejection and uniform `S^2` direction step still follow. This branch is explicitly part of the **floating reference algorithm**; it is not a change to the mathematical conditional density. A regression with `beta=f64::from_bits(1)` requires well-resolved link scalar values instead of the subnormal inverse-CDF collapse found during review.

When the staple norm is exactly zero, the local Wilson weight is independent of the link and the accepted Haar draw `R` is used directly.

The rejection loop is bounded by `max_heatbath_attempts_per_link`. Exhausting that bound is an error, not an invitation to silently bias the distribution.

## Over-relaxation

For nonzero staple norm Phase 2A implements

```text
U_new = Vhat^dagger U_old^dagger Vhat^dagger.
```

Under `R=U Vhat`, this sends `R -> R^dagger`, preserving `r0=(1/2)ReTr(R)` and the local Wilson action. A zero staple produces a no-op.

Over-relaxation is only a decorrelation step. It is **not** an ergodic sampler by itself. Each Phase 2A Markov sweep begins with one full heat-bath sweep and may then perform a configured number of full over-relaxation sweeps.

## Update order

Reference updates are deterministic conditional on the pseudorandom stream:

```text
site order:      x0, x1, x2, x3 lexicographic
direction order: mu = 0,1,2,3 at each site
```

Changing update order is a semantic sampler change and must not be hidden behind the existing contract identifier.

## Pseudorandom stream identity

Phase 2A freezes `SPLITMIX64-V1` as the **reference reproducibility stream**, not as a cryptographic generator and not as a physical object.

The state update is

```text
state <- state + 0x9e3779b97f4a7c15  (mod 2^64)
```

followed by the frozen SplitMix64 mixing function. Uniform binary64 values use the high 53 bits multiplied by `2^-53`.

The first five `u64` outputs from seed zero are sealed as:

```text
e220a8397b1dcdaf
6e789e6aa1b965f4
06c45d188009454f
f88bb8a8724c81ec
1b39896a51a8749b
```

A chain is identified by both `master_seed` and `chain_id`. The derived seed is versioned explicitly. Two chain IDs are different stream identities even when every lattice/action parameter is the same.

The repository does not claim cross-platform bitwise identity for floating-point transcendental functions used in Haar-direction sampling. The integer RNG stream itself is exactly versioned.

## Thermalisation and measurement cadence

The Phase 2A reference chain begins from the cold identity lattice.

The complete schedule records lattice/action parameters, master seed, chain ID/derived seed, discarded thermalisation sweeps, measurement count, sweeps between measurements, over-relaxation cadence, rejection bound, and autocorrelation maximum lag.

The reference bound applies to the **checked total** number of complete Markov sweeps:

```text
total = thermalization_sweeps
      + measurements * sweeps_between_measurements

total <= 2^20.
```

Both multiplication and addition are checked for overflow before a chain is admitted. Bounding only the two schedule pieces separately is not sufficient.

A recorded measurement is the finite-lattice average plaquette

```text
Pbar = (1/(6 V)) sum_x sum_(mu<nu) (1/2) ReTr P_mu_nu(x).
```

## Autocorrelation and effective sample size

Phase 2A uses a deliberately simple, versioned diagnostic rather than an unspecified library default.

For measurements `x_i`, it computes the arithmetic mean and population variance. The lag covariance uses divisor `N-lag`, and

```text
rho_lag = C_lag / C_0.
```

The integrated autocorrelation estimate is

```text
tau_int = 1/2 + sum rho_lag
```

where the sum includes consecutive positive `rho_lag` values starting at lag one and stops adding after the first non-positive lag. Later requested lags are still reported but not added to the window.

The effective sample size is

```text
ESS = min(N, N/(2 tau_int)),
```

and the diagnostic standard error is `sqrt(variance / ESS)`.

These diagnostics are finite-sample estimators. A small measured autocorrelation does not make a short chain representative of all physics and does not remove the need for independent chains, thermalisation checks, volume studies, or benchmark comparison.

## Machine-readable fail-closed boundary

The Phase 2A contract checker performs **exact recursive validation of the complete nested contract and vector structures**. Chain identity, schedule, reproducibility, heat-bath, over-relaxation, diagnostic, graduation, and claim-boundary fields are authority surfaces; merely retaining their top-level object names is not sufficient.

## Beta-zero sanity surface

At `beta=0`, the Wilson Boltzmann weight is constant and every link has Haar conditional distribution. This gives exact ensemble expectation

```text
< (1/2) ReTr P > = 0.
```

Phase 2A uses a fixed-seed `2^4` chain as a **sanity band**, not as a precision physics result. The reference executable checks a near-zero measured average, nontrivial ESS, rejection acceptance, and unit-link residuals.

## Published-baseline boundary

The full Phase 2 roadmap requires reproduction of a published plaquette expectation in a declared parameter regime. Phase 2A intentionally leaves

```text
published_plaquette_baseline_reproduced = false
phase2_graduation_gate_satisfied = false
su3_ensemble_enabled = false
```

until a benchmark is selected whose action, normalization, lattice geometry/volume, observable definition, and statistical treatment can be matched honestly.

Agreement with a literature number will be useful validation of the sampler. It will **not** prove confinement, construct continuum quantum Yang–Mills theory, or prove a positive mass gap.

## Claim boundary

Phase 2A measurements are `C2` finite-lattice numerical observations. Pseudorandom seeds are provenance/stream identity. Autocorrelation diagnostics quantify properties of a recorded chain. None of these is a continuum theorem.
