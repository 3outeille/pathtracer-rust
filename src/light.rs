use nalgebra::Vector3;

pub struct PointLight {
    pub position: Vector3<f32>,
    pub intensity: f32
}

impl PointLight {
    pub fn new(position_arg: Vector3<f32>, intensity_arg: f32) -> Self {
        Self {
            position: position_arg,
            intensity: intensity_arg
        }
    }
}