// service/src/resolver.rs

use te_core::dynamics::model::DynamicModel;
use te_core::dynamics::tennessee::model::TennesseeEastmanModel;
use te_core::dynamics::tennessee::initial_state::InitialState;

use te_core::method::integrator::Integrator;
use te_core::method::euler::Euler;
use te_core::method::rk4::RK4;

use te_core::params::Params;

use crate::config::{Config, ModelKind, IntegratorKind};

pub struct ResolvedPlant {
    pub model: Box<dyn DynamicModel>,
    pub integrator: Box<dyn Integrator>,
    pub initial_state: Vec<f64>,
}

pub fn resolve(config: &Config) -> ResolvedPlant {
    let integrator = resolve_integrator(&config.integrator);

    match &config.model {
        ModelKind::TennesseeEastman => {
            let params = Params::default();
            let initial = InitialState::from_file(&config.initial_state_path).unwrap();
            let flat = initial.flatten().to_vec();
            let model = Box::new(TennesseeEastmanModel { params });
            ResolvedPlant { model, integrator, initial_state: flat }
        }
    }
}

fn resolve_integrator(kind: &IntegratorKind) -> Box<dyn Integrator> {
    match kind {
        IntegratorKind::Euler => Box::new(Euler),
        IntegratorKind::RK4   => Box::new(RK4),
    }
}