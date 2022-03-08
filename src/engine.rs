
use image::ColorType;
use nalgebra::Vector3;
use serde_json::Value;
use std::{fs::{File, self}, path::Path, f32::INFINITY, rc::Rc, collections::HashMap};
use image::png::PNGEncoder;
// use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

use crate::{camera::Camera, texture_material::UniformTexture, objects::{Sphere, Plane, Triangle}, light::PointLight};

use {crate::scene::*, crate::ray::*};

pub struct Engine;

impl Engine {
    
    pub fn parse_scene(&self, filename: &str) -> Result<Scene, std::io::Error> {
        // TODO: clean code to make it more rustacean.

        let data = fs::read_to_string(filename).expect("Unable to read file");
        let res: Value = serde_json::from_str(&data).expect("Unable to parse");

        let origin = Vector3::new(
            res["camera"]["origin"][0].as_f64().unwrap() as f32,
            res["camera"]["origin"][1].as_f64().unwrap() as f32,
            res["camera"]["origin"][2].as_f64().unwrap() as f32
        );
        
        let target = Vector3::new(
            res["camera"]["target"][0].as_f64().unwrap() as f32,
            res["camera"]["target"][1].as_f64().unwrap() as f32,
            res["camera"]["target"][2].as_f64().unwrap() as f32
        );

        let up = Vector3::new(
            res["camera"]["up"][0].as_f64().unwrap() as f32,
            res["camera"]["up"][1].as_f64().unwrap() as f32,
            res["camera"]["up"][2].as_f64().unwrap() as f32
        );

        let fov_x: f32;
        if let Some(field) = res["camera"].get("fov_x") {
            fov_x = field.as_f64().unwrap() as f32;
        } else {
            fov_x = 90.0;
        }
        
        let near_clipping_range: f32;
        if let Some(field) = res["camera"].get("near_clipping_range") {
            near_clipping_range = field.as_f64().unwrap() as f32;
        } else {
            near_clipping_range = 1.0;
        }
        
        let far_clipping_range: f32;
        if let Some(field) = res["camera"].get("far_clipping_range") {
            far_clipping_range = field.as_f64().unwrap() as f32;
        } else {
            far_clipping_range = INFINITY;
        }

        let aspect_ratio = (res["camera"]["aspect_ratio_num"].as_f64().unwrap() / res["camera"]["aspect_ratio_den"].as_f64().unwrap()) as f32;

        let canvas_width = res["camera"]["canvas_width"].as_f64().unwrap() as usize;
        let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;

        let camera = Camera::new(
            origin,
            target,
            up,
            fov_x,
            near_clipping_range,
            far_clipping_range,
            aspect_ratio
        );
    
        println!("{}x{}", canvas_width, canvas_height);
        
        let mut scene = Scene::new(camera, canvas_width, canvas_height);
        
        // Texture
        let mut textures_map: HashMap<String, Rc<UniformTexture>> = HashMap::new();

        for v in res["textures"].as_array().unwrap() {
            let texture = v.as_object().unwrap();
            textures_map.insert(
                texture["metadata"]["type"].as_str().unwrap().to_owned(),
                Rc::new(
                    UniformTexture::new(
                        texture["ka"].as_f64().unwrap() as f32,
                        texture["kd"].as_f64().unwrap() as f32,
                        texture["ks"].as_f64().unwrap() as f32,
                        texture["ns"].as_f64().unwrap() as f32,
                        texture["kr"].as_f64().unwrap() as f32,
                        Vector3::new(
                            texture["color"][0].as_f64().unwrap() as f32,
                            texture["color"][1].as_f64().unwrap() as f32,
                            texture["color"][2].as_f64().unwrap() as f32)
                    )
                )   
            );
        }

        // Objects
        for v in res["objects"].as_array().unwrap() {
            let object = v.as_object().unwrap();
            if object["metadata"].as_object().unwrap().contains_key("type") {
                if object["metadata"]["type"].as_str().unwrap().eq("sphere") {
                    scene.add_object(
                        Rc::new(Sphere {
                            center: Vector3::new(
                                object["center"][0].as_f64().unwrap() as f32,
                                object["center"][1].as_f64().unwrap() as f32,
                                object["center"][2].as_f64().unwrap() as f32
                            ),
                            radius: object["radius"].as_f64().unwrap() as f32,
                            textmat: textures_map[object["textmat"].as_str().unwrap()].clone()
                        })
                    );
                } else if object["metadata"]["type"].as_str().unwrap().eq("plane") {
                    scene.add_object(
                        Rc::new(Plane {
                            center: Vector3::new(
                                object["center"][0].as_f64().unwrap() as f32,
                                object["center"][1].as_f64().unwrap() as f32,
                                object["center"][2].as_f64().unwrap() as f32
                            ),
                            normal: Vector3::new(
                                object["normal"][0].as_f64().unwrap() as f32,
                                object["normal"][1].as_f64().unwrap() as f32,
                                object["normal"][2].as_f64().unwrap() as f32
                            ),
                            textmat: textures_map[object["textmat"].as_str().unwrap()].clone()
                        })
                    );
                } else if object["metadata"]["type"].as_str().unwrap().eq("triangle") {
                    scene.add_object(
                        Rc::new(Triangle {
                            v0: Vector3::new(
                                object["v0"][0].as_f64().unwrap() as f32,
                                object["v0"][1].as_f64().unwrap() as f32,
                                object["v0"][2].as_f64().unwrap() as f32
                            ),
                            v1: Vector3::new(
                                object["v1"][0].as_f64().unwrap() as f32,
                                object["v1"][1].as_f64().unwrap() as f32,
                                object["v1"][2].as_f64().unwrap() as f32
                            ),
                            v2: Vector3::new(
                                object["v2"][0].as_f64().unwrap() as f32,
                                object["v2"][1].as_f64().unwrap() as f32,
                                object["v2"][2].as_f64().unwrap() as f32
                            ),
                            textmat: textures_map[object["textmat"].as_str().unwrap()].clone()
                        })
                    );
                }
            }
        }

        // Lights
        for v in res["lights"].as_array().unwrap() {
            let light = v.as_object().unwrap();
            scene.add_light(PointLight::new(
                Vector3::new(
                    light["position"][0].as_f64().unwrap() as f32,
                    light["position"][1].as_f64().unwrap() as f32,
                    light["position"][2].as_f64().unwrap() as f32
                ),
                light["intensity"].as_f64().unwrap() as f32)       
            );
        }

        return Ok(scene);
    }

    pub fn render_blobs(&self, scene: &Scene) -> () {
        for mesh in scene.meshes {
            for triangle in mesh.marching_cube() {
                scene.add_object(triangle);
            }
        }
    }

    pub fn render_scene(&self, scene: &Scene) -> Vec<u8> {
        let mut pixels = vec![0; scene.canvas_width * scene.canvas_height * 3];

        for j in 0..scene.canvas_height {
            for i in 0..scene.canvas_width {
                
                let u = (i as f32 * scene.camera.viewport_width) / (scene.canvas_width - 1) as f32;
                let v = (j as f32 * scene.camera.viewport_height) / (scene.canvas_height - 1) as f32;
                
                let target = scene.camera.top_left_start + u * scene.camera.right - v * scene.camera.up;
                let ray = Ray::new(scene.camera.origin, (target - scene.camera.origin).normalize());

                if let (intersect_point, Some(min_obj)) = scene.cast_ray(&ray, scene.camera.near_clipping_range, scene.camera.far_clipping_range) {

                    let pixel_color = scene.get_color_ray(&intersect_point, &min_obj, &ray,0);     
                    
                    let offset = j * scene.canvas_width + i;
                    pixels[offset * 3] = (255.0 * pixel_color.x)  as u8;
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

    pub fn save_scene(&self, filename: &str, pixels: &[u8], width: &usize, height: &usize) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, *width as u32, *height as u32, ColorType::RGB(8))?;
        return Ok(());
    }
}