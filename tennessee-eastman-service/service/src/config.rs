pub enum ModelKind {
    TennesseeEastman,
}

pub enum IntegratorKind {
    Euler,
}

pub struct Config {
    pub dt: f64,
    pub real_time: bool,
    pub initial_state_path: String,
    pub model: ModelKind,
    pub integrator: IntegratorKind,
}