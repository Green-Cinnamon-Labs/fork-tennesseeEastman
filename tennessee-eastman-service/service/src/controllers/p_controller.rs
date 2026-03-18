// controllers/p_controller.rs

use super::{Controller, ControllerInfo, ControllerParams};

/// Proportional controller: mv = clamp(bias + kp * (xmeas[xmeas_idx] - setpoint), 0, 100)
pub struct PController {
    pub id:        String,
    pub xmeas_idx: usize,
    pub xmv_idx:   usize,
    pub kp:        f64,
    pub setpoint:  f64,
    pub bias:      f64,
    pub enabled:   bool,
}

impl PController {
    pub fn new(id: impl Into<String>, xmeas_idx: usize, xmv_idx: usize, kp: f64, setpoint: f64, bias: f64) -> Self {
        Self { id: id.into(), xmeas_idx, xmv_idx, kp, setpoint, bias, enabled: true }
    }
}

impl Controller for PController {
    fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]) {
        if !self.enabled { return; }
        xmv[self.xmv_idx] =
            (self.bias + self.kp * (xmeas[self.xmeas_idx] - self.setpoint))
            .clamp(0.0, 100.0);
    }

    fn id(&self) -> &str { &self.id }

    fn info(&self) -> ControllerInfo {
        ControllerInfo {
            id: self.id.clone(),
            controller_type: "P".into(),
            xmeas_idx: self.xmeas_idx,
            xmv_idx: self.xmv_idx,
            kp: self.kp,
            ki: 0.0,
            kd: 0.0,
            setpoint: self.setpoint,
            bias: self.bias,
            enabled: self.enabled,
        }
    }

    fn update(&mut self, params: &ControllerParams) {
        if let Some(kp) = params.kp { self.kp = kp; }
        if let Some(sp) = params.setpoint { self.setpoint = sp; }
        if let Some(b) = params.bias { self.bias = b; }
        if let Some(en) = params.enabled { self.enabled = en; }
    }
}
