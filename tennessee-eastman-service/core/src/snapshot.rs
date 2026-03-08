// core/src/snapshot.rs

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
}
