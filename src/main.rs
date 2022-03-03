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
    let near_clipping_range = 1.0;
    let far_clipping_range = 100.0;
    let canvas_width = 1280_usize;
    let canvas_height = 720_usize;

    let camera = Camera::new(
        Vector3::new(0.0, 1.0, -2.0),
        Vector3::new(0.0, 0.0, near_clipping_range),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        near_clipping_range,
        far_clipping_range,
        aspect_ratio
    );

    println!("{}x{}", canvas_width, canvas_height);

    let mut scene = Scene::new(camera);

    let red  = Rc::new(
        UniformTexture::new(
            1.0,
            1.5,
            1.0,
            15.0,
            0.0,
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
            0.0,
            Vector3::new(0.3, 0.3, 0.8)
        )
    );

    // Ground
    scene.add_object(
        Rc::new(Plane {
            center: Vector3::new(0.0, -6.0, 10.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
            textmat: blue.clone()
        })
    );

    // Background
    scene.add_object(
        Rc::new(Plane {
            center: Vector3::new(0.0, 0.0, 50.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            textmat: red.clone()
        })
    );

    scene.add_object(
        Rc::new(Sphere {
            center: Vector3::new(-1.0, -2.0, 5.0),
            radius: 1.0,
            textmat: green.clone()
        })
    );

    scene.add_object(
        Rc::new(Sphere {
            center: Vector3::new(1.0, -2.0, 5.0),
            radius: 1.0,
            textmat: green.clone()
        })
    );

    scene.add_light(
        PointLight::new(
            Vector3::new(1.0, 1.0, 0.0),
            1.5
        )
    );

    let mut pixels = vec![0; canvas_width * canvas_height * 3];

    for j in 0..canvas_height {
        for i in 0..canvas_width {
            
            let u = (i as f32 * scene.camera.viewport_width) / (canvas_width - 1) as f32;
            let v = (j as f32 * scene.camera.viewport_height) / (canvas_height - 1) as f32;
            
            let target = scene.camera.top_left_start + u * scene.camera.right - v * scene.camera.up;
            let ray = Ray::new(scene.camera.origin, (target - scene.camera.origin).normalize());

            if let (intersect_point, Some(min_obj)) = scene.cast_ray(&ray, scene.camera.near_clipping_range, scene.camera.far_clipping_range) {

                let pixel_color = scene.get_color_ray(&intersect_point, &min_obj, &ray,0);     
                
                let offset = j * canvas_width + i;
                pixels[offset * 3] = (255.0 * pixel_color.x)  as u8;
                pixels[offset * 3 + 1] = (255.0 * pixel_color.y) as u8;
                pixels[offset * 3 + 2] = (255.0 * pixel_color.z) as u8;
            }
        }

    }

    write_image("output.png", &pixels, canvas_width, canvas_height).expect("error writing image");
}