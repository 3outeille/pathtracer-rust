use core::panic;
use std::rc::Rc;
use std::cmp::max;

use nalgebra::Vector3;

use crate::{camera::Camera, objects::{ObjectsTrait, self}, light::{PointLight, self}, Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Rc<dyn ObjectsTrait>>,
    pub lights: Vec<PointLight>,
}

impl Scene {
    
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(), 
            lights: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Rc<dyn ObjectsTrait>) -> () {
        self.objects.push(object);   
    }

    pub fn add_light(&mut self, light: PointLight) -> () {
        self.lights.push(light)
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> (Vector3<f32>, Option<Rc<dyn ObjectsTrait>>) {
        let target = self.camera.top_left_start + u * self.camera.x_axis - v * self.camera.y_axis;
        let ray = Ray::new(self.camera.origin, (target - self.camera.origin).normalize());

        let mut min_t = std::f32::MAX;
        let mut min_obj: Option<Rc<dyn ObjectsTrait>>= None;

        for object in &self.objects {

            // Find the nearest root.
            let t = object.intersects(&ray);
            if t == -1.0 { continue; }
            
            if t < min_t {
                min_obj = Some(object.clone());
                min_t = t;
            }
        }

        let intersection_point = ray.at(min_t);

        return (intersection_point, min_obj);
    }

    pub fn color_ray(&self, intersection_point: Vector3<f32>, obj: &Rc<dyn ObjectsTrait>,  offset: usize, pixels: &mut Vec<u8>) -> () {
        
        let normal = obj.get_normal(intersection_point).normalize();
        let material_color = obj.get_texture(intersection_point);

        let mut diffuse_light_intensity = 0.0 as f32;

        for light in &self.lights {
            let light_dir = (intersection_point - light.position).normalize();
            let dot_prod_res = (-light_dir.dot(&normal)).clamp(0.0, 1.0);
            diffuse_light_intensity += light.intensity * dot_prod_res;
        }
        
        let pixel_color = material_color * diffuse_light_intensity;

        pixels[offset * 3] = (255.0 * pixel_color.x)  as u8;
        pixels[offset * 3 + 1] = (255.0 * pixel_color.y) as u8;
        pixels[offset * 3 + 2] = (255.0 * pixel_color.z) as u8;
    }

}