// core/src/dynamics/model.rs

use crate::state::State;

pub trait DynamicModel {
    fn derivatives(&mut self, state: &State) -> Vec<f64>;
    fn measurements(&self) -> &[f64] { &[] }
}

impl DynamicModel for Box<dyn DynamicModel> {
    fn derivatives(&mut self, state: &State) -> Vec<f64> {
        self.as_mut().derivatives(state)
    }
}


