# Phase 2B — published SU(2) plaquette baseline

## Purpose

Phase 2B tests the frozen Phase 2A `SU(2)` sampler against a peer-reviewed external finite-lattice observable. It is a validation of the finite-lattice Markov-chain implementation in one declared regime. It is not a confinement result, a continuum construction, or a mass-gap theorem.

`YM-SU2-PLAQUETTE-BASELINE-V1` and `SU2-PLAQUETTE-BASELINE-PHASE2B-V1` are frozen as `phase-2b-frozen` after the external comparison, independent-chain diagnostics, and explicit post-thermalisation stationarity checks all passed the complete reference/contract surface.

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

The comparison additionally requires:

```text
minimum ESS per chain: 6
maximum chain-consistency z: 5
stationarity split: first 24 vs last 24 post-thermalisation measurements
stationarity autocorrelation max lag per half: 6
maximum stationarity drift z per chain: 5
maximum final link-unit residual: 1e-12
```

These are bounded reference-validation settings, not production-precision physics settings.

## Comparison and equilibration rules

For chain means `m_0,m_1` and Phase 2A diagnostic standard errors `s_0,s_1`, the reference summary records

```text
pooled_mean = (m_0 + m_1)/2
pooled_standard_error = sqrt(s_0^2+s_1^2)/2
chain_consistency_z = |m_0-m_1| / sqrt(s_0^2+s_1^2)
```

Independent-chain agreement is necessary but not sufficient because both reference chains start from the same cold identity configuration. To catch a shared residual cold-start transient, each 48-measurement post-thermalisation chain is also divided into its first 24 and last 24 measurements. Each half is analyzed with the frozen Phase 2A autocorrelation-aware diagnostics using `max_lag=6`, and the benchmark computes

```text
stationarity_z = |mean_first - mean_second|
                 / sqrt(SE_first^2 + SE_second^2).
```

Each chain must satisfy `stationarity_z <= 5`. This is a bounded within-chain stationarity diagnostic, not a proof of perfect equilibration or the absence of every systematic effect.

The Phase 2 graduation gate passes only if:

1. the pooled mean lies inside the predeclared published source-resolution envelope `[0.59,0.62]`;
2. both chains meet the ESS floor;
3. the independent-chain consistency diagnostic meets its declared bound;
4. both within-chain post-thermalisation stationarity diagnostics meet their declared bound;
5. final `SU(2)` link-unit residuals remain within tolerance;
6. heat-bath acceptance statistics remain finite and positive.

## Final hardened result

The external benchmark target, source envelope, volume, coupling, observable, seeds, and chain schedule were not changed after inspection. Review later identified that two cold-start chains could share a residual transient, so the additional stationarity gate above was added without moving those external or run targets.

The exact hardened run produced:

```text
chain 0 mean             = 0.6026867250625984
chain 1 mean             = 0.6018972107756385
pooled mean              = 0.6022919679191184
pooled diagnostic SE     = 0.0004173286094490
chain-consistency z      = 0.9459144054
minimum chain ESS        = 7.7458561747
maximum link-unit resid. = 6.6613381478e-16

chain 0 first-half mean  = 0.6037349175777675
chain 0 second-half mean = 0.6016385325474291
chain 0 stationarity z   = 2.3962050056

chain 1 first-half mean  = 0.6021866298531725
chain 1 second-half mean = 0.6016077916981041
chain 1 stationarity z   = 1.1383971897

maximum stationarity z   = 2.3962050056
```

The pooled mean lies inside the predeclared `[0.59,0.62]` source-resolution envelope, the independent chains satisfy their ESS/consistency gates, and both post-thermalisation drift statistics lie below the frozen maximum of 5.

The frozen Phase 2B authority therefore records

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
- first-half/second-half stationarity means and drift-z values for both chains;
- pooled comparison statistics and maximum final link-unit residual;
- explicit graduation and claim-boundary flags.

The runtime log is a commit/toolchain-bound evidence record. A scientific release should additionally archive those raw measurements as an explicit artifact or content-addressed dataset rather than relying only on CI retention.

## Evidence classification

The peer-reviewed source identity, benchmark definition, and source-resolution interpretation are C0 provenance/contract facts. The measured chain means, stationarity diagnostics, and benchmark comparison are C2 finite-lattice numerical observations.

Agreement validates the frozen `SU(2)` sampler in this declared regime. It does **not** imply:

```text
confinement
continuum quantum Yang-Mills existence
positive continuum mass gap
```

`SU(3)` ensemble generation remains disabled under this contract. A future `SU(3)` sampler requires its own separately versioned algorithm and review gate.
