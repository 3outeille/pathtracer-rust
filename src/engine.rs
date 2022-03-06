
use image::ColorType;
use nalgebra::Vector3;
use std::{fs::File, path::Path, f32::INFINITY, rc::Rc};
use image::png::PNGEncoder;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

use crate::{camera::Camera, texture_material::UniformTexture, objects::{Sphere, Plane}, light::PointLight};

use {crate::scene::*, crate::ray::*};


#[derive(Serialize, Deserialize)]
struct SomeDataType {}

pub struct Engine;

impl Engine {
    
    pub fn parse_scene(&self, filename: &str) -> Result<Scene, std::io::Error> {
        let json_file_path = Path::new(filename);
        let json_file = File::open(json_file_path).expect("file not found");
        let deserialized_camera: SomeDataType =
            serde_json::from_reader(json_file).expect("error while reading json");
        
        let aspect_ratio = 16.0 / 9.0;
        let up = Vector3::new(0.0, 1.0, 0.0);
        let near_clipping_range = 0.5;
        let far_clipping_range = INFINITY;    
        let canvas_width = 1280_usize;
        let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;
    
        let camera = Camera::new(
            Vector3::new(1.5, -0.1, -0.5),
            Vector3::new(0.5, 0.0, 1.0),
            up,
            130.0,
            near_clipping_range,
            far_clipping_range,
            aspect_ratio
        );
    
        println!("{}x{}", canvas_width, canvas_height);
        
        let mut scene = Scene::new(camera, canvas_width, canvas_height);

        let red  = Rc::new(
            UniformTexture::new(
                1.0,
                1.5,
                1.0,
                15.0,
                0.3,
                Vector3::new(0.3, 0.1, 0.1)
            )
        );
        
        let green  = Rc::new(
            UniformTexture::new(
                1.0,
                1.5,
                1.0,
                15.0,
                0.3,
                Vector3::new(0.1, 0.3, 0.1)
            )
        );

        let blue = Rc::new(
            UniformTexture::new(
                1.0,
                1.0,
                1.0,
                15.0,
                0.3,
                Vector3::new(0.3, 0.3, 0.8)
            )
        );

        // Ground
        // scene.add_object(
        //     Rc::new(Plane {
        //         center: Vector3::new(0.0, -6.0, 10.0),
        //         normal: Vector3::new(0.0, 1.0, 0.0),
        //         textmat: blue.clone()
        //     })
        // );

        scene.add_object(
            Rc::new(Sphere {
                center: Vector3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                textmat: blue.clone()
            })
        );

        // Right-Background
        scene.add_object(
            Rc::new(Plane {
                center: Vector3::new(0.0, 0.0, 50.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
                textmat: blue.clone()
            })
        );

        // Left-Background
        scene.add_object(
            Rc::new(Plane {
                center: Vector3::new(-50.0, 0.0, 50.0),
                normal: Vector3::new(1.0, 0.0, 0.0),
                textmat: blue.clone()
            })
        );


        scene.add_object(
            Rc::new(Sphere {
                center: Vector3::new(0.0, 0.0, 1.0),
                radius: 0.5,
                textmat: red.clone()
            })
        );

        scene.add_object(
            Rc::new(Sphere {
                center: Vector3::new(1.0, 0.0, 1.0),
                radius: 0.5,
                textmat: green.clone()
            })
        );

        scene.add_light(
            PointLight::new(
                Vector3::new(1.0, 1.0, 0.5),
                0.9
            )
        );
        return Ok(scene);
    }

    pub fn render_scene(&self, scene: Scene) -> Vec<u8> {
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
        let app = app::App::default();
        let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
        let mut frame = Frame::new(0, 0, 400, 200, "");
        let mut but = Button::new(160, 210, 80, 40, "Click me!");
        wind.end();
        wind.show();
        but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
        app.run().unwrap();
    }

    pub fn save_scene(&self, filename: &str, pixels: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
        return Ok(());
    }
}