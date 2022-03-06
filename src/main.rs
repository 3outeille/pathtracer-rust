#![allow(unused_imports, unused_variables)]

use std::{rc::Rc, f32::INFINITY, path::Path, fs::File};
use nalgebra::Vector3;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod objects;
mod light;
mod scene;
mod camera;
mod texture_material;
mod ray;
mod engine;

use { crate::objects::*, crate::light::*, crate::scene::*, crate::camera::*, crate::texture_material::*, crate::ray::*, crate::engine::*};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let canvas_width = 1280_usize;
    let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;

    // Parse JSON
    if let Ok(scene) = Engine.parse_scene("path/to/json") {
        let pixels = Engine.render_scene(scene);
        Engine.save_scene("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
    }

    // GUI
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();

}