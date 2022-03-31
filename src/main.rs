use engine::Engine;
use serde_yaml;
use std::error::Error;
use std::{env, fs::File};

mod camera;
mod engine;
mod light;
mod mesh;
mod objects;
mod ray;
mod scene;
mod texture_material;

use {crate::ray::*, crate::scene::*};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 4);

    let is_pathtracer = args[1] == "pathtracer";
    let file_arg = args[2].clone();
    let cpu = args[3].parse().unwrap();
    
    let file = File::open(&file_arg)?;
    let scene: Scene = serde_yaml::from_reader(file)?;

    let engine = Engine::from_scene(&scene);
    let width = engine.canvas_width;
    let height = engine.canvas_height;

    let pixels = engine.render(cpu, is_pathtracer);
    Engine::save("output.png", &pixels, width, height).expect("error writing image");

    return Ok(());
}
