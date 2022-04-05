use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PointLight {
    pub position: Vector3<f64>,
    pub intensity: f64,
    pub color: Vector3<f64>
}
