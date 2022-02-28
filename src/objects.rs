extern crate nalgebra;

use nalgebra::Vector3;
use dyn_clone::{clone_trait_object, DynClone};

use { crate::texture_material::TextureMaterial, crate::ray::Ray };

pub trait ObjectsTrait: DynClone {
    fn intersects(&self, ray: &Ray) -> f32;

    fn get_normal(&self, point: Vector3<f32>) -> Vector3<f32>;

    fn get_texture(&self, point: Vector3<f32>) -> Vector3<f32>;
}

clone_trait_object!(ObjectsTrait);

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub textmat: Box<dyn TextureMaterial>
}

impl ObjectsTrait for Sphere {

    fn intersects(&self, ray: &Ray) -> f32 {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - 4. * a * c;

        if discriminant < 0.0 {
            return -1.0;
        }
        
        let root = -b - discriminant.sqrt() / (2.0 * a);

        if root < 0.0 {
            return -1.0;
        }

        return root;
    }

    fn get_normal(&self, point: Vector3<f32>) -> Vector3<f32> {
        todo!()
    }

    fn get_texture(&self, point: Vector3<f32>) -> Vector3<f32> {
        todo!()
    }
}