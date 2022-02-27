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
    let canvas_width = 300_usize;
    let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;

    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        125.0,
        aspect_ratio
    );

    let mut scene = Scene::new(camera);

    scene.add_object(
        Box::new(Sphere {
            center: Vector3::new(-1.25, 0.0, 3.0),
            radius: 0.5,
            textmat: Box::new(UniformTexture {})
        })
    );
    
    scene.add_object(
        Box::new(Sphere {
            center: Vector3::new(0.0, 0.0, 2.0),
            radius: 0.75,
            textmat: Box::new(UniformTexture {})
        })
    );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for y in 0..canvas_height {
        for x in 0..canvas_width {
            let u = x as f32 / canvas_width as f32;
            let v = y as f32 / canvas_height as f32;

            let offset = y * canvas_width + x;

            let ray = scene.camera.cast_ray(u, v);

            for object in scene.objects.iter() {

                if object.intersects(&ray) {
                    pixels[offset * 3] = 255;
                    pixels[offset * 3 + 1] = 255;
                    pixels[offset * 3 + 2] = 255;
                }
            }
        }
    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}