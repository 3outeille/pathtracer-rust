#![allow(unused_imports, unused_variables)]

extern crate nalgebra;
use nalgebra::{Point3, Vector3};

mod objects;
use crate::objects::*;

mod light;
use crate::light::*;

mod utils;
use crate::utils::*;

mod scene;
use crate::scene::*;

mod camera;
use crate::camera::*;

mod texture_material;
use crate::texture_material::*;

fn main() {

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.0,
        0.,
        0.0,
    );

    let scene = Scene::new(
        camera,
        Vec::new(),
        Vec::new()
    );

    let width = 256;
    let height = 256;

    let mut pixels = vec![0; width * height * 3];

    for y in (0..height).rev() {
        for x in 0..width {

            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.25 as f32;

            let offset = y * width + x;
            pixels[offset * 3] = (255. * r) as u8;
            pixels[offset * 3 + 1] = (255. * g) as u8;
            pixels[offset * 3 + 2] = (255. * b) as u8;
        }
    }

    write_image("output.png", &pixels, width, height).expect("error writing image");
}