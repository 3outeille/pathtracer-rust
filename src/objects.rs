extern crate nalgebra;

use nalgebra::Vector3;
use serde::Deserialize;

use {crate::ray::Ray, crate::texture_material::TextureMaterial};

const EPSILON: f32 = 1e-6;

pub trait ObjectsTrait: Sync + Send {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<f32>;

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32>;

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>);
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Sphere {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);

        if root < near_clipping_range || root > far_clipping_range {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < near_clipping_range || root > far_clipping_range {
                return None;
            }
        }
        return Some(root);
    }

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return (point - self.center).normalize();
    }

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>) {
        return self.textmat.get_texture();
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Plane {
    pub center: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Plane {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<f32> {
        let denom = (-self.normal).dot(&ray.direction);

        if denom <= EPSILON {
            return None;
        }

        let t = (self.center - ray.origin).dot(&-self.normal) / denom;

        if t < near_clipping_range || t > far_clipping_range {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return self.normal;
    }

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>) {
        return self.textmat.get_texture();
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Triangle {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<f32> {
        // Möller-Trumbore algorithm

        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let p = (ray.direction).cross(&v0v2);
        let det = v0v1.dot(&p) as f32;

        if det > -EPSILON && det < EPSILON {
            return None; // Ray is parallel to triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v0;
        let u = (s.dot(&p) * inv_det) as f32;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&v0v1);
        let v = (ray.direction).dot(&q) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(&q) * inv_det as f32;

        if t < near_clipping_range || t > far_clipping_range {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return -(self.v1 - self.v0).cross(&(self.v2 - self.v0)).normalize();
    }

    fn get_texture(&self) -> (f32, f32, f32, f32, f32, Vector3<f32>) {
        return self.textmat.get_texture();
    }
}
