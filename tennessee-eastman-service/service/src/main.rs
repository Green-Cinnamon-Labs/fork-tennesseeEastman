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
        initial_state_path: "cases/te_exp3_snapshot.toml".into(),   // boot from Exp 3 snapshot
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.0,                                        // no cold start (Exp 4)
        active_idv: vec![],                                        // no disturbances
        max_sim_time_h: Some(5.0),                                 // stop at t=5h
        snapshot_path: Some("cases/te_exp4_snapshot.toml".into()), // save final state
    };
                                                             
    runtime::run(config);
}                