

use crate::dynamics::model::DynamicModel;
use crate::method::integrator::Integrator;
use crate::state::State;
use crate::bus::{Bus, Inputs};
use crate::params::Params;
use crate::snapshot::SimulationSnapshot;

pub struct Plant<M: DynamicModel, I: Integrator> {
    pub state: State,
    pub bus: Bus,
    pub model: M,
    pub params: Params,
    pub integrator: I,
    last_dt: f64,
    last_deriv_norm: f64,
}

impl<M: DynamicModel, I: Integrator> Plant<M, I> {

    pub fn new(model: M, params: Params, integrator: I) -> Self {
        let state = State::new(params.n_states);
        Self::from_state(state, model, params, integrator)
    }

    pub fn with_state_values(values: &[f64], model: M, params: Params, integrator: I) -> Self {
        let mut state = State::new(params.n_states);
        state.set(values);
        Self::from_state(state, model, params, integrator)
    }

    fn from_state(state: State, model: M, params: Params, integrator: I) -> Self {
        let mut bus = Bus::new(&params);
        let initial_mv = model.get_mv();
        bus.inputs.mv.iter_mut().zip(initial_mv.iter()).for_each(|(b, v)| *b = *v);
        Self { state, bus, model, params, integrator, last_dt: 0.0, last_deriv_norm: 0.0 }
    }

    pub fn set_inputs(&mut self, inputs: Inputs) {
        self.bus.inputs = inputs;
    }

    pub fn snapshot(&self) -> SimulationSnapshot {
        use crate::snapshot::SolverInfo;
        SimulationSnapshot {
            time:  self.bus.time,
            xmeas: self.bus.outputs.xmeas.clone(),
            xmv:   self.bus.inputs.mv.clone(),
            dv:    self.bus.inputs.dv.clone(),
            state: self.state.x.clone(),
            solver: SolverInfo {
                algorithm:   self.integrator.name(),
                dt:          self.last_dt,
                n_states:    self.params.n_states,
                deriv_norm:  self.last_deriv_norm,
            },
            alarms: self.model.alarms(),
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.model.set_inputs(&self.bus.inputs.mv, &self.bus.inputs.dv);
        let x_before = self.state.x.clone();
        self.integrator.step(&mut self.model, &mut self.state, dt);
        self.last_deriv_norm = x_before.iter().zip(self.state.x.iter())
            .map(|(a, b)| (b - a).abs() / dt)
            .fold(0.0_f64, f64::max);
        self.last_dt = dt;
        self.model.advance_time(dt);
        let measurements = self.model.measurements().to_vec();
        self.bus.outputs.xmeas
            .iter_mut()
            .zip(measurements.iter())
            .for_each(|(out, val)| *out = *val);
    }
}