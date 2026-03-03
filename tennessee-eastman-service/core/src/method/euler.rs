// core/src/method/euler.rs

use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::method::integrator::Integrator;

pub struct Euler;

impl Integrator for Euler {
    fn step(
        &self,
        model: &mut dyn DynamicModel,
        state: &mut State,
        dt: f64,
    ) {
        let dx = model.derivatives(state);
        for i in 0..state.x.len() {
            state.x[i] += dt * dx[i];
        }
    }
}