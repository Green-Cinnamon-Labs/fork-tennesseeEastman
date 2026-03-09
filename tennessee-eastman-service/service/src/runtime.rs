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

    loop {
        plant.step(config.dt);
        plant.bus.time += config.dt;

        // Minimum stabilizing controllers (open-loop TEP is unstable)
        // Sep level (XMEAS[11]) → Sep underflow valve (XMV[6], nominal 38.1%)
        let sep_level   = plant.bus.outputs.xmeas[11];
        plant.bus.inputs.mv[6] = (38.1 + 1.0 * (sep_level - 50.0)).clamp(0.0, 100.0);
        // Stripper level (XMEAS[14]) → Stripper product valve (XMV[7], nominal 46.5%)
        let strip_level = plant.bus.outputs.xmeas[14];
        plant.bus.inputs.mv[7] = (46.5 + 1.0 * (strip_level - 50.0)).clamp(0.0, 100.0);

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
