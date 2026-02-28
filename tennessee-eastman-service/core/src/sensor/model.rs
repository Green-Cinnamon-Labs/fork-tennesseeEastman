// sensor/model.rs

pub trait Sensor {
    fn measure(&mut self, physical_value: f64, dt: f64) -> f64;
}

pub struct IdealSensor;

impl Sensor for IdealSensor {
    fn measure(&mut self, physical_value: f64, _dt: f64) -> f64 {
        physical_value
    }
}
