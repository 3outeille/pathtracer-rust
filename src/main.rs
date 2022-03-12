#![allow(unused_imports, unused_variables)]

use std::{rc::Rc, f32::INFINITY, path::Path, fs::File, env, collections::HashMap};
use nalgebra::Vector3;
use std::error::Error;
use serde_yaml;

mod objects;
mod light;
mod scene;
mod camera;
mod texture_material;
mod ray;
mod engine;
mod mesh;

use { crate::objects::*, crate::light::*, crate::scene::*, crate::camera::*, crate::texture_material::*, crate::ray::*, crate::engine::*, crate::mesh::*};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let mut engine: Engine = serde_yaml::from_reader(file)?;

    if let Ok(scene) = engine.init_scene() {
        let pixels = engine.render_scene(&scene);
        engine.save_scene("output.png", &pixels, &scene.canvas_width, &scene.canvas_height).expect("error writing image");
    }

    return Ok(());
}