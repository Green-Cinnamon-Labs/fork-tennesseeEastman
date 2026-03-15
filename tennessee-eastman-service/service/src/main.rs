// main.rs

mod config;
mod dashboard;
mod resolver;
mod runtime;

use config::{Config, ModelKind, IntegratorKind};

fn main() {

    let config = Config {
        dt: 0.001,                                                 // back to 0.001 (Exp 6)
        real_time: false,
        step_delay_secs: 0.0,
        initial_state_path: "cases/te_exp3_snapshot.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.0,
        active_idv: vec![],
        max_sim_time_h: Some(5.0),
        snapshot_path: Some("cases/te_exp6_snapshot.toml".into()),
    };
                                                             
    runtime::run(config);
}                