// main.rs

mod config;
mod dashboard;
mod metadata;
mod resolver;
mod runtime;

use config::{Config, ModelKind, IntegratorKind};

fn main() {

    let config = Config {
        dt: 0.001,
        real_time: true,
        step_delay_secs: 2.0,
        initial_state_path: "cases/te_mode1_initial_state.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 2.0,   // 2 h simulated — feed valves ramp 0% → nominal
        active_idv: vec![4],  // IDV(4): reactor cooling water temp step (+5 °C)
    };
                                                             
    runtime::run(config);
}                