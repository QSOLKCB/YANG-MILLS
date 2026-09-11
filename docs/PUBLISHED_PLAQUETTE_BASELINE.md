# Phase 2B — published SU(2) plaquette baseline

## Purpose

Phase 2B tests the frozen Phase 2A `SU(2)` sampler against a peer-reviewed external finite-lattice observable. It is a validation of the finite-lattice Markov-chain implementation in one declared regime. It is not a confinement result, a continuum construction, or a mass-gap theorem.

`YM-SU2-PLAQUETTE-BASELINE-V1` and `SU2-PLAQUETTE-BASELINE-PHASE2B-V1` are frozen as `phase-2b-frozen` after the predeclared benchmark passed the full reference/contract surface.

## Primary source

The primary peer-reviewed authority is:

> Kurt Langfeld, Biagio Lucini, Antonio Rago, *Density of States in Gauge Theories*, Physical Review Letters **109**, 111601 (2012), DOI `10.1103/PhysRevLett.109.111601`.

The paper studies four-dimensional `SU(2)` Wilson gauge theory on `N^4` lattices. Figure 5 presents the average plaquette on a `10^4` lattice and compares density-of-states results with an independent local-hybrid Monte Carlo calculation.

The figure is the relevant published comparison surface, but it does not provide a convenient table of point estimates and statistical errors. The repository therefore does **not** invent a paper uncertainty. Before running the Phase 2B benchmark, it froze a deliberately conservative graphical source-resolution envelope at

```text
lattice: 10^4
beta:    2.3
observable: (1/(6V)) sum_x sum_(mu<nu) (1/2) ReTr P_mu_nu(x)
source-resolution envelope: [0.59, 0.62]
```

The interval is an acceptance envelope for reproducing the plotted published curve at the resolution needed for this validation step. It must never be described as a quoted error bar from the paper.

## Matching the repository convention

The source uses the standard Wilson plaquette action. Under frozen `YM-SU2-LATTICE-V1`, the repository uses

```text
S_W = beta * sum_p (1 - (1/2) ReTr P_p).
```

The additive constant does not change the Boltzmann distribution, so the coupling and normalized plaquette observable match the standard Wilson convention used for the comparison.

The Phase 2B observable is exactly the Phase 2A average plaquette:

```text
Pbar = (1/(6V)) sum_x sum_(mu<nu) (1/2) ReTr P_mu_nu(x).
```

No rescaling or fitted conversion is permitted after seeing the benchmark output.

## Frozen reference run

Before the benchmark result was inspected, Phase 2B froze two independent chain identities:

```text
master_seed: 2b00201211160101
chain_ids: 0, 1
lattice: 10^4
beta: 2.3
thermalization_sweeps: 40
measurements_per_chain: 48
sweeps_between_measurements: 1
overrelaxation_sweeps_per_heatbath: 1
max_heatbath_attempts_per_link: 10000
autocorrelation_max_lag: 12
```

The comparison additionally required:

```text
minimum ESS per chain: 6
maximum chain-consistency z: 5
maximum final link-unit residual: 1e-12
```

These are bounded reference-validation settings, not production-precision physics settings.

## Comparison rule

For chain means `m_0,m_1` and Phase 2A diagnostic standard errors `s_0,s_1`, the reference summary records

```text
pooled_mean = (m_0 + m_1)/2
pooled_standard_error = sqrt(s_0^2+s_1^2)/2
chain_consistency_z = |m_0-m_1| / sqrt(s_0^2+s_1^2)
```

The Phase 2 graduation candidate passes only if:

1. the pooled mean lies inside the predeclared published source-resolution envelope `[0.59,0.62]`;
2. both chains meet the ESS floor;
3. the independent-chain consistency diagnostic meets its declared bound;
4. final `SU(2)` link-unit residuals remain within tolerance;
5. heat-bath acceptance statistics remain finite and positive.

## First predeclared result

The first run using the frozen benchmark settings passed without changing the source envelope, volume, coupling, observable, seeds, or comparison thresholds after inspection.

```text
chain 0 mean             = 0.6026867250625984
chain 1 mean             = 0.6018972107756385
pooled mean              = 0.6022919679191184
pooled diagnostic SE     = 0.0004173286094490
chain-consistency z      = 0.9459144054
minimum chain ESS        = 7.7458561747
maximum link-unit resid. = 6.6613381478e-16
```

Therefore the pooled mean lies inside the predeclared `[0.59,0.62]` source-resolution envelope and the independent-chain diagnostic gates pass.

The frozen Phase 2B authority records

```text
published_plaquette_baseline_reproduced = true
phase2_graduation_gate_satisfied       = true
su3_ensemble_enabled                   = false
```

## Evidence archive

The canonical `ym-su2-plaquette-baseline` executable emits the complete finite-lattice evidence needed to audit the frozen reference run into the commit-tied GitHub Actions log:

- contract/source identity and DOI;
- lattice, coupling, and predeclared source envelope;
- master seed, chain IDs, and derived seeds;
- full raw measurement series for both chains;
- per-chain means, standard errors, ESS, integrated autocorrelation times, and heat-bath acceptance;
- pooled comparison statistics and maximum final link-unit residual;
- explicit graduation and claim-boundary flags.

The runtime log is a commit/toolchain-bound evidence record. A scientific release should additionally archive those raw measurements as an explicit artifact or content-addressed dataset rather than relying only on CI retention.

## Evidence classification

The peer-reviewed source identity, benchmark definition, and source-resolution interpretation are C0 provenance/contract facts. The measured chain means and benchmark comparison are C2 finite-lattice numerical observations.

Agreement validates the frozen `SU(2)` sampler in this declared regime. It does **not** imply:

```text
confinement
continuum quantum Yang-Mills existence
positive continuum mass gap
```

`SU(3)` ensemble generation remains disabled under this contract. A future `SU(3)` sampler requires its own separately versioned algorithm and review gate.
