// grpc_server.rs — gRPC service implementation
//
// Expõe apenas: StreamMetrics, GetPlantStatus, ListControllers, UpdateController.
// Não permite criar/remover controladores nem controlar distúrbios.

use std::time::Duration;
use tonic::{Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::shared::{SharedPlant, AlarmSnapshot};
use crate::controllers::ControllerParams;

pub mod pb {
    tonic::include_proto!("tep.v1");
}

use pb::plant_service_server::PlantService;
use pb::*;

pub struct PlantServiceImpl {
    shared: SharedPlant,
}

impl PlantServiceImpl {
    pub fn new(shared: SharedPlant) -> Self {
        Self { shared }
    }
}

fn alarm_to_pb(a: &AlarmSnapshot) -> Alarm {
    Alarm {
        variable: a.variable.clone(),
        condition: a.condition.clone(),
        active: a.active,
    }
}

fn info_to_pb(info: &crate::controllers::ControllerInfo, xmeas: &[f64], xmv: &[f64]) -> ControllerInfo {
    ControllerInfo {
        id: info.id.clone(),
        controller_type: info.controller_type.clone(),
        xmeas_index: info.xmeas_idx as u32,
        xmv_index: info.xmv_idx as u32,
        kp: info.kp,
        ki: info.ki,
        kd: info.kd,
        setpoint: info.setpoint,
        bias: info.bias,
        enabled: info.enabled,
        current_measurement: xmeas.get(info.xmeas_idx).copied().unwrap_or(0.0),
        current_output: xmv.get(info.xmv_idx).copied().unwrap_or(0.0),
        error: xmeas.get(info.xmeas_idx).copied().unwrap_or(0.0) - info.setpoint,
    }
}

#[tonic::async_trait]
impl PlantService for PlantServiceImpl {

    type StreamMetricsStream = ReceiverStream<Result<PlantMetrics, Status>>;

    async fn stream_metrics(
        &self,
        request: Request<StreamMetricsRequest>,
    ) -> Result<Response<Self::StreamMetricsStream>, Status> {
        let interval_ms = request.into_inner().interval_ms;
        let interval = if interval_ms > 0.0 {
            Duration::from_secs_f64(interval_ms / 1000.0)
        } else {
            Duration::from_millis(100) // default 100ms
        };

        let shared = self.shared.clone();
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            loop {
                let metrics = {
                    let state = shared.lock().unwrap();
                    PlantMetrics {
                        t_h: state.metrics.t_h,
                        xmeas: state.metrics.xmeas.clone(),
                        xmv: state.metrics.xmv.clone(),
                        alarms: state.metrics.alarms.iter().map(alarm_to_pb).collect(),
                        deriv_norm: state.metrics.deriv_norm,
                        isd_active: state.metrics.isd_active,
                    }
                };
                if tx.send(Ok(metrics)).await.is_err() {
                    break; // client disconnected
                }
                tokio::time::sleep(interval).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_plant_status(
        &self,
        _request: Request<GetPlantStatusRequest>,
    ) -> Result<Response<PlantStatus>, Status> {
        let state = self.shared.lock().unwrap();
        let xmeas = &state.metrics.xmeas;
        let xmv = &state.metrics.xmv;

        let controllers: Vec<ControllerInfo> = state.bank.list()
            .iter()
            .map(|info| info_to_pb(info, xmeas, xmv))
            .collect();

        let metrics = PlantMetrics {
            t_h: state.metrics.t_h,
            xmeas: xmeas.clone(),
            xmv: xmv.clone(),
            alarms: state.metrics.alarms.iter().map(alarm_to_pb).collect(),
            deriv_norm: state.metrics.deriv_norm,
            isd_active: state.metrics.isd_active,
        };

        Ok(Response::new(PlantStatus {
            metrics: Some(metrics),
            controllers,
            active_idv: state.active_idv.iter().map(|&v| v as u32).collect(),
        }))
    }

    async fn list_controllers(
        &self,
        _request: Request<ListControllersRequest>,
    ) -> Result<Response<ListControllersResponse>, Status> {
        let state = self.shared.lock().unwrap();
        let xmeas = &state.metrics.xmeas;
        let xmv = &state.metrics.xmv;

        let controllers = state.bank.list()
            .iter()
            .map(|info| info_to_pb(info, xmeas, xmv))
            .collect();

        Ok(Response::new(ListControllersResponse { controllers }))
    }

    async fn update_controller(
        &self,
        request: Request<UpdateControllerRequest>,
    ) -> Result<Response<UpdateControllerResponse>, Status> {
        let req = request.into_inner();
        let mut state = self.shared.lock().unwrap();

        let ctrl = match state.bank.get_mut(&req.id) {
            Some(c) => c,
            None => {
                return Ok(Response::new(UpdateControllerResponse {
                    success: false,
                    message: format!("controller '{}' not found", req.id),
                    controller: None,
                }));
            }
        };

        let params = ControllerParams {
            kp: req.kp,
            ki: req.ki,
            kd: req.kd,
            setpoint: req.setpoint,
            bias: req.bias,
            enabled: req.enabled,
        };
        ctrl.update(&params);

        let info = ctrl.info();
        let xmeas = &state.metrics.xmeas;
        let xmv = &state.metrics.xmv;
        let pb_info = info_to_pb(&info, xmeas, xmv);

        Ok(Response::new(UpdateControllerResponse {
            success: true,
            message: format!("controller '{}' updated", req.id),
            controller: Some(pb_info),
        }))
    }
}
