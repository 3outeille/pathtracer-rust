extern crate nalgebra;
use nalgebra::{Point3};

use crate::texture_material::TextureMaterial;

pub trait ObjectsTrait {
    // fn intersects(&self, ray: Ray) -> bool;

    fn get_normal(&self, point: Point3<f32>) -> Point3<f32>;

    fn get_texture(&self, point: Point3<f32>) -> Point3<f32>;
}

pub struct Sphere {
    pub textmat: dyn TextureMaterial
}

impl ObjectsTrait for Sphere {
    // fn intersects(&self, ray: Ray) -> bool {
    //     todo!()
    // }

    fn get_normal(&self, point: Point3<f32>) -> Point3<f32> {
        todo!()
    }

    fn get_texture(&self, point: Point3<f32>) -> Point3<f32> {
        todo!()
    }
}