// core/src/dynamics/model.rs

use crate::state::State;

pub trait DynamicModel {
    fn derivatives(&mut self, state: &State) -> Vec<f64>;
    fn measurements(&self) -> &[f64] { &[] }
    fn get_mv(&self) -> Vec<f64> { vec![] }
    fn set_inputs(&mut self, mv: &[f64], dv: &[f64]) { let _ = (mv, dv); }
    fn advance_time(&mut self, dt: f64) { let _ = dt; }
}

impl DynamicModel for Box<dyn DynamicModel> {
    fn derivatives(&mut self, state: &State) -> Vec<f64> {
        self.as_mut().derivatives(state)
    }
    fn measurements(&self) -> &[f64] {
        self.as_ref().measurements()
    }
    fn get_mv(&self) -> Vec<f64> {
        self.as_ref().get_mv()
    }
    fn set_inputs(&mut self, mv: &[f64], dv: &[f64]) {
        self.as_mut().set_inputs(mv, dv)
    }
    fn advance_time(&mut self, dt: f64) {
        self.as_mut().advance_time(dt)
    }
}


