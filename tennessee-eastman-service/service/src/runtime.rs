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

    let mut dashboard = Dashboard::new().expect("Failed to initialize terminal dashboard");

    loop {
        plant.step(config.dt);
        plant.bus.time += config.dt;

        let running = dashboard
            .render(
                plant.bus.time,
                &plant.bus.outputs.xmeas,
                &plant.bus.inputs.mv,
            )
            .expect("Failed to render dashboard");

        if !running {
            break;
        }

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
