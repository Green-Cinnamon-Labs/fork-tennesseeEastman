// controllers/mod.rs

pub mod p_controller;
pub use p_controller::PController;

/// Parameters that can be updated at runtime via gRPC.
pub struct ControllerParams {
    pub kp: Option<f64>,
    pub ki: Option<f64>,
    pub kd: Option<f64>,
    pub setpoint: Option<f64>,
    pub bias: Option<f64>,
    pub enabled: Option<bool>,
}

/// Read-only snapshot of a controller's configuration and live values.
pub struct ControllerInfo {
    pub id: String,
    pub controller_type: String, // "P", "PI", "PID"
    pub xmeas_idx: usize,
    pub xmv_idx: usize,
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub setpoint: f64,
    pub bias: f64,
    pub enabled: bool,
}

/// A single control loop: reads XMEAS, writes XMV.
///
/// The `step` call is made once per simulation tick, after `plant.step(dt)` and
/// after the ramp logic — see `docs/01-premissas.md § Premissas para o Desacoplamento`.
pub trait Controller: Send {
    /// Execute one control step. Disabled controllers must no-op.
    fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]);

    /// Unique identifier for this controller.
    fn id(&self) -> &str;

    /// Return a snapshot of this controller's configuration.
    fn info(&self) -> ControllerInfo;

    /// Apply partial parameter updates. Unsupported fields are silently ignored.
    fn update(&mut self, params: &ControllerParams);
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

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.controllers.len();
        self.controllers.retain(|c| c.id() != id);
        self.controllers.len() < before
    }

    pub fn get(&self, id: &str) -> Option<&(dyn Controller + '_)> {
        self.controllers.iter().find(|c| c.id() == id).map(|c| c.as_ref())
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Box<dyn Controller>> {
        self.controllers.iter_mut().find(|c| c.id() == id)
    }

    pub fn list(&self) -> Vec<ControllerInfo> {
        self.controllers.iter().map(|c| c.info()).collect()
    }

    pub fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]) {
        for ctrl in &mut self.controllers {
            ctrl.step(xmeas, xmv);
        }
    }
}
