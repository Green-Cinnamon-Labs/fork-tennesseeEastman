// runtime.rs

use te_core::plant::Plant;
use te_core::state::State;
use te_core::params::Params;

use crate::config::Config;
use crate::metadata::{MEASUREMENTS, MANIPULATED};
use crate::resolver::resolve;


const IDX_ETR: usize = 8;
const IDX_ETS: usize = 17;
const IDX_ETC: usize = 26;
const IDX_ETV: usize = 35;
const IDX_TWR: usize = 36;
const IDX_TWS: usize = 37;

const IDX_VPOS_START: usize = 38;
const N_VALVES: usize = 12;

const VALVE_NAMES: [&str; N_VALVES] = [
    "d_feed",
    "e_feed",
    "a_feed",
    "a_c_feed",
    "compressor_recycle_valve",
    "purge_valve",
    "separator_underflow",
    "stripper_product",
    "stripper_steam_valve",
    "reactor_cooling_water",
    "condenser_cooling_water",
    "agitator_speed",
];

fn print_startup_alignment(initial_state: &[f64], bus_mv: &[f64]) {
    if initial_state.len() < IDX_VPOS_START + N_VALVES {
        eprintln!("[DEBUG] initial_state menor que 50, len={}", initial_state.len());
        return;
    }
    println!("=== DEBUG STARTUP: TOML(VPOS) vs BUS(MV) ===");
    for i in 0..N_VALVES {
        let vpos0 = initial_state[IDX_VPOS_START + i];
        let mv_bus = bus_mv.get(i).copied().unwrap_or(f64::NAN);
        println!(
            "[VALVE {:02}] {:28} VPOS_init={:8.4}   BUS_MV={:8.4}",
            i + 1,
            VALVE_NAMES[i],
            vpos0,
            mv_bus
        );
    }
    println!("1 =============================================");
}

fn debug_reactor_from_state(state: &State, xmeas: &[f64], sim_time: f64) {
    if state.x.len() < 50 {
        eprintln!("[DEBUG t={:.4}] state.x len inválido: {}", sim_time, state.x.len());
        return;
    }

    // 1) YY bruto relevante
    let etr = state.x[IDX_ETR];
    let ets = state.x[IDX_ETS];
    let etc = state.x[IDX_ETC];
    let etv = state.x[IDX_ETV];
    let twr = state.x[IDX_TWR];
    let tws = state.x[IDX_TWS];

    // 2) Somatórios (iguais ao bloco 14 do modelo)
    let utlr: f64 = state.x[3..8].iter().sum();    // UCLR (comp 4..8)
    let utls: f64 = state.x[12..17].iter().sum();  // UCLS (comp 4..8)
    let utlc: f64 = state.x[18..26].iter().sum();  // UCLC (comp 1..8)
    let utvv: f64 = state.x[27..35].iter().sum();  // UCVV (comp 1..8)

    // 3) Composição normalizada (amostra: comp D..H no líquido do reator)
    let mut xlr = [0.0_f64; 8];
    if utlr > 0.0 && utlr.is_finite() {
        for i in 3..8 {
            xlr[i] = state.x[i] / utlr;
        }
    }

    // 4) Energia específica
    let esr = if utlr > 0.0 { etr / utlr } else { f64::NAN };
    let ess = if utls > 0.0 { ets / utls } else { f64::NAN };
    let esc = if utlc > 0.0 { etc / utlc } else { f64::NAN };
    let esv = if utvv > 0.0 { etv / utvv } else { f64::NAN };

    // 5/6) Temperatura/pressão finais: use XMEAS já computado pelo modelo
    let t_reactor = xmeas.get(8).copied().unwrap_or(f64::NAN); // XMEAS(9)
    let p_reactor = xmeas.get(6).copied().unwrap_or(f64::NAN); // XMEAS(7)

    println!(
        "[DBG t={:.4}] etr={:.6} ets={:.6} etc={:.6} etv={:.6} twr={:.6} tws={:.6}",
        sim_time, etr, ets, etc, etv, twr, tws
    );
    println!(
        "[DBG t={:.4}] utlr={:.6} utls={:.6} utlc={:.6} utvv={:.6}",
        sim_time, utlr, utls, utlc, utvv
    );
    println!(
        "[DBG t={:.4}] xlr(D..H)= [{:.5}, {:.5}, {:.5}, {:.5}, {:.5}]  sum={:.5}",
        sim_time, xlr[3], xlr[4], xlr[5], xlr[6], xlr[7], xlr[3] + xlr[4] + xlr[5] + xlr[6] + xlr[7]
    );
    println!(
        "[DBG t={:.4}] esr={:.6} ess={:.6} esc={:.6} esv={:.6} | XMEAS_Treactor={:.6} XMEAS_Preactor={:.6}",
        sim_time, esr, ess, esc, esv, t_reactor, p_reactor
    );
    println!("2 =============================================");

}




pub fn run(config: Config) {
    let resolved = resolve(&config);

    let params = Params::default();
    let mut plant = Plant::with_state_values(
        &resolved.initial_state,
        resolved.model,
        params,
        resolved.integrator,
    );

    // Diagnóstico de origem dos valores
    print_startup_alignment(&resolved.initial_state, &plant.bus.inputs.mv);

    let mut k: u64 = 0;
    loop {
        plant.step(config.dt);
        plant.bus.time += config.dt;
        k += 1;

        // Debug periódico: 5 primeiros passos e depois 1 a cada 100
        if k <= 5 || k % 100 == 0 {
            debug_reactor_from_state(&plant.state, &plant.bus.outputs.xmeas, plant.bus.time);

            // Posições reais das válvulas (estado), não BUS_MV
            for i in 0..N_VALVES {
                let vpos = plant.state.x[IDX_VPOS_START + i];
                println!(
                    "[VPOS({:02})] {:28} = {:8.4} %",
                    i + 1, VALVE_NAMES[i], vpos
                );
            }

            // BUS_MV (comando externo)
            for i in 0..N_VALVES {
                let mv = plant.bus.inputs.mv.get(i).copied().unwrap_or(f64::NAN);
                println!(
                    "[BUS_MV({:02})] {:25} = {:8.4} %",
                    i + 1, VALVE_NAMES[i], mv
                );
            }
        }

        // seus prints atuais podem ficar, mas saiba a fonte:
        // XMEAS = saída do modelo
        // XMV do jeito atual = bus.inputs.mv (não é VPOS)

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
