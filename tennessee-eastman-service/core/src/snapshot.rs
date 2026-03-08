// core/src/snapshot.rs

/// Numerical integration diagnostics for one time step.
#[derive(Debug, Clone)]
pub struct SolverInfo {
    /// Integrator algorithm name (e.g. "RK4", "Euler")
    pub algorithm: &'static str,
    /// Step size used (s)
    pub dt: f64,
    /// Number of state variables
    pub n_states: usize,
    /// Max |Δxᵢ / dt| across all states — proxy for derivative magnitude
    pub deriv_norm: f64,
}

/// A single named binary condition reported by the model.
#[derive(Debug, Clone)]
pub struct Alarm {
    pub name: &'static str,
    pub active: bool,
}

/// A single coherent frame of simulation state captured at one time step.
///
/// Consuming code (dashboards, loggers, analysis tools) should depend only
/// on this struct — never on `Plant`, `Bus`, or any model internals.
#[derive(Debug, Clone)]
pub struct SimulationSnapshot {
    /// Simulation time (s)
    pub time: f64,

    /// Process measurements — XMEAS vector (0-based indexing)
    pub xmeas: Vec<f64>,

    /// Manipulated variable commands — XMV vector (0-based indexing)
    pub xmv: Vec<f64>,

    /// Disturbance channel values — IDV vector (0-based indexing)
    pub dv: Vec<f64>,

    /// Full internal dynamic state vector — YY (0-based indexing)
    pub state: Vec<f64>,

    /// Numerical integration diagnostics
    pub solver: SolverInfo,

    /// Active alarm conditions reported by the model
    pub alarms: Vec<Alarm>,
}
