// runtime.rs

use te_core::plant::Plant;
use te_core::params::Params;

use crate::config::Config;
use crate::metadata::{MEASUREMENTS, MANIPULATED};
use crate::resolver::resolve;

pub fn run(config: Config) {
    let resolved = resolve(&config);

    let params = Params::default();
    let mut plant = Plant::with_state_values(
        &resolved.initial_state,
        resolved.model,
        params,
        resolved.integrator,
    );

    loop {
        plant.step(config.dt);
        plant.bus.time += config.dt;

        for meta in MEASUREMENTS {
            let value = plant.bus.outputs.xmeas[meta.index];
            println!("[{}] {} = {:.4} {}", meta.tag, meta.name, value, meta.unit);
        }

        for meta in MANIPULATED {
            let value = plant.bus.inputs.mv[meta.index];
            println!("[{}] {} = {:.2} {}", meta.tag, meta.name, value, meta.unit);
        }

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
