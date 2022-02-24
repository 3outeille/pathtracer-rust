extern crate nalgebra;
use nalgebra::{Point3, Vector3};

pub struct Camera {
    pub center: Point3<f32>,
    pub point: Point3<f32>,
    pub up: Vector3<f32>,
    pub x: f32,
    pub y: f32,
    pub z_min: f32
}

impl Camera {
    pub fn new(center: Point3<f32>, point: Point3<f32>, up: Vector3<f32>, x: f32, y: f32, z_min: f32) -> Self {
        Self {
            center,
            point,
            up,
            x,
            y,
            z_min
        }
    }
}