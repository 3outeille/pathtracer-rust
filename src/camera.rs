extern crate nalgebra;
use std::f32::{consts::PI, INFINITY};

use nalgebra::{Matrix3, Vector3};
use rand::Rng;
use serde::Deserialize;

use crate::ray::Ray;

fn default_canvas_fov_x() -> f32 {
    return 130.0;
}

fn default_near_clipping_range() -> f32 {
    return 0.5;
}

fn default_far_clipping_range() -> f32 {
    return INFINITY;
}

fn default_canvas_width() -> u32 {
    return 1280;
}

fn default_canvas_height() -> u32 {
    return 720;
}

fn default_camera_right() -> Vector3<f32> {
    return Vector3::new(1., 0., 0.);
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Camera {
    pub origin: Vector3<f32>,
    pub up: Vector3<f32>,
    pub forward: Vector3<f32>,
    #[serde(default = "default_camera_right")]
    pub right: Vector3<f32>,
    #[serde(default = "default_canvas_fov_x")]
    pub fov_x_deg: f32,
    #[serde(default = "default_near_clipping_range")]
    pub near_clipping_range: f32,
    #[serde(default = "default_far_clipping_range")]
    pub far_clipping_range: f32,
    #[serde(default = "default_canvas_width")]
    pub canvas_width: u32,
    #[serde(default = "default_canvas_height")]
    pub canvas_height: u32,
}

impl Camera {
    pub fn aspect_ratio(&self) -> f32 {
        return self.canvas_width as f32 / self.canvas_height as f32;
    }
    pub fn viewport_width(&self) -> f32 {
        let fov_x_rad = ((self.fov_x_deg * 0.5) / 180.0) * PI; // radian
        return 2.0 * self.near_clipping_range * fov_x_rad.tan();
    }
    pub fn viewport_height(&self) -> f32 {
        return self.viewport_width() / self.aspect_ratio();
    }
    pub fn top_left_start(&self) -> Vector3<f32> {
        return self.origin + (self.forward * self.near_clipping_range)
            - ((self.viewport_width() / 2.0) * self.right)
            + ((self.viewport_height() / 2.0) * self.up);
    }

    #[allow(dead_code)]
    pub fn rotate_around_up(&mut self, angle_degree: f32) -> () {
        let angle_rad = (angle_degree / 180.0) * PI;

        let rotation_mat = Matrix3::new(
            angle_rad.cos(),
            0.0,
            (-angle_rad).sin(),
            0.0,
            1.0,
            0.0,
            angle_rad.sin(),
            0.0,
            angle_rad.cos(),
        );

        self.right = (rotation_mat * self.right).normalize();
        self.forward = (rotation_mat * self.forward).normalize();
    }

    #[allow(dead_code)]
    pub fn rotate_around_forward(&mut self, angle_degree: f32) -> () {
        let angle_rad = (angle_degree / 180.0) * PI;

        let rotation_mat = Matrix3::new(
            angle_rad.cos(),
            angle_rad.sin(),
            0.0,
            (-angle_rad).sin(),
            angle_rad.cos(),
            0.0,
            0.0,
            0.0,
            1.0,
        );

        self.up = (rotation_mat * self.up).normalize();
        self.right = (rotation_mat * self.right).normalize();
    }

    #[allow(dead_code)]
    pub fn rotate_around_right(&mut self, angle_degree: f32) -> () {
        let angle_rad = (angle_degree / 180.0) * PI;

        let rotation_mat = Matrix3::new(
            1.0,
            0.0,
            0.0,
            0.0,
            angle_rad.cos(),
            angle_rad.sin(),
            0.0,
            (-angle_rad).sin(),
            angle_rad.cos(),
        );

        self.up = (rotation_mat * self.up).normalize();
        self.forward = (rotation_mat * self.forward).normalize();
    }

    pub fn create_ray(&self, x: usize, y: usize) -> Ray {
        let mut rng = rand::thread_rng();
        let dx = rng.gen::<f32>();
        let dy = rng.gen::<f32>();

        let u = ((x as f32 + dx) * self.viewport_width()) / (self.canvas_width - 1) as f32;
        let v = ((y as f32 + dy) * self.viewport_height()) / (self.canvas_height - 1) as f32;
        let target = self.top_left_start() + u * self.right - v * self.up;

        let ray = Ray::new(self.origin, (target - self.origin).normalize());
        ray
    }
}
