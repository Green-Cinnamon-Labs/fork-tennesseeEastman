// main.rs

mod config;
mod controllers;
mod dashboard;
mod resolver;
mod runtime;

use config::{Config, ModelKind, IntegratorKind};
use controllers::{ControllerBank, PController};

fn main() {

    let config = Config {
        dt: 0.001,
        real_time: false,
        step_delay_secs: 0.0,
        initial_state_path: "cases/te_exp3_snapshot.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.0,
        active_idv: vec![],                                        // baseline — no disturbances
        max_sim_time_h: Some(20.0),
        snapshot_path: Some("cases/te_exp11_snapshot.toml".into()),
    };

    // ── Exp 11: 3-loop baseline — parameters identical to Exp 10 ──────────────
    // xmeas indices are 0-based; XMEAS(7)=idx 6, XMEAS(12)=idx 11, XMEAS(15)=idx 14
    // xmv   indices are 0-based; XMV(6)=idx 5,  XMV(7)=idx 6,    XMV(8)=idx 7
    let mut bank = ControllerBank::default();
    bank.add(Box::new(PController { xmeas_idx: 6,  xmv_idx: 5, kp: 0.1, setpoint: 2705.0, bias: 40.06 })); // reactor P → purge
    bank.add(Box::new(PController { xmeas_idx: 11, xmv_idx: 6, kp: 1.0, setpoint: 50.0,   bias: 38.1  })); // sep level → sep underflow
    bank.add(Box::new(PController { xmeas_idx: 14, xmv_idx: 7, kp: 1.0, setpoint: 50.0,   bias: 46.5  })); // strip level → strip product

    runtime::run(config, bank);
}
