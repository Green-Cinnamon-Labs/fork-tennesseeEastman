// bus.rs
use crate::params::Params;

#[derive(Clone)]
pub struct Inputs {
    pub mv: Vec<f64>,   // manipuladas
    pub dv: Vec<f64>,   // distúrbios
}

impl Inputs {
    pub fn new(n_mv: usize, n_dv: usize) -> Self {
        Self {
            mv: vec![0.0; n_mv],
            dv: vec![0.0; n_dv],
        }
    }
}

#[derive(Clone)]
pub struct Outputs {
    pub xmeas: Vec<f64>,   // medições
}

impl Outputs {
    pub fn new(n_outputs: usize) -> Self {
        Self {
            xmeas: vec![0.0; n_outputs],
        }
    }
}

pub struct Bus {
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub time: f64,
}

impl Bus {

    pub fn new(params: &Params) -> Self {
        Self {
            inputs: Inputs::new(params.n_mv, params.n_dv),
            outputs: Outputs::new(params.n_outputs),
            time: 0.0,
        }
    }
}
