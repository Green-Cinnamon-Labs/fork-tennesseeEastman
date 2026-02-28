// state.rs

#[derive(Clone)]
pub struct State {
    /// Estados dinâmicos da planta (ex: 50 estados do TE)
    pub x: Vec<f64>,
}

impl State {
    
    pub fn new(n: usize) -> Self {
        Self {
            x: vec![0.0; n],
        }
    }

    pub fn set(&mut self, values: &[f64]) {
        assert!(
            values.len() == self.x.len(),
            "State::set: tamanho inválido"
        );
        self.x.copy_from_slice(values);
    }
}
