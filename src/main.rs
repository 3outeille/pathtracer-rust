#![allow(unused_imports, unused_variables)]

use nalgebra::Vector3;
use serde_yaml;
use std::error::Error;
use std::{collections::HashMap, env, f32::INFINITY, fs::File, path::Path, rc::Rc};

mod camera;
mod engine;
mod light;
mod mesh;
mod objects;
mod ray;
mod scene;
mod texture_material;

use {
    crate::camera::*, crate::engine::*, crate::light::*, crate::mesh::*, crate::objects::*,
    crate::ray::*, crate::scene::*, crate::texture_material::*,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let mut engine: Engine = serde_yaml::from_reader(file)?;

    if let Ok(scene) = engine.init_scene() {
        let pixels = engine.render_scene(&scene);
        engine
            .save_scene(
                "output.png",
                &pixels,
                &scene.canvas_width,
                &scene.canvas_height,
            )
            .expect("error writing image");
    }

    return Ok(());
}
