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
    let canvas_width = 700_usize;
    let canvas_height = (canvas_width as f32 / aspect_ratio) as usize;

    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        125.0,
        aspect_ratio
    );

    let mut scene = Scene::new(camera);

    scene.add_object(
        Box::new(Sphere {
            center: Vector3::new(0.0, 0.0, 2.0),
            radius: 1.0,
            textmat: Box::new(UniformTexture {})
        })
    );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for y in 0..canvas_height {
        for x in 0..canvas_width {
            
            let u = x as f32 / canvas_width as f32;
            let v = y as f32 / canvas_height as f32;
            let ray = scene.camera.cast_ray(u, v);

            let offset = y * canvas_width + x;

            if scene.objects[0].intersects(&ray) {
                pixels[offset * 3] = 0;
                pixels[offset * 3 + 1] = 0;
                pixels[offset * 3 + 2] = 0;
            } else {
                let r = x as f32 / canvas_width as f32;
                let g = y as f32 / canvas_height as f32;
                let b = 0.25 as f32;
                pixels[offset * 3] = (255. * r) as u8;
                pixels[offset * 3 + 1] = (255. * g) as u8;
                pixels[offset * 3 + 2] = (255. * b) as u8;
            }
        }
    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}