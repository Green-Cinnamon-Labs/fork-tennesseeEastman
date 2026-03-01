// main.rs

mod config;
mod metadata;
mod resolver;
mod runtime;

use config::{Config, ModelKind};

fn main() {
    let config = Config {
        dt: 0.1,
        real_time: true,
        initial_state_path: "cases/te_mode1_initial_state.toml".into(),
        model: ModelKind::TennesseeEastman,
    };

    runtime::run(config);
}