// main.rs

mod config;
mod controllers;
mod dashboard;
mod grpc_server;
mod resolver;
mod runtime;
mod shared;

use std::sync::{Arc, Mutex};

use config::{Config, ModelKind, IntegratorKind};
use controllers::PController;
use shared::SharedState;
use grpc_server::PlantServiceImpl;
use grpc_server::pb::plant_service_server::PlantServiceServer;

#[tokio::main]
async fn main() {

    let headless = std::env::args().any(|a| a == "--headless");

    let config = Config {
        dt: 0.001,
        real_time: false,
        step_delay_secs: 0.0,
        initial_state_path: "cases/te_exp3_snapshot.toml".into(),
        model: ModelKind::TennesseeEastman,
        integrator: IntegratorKind::RK4,
        ramp_duration: 0.0,
        active_idv: vec![],                                        // baseline — no disturbances
        max_sim_time_h: None,                                      // continuous operation
        snapshot_path: Some("cases/te_exp11_snapshot.toml".into()),
        headless,
    };

    // ── Controllers: 3-loop baseline — parameters identical to Exp 10 ─────────
    let mut bank = controllers::ControllerBank::default();
    bank.add(Box::new(PController::new("pressure_reactor", 6,  5, 0.1, 2705.0, 40.06))); // reactor P → purge
    bank.add(Box::new(PController::new("level_separator",  11, 6, 1.0, 50.0,   38.1 ))); // sep level → underflow
    bank.add(Box::new(PController::new("level_stripper",   14, 7, 1.0, 50.0,   46.5 ))); // strip level → product

    let shared = Arc::new(Mutex::new(SharedState::new(bank)));

    // ── Simulation thread ─────────────────────────────────────────────────────
    let sim_shared = shared.clone();
    let sim_handle = std::thread::spawn(move || {
        runtime::run(config, sim_shared);
    });

    // ── gRPC server ───────────────────────────────────────────────────────────
    let addr = "[::]:50051".parse().unwrap();
    let svc = PlantServiceImpl::new(shared.clone());

    eprintln!("gRPC server listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(PlantServiceServer::new(svc))
        .serve_with_shutdown(addr, async {
            // Shutdown when the simulation thread finishes
            tokio::task::spawn_blocking(move || {
                let _ = sim_handle.join();
            })
            .await
            .ok();
        })
        .await
        .unwrap();
}
