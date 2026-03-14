// runtime.rs

use std::fs::File;
use std::io::{BufWriter, Write};

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

    // ── Cold start ─────────────────────────────────────────────────────────────
    let nominal_mv: Vec<f64> = plant.bus.inputs.mv.clone();
    let ramp_duration = config.ramp_duration;

    // Disable disturbances during ramp
    let saved_dv = plant.bus.inputs.dv.clone();
    for v in plant.bus.inputs.dv.iter_mut() {
        *v = 0.0;
    }

    // Close feed valves at t=0
    for i in 0..4 {
        plant.bus.inputs.mv[i] = 0.0;
    }

    let mut disturbances_restored = false;

    // ── CSV logger ─────────────────────────────────────────────────────────────
    let csv_file = File::create("simulation_log.csv").expect("Failed to create CSV file");
    let mut csv = BufWriter::new(csv_file);

    // Header
    let mut header = String::from("t_h");
    for i in 1..=22 { header.push_str(&format!(",XMEAS({})", i)); }
    for i in 1..=12 { header.push_str(&format!(",XMV({})", i)); }
    header.push_str(",deriv_norm");
    writeln!(csv, "{}", header).unwrap();

    let mut dashboard = Dashboard::new().expect("Failed to initialize terminal dashboard");

    let mut t_simulation  = 0.0_f64;
    let mut t_operational = 0.0_f64;
    let mut isd_active    = false;

    loop {
        if !isd_active {
            plant.step(config.dt);
            t_simulation   += config.dt;
            plant.bus.time += config.dt;

            // ── Cold start ramp ────────────────────────────────────────────────
            let progress = (t_simulation / ramp_duration).clamp(0.0, 1.0);
            for i in 0..4 {
                plant.bus.inputs.mv[i] = nominal_mv[i] * progress;
            }

            // Restore disturbances once ramp completes
            if progress >= 1.0 && !disturbances_restored {
                plant.bus.inputs.dv = saved_dv.clone();
                disturbances_restored = true;
            }

            // ── Stabilizing controllers (always active) ────────────────────────
            let reactor_p = plant.bus.outputs.xmeas[6];
            plant.bus.inputs.mv[5] =
                (40.06 + 0.10 * (reactor_p - 2705.0)).clamp(0.0, 100.0);

            let sep_level = plant.bus.outputs.xmeas[11];
            plant.bus.inputs.mv[6] =
                (38.1 + 1.0 * (sep_level - 50.0)).clamp(0.0, 100.0);

            let strip_level = plant.bus.outputs.xmeas[14];
            plant.bus.inputs.mv[7] =
                (46.5 + 1.0 * (strip_level - 50.0)).clamp(0.0, 100.0);

            let snap = plant.snapshot();

            // ── CSV row ────────────────────────────────────────────────────────
            let mut row = format!("{:.6}", t_simulation);
            for i in 0..22 { row.push_str(&format!(",{:.6}", snap.xmeas[i])); }
            for i in 0..12 { row.push_str(&format!(",{:.6}", snap.xmv[i])); }
            row.push_str(&format!(",{:.6e}", snap.solver.deriv_norm));
            writeln!(csv, "{}", row).unwrap();

            // Advance t_operational only while no alarms are active.
            let any_alarm = snap.alarms.iter().any(|a| a.active);
            if !any_alarm {
                t_operational += config.dt;
            }

            // Shutdown detection
            if snap.solver.deriv_norm == 0.0 && any_alarm {
                isd_active = true;
                eprintln!("SIMULATION STOPPED: plant shutdown condition reached");
                eprintln!("  t_simulation  = {:.3} h", t_simulation);
                eprintln!("  t_operational = {:.3} h", t_operational);
                let _ = csv.flush();
            }

            let running = dashboard.render(&snap).expect("Failed to render dashboard");
            if !running { break; }
        } else {
            let snap = plant.snapshot();
            let running = dashboard.render(&snap).expect("Failed to render dashboard");
            if !running { break; }

            if config.real_time {
                std::thread::sleep(std::time::Duration::from_secs_f64(config.step_delay_secs));
            }
        }

        if !isd_active && config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.step_delay_secs));
        }
    }

    let _ = csv.flush();
}
