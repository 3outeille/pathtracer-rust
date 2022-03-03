extern crate nalgebra;
use std::f32::consts::PI;

use nalgebra::{Vector3, Vector2};

use crate::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub top_left_start: Vector3<f32>,
}

impl Camera {
    pub fn new(origin_arg: Vector3<f32>, tartget_arg: Vector3<f32>, up_arg: Vector3<f32>, fov_x_arg: f32, z_min: f32, aspect_ratio: f32) -> Self {

        let origin = origin_arg;
        let forward = (tartget_arg - origin_arg).normalize();
        let up = up_arg.normalize();
        let right = up.cross(&forward);

        let fov_x = ((fov_x_arg * 0.5) / 180.0) * PI; // radian
        let viewport_width = 2.0 * z_min * fov_x.tan();
        let viewport_height = viewport_width / aspect_ratio;

        let top_left_start = origin + (forward * z_min) - ((viewport_width/2.0)*right) + ((viewport_height/2.0) * up);

        Self {
            origin,
            forward,
            up,
            right,
            viewport_width,
            viewport_height,
            top_left_start,
        }
    }
}