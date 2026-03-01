// core/src/method/integrator.rs

use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::bus::Inputs;
use crate::params::Params;

pub trait Integrator {
    fn step(
        &self,
        model: &dyn DynamicModel,
        state: &mut State,
        inputs: &Inputs,
        params: &Params,
        dt: f64,
    );
}

impl Integrator for Box<dyn Integrator> {
    fn step(
        &self,
        model: &dyn DynamicModel,
        state: &mut State,
        inputs: &Inputs,
        params: &Params,
        dt: f64,
    ) {
        self.as_ref().step(model, state, inputs, params, dt);
    }
}