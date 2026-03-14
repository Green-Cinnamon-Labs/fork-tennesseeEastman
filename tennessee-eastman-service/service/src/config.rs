pub enum ModelKind {
    TennesseeEastman,
}

pub enum IntegratorKind {
    RK4,
}

pub struct Config {
    pub dt: f64,
    pub real_time: bool,
    /// Wall-clock pause between simulation steps when real_time is true (seconds).
    /// Independent of dt (the integration step in model time units).
    pub step_delay_secs: f64,
    pub initial_state_path: String,
    pub model: ModelKind,
    pub integrator: IntegratorKind,
    /// Cold start ramp duration in hours of simulated time.
    /// Feed valves (mv[0..4]) ramp linearly from 0% to nominal over this period.
    pub ramp_duration: f64,
    /// IDV channels to activate at startup (1-based, e.g. vec![4] activates IDV 4).
    /// Disturbances are held off during the ramp and enabled when it completes.
    pub active_idv: Vec<usize>,
}