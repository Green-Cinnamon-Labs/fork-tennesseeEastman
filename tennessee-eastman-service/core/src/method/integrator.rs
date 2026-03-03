// core/src/method/integrator.rs

use crate::dynamics::model::DynamicModel;
use crate::state::State;

pub trait Integrator {
    fn step(
        &self,
        model: &mut dyn DynamicModel,
        state: &mut State,
        dt: f64,
    );
}

impl Integrator for Box<dyn Integrator> {
    fn step(
        &self,
        model: &mut dyn DynamicModel,
        state: &mut State,
        dt: f64,
    ) {
        self.as_ref().step(model, state, dt);
    }
}