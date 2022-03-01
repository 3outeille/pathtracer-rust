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

    let ivory  = Rc::new(UniformTexture::new(Vector3::new(0.4, 0.4, 0.3)));
    let red  = Rc::new(UniformTexture::new(Vector3::new(0.3, 0.1, 0.1)));

    scene.add_object(
        Rc::new(Sphere {
            center: Vector3::new(0.0, 0.0, 7.0),
            radius: 1.0,
            textmat: ivory.clone()
        })
    );

    scene.add_light(
        PointLight::new(
            Vector3::new(0.0, 1.5, 5.0),
            2.5
        )
    );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for y in 0..canvas_height {
        for x in 0..canvas_width {
            
            let u = x as f32 / canvas_width as f32;
            let v = y as f32 / canvas_height as f32;

            if let (intersect_point, Some(min_obj)) = scene.cast_ray(u, v) {
                scene.color_ray(intersect_point, &min_obj, y * canvas_width + x, &mut pixels);     
            }
        }
    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}