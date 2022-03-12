use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PointLight {
    pub position: Vector3<f32>,
    pub intensity: f32
}

impl PointLight {
    pub fn new(position: Vector3<f32>, intensity: f32) -> Self {
        Self {
            position,
            intensity
        }
    }
}