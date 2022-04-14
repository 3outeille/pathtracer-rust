extern crate nalgebra;

use std::mem::swap;

use nalgebra::Vector3;
use serde::Deserialize;

use {crate::ray::Ray, crate::texture_material::TextureMaterial};

const EPSILON: f64 = 1e-6;

pub struct HitRecord {
    pub t: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
}

impl HitRecord {
    pub fn new(t: f64, point: Vector3<f64>, normal: Vector3<f64>) -> Self {
        Self { t, point, normal }
    }
}

pub trait ObjectsTrait: Sync + Send {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<HitRecord>;

    fn get_texture(&self) -> TextureMaterial;
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Sphere {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<HitRecord> {
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

        let intersection_point = ray.at(root);
        let normal = (intersection_point - self.center).normalize();
        return Some(HitRecord::new(root, intersection_point, normal));
    }

    fn get_texture(&self) -> TextureMaterial {
        return self.textmat;
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Plane {
    pub center: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Plane {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<HitRecord> {
        let denom = (-self.normal).dot(&ray.direction);

        if denom <= EPSILON {
            return None;
        }

        let t = (self.center - ray.origin).dot(&-self.normal) / denom;

        if t < near_clipping_range || t > far_clipping_range {
            return None;
        }

        let intersection_point = ray.at(t);
        let normal = (intersection_point - self.center).normalize();
        return Some(HitRecord::new(t, intersection_point, normal));
    }

    fn get_texture(&self) -> TextureMaterial {
        return self.textmat;
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Triangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Triangle {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<HitRecord> {
        // Möller-Trumbore algorithm

        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let p = (ray.direction).cross(&v0v2);
        let det = v0v1.dot(&p) as f64;

        if det > -EPSILON && det < EPSILON {
            return None; // Ray is parallel to triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v0;
        let u = (s.dot(&p) * inv_det) as f64;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&v0v1);
        let v = (ray.direction).dot(&q) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(&q) * inv_det as f64;

        if t < near_clipping_range || t > far_clipping_range {
            return None;
        }

        let intersection_point = ray.at(t);

        let normal = -(self.v1 - self.v0).cross(&(self.v2 - self.v0)).normalize();

        return Some(HitRecord::new(t, intersection_point, normal));
    }

    fn get_texture(&self) -> TextureMaterial {
        return self.textmat;
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounds: [Vector3<f64>; 2],
    pub textmat: TextureMaterial,
}

impl ObjectsTrait for Mesh {
    fn intersects(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<HitRecord> {
        if !self.intersect_aabb(ray) {
            return None;
        }

        let mut min_t = far_clipping_range;
        let mut min_obj = None;

        for object in &self.triangles {
            // Find the nearest root.
            if let Some(record) = object.intersects(&ray, near_clipping_range, min_t) {
                min_t = record.t;
                min_obj = Some(record);
            };
        }

        return min_obj;
    }

    fn get_texture(&self) -> TextureMaterial {
        return self.textmat;
    }
}

impl Mesh {
    fn intersect_aabb(&self, ray: &Ray) -> bool {
        let mut tmin = (self.bounds[0].x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.bounds[1].x - ray.origin.x) / ray.direction.x;
        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }
        let mut tymin = (self.bounds[0].y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.bounds[1].y - ray.origin.y) / ray.direction.y;
        if tymin > tymax {
            swap(&mut tymin, &mut tymax);
        }
        if tmin > tymax || tymin > tmax {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let mut tzmin = (self.bounds[0].z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.bounds[1].z - ray.origin.z) / ray.direction.z;
        if tzmin > tzmax {
            swap(&mut tzmin, &mut tzmax);
        }
        if tmin > tzmax || tzmin > tmax {
            return false;
        }
        true
    }
}
