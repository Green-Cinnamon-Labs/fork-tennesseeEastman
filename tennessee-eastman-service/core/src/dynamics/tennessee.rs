use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::bus::Inputs;
use crate::params::Params;

pub struct TennesseeEastmanModel {
    pub params: Params,
}

impl DynamicModel for TennesseeEastmanModel {
    fn derivatives(
        &self,
        state: &State,
        inputs: &Inputs,
        params: &Params,
    ) -> Vec<f64> {
        vec![0.0; state.x.len()]
    }
}