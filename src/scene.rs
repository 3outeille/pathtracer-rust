use core::panic;
use std::rc::Rc;
use std::cmp::max;

use nalgebra::Vector3;

use crate::{camera::Camera, objects::{ObjectsTrait, self}, light::{PointLight, self}, Ray};

const EPSILON: f32 = 1e-4;
const REFLECTION_DEPTH: i32 = 5;

pub struct Scene {
    pub camera: Camera,
    // pub blobs: Vec<Blob>,
    pub objects: Vec<Rc<dyn ObjectsTrait>>,
    pub lights: Vec<PointLight>,
    pub canvas_width: usize,
    pub canvas_height: usize
}

impl Scene {
    
    pub fn new(camera: Camera, canvas_width: usize, canvas_height: usize) -> Self {
        Self {
            camera,
            // blobs: Vec::new(),
            objects: Vec::new(), 
            lights: Vec::new(),
            canvas_width,
            canvas_height
        }
    }

    // pub fn add_blob(&mut self, blob: Blob) -> () {
    //     self.blobs.push(mesh);   
    // }

    pub fn add_object(&mut self, object: Rc<dyn ObjectsTrait>) -> () {
        self.objects.push(object);   
    }

    pub fn add_light(&mut self, light: PointLight) -> () {
        self.lights.push(light)
    }

    pub fn cast_ray(&self, ray: &Ray, near_clipping_range: f32, far_clipping_range: f32) -> (Option<Vector3<f32>>, Option<Rc<dyn ObjectsTrait>>) {
        let mut min_t = std::f32::MAX;
        let mut min_obj: Option<Rc<dyn ObjectsTrait>>= None;

        for object in &self.objects {

            // Find the nearest root.
            match object.intersects(&ray, near_clipping_range, far_clipping_range) {
                Some(t) if (t < min_t) =>  {
                    min_obj = Some(object.clone());
                    min_t = t;
                }
                _ => { continue; }
            };
        }

        let intersection_point = if min_t != std::f32::MAX { Some(ray.at(min_t)) } else { None };

        return (intersection_point, min_obj);
    }

    pub fn get_color_ray(&self, intersection_point: &Option<Vector3<f32>>, obj: &Rc<dyn ObjectsTrait>, ray: &Ray, depth: i32) -> Vector3<f32> {
        
        let mut pixel_color = Vector3::<f32>::zeros();

        if intersection_point.is_none() {
            return pixel_color;
        }

        let (ka, kd, ks, ns, kr, material_color) = obj.get_texture();
        let normal = obj.get_normal(&intersection_point.unwrap()).normalize();
        let reflection = (ray.direction - (2.0 * ray.direction.dot(&normal) * normal)).normalize();

        // Phong Model
        let ambient = material_color;
        let mut diffuse = Vector3::<f32>::zeros();
        let mut specular = Vector3::<f32>::zeros();

        for light in &self.lights {

            let light_dir = (light.position - intersection_point.unwrap()).normalize();

            diffuse += {
                let dot_prod = light_dir.dot(&normal).clamp(0.0, 1.0);
                material_color * light.intensity * dot_prod
            };

            specular = specular.add_scalar({
                let dot_prod = light_dir.dot(&reflection).clamp(0.0, 1.0).powf(ns);
                light.intensity * dot_prod
            });
            
            let shadow_ray = Ray::new(intersection_point.unwrap() + (normal * EPSILON), light_dir);

            if let (Some(intersect_point_towards_light), new_obj) = self.cast_ray(&shadow_ray, self.camera.near_clipping_range, self.camera.far_clipping_range) {
                
                // Hit object towards light must be in-between the intial intersection point and the light.
                if (intersect_point_towards_light - intersection_point.unwrap()).magnitude_squared() < (light.position - intersection_point.unwrap()).magnitude_squared() {
                    return pixel_color + (ka * ambient);
                }
            }
        }

        pixel_color += (ka * ambient) + (kd * diffuse) + (ks * specular);

        if depth >= REFLECTION_DEPTH {
            return pixel_color;
        }

        // When casting rays using previous intersection point, ray may hit under the surface
        // due to numerical precision of the intersection point calculation (discriminant).
        // The more rays are casted using previous intersection point, the more the error accumulate.
        let reflected_ray = Ray::new(intersection_point.unwrap() + (normal * EPSILON), reflection);
        let (reflected_intersection_point, new_obj) = self.cast_ray(&reflected_ray, self.camera.near_clipping_range, self.camera.far_clipping_range);
        
        if new_obj.is_none() { 
            return pixel_color; 
        }

        let reflection = self.get_color_ray(&reflected_intersection_point, &new_obj.unwrap(), &reflected_ray, depth + 1);

        pixel_color += kr * reflection;

        return pixel_color; 
    }
}