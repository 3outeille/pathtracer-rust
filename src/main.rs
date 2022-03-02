#![allow(unused_imports, unused_variables)]

extern crate nalgebra;
use std::rc::Rc;

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

    // println!("origin = {}", camera.origin);
    // println!("x = {}, y = {}, z = {}", camera.x_axis, camera.y_axis, camera.z_axis);

    let mut scene = Scene::new(camera);
    
    // Create grid of balls
    let mut y = 1.0;
    let mut ks_arg = 1.0;

    for _ in 0..3 {

        let mut x = -2.45;
        let mut ns_arg = 15.0;

        for _ in 0..5 {

            let red  = Rc::new(
                UniformTexture::new(
                    1.0,
                    1.0,
                    ks_arg,
                    ns_arg,
                    Vector3::new(0.3, 0.1, 0.1)
                )
            );

            scene.add_object(
                Rc::new(Sphere {
                    center: Vector3::new(x, y, 7.0),
                    radius: 0.3,
                    textmat: red.clone()
                })
            );
    
            x += 1.20;
            ns_arg += 15.0;
        }

        y -= 1.0;
        ks_arg += 2.0;
    }

    scene.add_light(
        PointLight::new(
            Vector3::new(0.0, 0.0, 0.0),
            1.5
        )
    );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for y in 0..canvas_height {
        for x in 0..canvas_width {
            
            let u = x as f32 / canvas_width as f32;
            let v = y as f32 / canvas_height as f32;

            if let (intersect_point, Some(min_obj), ray) = scene.cast_ray(u, v) {
                scene.color_ray(intersect_point, &min_obj, &ray, y * canvas_width + x, &mut pixels);     
            }
        }
    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}