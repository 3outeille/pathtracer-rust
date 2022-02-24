extern crate nalgebra;
use nalgebra::{Point3};

pub trait Objects {
    fn intersects(&self, ray: Ray) -> bool;

    fn get_normal(&self, point: Point3<f32>) -> Point3<f32>;

    fn get_texture(&self, point: Point3<f32>) -> Point3<f32>;
}