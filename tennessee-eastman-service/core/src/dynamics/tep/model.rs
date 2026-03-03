// core/src/dynamics/tep/model.rs

use crate::dynamics::model::DynamicModel;
use crate::dynamics::tep::constants::TepConstants;
use crate::dynamics::tep::disturbance_state::TepDisturbanceState;
use crate::state::State;

pub struct TennesseeEastmanModel {
    pub constants:   TepConstants,
    pub disturbance: TepDisturbanceState,
    pub xmv:         [f64; 12],
    pub idv:         [i32; 20],
}

impl TennesseeEastmanModel {
    pub fn new() -> Self {
        Self {
            constants:   TepConstants::new(),
            disturbance: TepDisturbanceState::new(),
            xmv:         [0.0; 12],
            idv:         [0; 20],
        }
    }
}

impl DynamicModel for TennesseeEastmanModel {
    fn derivatives(&mut self, state: &State) -> Vec<f64> {
        vec![0.0; state.x.len()]
    }
}