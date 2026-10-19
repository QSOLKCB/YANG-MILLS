// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};
#[path = "../reference/rust/src/sampling_efficiency.rs"]
mod sampling_efficiency;
#[path = "../reference/rust/src/su2_ensemble.rs"]
mod su2_ensemble;
use sampling_efficiency::{evaluate, EfficiencyError, SamplingCost, MAX_SAMPLES};
const VALUES: [f64; 4] = [0.0, 1.0, 0.0, 1.0];
fn cost() -> SamplingCost {
    SamplingCost {
        setup: 4.0,
        thermalization: 8.0,
        sampling: 20.0,
    }
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12 * b.abs().max(1.0), "{a} != {b}");
}
#[test]
fn unit_weights_replay_frozen_autocorrelation_and_costs() {
    let r = evaluate(&VALUES, None, 2, cost()).unwrap();
    assert_eq!(
        r.diagnostics,
        su2_ensemble::diagnostics(&VALUES, 2).unwrap()
    );
    close(r.diagnostics.integrated_autocorrelation_time, 0.5);
    close(r.reweighting_efficiency, 1.0);
    close(r.combined_ess_proxy, 4.0);
    close(r.sampling_cost_per_effective_sample_proxy, 5.0);
    close(r.total_cost_per_effective_sample_proxy, 8.0);
}
#[test]
fn weights_have_analytic_kish_efficiency_and_shift_invariance() {
    let logs = [0.0, 0.0, 0.0, 3.0_f64.ln()];
    let r = evaluate(&VALUES, Some(&logs), 2, cost()).unwrap();
    close(r.reweighting_efficiency, 0.75); // 6^2 / (4*(1+1+1+9))
    close(r.weight_only_ess, 3.0);
    close(r.combined_ess_proxy, 3.0);
    close(r.total_cost_per_effective_sample_proxy, 32.0 / 3.0);
    for shift in [-1000.0, 1000.0] {
        let shifted = logs.map(|v| v + shift);
        close(
            evaluate(&VALUES, Some(&shifted), 2, cost())
                .unwrap()
                .reweighting_efficiency,
            0.75,
        );
    }
    let dominant = evaluate(
        &VALUES,
        Some(&[1000.0, -1000.0, -1000.0, -1000.0]),
        2,
        cost(),
    )
    .unwrap();
    close(dominant.weight_only_ess, 1.0);
    close(dominant.total_cost_per_effective_sample_proxy, 32.0);
}
#[test]
fn serial_correlation_and_weights_are_distinct_penalties() {
    let values = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let r = evaluate(&values, None, 3, cost()).unwrap();
    close(r.diagnostics.integrated_autocorrelation_time, 1.1);
    close(r.diagnostics.effective_sample_size, 6.0 / 2.2);
    close(r.reweighting_efficiency, 1.0);
    close(r.total_cost_per_effective_sample_proxy, 32.0 * 2.2 / 6.0);
}
#[test]
fn rejects_bad_values_weights_lengths_lags_costs_and_degenerate_series() {
    for values in [
        &[][..],
        &[1.0][..],
        &[f64::NAN, 1.0][..],
        &[f64::INFINITY, 1.0][..],
    ] {
        assert!(evaluate(values, None, 1, cost()).is_err());
    }
    assert!(evaluate(&vec![1.0; MAX_SAMPLES + 1], None, 1, cost()).is_err());
    assert_eq!(
        evaluate(&[1.0; 4], None, 1, cost()).unwrap_err(),
        EfficiencyError::DegenerateObservable
    );
    for lag in [0, 4, usize::MAX] {
        assert!(evaluate(&VALUES, None, lag, cost()).is_err());
    }
    for logs in [
        &[0.0][..],
        &[0.0, 0.0, 0.0, f64::NAN],
        &[0.0, 0.0, 0.0, f64::INFINITY],
        &[f64::NEG_INFINITY; 4],
    ] {
        assert!(evaluate(&VALUES, Some(logs), 1, cost()).is_err());
    }
    for invalid in [-1.0, f64::NAN, f64::INFINITY] {
        for c in [
            SamplingCost {
                setup: invalid,
                ..cost()
            },
            SamplingCost {
                thermalization: invalid,
                ..cost()
            },
            SamplingCost {
                sampling: invalid,
                ..cost()
            },
        ] {
            assert!(evaluate(&VALUES, None, 1, c).is_err());
        }
    }
    assert!(evaluate(
        &VALUES,
        None,
        1,
        SamplingCost {
            sampling: 0.0,
            ..cost()
        }
    )
    .is_err());
    assert!(evaluate(
        &VALUES,
        None,
        1,
        SamplingCost {
            setup: f64::MAX,
            thermalization: f64::MAX,
            sampling: 1.0
        }
    )
    .is_err());
}
#[test]
fn finite_overflow_and_underflow_fail_closed() {
    assert!(evaluate(&[f64::MAX, -f64::MAX], None, 1, cost()).is_err());
    assert!(evaluate(&[0.0, 1e-300], None, 1, cost()).is_err());
    assert!(evaluate(
        &VALUES,
        None,
        1,
        SamplingCost {
            setup: 0.0,
            thermalization: 0.0,
            sampling: f64::from_bits(1)
        }
    )
    .is_err());
}

#[test]
fn identical_non_binary_exact_values_are_not_evidence_of_mixing() {
    for n in [3, 100, MAX_SAMPLES] {
        assert_eq!(
            evaluate(&vec![0.1; n], None, 1, cost()).unwrap_err(),
            EfficiencyError::DegenerateObservable
        );
    }
}

#[test]
fn positive_subnormal_variance_cannot_report_zero_uncertainty() {
    assert_eq!(
        evaluate(&[0.0, 4e-162, 0.0, 4e-162], None, 1, cost()).unwrap_err(),
        EfficiencyError::NonFiniteResult
    );
}
