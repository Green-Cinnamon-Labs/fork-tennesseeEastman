// params.rs

#[derive(Clone, Copy)]

pub struct Params {
    pub n_states: usize,
    pub n_mv: usize,
    pub n_dv: usize,
    pub n_outputs: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            n_states: 50,
            n_mv: 12,
            n_dv: 20,
            n_outputs: 41,
        }
    }
}
