use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::bus::Inputs;
use crate::params::Params;

pub struct Euler;

impl Euler {
    pub fn step<M: DynamicModel>(
        model: &M,
        state: &mut State,
        inputs: &Inputs,
        params: &Params,
        dt: f64,
    ) {
        let dx = model.derivatives(state, inputs, params);
        for i in 0..state.x.len() {
            state.x[i] += dt * dx[i];
        }
    }
}