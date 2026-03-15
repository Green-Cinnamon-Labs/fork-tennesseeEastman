// controllers/p_controller.rs

use super::Controller;

/// Proportional controller: mv = clamp(bias + kp * (xmeas[xmeas_idx] - setpoint), 0, 100)
pub struct PController {
    pub xmeas_idx: usize,
    pub xmv_idx:   usize,
    pub kp:        f64,
    pub setpoint:  f64,
    pub bias:      f64,
}

impl Controller for PController {
    fn step(&mut self, xmeas: &[f64], xmv: &mut [f64]) {
        xmv[self.xmv_idx] =
            (self.bias + self.kp * (xmeas[self.xmeas_idx] - self.setpoint))
            .clamp(0.0, 100.0);
    }
}
