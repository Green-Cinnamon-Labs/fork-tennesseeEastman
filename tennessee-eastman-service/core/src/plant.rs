use crate::dynamics::model::DynamicModel;
use crate::method::euler::Euler;
use crate::state::State;
use crate::bus::{Bus, Inputs, Outputs};
use crate::params::Params;

pub struct Plant<M: DynamicModel> {
    pub state: State,
    pub bus: Bus,
    pub model: M,
    pub params: Params,
}

impl<M: DynamicModel> Plant<M> {

    pub fn new(model: M, params: Params) -> Self {
        let state = State::new(params.n_states);
        Self::from_state(state, model, params)
    }

    pub fn with_state_values(values: &[f64], model: M, params: Params) -> Self {
        let mut state = State::new(params.n_states);
        state.set(values);
        Self::from_state(state, model, params)
    }

    fn from_state(state: State, model: M, params: Params) -> Self {
        let bus = Bus::new(&params);
        Self { state, bus, model, params }
    }

    pub fn set_inputs(&mut self, inputs: Inputs) {
        self.bus.inputs = inputs;
    }

    pub fn step(&mut self, dt: f64) {
        Euler::step(&self.model, &mut self.state, &self.bus.inputs, &self.params, dt);
    }
}