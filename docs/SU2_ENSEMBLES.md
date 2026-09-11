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

Because every `SU(2)` matrix in the repository is represented by a real quaternion, the staple sum has the same quaternion matrix form. For nonzero staple norm,

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

where `r0=(1/2)ReTr(R)` and `d_mu_Haar` is Haar measure on `SU(2)`.

## Exact bounded heat-bath draw

Writing a unit quaternion as

```text
R = (r0, sqrt(1-r0^2) n_hat),
```

with `n_hat` uniform on the unit two-sphere, Haar measure gives the scalar marginal

```text
p(r0) proportional to sqrt(1-r0^2) exp(alpha r0),
-1 <= r0 <= 1.
```

The Phase 2A reference sampler therefore:

1. proposes `r0` from the exact truncated exponential density proportional to `exp(alpha r0)` on `[-1,1]`;
2. accepts that proposal with probability `sqrt(1-r0^2)`;
3. samples the vector direction uniformly on `S^2`;
4. reconstructs

```text
U_new = R Vhat^dagger.
```

When the staple norm is exactly zero, the local Wilson weight is independent of the link and the accepted Haar draw `R` is used directly.

The rejection loop is bounded by `max_heatbath_attempts_per_link`. Exhausting that bound is an error, not an invitation to silently bias the distribution.

## Over-relaxation

For nonzero staple norm Phase 2A also implements the microcanonical reflection

```text
U_new = Vhat^dagger U_old^dagger Vhat^dagger.
```

Under the frozen local variable `R=U Vhat`, this sends

```text
R -> R^dagger,
```

which preserves `r0=(1/2)ReTr(R)` and therefore preserves the local Wilson action. A zero staple produces a no-op.

Over-relaxation is used only as a decorrelation step. It is **not** described as an ergodic sampler by itself. Each Phase 2A Markov sweep begins with a full heat-bath sweep and may then perform a configured number of full over-relaxation sweeps.

## Update order

Reference updates are deterministic conditional on the pseudorandom stream:

```text
site order:      x0, x1, x2, x3 lexicographic
direction order: mu = 0,1,2,3 at each site
```

Changing update order is a semantic sampler change and must not be hidden behind the existing contract identifier.

## Pseudorandom stream identity

Phase 2A freezes `SPLITMIX64-V1` as the **reference reproducibility stream**, not as a cryptographic generator and not as a physical object.

The stream state update is

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

A chain is identified by both `master_seed` and `chain_id`. The derived seed is versioned explicitly. Two chain IDs are different stream identities even when every physical/lattice parameter is the same.

The repository does not claim cross-platform bitwise identity for floating-point transcendental functions used in Haar-direction sampling. The integer RNG stream itself is exactly versioned.

## Thermalisation and measurement cadence

The Phase 2A reference chain begins from the cold identity lattice.

The complete schedule records:

- lattice extents;
- `beta`;
- master seed;
- chain ID and derived seed;
- number of discarded thermalisation sweeps;
- number of measurements;
- full Markov sweeps between measurements;
- number of over-relaxation sweeps following each heat-bath sweep;
- heat-bath rejection attempt bound;
- autocorrelation maximum lag.

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

where the sum includes consecutive positive `rho_lag` values starting at lag one and stops adding after the first non-positive lag. Later requested lags are still reported but are not added to the window.

The effective sample size is

```text
ESS = min(N, N/(2 tau_int)),
```

and the diagnostic standard error is

```text
sqrt(variance / ESS).
```

These diagnostics are finite-sample estimators. A small measured autocorrelation does not magically make a short chain representative of all physics, and it does not remove the need for independent chains, thermalisation checks, volume studies, or later benchmark comparison.

## Beta-zero sanity surface

At `beta=0`, the Wilson Boltzmann weight is constant and every link has Haar conditional distribution. This gives the exact ensemble expectation

```text
< (1/2) ReTr P > = 0
```

for a plaquette.

Phase 2A uses a fixed-seed `2^4` chain as a **sanity band**, not as a precision physics result. The reference executable requires the measured average plaquette to stay near zero, checks nontrivial ESS, checks the rejection-sampler acceptance rate, and verifies that all links remain unit `SU(2)` elements.

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
