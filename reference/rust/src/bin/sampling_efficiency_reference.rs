// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};
#[path = "../sampling_efficiency.rs"]
mod sampling_efficiency;
#[path = "../su2_ensemble.rs"]
mod su2_ensemble;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../su3_lattice.rs"]
mod su3_lattice;
use sampling_efficiency::{evaluate, SamplingCost, CONTRACT_ID};
use su3_ensemble::{run_chain, Su3EnsembleConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Exactly the existing Phase 2C1 mechanics chain. No new production run.
    let config = Su3EnsembleConfig {
        extents: [2; 4],
        beta: 0.0,
        master_seed: 0x3c00_1982_1223_0101,
        chain_id: 0,
        thermalization_sweeps: 4,
        measurements: 16,
        sweeps_between_measurements: 1,
        proposal_angle: 0.35,
        autocorrelation_max_lag: 4,
    };
    let chain = run_chain(config)?;
    let proposals_per_sweep = config.extents.iter().product::<usize>() * 4 * 8;
    let cost = SamplingCost {
        setup: 0.0,
        thermalization: (proposals_per_sweep * config.thermalization_sweeps) as f64,
        sampling: (proposals_per_sweep * config.measurements * config.sweeps_between_measurements)
            as f64,
    };
    let report = evaluate(
        &chain.measurements,
        None,
        config.autocorrelation_max_lag,
        cost,
    )?;
    if report.total_cost != chain.metropolis_proposals as f64 {
        return Err("proposal cost does not replay sampler accounting".into());
    }
    println!("contract={CONTRACT_ID}");
    println!("evidence_class=mechanics-regression-not-equilibrium-validation");
    println!("observable=average-plaquette-not-topological-charge");
    println!("sampler_id={}", chain.sampler_id);
    println!("stream_id={}", chain.stream_id);
    println!("config={config:?}");
    println!("derived_seed={:016x}", chain.derived_seed);
    println!("measurements={:?}", chain.measurements);
    println!("autocorrelation={:?}", report.diagnostics.autocorrelation);
    println!(
        "positive_window_lag={}",
        report.diagnostics.positive_window_lag
    );
    println!(
        "autocorrelation_window_truncated={}",
        report.diagnostics.positive_window_lag == config.autocorrelation_max_lag
    );
    println!(
        "tau_int={:.17e}",
        report.diagnostics.integrated_autocorrelation_time
    );
    println!(
        "autocorrelation_ess={:.17e}",
        report.diagnostics.effective_sample_size
    );
    println!("reweighting=unit-weights-no-flow-implemented");
    println!(
        "reweighting_efficiency={:.17e}",
        report.reweighting_efficiency
    );
    println!("weight_only_ess={:.17e}", report.weight_only_ess);
    println!("combined_ess_proxy={:.17e}", report.combined_ess_proxy);
    println!("cost_unit=generator-proposals-not-wall-clock-seconds");
    println!("cost_scope=sampler-proposals-only-excludes-analysis-and-io");
    println!("cost={cost:?}");
    println!("total_cost={:.17e}", report.total_cost);
    println!(
        "sampling_cost_per_effective_sample_proxy={:.17e}",
        report.sampling_cost_per_effective_sample_proxy
    );
    println!(
        "total_cost_per_effective_sample_proxy={:.17e}",
        report.total_cost_per_effective_sample_proxy
    );
    println!("topological_freezing_resolved=false");
    println!("mass_gap_proved=false");
    Ok(())
}
