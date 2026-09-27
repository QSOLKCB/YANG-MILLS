// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Machine-pinned qualitative receipt for the first Phase 3H observation.
//!
//! Phase 3H completed correctly but its scientific gate remained negative.
//! This guard turns later deterministic drift into a reproducibility failure
//! without converting the retained negative result itself into a CI failure.

use crate::glueball_scalar_geometry_recovery::{
    CHAIN_COUNT, EXPECTED_PROPOSALS_PER_CHAIN, MASTER_SEED,
};
use crate::glueball_scalar_production::{
    ChainEvidence, OperatorKind, ProductionSummary, MAX_ACCEPTANCE, MIN_ACCEPTANCE,
};
use crate::su3_ensemble::{derive_su3_chain_seed, LINK_RESIDUAL_LIMIT};

pub const OBSERVATION_HEAD: &str = "c241991e58860795d247f70d7f92b9e984c96a64";
pub const OBSERVATION_WORKFLOW_RUN_ID: u64 = 34_901_768_744;
pub const OBSERVED_REJECTION_CODE: &str = "effective-mass-rejected";

pub const OBSERVED_SURFACE_OUTCOMES: [(&str, OperatorKind, &str); 8] = [
    ("raw", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    ("raw", OperatorKind::Rectangle2x1, OBSERVED_REJECTION_CODE),
    ("stout-1", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    ("stout-1", OperatorKind::Rectangle2x1, OBSERVED_REJECTION_CODE),
    ("stout-2", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    ("stout-2", OperatorKind::Rectangle2x1, OBSERVED_REJECTION_CODE),
    ("block-1", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    ("block-1", OperatorKind::Rectangle2x1, OBSERVED_REJECTION_CODE),
];

fn retained_chain_identity_matches(evidence: &[ChainEvidence]) -> bool {
    evidence.len() == CHAIN_COUNT
        && evidence.iter().enumerate().all(|(index, chain)| {
            let chain_id = index as u64;
            let expected_acceptance = chain.accepts as f64 / EXPECTED_PROPOSALS_PER_CHAIN as f64;
            chain.chain_id == chain_id
                && chain.derived_seed == derive_su3_chain_seed(MASTER_SEED, chain_id)
                && chain.proposals == EXPECTED_PROPOSALS_PER_CHAIN
                && chain.accepts <= chain.proposals
                && (MIN_ACCEPTANCE..=MAX_ACCEPTANCE).contains(&expected_acceptance)
                && chain.acceptance.to_bits() == expected_acceptance.to_bits()
                && chain.plaquette.is_finite()
                && chain.unitarity_residual.is_finite()
                && chain.determinant_residual.is_finite()
                && chain.unitarity_residual >= 0.0
                && chain.determinant_residual >= 0.0
                && chain.unitarity_residual <= LINK_RESIDUAL_LIMIT
                && chain.determinant_residual <= LINK_RESIDUAL_LIMIT
        })
}

pub fn observed_phase3h_receipt_matches(summary: &ProductionSummary) -> bool {
    let surfaces_match = summary.surfaces.len() == OBSERVED_SURFACE_OUTCOMES.len()
        && summary
            .surfaces
            .iter()
            .zip(OBSERVED_SURFACE_OUTCOMES.iter())
            .all(|(record, expected)| {
                let (transformation, operator, rejection_code) = *expected;
                record.outcome.transformation == transformation
                    && record.outcome.operator == operator
                    && record.outcome.fit.is_none()
                    && record.outcome.error_code == Some(rejection_code)
            });

    summary.sampler_diagnostics_passed
        && retained_chain_identity_matches(&summary.chain_evidence)
        && surfaces_match
        && summary.primary_window_successes == 0
        && summary.primary_window_max_z.is_none()
        && summary.operator_stability_z.is_none()
        && summary.transformation_stability_successes == 0
        && summary.transformation_stability_max_z.is_none()
        && !summary.scalar_pipeline_validated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glueball_scalar_production::{CorrelatorEvidence, SurfaceOutcome, SurfaceRecord};

    fn pinned_chain_evidence() -> Vec<ChainEvidence> {
        (0..CHAIN_COUNT)
            .map(|index| {
                let chain_id = index as u64;
                let accepts = EXPECTED_PROPOSALS_PER_CHAIN / 2 + index as u64;
                ChainEvidence {
                    chain_id,
                    derived_seed: derive_su3_chain_seed(MASTER_SEED, chain_id),
                    proposals: EXPECTED_PROPOSALS_PER_CHAIN,
                    accepts,
                    acceptance: accepts as f64 / EXPECTED_PROPOSALS_PER_CHAIN as f64,
                    plaquette: 0.5,
                    unitarity_residual: 0.0,
                    determinant_residual: 0.0,
                }
            })
            .collect()
    }

    fn pinned_summary() -> ProductionSummary {
        let surfaces = OBSERVED_SURFACE_OUTCOMES
            .iter()
            .map(|&(transformation, operator, rejection_code)| SurfaceRecord {
                outcome: SurfaceOutcome {
                    transformation,
                    operator,
                    fit: None,
                    error_code: Some(rejection_code),
                },
                correlator: CorrelatorEvidence {
                    full: Vec::new(),
                    jackknife: Vec::new(),
                },
            })
            .collect();

        ProductionSummary {
            chain_evidence: pinned_chain_evidence(),
            surfaces,
            sampler_diagnostics_passed: true,
            primary_window_successes: 0,
            primary_window_max_z: None,
            operator_stability_z: None,
            transformation_stability_successes: 0,
            transformation_stability_max_z: None,
            scalar_pipeline_validated: false,
        }
    }

    #[test]
    fn pinned_receipt_accepts_only_the_recorded_negative_result() {
        let mut summary = pinned_summary();
        assert!(observed_phase3h_receipt_matches(&summary));
        summary.scalar_pipeline_validated = true;
        assert!(!observed_phase3h_receipt_matches(&summary));
    }

    #[test]
    fn pinned_receipt_requires_all_eight_rejected_surfaces() {
        let mut summary = pinned_summary();
        summary.surfaces.pop();
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.surfaces[7].outcome.error_code = Some("no-accepted-plateau-window");
        assert!(!observed_phase3h_receipt_matches(&summary));
    }

    #[test]
    fn pinned_receipt_rejects_chain_identity_or_accounting_drift() {
        let mut summary = pinned_summary();
        summary.chain_evidence[1] = summary.chain_evidence[0];
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.chain_evidence[7].derived_seed ^= 1;
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.chain_evidence[12].proposals -= 1;
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.chain_evidence[5].accepts = EXPECTED_PROPOSALS_PER_CHAIN + 1;
        assert!(!observed_phase3h_receipt_matches(&summary));
    }

    #[test]
    fn pinned_receipt_rechecks_finite_statistics_and_group_residuals() {
        let mut summary = pinned_summary();
        summary.chain_evidence[2].plaquette = f64::NAN;
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.chain_evidence[6].unitarity_residual = LINK_RESIDUAL_LIMIT * 2.0;
        assert!(!observed_phase3h_receipt_matches(&summary));

        let mut summary = pinned_summary();
        summary.chain_evidence[10].determinant_residual = -1.0;
        assert!(!observed_phase3h_receipt_matches(&summary));
    }
}
