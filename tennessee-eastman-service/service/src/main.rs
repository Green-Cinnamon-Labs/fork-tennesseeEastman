// main.rs

mod config;
mod dashboard;
mod metadata;
mod resolver;
mod runtime;

use config::{Config, ModelKind, IntegratorKind};

fn main() {

    let config = Config {
        dt: 0.1,
        real_time: true,
        initial_state_path: "cases/te_mode1_initial_state.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        active_idv: vec![4],  // IDV(4): reactor cooling water temp step (+5 °C)
    };

    runtime::run(config);
}