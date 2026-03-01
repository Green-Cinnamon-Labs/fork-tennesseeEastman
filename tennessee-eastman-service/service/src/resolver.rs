// service/src/resolver.rs

use te_core::dynamics::model::DynamicModel;
use te_core::dynamics::tennessee::model::TennesseeEastmanModel;
use te_core::dynamics::tennessee::initial_state::InitialState;
use te_core::params::Params;

use crate::config::{Config, ModelKind};

pub struct ResolvedPlant {
    pub model: Box<dyn DynamicModel>,
    pub initial_state: Vec<f64>,
}

pub fn resolve(kind: &ModelKind, config: &Config) -> ResolvedPlant {
    match kind {
        ModelKind::TennesseeEastman => {
            let params = Params::default();
            let initial = InitialState::from_file(&config.initial_state_path).unwrap();
            let flat = initial.flatten().to_vec();
            let model = Box::new(TennesseeEastmanModel { params });
            ResolvedPlant { model, initial_state: flat }
        }
    }
}