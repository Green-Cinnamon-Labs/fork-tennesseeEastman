// main.rs

mod config;
mod dashboard;
mod resolver;
mod runtime;

use config::{Config, ModelKind, IntegratorKind};

fn main() {

    let config = Config {
        dt: 0.001,
        real_time: false,
        step_delay_secs: 0.0,
        initial_state_path: "cases/te_exp3_snapshot.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.0,
        active_idv: vec![1],                                       // IDV(1): A/C feed ratio step
        max_sim_time_h: Some(20.0),
        snapshot_path: Some("cases/te_exp12_snapshot.toml".into()),
    };
                                                             
    runtime::run(config);
}                