
use crate::dynamics::model::DynamicModel;
use crate::method::euler::Euler;
use crate::state::State;

pub struct Plant<M: DynamicModel> {
    pub state: State,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub model: M,
}


impl Plant {
    
    /// Inicialização padrão: estado zerado
    pub fn new(params: Params) -> Self { 
        let state = State::new(params.n_states);

        Self::from_state(state, params)
    }

    /// Inicialização explícita a partir de valores de estado conhecidos
    pub fn with_state_values(values: &[f64], params: Params) -> Self {
        let mut state = State::new(params.n_states);
        state.set(values);

        Self::from_state(state, params)
    }

    // Construtor privado para evitar repetição de código
    fn from_state(state: State, params: Params) -> Self {
        Self {
            state,
            inputs: Inputs::new(params.n_mv, params.n_dv),
            outputs: Outputs::new(params.n_outputs),
            params,
        }
    }


    // 
    pub fn set_inputs(&mut self, inputs: Inputs) {
        self.inputs = inputs;
    }

    pub fn step(&mut self, dt: f64) {
        Euler::step(&self.model, &mut self.state.x, &self.inputs, dt);
    }
}
