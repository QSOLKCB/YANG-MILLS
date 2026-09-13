#!/usr/bin/env python3
"""One-shot idempotent patch for PR #10 review hardening; delete after application."""

from pathlib import Path


def replace_once(path_str: str, old: str, new: str) -> None:
    path = Path(path_str)
    text = path.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"expected review-fix block not found in {path_str}")
    path.write_text(text.replace(old, new, 1))


# Shared acceptance evidence tolerance and regression.
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    "pub const PLAQUETTE_SAMPLE_TOLERANCE: f64 = 1.0e-12;\n",
    "pub const PLAQUETTE_SAMPLE_TOLERANCE: f64 = 1.0e-12;\npub const ACCEPTANCE_EVIDENCE_TOLERANCE: f64 = 1.0e-12;\n",
)
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    "fn acceptance_from_counts(report: &Su3ChainReport) -> Result<f64, BenchmarkError> {\n",
    "pub(crate) fn acceptance_from_counts(report: &Su3ChainReport) -> Result<f64, BenchmarkError> {\n",
)
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    "        || report.mean_metropolis_acceptance != acceptance\n",
    "        || (report.mean_metropolis_acceptance - acceptance).abs()\n            > ACCEPTANCE_EVIDENCE_TOLERANCE\n",
)
replace_once(
    "reference/rust/src/su3_benchmark.rs",
    "        let mut broken = first.clone();\n        broken.metropolis_accepts = EXPECTED_PROPOSALS;\n",
    "        let mut rounded = first.clone();\n        rounded.mean_metropolis_acceptance += ACCEPTANCE_EVIDENCE_TOLERANCE * 0.5;\n        assert!(summarize_two_chains(&rounded, &second).is_ok());\n\n        let mut broken = first.clone();\n        broken.mean_metropolis_acceptance += ACCEPTANCE_EVIDENCE_TOLERANCE * 2.0;\n        assert_eq!(\n            summarize_two_chains(&broken, &second),\n            Err(BenchmarkError::InvalidAcceptanceEvidence)\n        );\n\n        let mut broken = first.clone();\n        broken.metropolis_accepts = EXPECTED_PROPOSALS;\n",
)

# Reuse the same tolerance-aware acceptance verifier in V2.
replace_once(
    "reference/rust/src/su3_confirmation.rs",
    "use crate::su3_benchmark::{\n    stationarity_diagnostic, BenchmarkError, StationarityDiagnostic, AUTOCORRELATION_MAX_LAG, BETA,\n",
    "use crate::su3_benchmark::{\n    acceptance_from_counts, stationarity_diagnostic, BenchmarkError, StationarityDiagnostic,\n    AUTOCORRELATION_MAX_LAG, BETA,\n",
)
replace_once(
    "reference/rust/src/su3_confirmation.rs",
    '''fn acceptance_from_counts(report: &Su3ChainReport) -> Result<f64, BenchmarkError> {
    if report.metropolis_proposals != EXPECTED_PROPOSALS
        || report.metropolis_accepts == 0
        || report.metropolis_accepts > report.metropolis_proposals
    {
        return Err(BenchmarkError::InvalidMetropolisAccounting);
    }
    let acceptance = report.metropolis_accepts as f64 / report.metropolis_proposals as f64;
    if !acceptance.is_finite()
        || report.mean_metropolis_acceptance != acceptance
        || !(MIN_METROPOLIS_ACCEPTANCE..=MAX_METROPOLIS_ACCEPTANCE).contains(&acceptance)
    {
        return Err(BenchmarkError::InvalidAcceptanceEvidence);
    }
    Ok(acceptance)
}

''',
    "",
)

# V1 archive replay must reject any newly introduced group-residual failure.
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    "    MASTER_SEED, MAX_CHAIN_CONSISTENCY_Z, MAX_METROPOLIS_ACCEPTANCE, MAX_STATIONARITY_Z,\n    MEASUREMENTS, MIN_CHAIN_ESS, MIN_METROPOLIS_ACCEPTANCE, PRIMARY_DOI, PROPOSAL_ANGLE,\n",
    "    MASTER_SEED, MAX_CHAIN_CONSISTENCY_Z, MAX_LINK_DETERMINANT_RESIDUAL,\n    MAX_LINK_UNITARITY_RESIDUAL, MAX_METROPOLIS_ACCEPTANCE, MAX_STATIONARITY_Z, MEASUREMENTS,\n    MIN_CHAIN_ESS, MIN_METROPOLIS_ACCEPTANCE, PRIMARY_DOI, PROPOSAL_ANGLE,\n",
)
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    "        && summary.chain_consistency_z <= MAX_CHAIN_CONSISTENCY_Z\n        && summary.max_stationarity_z <= MAX_STATIONARITY_Z;\n",
    "        && summary.chain_consistency_z <= MAX_CHAIN_CONSISTENCY_Z\n        && summary.max_stationarity_z <= MAX_STATIONARITY_Z\n        && summary.max_link_unitarity_residual <= MAX_LINK_UNITARITY_RESIDUAL\n        && summary.max_link_determinant_residual <= MAX_LINK_DETERMINANT_RESIDUAL;\n",
)

# Emit the complete V1 diagnostic evidence promised by AGENTS.md.
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    '''    println!("chain0_mean={:.17e}", summary.chain_means[0]);
    println!("chain1_mean={:.17e}", summary.chain_means[1]);
    println!(
        "chain0_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[0]
    );
    println!(
        "chain1_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[1]
    );
''',
    '''    println!("chain0_mean={:.17e}", summary.chain_means[0]);
    println!("chain1_mean={:.17e}", summary.chain_means[1]);
    println!("chain0_variance={:.17e}", first.diagnostics.variance);
    println!("chain1_variance={:.17e}", second.diagnostics.variance);
    println!(
        "chain0_autocorrelation={}",
        series_text(&first.diagnostics.autocorrelation)
    );
    println!(
        "chain1_autocorrelation={}",
        series_text(&second.diagnostics.autocorrelation)
    );
    println!(
        "chain0_positive_window_lag={}",
        first.diagnostics.positive_window_lag
    );
    println!(
        "chain1_positive_window_lag={}",
        second.diagnostics.positive_window_lag
    );
    println!(
        "chain0_tau_int={:.17e}",
        first.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "chain1_tau_int={:.17e}",
        second.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "chain0_standard_error={:.17e}",
        summary.chain_standard_errors[0]
    );
    println!(
        "chain1_standard_error={:.17e}",
        summary.chain_standard_errors[1]
    );
    println!(
        "chain0_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[0]
    );
    println!(
        "chain1_effective_sample_size={:.17e}",
        summary.chain_effective_sample_sizes[1]
    );
    println!(
        "chain0_stationarity_first_half_mean={:.17e}",
        summary.stationarity[0].first_half_mean
    );
    println!(
        "chain0_stationarity_second_half_mean={:.17e}",
        summary.stationarity[0].second_half_mean
    );
    println!(
        "chain0_stationarity_first_half_standard_error={:.17e}",
        summary.stationarity[0].first_half_standard_error
    );
    println!(
        "chain0_stationarity_second_half_standard_error={:.17e}",
        summary.stationarity[0].second_half_standard_error
    );
    println!(
        "chain0_stationarity_z={:.17e}",
        summary.stationarity[0].drift_z
    );
    println!(
        "chain1_stationarity_first_half_mean={:.17e}",
        summary.stationarity[1].first_half_mean
    );
    println!(
        "chain1_stationarity_second_half_mean={:.17e}",
        summary.stationarity[1].second_half_mean
    );
    println!(
        "chain1_stationarity_first_half_standard_error={:.17e}",
        summary.stationarity[1].first_half_standard_error
    );
    println!(
        "chain1_stationarity_second_half_standard_error={:.17e}",
        summary.stationarity[1].second_half_standard_error
    );
    println!(
        "chain1_stationarity_z={:.17e}",
        summary.stationarity[1].drift_z
    );
''',
)
replace_once(
    "reference/rust/src/bin/su3_benchmark_reference.rs",
    "    println!(\"pooled_mean={:.17e}\", summary.pooled_mean);\n",
    "    println!(\"max_link_unitarity_residual={:.17e}\", summary.max_link_unitarity_residual);\n    println!(\"max_link_determinant_residual={:.17e}\", summary.max_link_determinant_residual);\n    println!(\"pooled_mean={:.17e}\", summary.pooled_mean);\n",
)

# Emit the same complete diagnostic evidence for every V2 chain.
replace_once(
    "reference/rust/src/bin/su3_confirmation_reference.rs",
    '''    println!("chain{index}_mean={:.17e}", summary.chain_means[index]);
    println!(
        "chain{index}_standard_error={:.17e}",
        summary.chain_standard_errors[index]
    );
''',
    '''    println!("chain{index}_mean={:.17e}", summary.chain_means[index]);
    println!("chain{index}_variance={:.17e}", report.diagnostics.variance);
    println!(
        "chain{index}_autocorrelation={}",
        series_text(&report.diagnostics.autocorrelation)
    );
    println!(
        "chain{index}_positive_window_lag={}",
        report.diagnostics.positive_window_lag
    );
    println!(
        "chain{index}_standard_error={:.17e}",
        summary.chain_standard_errors[index]
    );
''',
)
replace_once(
    "reference/rust/src/bin/su3_confirmation_reference.rs",
    '''    println!(
        "chain{index}_stationarity_z={:.17e}",
        summary.stationarity[index].drift_z
    );
''',
    '''    println!(
        "chain{index}_stationarity_first_half_standard_error={:.17e}",
        summary.stationarity[index].first_half_standard_error
    );
    println!(
        "chain{index}_stationarity_second_half_standard_error={:.17e}",
        summary.stationarity[index].second_half_standard_error
    );
    println!(
        "chain{index}_stationarity_z={:.17e}",
        summary.stationarity[index].drift_z
    );
''',
)

# State the post-V1 ESS semantic change explicitly instead of calling every diagnostic threshold unchanged.
replace_once(
    "docs/PUBLISHED_SU3_PLAQUETTE_BASELINE.md",
    "After V1 failed only its statistical-power gate, V2 was committed **before any V2 result was inspected**. It retained the source surface, source envelope, sampler, proposal scale, per-chain schedule, and diagnostic thresholds while introducing a fresh master seed and four independent streams.\n",
    "After V1 failed only its statistical-power gate, V2 was committed **before any V2 result was inspected**. It retained the source surface, source envelope, sampler, proposal scale, per-chain schedule, acceptance window, stationarity threshold, pairwise-consistency threshold, and group-residual limits while introducing a fresh master seed and four independent streams. The ESS rule was deliberately versioned: V1 required ESS >= 8 for every chain, while V2 requires each chain ESS to remain finite and positive plus aggregate ESS >= 16 across four fresh chains.\n",
)
replace_once(
    "ROADMAP.md",
    "Phase 2C1 does **not** authorize later `SU(3)` physics analysis by itself. Phase 2C2 preserves the failed first attempt rather than rewriting it: `YM-SU3-PLAQUETTE-BASELINE-V1` reproduced the published plaquette envelope but failed its predeclared per-chain ESS floor (`4.510 < 8`). A separately predeclared fresh-stream `YM-SU3-PLAQUETTE-BASELINE-V2` then retained the same source surface, sampler, proposal scale, per-chain schedule, and diagnostic thresholds while using four fresh independent chains and an aggregate ESS floor of 16.\n",
    "Phase 2C1 does **not** authorize later `SU(3)` physics analysis by itself. Phase 2C2 preserves the failed first attempt rather than rewriting it: `YM-SU3-PLAQUETTE-BASELINE-V1` reproduced the published plaquette envelope but failed its predeclared per-chain ESS floor (`4.510 < 8`). A separately predeclared fresh-stream `YM-SU3-PLAQUETTE-BASELINE-V2` then retained the same source surface, sampler, proposal scale, per-chain schedule, acceptance/stationarity/consistency thresholds, and group-residual limits while using four fresh independent chains. V2 explicitly changed the ESS criterion from a per-chain floor of 8 to finite-positive ESS per chain plus an aggregate ESS floor of 16.\n",
)

print("PR #10 review hardening patch applied")
