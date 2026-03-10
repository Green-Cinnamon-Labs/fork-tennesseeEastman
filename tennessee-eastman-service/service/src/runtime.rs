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

    // Diagnostic: focused snapshot before first step.
    {
        plant.model.set_inputs(&plant.bus.inputs.mv, &plant.bus.inputs.dv);
        let dy = plant.model.derivatives(&plant.state);

        let all_zero = dy.iter().all(|&v| v == 0.0);
        let xm = plant.model.measurements();
        let yy = &plant.state.x;

        // utls uses indices 12..17 because sep condensables are yy[9+3..9+8]
        let utlr: f64 = yy[3..8].iter().sum();
        let utls: f64 = yy[12..17].iter().sum();
        let utlc: f64 = yy[18..26].iter().sum();

        let mut ranked: Vec<(usize, f64)> = dy.iter().enumerate()
            .map(|(i, &v)| (i, v.abs())).collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        eprintln!("=== t=0 diagnostic ===");
        eprintln!("  isd at t=0 (all dy==0): {}", all_zero);
        eprintln!("  Reactor P   xmeas[06]={:.1} kPa  (shut >3000)", xm[6]);
        eprintln!("  Reactor T   xmeas[08]={:.1} °C   (shut >175)", xm[8]);
        eprintln!("  Reactor Lv  xmeas[07]={:.1} %    (shut <10% or >90%)", xm[7]);
        eprintln!("  Sep Lv      xmeas[11]={:.1} %    (shut <2.7% or >90%)", xm[11]);
        eprintln!("  Stripper Lv xmeas[14]={:.1} %    (shut <? or >?)", xm[14]);
        eprintln!("  CW Reactor  twr=yy[36]={:.2} °C  (nominal ~94.6)", yy[36]);
        eprintln!("  CW Sep      tws=yy[37]={:.2} °C  (nominal ~77.3)", yy[37]);
        eprintln!("  utlr={:.3}  utls={:.3}  utlc={:.3}  kmol", utlr, utls, utlc);
        eprintln!("  Top 5 |dy|:");
        for (i, _) in ranked.iter().take(5) {
            eprintln!("    dy[{:02}] = {:+.6e}", i, dy[*i]);
        }
        eprintln!("======================");
    }

    let mut dashboard = Dashboard::new().expect("Failed to initialize terminal dashboard");

    // [PATCH:RUNTIME_TIMERS] Two independent time counters.
    //
    // t_simulation : total simulation time (always advances every step)
    // t_operational: time the plant was inside normal operating bounds
    //                (advances only when no alarm is active)
    //
    // t_operational is useful as a KPI: how long did the plant run before
    // the first shutdown event?  Both values are printed on shutdown.
    let mut t_simulation  = 0.0_f64;
    let mut t_operational = 0.0_f64;
    let mut isd_active    = false;

    loop {
        if !isd_active {
            plant.step(config.dt);
            t_simulation   += config.dt;
            plant.bus.time += config.dt;

            // ── Minimum stabilizing controllers (open-loop TEP is unstable) ─────
            // Sep level (XMEAS[11]) → Sep underflow valve (XMV[6], nominal 38.1%)
            let sep_level = plant.bus.outputs.xmeas[11];
            plant.bus.inputs.mv[6] = (38.1 + 1.0 * (sep_level - 50.0)).clamp(0.0, 100.0);
            // Stripper level (XMEAS[14]) → Stripper product valve (XMV[7], nominal 46.5%)
            let strip_level = plant.bus.outputs.xmeas[14];
            plant.bus.inputs.mv[7] = (46.5 + 1.0 * (strip_level - 50.0)).clamp(0.0, 100.0);

            // ── TEMP CONTROL PATCH (2026-03-10) [PATCH:REACTOR_P_CTRL] ─────────
            // Simple proportional controller: reactor pressure → purge valve.
            //
            // Purpose: prevent non-condensable gases (A, B, C) from accumulating in
            // the reactor and driving pressure above the 3000 kPa ISD threshold.
            // Without this patch the reactor pressure escalates from ~2705 kPa to
            // >3000 kPa within seconds, triggering shutdown.
            //
            // This is a TEMPORARY stabilization patch for the Rust model and is
            // NOT part of the original FORTRAN Tennessee Eastman reference
            // implementation.  The structural root cause (initial state not being
            // a true Rust-model steady state / compressor curve behaviour) is
            // tracked separately and must be resolved before this patch is removed.
            //
            // Tuning (proportional only — no integral, no anti-windup needed):
            //   Controlled variable : XMEAS[6]  (reactor pressure, kPa)
            //   Manipulated variable: XMV[5]    (purge valve, %; 0-based index)
            //   Setpoint            : 2705 kPa  (nominal steady-state value)
            //   Nominal MV          :   40.06 % (from FORTRAN TEINIT initial state)
            //   Gain                : +0.10 %/kPa  (higher P → open purge more)
            //   Clamp               : [0, 100] %
            //
            // At setpoint (2705 kPa): purge = 40.06 % (no change from nominal)
            // At ISD limit (3000 kPa): purge ≈ 69.6 %  (aggressive venting)
            // [PATCH:REACTOR_P_CTRL]
            let reactor_p = plant.bus.outputs.xmeas[6];
            plant.bus.inputs.mv[5] =
                (40.06 + 0.10 * (reactor_p - 2705.0)).clamp(0.0, 100.0);
            // ───────────────────────────────────────────────────────────────────

            let snap = plant.snapshot();

            // [PATCH:RUNTIME_TIMERS] Advance t_operational only while the plant is
            // inside all operating bounds (no active alarms).
            let any_alarm = snap.alarms.iter().any(|a| a.active);
            if !any_alarm {
                t_operational += config.dt;
            }

            // [PATCH:RUNTIME_TIMERS] Shutdown detection.
            //
            // The TEP model sets all 50 derivatives to exactly 0.0 when any ISD
            // condition is met (see model.rs Block 41).  After integration this
            // produces deriv_norm == 0.0, which is a reliable ISD indicator.
            // The `any_alarm` guard prevents a false-positive if the plant
            // happens to reach a true equilibrium with deriv_norm naturally ≈ 0.
            if snap.solver.deriv_norm == 0.0 && any_alarm {
                isd_active = true;
                eprintln!("SIMULATION STOPPED: plant shutdown condition reached");
                eprintln!(
                    "  t_simulation  = {:.3} s  ({:.5} h)",
                    t_simulation,
                    t_simulation / 3600.0
                );
                eprintln!(
                    "  t_operational = {:.3} s  ({:.5} h)",
                    t_operational,
                    t_operational / 3600.0
                );
            }

            // Render after all state/alarm updates so the dashboard shows the
            // final frozen state when ISD fires.
            let running = dashboard.render(&snap).expect("Failed to render dashboard");
            if !running { break; }
        } else {
            // ISD active: simulation is frozen; keep the dashboard alive so the
            // operator can inspect the state. Only 'q' exits.
            let snap = plant.snapshot();
            let running = dashboard.render(&snap).expect("Failed to render dashboard");
            if !running { break; }

            if config.real_time {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        if !isd_active && config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
