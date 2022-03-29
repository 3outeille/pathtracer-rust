use core::panic;
use std::rc::Rc;
use std::{cmp::max, fs::File};

use image::png::PNGEncoder;
use image::ColorType;
use nalgebra::Vector3;

use crate::scene::Scene;
use crate::{
    camera::Camera,
    light::{self, PointLight},
    objects::{self, ObjectsTrait},
    Ray,
};

const EPSILON: f32 = 1e-3;
const REFLECTION_DEPTH: i32 = 5;

pub struct Engine {
    pub camera: Camera,
    // pub blobs: Vec<Blob>,
    pub objects: Vec<Rc<dyn ObjectsTrait>>,
    pub lights: Vec<PointLight>,
    pub canvas_width: usize,
    pub canvas_height: usize,
}

impl Engine {
    pub fn new(camera: Camera, canvas_width: usize, canvas_height: usize) -> Self {
        Self {
            camera,
            // blobs: Vec::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            canvas_width,
            canvas_height,
        }
    }

    pub fn from_scene(scene: &Scene) -> Self {
        let mut camera = scene.camera.clone();

        // Init camera
        camera.up = camera.up.normalize();
        camera.forward = camera.forward.normalize();
        camera.right = scene.camera.up.cross(&camera.forward);

        let mut engine = Engine::new(
            camera,
            scene.camera.canvas_width as usize,
            scene.camera.canvas_height as usize,
        );

        for sphere in &scene.spheres {
            engine.add_object(Rc::new(sphere.clone()));
        }

        for triangle in &scene.triangles {
            engine.add_object(Rc::new(triangle.clone()));
        }

        for mesh in &scene.meshes {
            mesh.convert_to_triangles(&mut engine);
        }

        for plane in &scene.planes {
            engine.add_object(Rc::new(plane.clone()));
        }

        for light in &scene.lights {
            engine.add_light(light.clone());
        }

        return engine;
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

    pub fn render(&self) -> Vec<u8> {
        let mut pixels = vec![0; self.canvas_width * self.canvas_height * 3];

        for j in 0..self.canvas_height {
            for i in 0..self.canvas_width {
                let u = (i as f32 * self.camera.viewport_width()) / (self.canvas_width - 1) as f32;
                let v =
                    (j as f32 * self.camera.viewport_height()) / (self.canvas_height - 1) as f32;

                let target =
                    self.camera.top_left_start() + u * self.camera.right - v * self.camera.up;
                let ray = Ray::new(
                    self.camera.origin,
                    (target - self.camera.origin).normalize(),
                );

                let cast_result = self.cast_ray(
                    &ray,
                    self.camera.near_clipping_range,
                    self.camera.far_clipping_range,
                );
                let pixel_color = self.get_color_ray(cast_result, &ray, 0);
                let offset = j * self.canvas_width + i;

                pixels[offset * 3] = (255.0 * pixel_color.x) as u8;
                pixels[offset * 3 + 1] = (255.0 * pixel_color.y) as u8;
                pixels[offset * 3 + 2] = (255.0 * pixel_color.z) as u8;
            }
        }

        return pixels;
    }

    pub fn save(
        &self,
        filename: &str,
        pixels: &[u8],
        width: &usize,
        height: &usize,
    ) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, *width as u32, *height as u32, ColorType::RGB(8))?;
        return Ok(());
    }

    // pub fn render_blobs(&self, scene: &Scene) -> () {
    //     for blob in scene.blobs {
    //         let triangles =  blob.marching_cube();
    //         for triangle in triangles {
    //             scene.add_object(triangle);
    //         }
    //     }
    // }

    pub fn render_scene_realtime(&self) {
        // fltk = "1.2"

        // let app = app::App::default();
        // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
        // let mut frame = Frame::new(0, 0, 400, 200, "");
        // let mut but = Button::new(160, 210, 80, 40, "Click me!");
        // wind.end();
        // wind.show();
        // but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
        // app.run().unwrap();
        todo!()
    }

    pub fn cast_ray(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<(Vector3<f32>, Rc<dyn ObjectsTrait>)> {
        let mut min_t = std::f32::MAX;
        let mut min_obj: Option<Rc<dyn ObjectsTrait>> = None;

        for object in &self.objects {
            // Find the nearest root.
            match object.intersects(&ray, near_clipping_range, far_clipping_range) {
                Some(t) if (t < min_t) => {
                    min_obj = Some(object.clone());
                    min_t = t;
                }
                _ => {
                    continue;
                }
            };
        }

        if let Some(obj) = min_obj {
            Some((ray.at(min_t), obj))
        } else {
            None
        }
    }

    pub fn get_color_ray(
        &self,
        cast_result: Option<(Vector3<f32>, Rc<dyn ObjectsTrait>)>,
        ray: &Ray,
        depth: i32,
    ) -> Vector3<f32> {
        if cast_result.is_none() {
            return Vector3::zeros();
        }

        let (intersection_point, obj) = cast_result.unwrap();

        let (ka, kd, ks, ns, kr, material_color) = obj.get_texture();
        let normal = obj.get_normal(&intersection_point);
        let reflected_dir =
            (ray.direction - (2.0 * ray.direction.dot(&normal) * normal)).normalize();

        // Phong Model
        let ambient = material_color * 0.2;
        let mut diffuse = Vector3::zeros();
        let mut specular = Vector3::zeros();

        for light in &self.lights {
            let light_vec = light.position - intersection_point;
            let light_dir = light_vec.normalize();
            let light_distance = light_vec.norm();
            let light_value = light.intensity * light.color;

            let shadow_ray = Ray::new(intersection_point, light_dir + normal * EPSILON);

            if self
                .cast_ray(&shadow_ray, EPSILON, light_distance)
                .is_some()
            {
                continue;
            }

            diffuse += {
                let dot_prod = light_dir.dot(&normal).clamp(0.0, 1.0);
                material_color.component_mul(&light_value) * dot_prod
            };

            specular += {
                let dot_prod = light_dir.dot(&reflected_dir).clamp(0.0, 1.0).powf(ns);
                light_value * dot_prod
            };
        }

        if depth >= REFLECTION_DEPTH {
            return Vector3::zeros();
        }

        let reflection = {
            // When casting rays using previous intersection point, ray may hit under the surface
            // due to numerical precision of the intersection point calculation (discriminant).
            // The more rays are casted using previous intersection point, the more the error accumulate.
            let reflected_ray = Ray::new(intersection_point + (normal * EPSILON), reflected_dir);

            let cast_result = self.cast_ray(
                &reflected_ray,
                self.camera.near_clipping_range,
                self.camera.far_clipping_range,
            );

            self.get_color_ray(cast_result, &reflected_ray, depth + 1)
        };

        return (ka * ambient) + (kd * diffuse) + (ks * specular) + (kr * reflection);
    }
}
