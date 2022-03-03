extern crate nalgebra;

use std::rc::Rc;

use nalgebra::Vector3;

use { crate::texture_material::TextureMaterial, crate::ray::Ray };

pub trait ObjectsTrait {
    fn intersects(&self, ray: &Ray, near_clipping_range: f32, far_clipping_range: f32) -> Option<f32>;

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32>;

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>);
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub textmat: Rc<dyn TextureMaterial>
}

impl ObjectsTrait for Sphere {

    fn intersects(&self, ray: &Ray, near_clipping_range: f32, far_clipping_range: f32) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - 4. * a * c;

        if discriminant < 0.0 { return None; }
        
        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);

        if root < near_clipping_range || root > far_clipping_range {
            root =  (-b + discriminant.sqrt()) / (2.0 * a);
            if root < near_clipping_range || root > far_clipping_range {
                return None;
            }
        }
        return Some(root);
    }

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return point - self.center;
    }

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>) {
        return self.textmat.get_texture();
    }
}
pub struct Plane {
    pub center: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub textmat: Rc<dyn TextureMaterial>
}

impl ObjectsTrait for Plane {
    
    fn intersects(&self, ray: &Ray, near_clipping_range: f32, far_clipping_range: f32) -> Option<f32> {
        
        let denom = (-self.normal).dot(&ray.direction);

        if denom <= 1e-6 {
            return None;
        }

        let t = (self.center - ray.origin).dot(&-self.normal) / denom;
        
        if t < near_clipping_range || t > far_clipping_range {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return self.normal
    }
    
    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>) {
        return self.textmat.get_texture();
    }
}