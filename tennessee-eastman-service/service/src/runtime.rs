// runtime.rs

use te_core::plant::Plant;
use te_core::params::Params;

use crate::config::Config;
use crate::dashboard::Dashboard;
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

    for &idv in &config.active_idv {
        if idv >= 1 && idv <= plant.bus.inputs.dv.len() {
            plant.bus.inputs.dv[idv - 1] = 1.0;
        }
    }

    let mut dashboard = Dashboard::new().expect("Failed to initialize terminal dashboard");

    loop {
        plant.step(config.dt);
        plant.bus.time += config.dt;

        let snap = plant.snapshot();
        let running = dashboard.render(&snap).expect("Failed to render dashboard");

        if !running {
            break;
        }

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
