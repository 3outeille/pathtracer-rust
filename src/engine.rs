use std::f32::consts::PI;
use std::f32::INFINITY;
use std::fs::File;
use std::sync::Arc;
use std::thread;

use image::png::PNGEncoder;
use image::ColorType;
use nalgebra::{Rotation3, Vector3};
use rand::Rng;

use crate::scene::Scene;
use crate::texture_material::TextureMaterial;
use crate::{camera::Camera, light::PointLight, objects::ObjectsTrait, Ray};

const EPSILON: f32 = 1e-3;
const REFLECTION_DEPTH: u32 = 5;

pub struct Engine {
    pub camera: Camera,
    // pub blobs: Vec<Blob>,
    pub objects: Vec<Box<dyn ObjectsTrait>>,
    pub lights: Vec<PointLight>,
    pub canvas_width: usize,
    pub canvas_height: usize,
}

impl Engine {
    pub fn new(camera: Camera, canvas_width: usize, canvas_height: usize) -> Self {
        Self {
            camera,
            // blobs: Vec::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            canvas_width,
            canvas_height,
        }
    }

    pub fn from_scene(scene: &Scene) -> Self {
        let mut camera = scene.camera.clone();

        // Init camera
        camera.up = camera.up.normalize();
        camera.forward = camera.forward.normalize();
        camera.right = scene.camera.up.cross(&camera.forward);

        let mut engine = Engine::new(
            camera,
            scene.camera.canvas_width as usize,
            scene.camera.canvas_height as usize,
        );

        for sphere in &scene.spheres {
            engine.add_object(Box::new(sphere.clone()));
        }

        for triangle in &scene.triangles {
            engine.add_object(Box::new(triangle.clone()));
        }

        for mesh in &scene.meshes {
            mesh.convert_to_triangles(&mut engine);
        }

        for plane in &scene.planes {
            engine.add_object(Box::new(plane.clone()));
        }

        for light in &scene.lights {
            engine.add_light(light.clone());
        }

        return engine;
    }

    // pub fn add_blob(&mut self, blob: Blob) -> () {
    //     self.blobs.push(mesh);
    // }

    pub fn add_object(&mut self, object: Box<dyn ObjectsTrait>) -> () {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) -> () {
        self.lights.push(light)
    }

    pub fn render(self, cpu: usize, is_pathtracer: bool) -> Vec<u8> {
        assert!((self.camera.canvas_width * self.camera.canvas_height) as usize % cpu == 0);

        let mut handles = vec![];

        let engine = Arc::new(self);

        for w in 0..cpu {
            let engine = engine.clone();

            let t = thread::spawn(move || {
                let mut thread_res =
                    vec![
                        Vector3::zeros();
                        (engine.camera.canvas_width * engine.camera.canvas_height) as usize / cpu
                    ];

                for y in 0..engine.canvas_height {
                    for x in 0..engine.canvas_width {
                        let offset = y * engine.canvas_width + x;

                        if offset % cpu != w {
                            continue;
                        }

                        let ray = engine.camera.create_ray(x, y);

                        let cast_result = engine.cast_ray(
                            &ray,
                            engine.camera.near_clipping_range,
                            engine.camera.far_clipping_range,
                        );

                        let color = if is_pathtracer {
                            let mut samples = vec![];

                            for _ in 0..1024 {
                                samples.push(engine.color_ray_pathtracer(
                                    cast_result,
                                    &ray,
                                    REFLECTION_DEPTH,
                                ))
                            }

                            samples.iter().fold(Vector3::zeros(), |a, b| a + b)
                                / samples.len() as f32
                        } else {
                            engine.color_ray(cast_result, &ray, 0)
                        };

                        thread_res[offset / cpu] = color;
                    }
                }

                thread_res
            });

            handles.push(t);
        }

        let mut pixels = vec![0; engine.canvas_width * engine.canvas_height * 3];

        for (i, handle) in handles.into_iter().enumerate() {
            let thread_res = handle.join().unwrap();

            for (j, pixel) in thread_res.iter().enumerate() {
                let offset = j * cpu as usize + i;

                pixels[offset * 3] = (255.0 * pixel.x) as u8;
                pixels[offset * 3 + 1] = (255.0 * pixel.y) as u8;
                pixels[offset * 3 + 2] = (255.0 * pixel.z) as u8;
            }
        }

        return pixels;
    }

    pub fn save(
        filename: &str,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
        return Ok(());
    }

    // pub fn render_blobs(&self, scene: &Scene) -> () {
    //     for blob in scene.blobs {
    //         let triangles =  blob.marching_cube();
    //         for triangle in triangles {
    //             scene.add_object(triangle);
    //         }
    //     }
    // }

    pub fn render_scene_realtime(&self) {
        // fltk = "1.2"

        // let app = app::App::default();
        // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
        // let mut frame = Frame::new(0, 0, 400, 200, "");
        // let mut but = Button::new(160, 210, 80, 40, "Click me!");
        // wind.end();
        // wind.show();
        // but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
        // app.run().unwrap();
        todo!()
    }

    pub fn cast_ray(
        &self,
        ray: &Ray,
        near_clipping_range: f32,
        far_clipping_range: f32,
    ) -> Option<(Vector3<f32>, &Box<dyn ObjectsTrait>)> {
        let mut min_t = std::f32::MAX;
        let mut min_obj: Option<&Box<dyn ObjectsTrait>> = None;

        for object in &self.objects {
            // Find the nearest root.
            match object.intersects(&ray, near_clipping_range, far_clipping_range) {
                Some(t) if (t < min_t) => {
                    min_obj = Some(&object);
                    min_t = t;
                }
                _ => {
                    continue;
                }
            };
        }

        if let Some(obj) = min_obj {
            Some((ray.at(min_t), obj))
        } else {
            None
        }
    }

    pub fn color_ray(
        &self,
        cast_result: Option<(Vector3<f32>, &Box<dyn ObjectsTrait>)>,
        ray: &Ray,
        depth: u32,
    ) -> Vector3<f32> {
        if cast_result.is_none() || depth == 0 {
            return Vector3::zeros();
        }

        let (intersection_point, obj) = cast_result.unwrap();

        let TextureMaterial { color, surface } = obj.get_texture();
        let normal = obj.get_normal(&intersection_point);
        let reflected_dir =
            (ray.direction - (2.0 * ray.direction.dot(&normal) * normal)).normalize();

        // Phong Model
        let ambient = color * 0.2;
        let mut diffuse = Vector3::zeros();
        let mut specular = Vector3::zeros();

        for light in &self.lights {
            let light_vec = light.position - intersection_point;
            let light_dir = light_vec.normalize();
            let light_distance = light_vec.norm();
            let light_value = light.intensity * light.color;

            let shadow_ray = Ray::new(intersection_point, light_dir + normal * EPSILON);

            if self
                .cast_ray(&shadow_ray, EPSILON, light_distance)
                .is_some()
            {
                continue;
            }

            diffuse += {
                let dot_prod = light_dir.dot(&normal).clamp(0.0, 1.0);
                color.component_mul(&light_value) * dot_prod
            };

            specular += {
                let dot_prod = light_dir
                    .dot(&reflected_dir)
                    .clamp(0.0, 1.0)
                    .powf(surface.specular.ns);
                light_value * dot_prod
            };
        }

        let reflection = {
            // When casting rays using previous intersection point, ray may hit under the surface
            // due to numerical precision of the intersection point calculation (discriminant).
            // The more rays are casted using previous intersection point, the more the error accumulate.
            let reflected_ray = Ray::new(intersection_point + (normal * EPSILON), reflected_dir);

            let cast_result = self.cast_ray(
                &reflected_ray,
                self.camera.near_clipping_range,
                self.camera.far_clipping_range,
            );

            self.color_ray(cast_result, &reflected_ray, depth - 1)
        };

        return (surface.ambient.ka * ambient)
            + (surface.diffuse.kd * diffuse)
            + (surface.specular.ks * specular)
            + (surface.reflection.kr * reflection);
    }

    pub fn color_ray_pathtracer(
        &self,
        cast_result: Option<(Vector3<f32>, &Box<dyn ObjectsTrait>)>,
        ray: &Ray,
        depth: u32,
    ) -> Vector3<f32> {
        if cast_result.is_none() || depth == 0 {
            return Vector3::zeros();
        }

        let (intersection_point, obj) = cast_result.unwrap();

        let TextureMaterial { color, surface } = obj.get_texture();
        let ambiant = color * 0.2;

        let normal = obj.get_normal(&intersection_point);

        let diffuse = {
            let sample = {
                // Generate two floats with uniform distribution 0..1
                let mut rng = rand::thread_rng();
                let r1 = rng.gen::<f32>();
                let r2 = rng.gen::<f32>();

                // cos(theta) = u1 = y
                // cos^2(theta) + sin^2(theta) = 1 -> sin(theta) = srtf(1 - cos^2(theta))
                let sin_theta = (1. - r1 * r1).sqrt();
                let phi = 2. * PI * r2;
                let x = sin_theta * phi.cos();
                let z = sin_theta * phi.sin();

                Vector3::new(x, r1, z)
            };

            let transform_matrix = {
                let nx = if normal.x.is_normal() {
                    Vector3::new(normal.y, -normal.x, 0.).normalize()
                } else {
                    Vector3::new(0., -normal.z, normal.y).normalize()
                };

                let nz = normal.cross(&nx);

                Rotation3::from_basis_unchecked(&[nx, normal, nz])
            };

            let world_sample = transform_matrix * sample;

            let sample_ray = Ray::new(intersection_point + normal * EPSILON, world_sample);
            let sample_cast_result = self.cast_ray(&sample_ray, 0., INFINITY);
            let sample_color =
                self.color_ray_pathtracer(sample_cast_result, &sample_ray, depth - 1);

            color.component_mul(&(sample_color * surface.diffuse.kd))
        };

        let emittance = surface.emittance.map(|e| e.ke).unwrap_or(0.) * color;

        return emittance + diffuse;
    }
}
