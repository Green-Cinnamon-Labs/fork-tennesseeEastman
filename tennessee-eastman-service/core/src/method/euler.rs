// core/src/method/euler.rs

use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::bus::Inputs;
use crate::params::Params;
use crate::method::integrator::Integrator;

pub struct Euler;

impl Integrator for Euler {
    fn step(
        &self,
        model: &dyn DynamicModel,
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