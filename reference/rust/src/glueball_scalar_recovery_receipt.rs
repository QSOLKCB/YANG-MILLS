// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Machine-pinned qualitative receipt for the first Phase 3G observation.
//!
//! A negative scientific gate remains a valid completed experiment. This
//! receipt guard only turns a later deterministic replay that no longer
//! matches the recorded observation into a reproducibility failure.

use crate::glueball_scalar_production::{OperatorKind, ProductionSummary};

pub const OBSERVATION_HEAD: &str = "65dacc9f03cb0fd74826cd211ee7bd586668e08a";
pub const OBSERVATION_WORKFLOW_RUN_ID: u64 = 34_875_522_823;
pub const OBSERVED_REJECTION_CODE: &str = "effective-mass-rejected";

pub const OBSERVED_SURFACE_OUTCOMES: [(&str, OperatorKind, &str); 6] = [
    ("raw", OperatorKind::Plaquette1x1, OBSERVED_REJECTION_CODE),
    ("raw", OperatorKind::Rectangle2x1, OBSERVED_REJECTION_CODE),
    (
        "stout-1",
        OperatorKind::Plaquette1x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-1",
        OperatorKind::Rectangle2x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-2",
        OperatorKind::Plaquette1x1,
        OBSERVED_REJECTION_CODE,
    ),
    (
        "stout-2",
        OperatorKind::Rectangle2x1,
        OBSERVED_REJECTION_CODE,
    ),
];

pub fn observed_phase3g_receipt_matches(summary: &ProductionSummary) -> bool {
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
        && summary.chain_evidence.len() == 32
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
    use crate::glueball_scalar_production::{
        ChainEvidence, CorrelatorEvidence, SurfaceOutcome, SurfaceRecord,
    };

    fn pinned_summary() -> ProductionSummary {
        let chain = ChainEvidence {
            chain_id: 0,
            derived_seed: 1,
            proposals: 1,
            accepts: 1,
            acceptance: 0.5,
            plaquette: 0.5,
            unitarity_residual: 0.0,
            determinant_residual: 0.0,
        };
        let chain_evidence = vec![chain; 32];
        let surfaces = OBSERVED_SURFACE_OUTCOMES
            .iter()
            .map(
                |&(transformation, operator, rejection_code)| SurfaceRecord {
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
                },
            )
            .collect();

        ProductionSummary {
            chain_evidence,
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
        assert!(observed_phase3g_receipt_matches(&summary));
        summary.scalar_pipeline_validated = true;
        assert!(!observed_phase3g_receipt_matches(&summary));
    }

    #[test]
    fn pinned_receipt_requires_all_32_independent_units() {
        let mut summary = pinned_summary();
        summary.chain_evidence.pop();
        assert!(!observed_phase3g_receipt_matches(&summary));
    }
}
