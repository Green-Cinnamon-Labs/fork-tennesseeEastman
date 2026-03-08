pub enum ModelKind {
    TennesseeEastman,
}

pub enum IntegratorKind {
    Euler,
    RK4
}

pub struct Config {
    pub dt: f64,
    pub real_time: bool,
    pub initial_state_path: String,
    pub model: ModelKind,
    pub integrator: IntegratorKind,
    /// IDV channels to activate at startup (1-based, e.g. vec![4] activates IDV 4)
    pub active_idv: Vec<usize>,
}