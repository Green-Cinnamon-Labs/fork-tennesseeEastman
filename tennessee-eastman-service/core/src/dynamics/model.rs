// core/src/dynamics/model.rs

use crate::state::State;
use crate::bus::Inputs;
use crate::params::Params;

pub trait DynamicModel {
    fn derivatives(
        &self,
        state: &State,
        inputs: &Inputs,
        params: &Params,
    ) -> Vec<f64>;
}

impl DynamicModel for Box<dyn DynamicModel> {
    fn derivatives(&self, state: &State, inputs: &Inputs, params: &Params) -> Vec<f64> {
        self.as_ref().derivatives(state, inputs, params)
    }
}