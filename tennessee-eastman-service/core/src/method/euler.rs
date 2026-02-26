use crate::dynamics::model::DynamicModel;
use crate::state::State;
use crate::inputs::Inputs;

pub struct Euler;

impl Euler {
    pub fn step<M: DynamicModel>(
        model: &M,
        state: &mut [f64],
        inputs: &Inputs,
        dt: f64,
    ) {
        let mut dx = vec![0.0; state.len()];

        model.derivatives(state, inputs, &mut dx);

        for i in 0..state.len() {
            state[i] += dt * dx[i];
        }
    }
}