extern crate nalgebra;
use std::f32::consts::PI;

use nalgebra::{Vector3, Vector2};

use crate::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub x_axis: Vector3<f32>,
    pub y_axis: Vector3<f32>,
    pub z_axis: Vector3<f32>,
    pub fov_x: f32, // horizontal field of view
    pub top_left_start: Vector3<f32>,
}

impl Camera {
    pub fn new(origin_arg: Vector3<f32>, fov_x_arg: f32, aspect_ratio_arg: f32, focal_distance_arg: f32) -> Self {

        let fov_x = ((fov_x_arg * 0.5) / 180.0) * PI; // radian
        let viewport_width = 2.0 * fov_x.tan() * focal_distance_arg;
        let viewport_height = viewport_width / aspect_ratio_arg;

        let origin = origin_arg;
        let x_axis = Vector3::new(viewport_width, 0.0, 0.0).normalize(); // right
        let y_axis = Vector3::new(0.0, viewport_height, 0.0).normalize(); // up
        let z_axis = x_axis.cross(&y_axis).normalize();  // depth

        let top_left_start = origin - x_axis/2.0 + y_axis/2.0 + z_axis;

        Self {
            origin,
            x_axis,
            y_axis,
            z_axis,
            fov_x,
            top_left_start,
        }
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        let target = self.top_left_start + u * self.x_axis - v * self.y_axis;
        return Ray::new(self.origin, target - self.origin);
    }
}