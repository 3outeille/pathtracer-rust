extern crate nalgebra;
use nalgebra::Vector3;

use { crate::texture_material::TextureMaterial, crate::ray::Ray };


pub trait ObjectsTrait {
    fn intersects(&self, ray: &Ray) -> bool;

    fn get_normal(&self, point: Vector3<f32>) -> Vector3<f32>;

    fn get_texture(&self, point: Vector3<f32>) -> Vector3<f32>;
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub textmat: Box<dyn TextureMaterial>
}

impl ObjectsTrait for Sphere {

    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - 4. * a * c;

        return discriminant >= 0.;
    }

    fn get_normal(&self, point: Vector3<f32>) -> Vector3<f32> {
        todo!()
    }

    fn get_texture(&self, point: Vector3<f32>) -> Vector3<f32> {
        todo!()
    }
}