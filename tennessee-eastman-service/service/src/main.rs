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
        ramp_duration: 0.5,   // 0.5 h simulated — feed valves ramp 0% → nominal
        active_idv: vec![4],  // IDV(4): reactor cooling water temp step (+5 °C)
    };
                                                             
    runtime::run(config);
}                