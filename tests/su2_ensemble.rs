// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../reference/rust/src/su2_ensemble.rs"]
mod su2_ensemble;

use su2_ensemble::{
    average_plaquette, derive_chain_seed, diagnostics, heatbath_sweep, overrelaxation_sweep,
    run_chain, EnsembleConfig, EnsembleError, SplitMix64,
};

fn beta_zero_config(chain_id: u64) -> EnsembleConfig {
    EnsembleConfig {
        extents: [2, 2, 2, 2],
        beta: 0.0,
        master_seed: 0x0123_4567_89ab_cdef,
        chain_id,
        thermalization_sweeps: 1,
        measurements: 64,
        sweeps_between_measurements: 1,
        overrelaxation_sweeps_per_heatbath: 1,
        max_heatbath_attempts_per_link: 10_000,
        autocorrelation_max_lag: 8,
    }
}

#[test]
fn splitmix64_stream_is_exactly_versioned() {
    let mut rng = SplitMix64::new(0);
    let expected = [
        0xe220_a839_7b1d_cdafu64,
        0x6e78_9e6a_a1b9_65f4,
        0x06c4_5d18_8009_454f,
        0xf88b_b8a8_724c_81ec,
        0x1b39_896a_51a8_749b,
    ];
    for value in expected {
        assert_eq!(rng.next_u64(), value);
    }
}

#[test]
fn chain_seed_identity_is_explicit_and_distinct() {
    let a = derive_chain_seed(7, 0);
    let b = derive_chain_seed(7, 1);
    let replay = derive_chain_seed(7, 0);
    assert_eq!(a, replay);
    assert_ne!(a, b);
}

#[test]
fn fixed_chain_replays_exactly() {
    let config = beta_zero_config(3);
    let first = run_chain(config).unwrap();
    let second = run_chain(config).unwrap();
    assert_eq!(first.derived_seed, second.derived_seed);
    assert_eq!(first.measurements, second.measurements);
    assert_eq!(first.diagnostics, second.diagnostics);
}

#[test]
fn distinct_chain_ids_produce_distinct_streams() {
    let first = run_chain(beta_zero_config(3)).unwrap();
    let second = run_chain(beta_zero_config(4)).unwrap();
    assert_ne!(first.derived_seed, second.derived_seed);
    assert_ne!(first.measurements, second.measurements);
}

#[test]
fn beta_zero_chain_matches_haar_plaquette_sanity_band() {
    let report = run_chain(beta_zero_config(0)).unwrap();
    assert!(report.diagnostics.mean.abs() <= 0.12);
    assert!(report.diagnostics.effective_sample_size >= 8.0);
    assert!((0.45..=1.0).contains(&report.mean_heatbath_acceptance));
    assert!(report.final_link_unit_residual <= 1.0e-12);
}

#[test]
fn heatbath_sweep_preserves_su2_link_invariant() {
    let mut lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let mut rng = SplitMix64::new(1234);
    let (_, updates) = heatbath_sweep(&mut lattice, 2.3, &mut rng, 100_000).unwrap();
    assert_eq!(updates as usize, lattice.link_count());
    assert!(lattice.max_link_unit_residual() <= 1.0e-12);
    assert!(average_plaquette(&lattice).unwrap().is_finite());
}

#[test]
fn overrelaxation_is_microcanonical_within_tolerance() {
    let beta = 2.0;
    let mut lattice = Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let mut rng = SplitMix64::new(0xfeed_face_cafe_beef);
    heatbath_sweep(&mut lattice, beta, &mut rng, 100_000).unwrap();
    let before = lattice.wilson_action(beta).unwrap();
    overrelaxation_sweep(&mut lattice).unwrap();
    let after = lattice.wilson_action(beta).unwrap();
    assert!((after - before).abs() <= 2.0e-10);
    assert!(lattice.max_link_unit_residual() <= 1.0e-12);
}

#[test]
fn diagnostics_use_positive_autocorrelation_window() {
    let independent_like = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
    let result = diagnostics(&independent_like, 3).unwrap();
    assert_eq!(result.autocorrelation[0], 1.0);
    assert_eq!(result.positive_window_lag, 0);
    assert_eq!(result.integrated_autocorrelation_time, 0.5);
    assert_eq!(result.effective_sample_size, independent_like.len() as f64);
}

#[test]
fn invalid_sampler_inputs_fail_closed() {
    let mut config = beta_zero_config(0);
    config.beta = f64::NAN;
    assert_eq!(
        config.validate().unwrap_err(),
        EnsembleError::BetaOutsideReferenceRange
    );

    let mut config = beta_zero_config(0);
    config.measurements = 1;
    assert_eq!(config.validate().unwrap_err(), EnsembleError::TooFewMeasurements);

    let mut config = beta_zero_config(0);
    config.sweeps_between_measurements = 0;
    assert_eq!(
        config.validate().unwrap_err(),
        EnsembleError::ZeroMeasurementStride
    );

    let mut config = beta_zero_config(0);
    config.max_heatbath_attempts_per_link = 0;
    assert_eq!(
        config.validate().unwrap_err(),
        EnsembleError::ZeroHeatbathAttemptLimit
    );

    let mut config = beta_zero_config(0);
    config.autocorrelation_max_lag = config.measurements;
    assert_eq!(
        config.validate().unwrap_err(),
        EnsembleError::AutocorrelationLagOutOfRange
    );
}
