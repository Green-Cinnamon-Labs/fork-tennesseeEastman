

use crate::dynamics::model::DynamicModel;
use crate::method::integrator::Integrator;
use crate::state::State;
use crate::bus::{Bus, Inputs};
use crate::params::Params;

pub struct Plant<M: DynamicModel, I: Integrator> {
    pub state: State,
    pub bus: Bus,
    pub model: M,
    pub params: Params,
    pub integrator: I,
}

impl<M: DynamicModel, I: Integrator> Plant<M, I> {

    pub fn new(model: M, params: Params, integrator: I) -> Self {
        let state = State::new(params.n_states);
        Self::from_state(state, model, params, integrator)
    }

    pub fn with_state_values(values: &[f64], model: M, params: Params, integrator: I) -> Self {
        let mut state = State::new(params.n_states);
        state.set(values);
        Self::from_state(state, model, params, integrator)
    }

    fn from_state(state: State, model: M, params: Params, integrator: I) -> Self {
        let mut bus = Bus::new(&params);
        let initial_mv = model.get_mv();
        bus.inputs.mv.iter_mut().zip(initial_mv.iter()).for_each(|(b, v)| *b = *v);
        Self { state, bus, model, params, integrator }
    }

    pub fn set_inputs(&mut self, inputs: Inputs) {
        self.bus.inputs = inputs;
    }

    pub fn step(&mut self, dt: f64) {
        self.model.set_inputs(&self.bus.inputs.mv, &self.bus.inputs.dv);
        self.integrator.step(&mut self.model, &mut self.state, dt);
        self.model.advance_time(dt);
        let measurements = self.model.measurements().to_vec();
        self.bus.outputs.xmeas
            .iter_mut()
            .zip(measurements.iter())
            .for_each(|(out, val)| *out = *val);
    }
}