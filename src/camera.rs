extern crate nalgebra;
use std::f64::{consts::PI, INFINITY};

use nalgebra::{Vector3};
use rand::Rng;
use serde::Deserialize;

use crate::ray::Ray;

fn default_canvas_fov_x() -> f64 {
    return 130.0;
}

fn default_near_clipping_range() -> f64 {
    return 0.5;
}

fn default_far_clipping_range() -> f64 {
    return INFINITY;
}

fn default_canvas_width() -> u32 {
    return 1280;
}

fn default_canvas_height() -> u32 {
    return 720;
}

fn default_camera_right() -> Vector3<f64> {
    return Vector3::new(1., 0., 0.);
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Camera {
    pub origin: Vector3<f64>,
    pub up: Vector3<f64>,
    pub forward: Vector3<f64>,
    #[serde(default = "default_camera_right")]
    pub right: Vector3<f64>,
    #[serde(default = "default_canvas_fov_x")]
    pub fov_x_deg: f64,
    #[serde(default = "default_near_clipping_range")]
    pub near_clipping_range: f64,
    #[serde(default = "default_far_clipping_range")]
    pub far_clipping_range: f64,
    #[serde(default = "default_canvas_width")]
    pub canvas_width: u32,
    #[serde(default = "default_canvas_height")]
    pub canvas_height: u32,
}

impl Camera {
    pub fn create_ray(&self, x: usize, y: usize) -> Ray {
        let width = self.canvas_width;
        let height = self.canvas_height;

        let fov = self.fov_x_deg * PI / 180.;

        let viewport_width = (fov / 2.).tan();
        let viewport_height = viewport_width * (height - 1) as f64 / (width - 1) as f64;

        let step_x = ((2. * viewport_width) / (width - 1) as f64) * self.right;
        let step_y = ((2. * viewport_height) / (height - 1) as f64) * -self.up;

        let viewport_top_left =
            self.forward - viewport_width * self.right + viewport_height * self.up;

        let mut rng = rand::thread_rng();
        let dx = rng.gen::<f64>() - 0.5;
        let dy = rng.gen::<f64>() - 0.5;

        let direction = viewport_top_left + step_x * (x as f64 + dx) + step_y * (y as f64 + dy);

        Ray::new(self.origin, direction.normalize())
    }
}
