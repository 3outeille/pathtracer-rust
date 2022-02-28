#![allow(unused_imports, unused_variables)]

extern crate nalgebra;
use nalgebra::Vector3;

mod objects;
mod light;
mod utils;
mod scene;
mod camera;
mod texture_material;
mod ray;

use { crate::objects::*, crate::light::*, crate::scene::*, crate::camera::*, crate::texture_material::*, crate::utils::*, crate::ray::* };

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let canvas_width = 400_usize;
    let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;

    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        90.0,
        aspect_ratio,
        1.0
    );

    let mut scene = Scene::new(camera);

    scene.add_object(
        Box::new(Sphere {
            center: Vector3::new(0.0, 0.0, 10.0),
            radius: 0.5,
            textmat: Box::new(UniformTexture {})
        })
    );
    
    // scene.add_object(
    //     Box::new(Sphere {
    //         center: Vector3::new(0.0, 0.0, 2.0),
    //         radius: 0.75,
    //         textmat: Box::new(UniformTexture {})
    //     })
    // );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for y in 0..canvas_height {
        for x in 0..canvas_width {
            
            let u = x as f32 / canvas_width as f32;
            let v = y as f32 / canvas_height as f32;
            
            let ray = scene.camera.cast_ray(u, v);
            scene.color_ray(&ray, y * canvas_width + x, &mut pixels);
        }
    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}