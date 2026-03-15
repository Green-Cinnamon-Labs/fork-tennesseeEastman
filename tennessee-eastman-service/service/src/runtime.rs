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
    // Internal ODE states: UCVR YY[0..10] (reactor vapor + energy) and UCVV YY[27..35] (compressor vapor + energy)
    for i in 0..10  { header.push_str(&format!(",YY[{}]", i)); }
    for i in 27..35 { header.push_str(&format!(",YY[{}]", i)); }
    writeln!(csv, "{}", header).unwrap();

    let mut dashboard = Dashboard::new().expect("Failed to initialize terminal dashboard");

    let mut t_simulation  = 0.0_f64;
    let mut t_operational = 0.0_f64;
    let mut isd_active    = false;
    let mut last_state: Option<Vec<f64>> = None;
    let mut clean_exit    = false;

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
                (40.06 + 0.10 * (reactor_p - 2705.0)).clamp(0.0, 100.0); // setpoint reverted 2680→2705 (Exp 9)

            let sep_level = plant.bus.outputs.xmeas[11];
            plant.bus.inputs.mv[6] =
                (38.1 + 1.0 * (sep_level - 50.0)).clamp(0.0, 100.0);

            let strip_level = plant.bus.outputs.xmeas[14];
            plant.bus.inputs.mv[7] =
                (46.5 + 1.0 * (strip_level - 50.0)).clamp(0.0, 100.0);

            // ── 4th controller: reactor level → A feed (Exp 9) ────────────────
            // More A feed → more reaction → more liquid product → level rises.
            // Negative feedback: level high → reduce A feed.
            let reactor_lv = plant.bus.outputs.xmeas[7];
            plant.bus.inputs.mv[2] =
                (nominal_mv[2] - 0.5 * (reactor_lv - 69.0)).clamp(0.0, 100.0);

            let snap = plant.snapshot();

            // ── CSV row ────────────────────────────────────────────────────────
            let mut row = format!("{:.6}", t_simulation);
            for i in 0..22 { row.push_str(&format!(",{:.6}", snap.xmeas[i])); }
            for i in 0..12 { row.push_str(&format!(",{:.6}", snap.xmv[i])); }
            row.push_str(&format!(",{:.6e}", snap.solver.deriv_norm));
            for i in 0..10  { row.push_str(&format!(",{:.6e}", snap.state.get(i).copied().unwrap_or(f64::NAN))); }
            for i in 27..35 { row.push_str(&format!(",{:.6e}", snap.state.get(i).copied().unwrap_or(f64::NAN))); }
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

            // ── Time limit ────────────────────────────────────────────────────
            last_state = Some(snap.state.clone());
            if let Some(max_t) = config.max_sim_time_h {
                if t_simulation >= max_t {
                    eprintln!("SIMULATION TIME LIMIT: {:.3} h reached", t_simulation);
                    let _ = csv.flush();
                    clean_exit = true;
                    break;
                }
            }

            let running = dashboard.render(&snap).expect("Failed to render dashboard");
            if !running {
                clean_exit = true;
                break;
            }
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

    // ── Snapshot on clean exit ─────────────────────────────────────────────────
    if clean_exit {
        if let (Some(ref path), Some(ref state)) = (&config.snapshot_path, &last_state) {
            write_snapshot_toml(path, state, t_simulation);
        }
    }
}

fn write_snapshot_toml(path: &str, state: &[f64], t_h: f64) {
    let file = File::create(path).expect("Failed to create snapshot file");
    let mut w = BufWriter::new(file);

    let comps = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let valve_names = [
        "d_feed", "e_feed", "a_feed", "a_c_feed",
        "compressor_recycle_valve", "purge_valve",
        "separator_underflow", "stripper_product", "stripper_steam_valve",
        "reactor_cooling_water", "condenser_cooling_water", "agitator_speed",
    ];

    writeln!(w, "[meta]").unwrap();
    writeln!(w, "mode = 1").unwrap();
    writeln!(w, "description = \"TEP snapshot at t = {:.4} h\"", t_h).unwrap();
    writeln!(w, "source = \"auto-snapshot from simulation\"").unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.reactor_vapor]").unwrap();
    for (i, c) in comps.iter().enumerate() {
        writeln!(w, "{} = {}", c, state[i]).unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "[state.reactor]").unwrap();
    writeln!(w, "energy = {}", state[8]).unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.separator_vapor]").unwrap();
    for (i, c) in comps.iter().enumerate() {
        writeln!(w, "{} = {}", c, state[9 + i]).unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "[state.separator]").unwrap();
    writeln!(w, "energy = {}", state[17]).unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.stripper_liquid]").unwrap();
    for (i, c) in comps.iter().enumerate() {
        writeln!(w, "{} = {}", c, state[18 + i]).unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "[state.stripper]").unwrap();
    writeln!(w, "energy = {}", state[26]).unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.compressor_vapor]").unwrap();
    for (i, c) in comps.iter().enumerate() {
        writeln!(w, "{} = {}", c, state[27 + i]).unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "[state.compressor]").unwrap();
    writeln!(w, "energy = {}", state[35]).unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.cooling]").unwrap();
    writeln!(w, "reactor_water_temp   = {}", state[36]).unwrap();
    writeln!(w, "separator_water_temp = {}", state[37]).unwrap();
    writeln!(w).unwrap();

    writeln!(w, "[state.valves]").unwrap();
    for (i, name) in valve_names.iter().enumerate() {
        writeln!(w, "{:<25} = {}", name, state[38 + i]).unwrap();
    }

    w.flush().unwrap();
    eprintln!("Snapshot written → {}", path);
}
