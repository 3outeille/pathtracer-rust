use nalgebra::Vector3;

use crate::{camera::Camera, objects::{ObjectsTrait, self}, light::{LightTrait, self}, Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn ObjectsTrait>>,
    pub lights: Vec<Box<dyn LightTrait>>,
}

impl Scene {
    
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(), 
            lights: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Box<dyn ObjectsTrait>) -> () {
        self.objects.push(object);   
    }

    pub fn add_light(&mut self, light: Box<dyn LightTrait>) -> () {
        self.lights.push(light)
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> (Vector3<f32>, Option<Box<dyn ObjectsTrait>>) {
        let target = self.camera.top_left_start + u * self.camera.x_axis - v * self.camera.y_axis;
        let ray = Ray::new(self.camera.origin, target - self.camera.origin);

        let mut t = 0.0 as f32;
        let mut min_t = std::f32::MAX;
        let mut min_obj: Option<Box<dyn ObjectsTrait>>= None;

        for object in self.objects.iter() {

            // Find the nearest root.
            t = object.intersects(&ray);
            if t == -1.0 { continue; }
            
            if t < min_t {
                min_obj = Some(object.clone());
                min_t = t;
            }
        }

        let intersection_point = ray.at(min_t);

        return (intersection_point, min_obj);
    }

    pub fn color_ray(&self, intersection_point: Vector3<f32>, offset: usize, pixels: &mut Vec<u8>) -> () {
        pixels[offset * 3] = 255;
        pixels[offset * 3 + 1] = 255;
        pixels[offset * 3 + 2] = 255;
    }

}