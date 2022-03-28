use image::png::PNGEncoder;
use image::ColorType;
use nalgebra::Vector3;
use serde::{de::Error, Deserialize};
use std::{
    collections::HashMap,
    f32::INFINITY,
    fs::{self, File},
    path::Path,
    rc::Rc,
};
// use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

use crate::{
    camera::Camera,
    light::{self, PointLight},
    mesh::Mesh,
    objects::{Plane, Sphere, Triangle},
};

use {crate::ray::*, crate::scene::*};

#[derive(Debug, Deserialize)]
pub struct Engine {
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    #[serde(default = "Vec::new")]
    pub spheres: Vec<Sphere>,
    #[serde(default = "Vec::new")]
    pub triangles: Vec<Triangle>,
    #[serde(default = "Vec::new")]
    pub planes: Vec<Plane>,
    #[serde(default = "Vec::new")]
    pub meshes: Vec<Mesh>,
}

impl Engine {
    pub fn init_scene(&mut self) -> Result<Scene, std::io::Error> {
        // Init camera
        self.camera.forward = Some((self.camera.target - self.camera.origin).normalize());
        self.camera.right = Some(self.camera.up().cross(&self.camera.forward()));

        let mut scene = Scene::new(
            self.camera,
            self.camera.canvas_width as usize,
            self.camera.canvas_height as usize,
        );

        for sphere in &self.spheres {
            scene.add_object(Rc::new(sphere.clone()));
        }

        for triangle in &self.triangles {
            scene.add_object(Rc::new(triangle.clone()));
        }

        for mesh in self.meshes.iter_mut() {
            mesh.convert_to_triangles(&mut scene);
        }

        for plane in &self.planes {
            scene.add_object(Rc::new(plane.clone()));
        }

        for light in &self.lights {
            scene.add_light(light.clone());
        }

        return Ok(scene);
    }

    // pub fn render_blobs(&self, scene: &Scene) -> () {
    //     for blob in scene.blobs {
    //         let triangles =  blob.marching_cube();
    //         for triangle in triangles {
    //             scene.add_object(triangle);
    //         }
    //     }
    // }

    pub fn render_scene(&self, scene: &Scene) -> Vec<u8> {
        let mut pixels = vec![0; scene.canvas_width * scene.canvas_height * 3];

        for j in 0..scene.canvas_height {
            for i in 0..scene.canvas_width {
                let u =
                    (i as f32 * scene.camera.viewport_width()) / (scene.canvas_width - 1) as f32;
                let v =
                    (j as f32 * scene.camera.viewport_height()) / (scene.canvas_height - 1) as f32;

                let target =
                    scene.camera.top_left_start() + u * scene.camera.right() - v * scene.camera.up;
                let ray = Ray::new(
                    scene.camera.origin,
                    (target - scene.camera.origin).normalize(),
                );

                if let (intersect_point, Some(min_obj)) = scene.cast_ray(
                    &ray,
                    scene.camera.near_clipping_range,
                    scene.camera.far_clipping_range,
                ) {
                    let pixel_color = scene.get_color_ray(&intersect_point, &min_obj, &ray, 0);

                    let offset = j * scene.canvas_width + i;
                    pixels[offset * 3] = (255.0 * pixel_color.x) as u8;
                    pixels[offset * 3 + 1] = (255.0 * pixel_color.y) as u8;
                    pixels[offset * 3 + 2] = (255.0 * pixel_color.z) as u8;
                }
            }
        }

        return pixels;
    }

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

    pub fn save_scene(
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
}
