use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PointLight {
    pub position: Vector3<f32>,
    pub intensity: f32,
}
