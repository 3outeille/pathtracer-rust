#![allow(unused_imports, unused_variables)]

use std::{rc::Rc, f32::INFINITY, path::Path, fs::{self}, env, collections::HashMap};
use nalgebra::Vector3;
// use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

// #[macro_use]
// extern crate serde_derive;

// extern crate serde;
extern crate serde_json;
use serde_json::{Value};

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

    let args: Vec<String> = env::args().collect();

    // Parse JSON scene
    if let Ok(scene) = Engine.parse_scene(&args[1]) {
        let pixels = Engine.render_scene(scene);
        Engine.save_scene("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
    }
}