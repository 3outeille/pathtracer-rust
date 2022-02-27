use crate::{camera::Camera, objects::{ObjectsTrait, self}, light::{LightTrait, self}};

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
}