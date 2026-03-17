// controllers/mod.rs

pub mod p_controller;
pub use p_controller::PController;

/// A single control loop: reads XMEAS, writes XMV.
///
/// The `step` call is made once per simulation tick, after `plant.step(dt)` and
/// after the ramp logic — see `docs/01-premissas.md § Premissas para o Desacoplamento`.
pub trait Controller: Send {
    fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]);
}

/// Holds an ordered list of controllers and applies them sequentially each tick.
///
/// Each controller writes to its own XMV index. No arbitration is performed —
/// if two controllers target the same index, the last one wins.
#[derive(Default)]
pub struct ControllerBank {
    controllers: Vec<Box<dyn Controller>>,
}

impl ControllerBank {
    pub fn add(&mut self, ctrl: Box<dyn Controller>) {
        self.controllers.push(ctrl);
    }

    pub fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]) {
        for ctrl in &mut self.controllers {
            ctrl.step(xmeas, xmv);
        }
    }
}
