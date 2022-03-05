#![allow(unused_imports, unused_variables)]

extern crate nalgebra;
use std::{rc::Rc, f32::INFINITY};

use nalgebra::Vector3;

mod objects;
mod light;
mod utils;
mod scene;
mod camera;
mod texture_material;
mod ray;
mod engine;

use { crate::objects::*, crate::light::*, crate::scene::*, crate::camera::*, crate::texture_material::*, crate::utils::*, crate::ray::*, crate::engine::*};

fn main() {
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

    let pixels = Engine.render_scene(scene);
    Engine.save_scene("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}