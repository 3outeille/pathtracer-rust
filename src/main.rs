#![allow(unused_imports, unused_variables)]

use std::{rc::Rc, f32::INFINITY, path::Path, fs::{self}, env, collections::HashMap};
use nalgebra::Vector3;
use serde_json::Value;

mod objects;
mod light;
mod scene;
mod camera;
mod texture_material;
mod ray;
mod engine;

use { crate::objects::*, crate::light::*, crate::scene::*, crate::camera::*, crate::texture_material::*, crate::ray::*, crate::engine::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse JSON scene
    if let Ok(scene) = Engine.parse_scene(&args[1]) {
        let pixels = Engine.render_scene(&scene);
        Engine.save_scene("output.png", &pixels, &scene.canvas_width, &scene.canvas_height).expect("error writing image");
    }
}