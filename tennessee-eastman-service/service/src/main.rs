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
        initial_state_path: "cases/te_mode1_initial_state.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.5,                                        // 0.5 h cold start ramp
        active_idv: vec![],                                        // no disturbances (Exp 3)
        max_sim_time_h: Some(20.0),                                // stop at t=20h
        snapshot_path: Some("cases/te_exp3_snapshot.toml".into()), // save final state
    };
                                                             
    runtime::run(config);
}                