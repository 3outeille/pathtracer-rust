use std::f64::consts::PI;
use std::fs::File;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;

use image::png::PNGEncoder;
use image::ColorType;
use nalgebra::{Rotation3, Vector3};
use rand::Rng;

use crate::scene::Scene;
use crate::texture_material::TextureMaterial;
use crate::{camera::Camera, light::PointLight, objects::ObjectsTrait, Ray};

const EPSILON: f64 = 1e-4;
const REFLECTION_DEPTH: u32 = 6;

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

    pub fn buffer_float_to_u8(float_buffer: &Vec<Vector3<f64>>) -> Vec<u8> {
        let mut u8_buffer = vec![0; float_buffer.len() * 3];

        let apply_gamma_corr = |x: f64| (x.clamp(0., 1.).sqrt() * 255.) as u8;

        for (i, pixel) in float_buffer.iter().enumerate() {
            u8_buffer[i * 3] = apply_gamma_corr(pixel.x);
            u8_buffer[i * 3 + 1] = apply_gamma_corr(pixel.y);
            u8_buffer[i * 3 + 2] = apply_gamma_corr(pixel.z);
        }

        return u8_buffer;
    }

    #[allow(dead_code)]
    pub fn render(self, cpu: usize) -> Vec<u8> {
        Engine::buffer_float_to_u8(&self.stream_render(cpu, 1).recv().unwrap())
    }

    pub fn stream_render(self, cpu: usize, num_samples: u32) -> Receiver<Vec<Vector3<f64>>> {
        assert!((self.camera.canvas_width * self.camera.canvas_height) as usize % cpu == 0);

        let engine = Arc::new(self);

        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            for _ in 0..num_samples {
                let mut handles = vec![];

                for w in 0..cpu {
                    let engine = engine.clone();

                    let t = thread::spawn(move || {
                        let mut thread_res = vec![
                            Vector3::zeros();
                            (engine.camera.canvas_width * engine.camera.canvas_height)
                                as usize
                                / cpu
                        ];

                        for y in 0..engine.canvas_height {
                            for x in 0..engine.canvas_width {
                                let offset = y * engine.canvas_width + x;

                                if offset % cpu != w {
                                    continue;
                                }

                                let mut samples = vec![];

                                for _ in 0..1 {
                                    let ray = engine.camera.create_ray(x, y);
                                    samples.push(engine.trace_ray(&ray, REFLECTION_DEPTH));
                                }

                                let pixel = samples.iter().fold(Vector3::zeros(), |a, b| a + b)
                                    / samples.len() as f64;

                                thread_res[offset / cpu] = pixel;
                            }
                        }

                        thread_res
                    });

                    handles.push(t);
                }

                let mut pixels = vec![Vector3::zeros(); engine.canvas_width * engine.canvas_height];

                for (i, handle) in handles.into_iter().enumerate() {
                    let thread_res = handle.join().unwrap();

                    for (j, pixel) in thread_res.into_iter().enumerate() {
                        pixels[j * cpu as usize + i] = pixel;
                    }
                }

                sender.send(pixels).unwrap();
            }
        });

        return receiver;
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

    pub fn get_closest_hit(
        &self,
        ray: &Ray,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Option<(Vector3<f64>, &Box<dyn ObjectsTrait>)> {
        let mut min_t = std::f64::MAX;
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

    fn sample_hemisphere(&self, normal: Vector3<f64>) -> (Vector3<f64>, f64) {
        let sample = {
            // Generate two floats with uniform distribution 0..1
            let mut rng = rand::thread_rng();
            let r1 = rng.gen::<f64>();
            let r2 = rng.gen::<f64>();

            // cos(theta) = u1 = y
            // cos^2(theta) + sin^2(theta) = 1 -> sin(theta) = srtf(1 - cos^2(theta))
            let sin_theta = (1. - r1 * r1).sqrt();
            let phi = 2. * PI * r2;
            let x = sin_theta * phi.cos();
            let z = sin_theta * phi.sin();

            Vector3::new(x, r1, z)
        };

        // Rotate sample direction to world coordinate
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
        let cos_theta2 = sample.y;

        return (world_sample, cos_theta2);
    }

    pub fn trace_ray(&self, ray: &Ray, depth: u32) -> Vector3<f64> {
        match self.get_closest_hit(
            &ray,
            self.camera.near_clipping_range,
            self.camera.far_clipping_range,
        ) {
            None => Vector3::<f64>::zeros(),
            Some((intersection_point, obj)) => {
                if depth == 0 {
                    return Vector3::zeros();
                }

                let TextureMaterial { color, surface } = obj.get_texture();
                // let ambiant = color * 0.2;
                let normal = obj.get_normal(&intersection_point);

                let going_into = normal.dot(&ray.direction) < 0.;
                let relative_normal = if going_into { normal } else { -normal };
                let cos_theta = -relative_normal.dot(&ray.direction);

                let emittance = color * surface.emittance.map(|e| e.ke).unwrap_or(0.);

                let indirect_lightning = {
                    let (wi, cos_theta2) = self.sample_hemisphere(relative_normal);
                    let sample_ray = Ray::new(intersection_point + relative_normal * EPSILON, wi);
                    let sample_color = cos_theta2 * self.trace_ray(&sample_ray, depth - 1);

                    surface.diffuse.kd * color.component_mul(&sample_color)
                };

                let reflection = if surface.reflection.kr > 0. {
                    let reflected_ray = Ray::new(
                        intersection_point + (relative_normal * EPSILON),
                        (ray.direction
                            - (2.0 * ray.direction.dot(&relative_normal) * relative_normal))
                            .normalize(),
                    );

                    surface.reflection.kr * color.component_mul(&self.trace_ray(&reflected_ray, depth - 1))
                } else {
                    Vector3::zeros()
                };

                let (refraction, fresnel) = if surface.transmission.kt > 0. {
                    let (n_air, n_glass): (f64, f64) = (1., 1.5);
                    let n_ratio: f64 = if going_into {
                        n_air / n_glass
                    } else {
                        n_glass / n_air
                    };
                    let sin_theta_sqr = 1. - cos_theta.powi(2);
                    let sin_theta2_sqr = n_ratio.powi(2) * sin_theta_sqr;
                    let cos_theta2_sqr = 1. - sin_theta2_sqr;

                    if cos_theta2_sqr < 0. {
                        return reflection; // total reflection
                    }

                    let refracted_ray = Ray::new(
                        intersection_point - (relative_normal * EPSILON),
                        ray.direction * n_ratio
                            + relative_normal * (n_ratio * cos_theta - cos_theta2_sqr.sqrt()),
                    );
                
                    // Schlick's Fresnel approximation
                    let fresnel = {                        
                        let r0 = ((n_glass - n_air) / (n_glass + n_air)).powi(2);
                        r0 + (1. - r0) * (1. - cos_theta2_sqr.sqrt()).powi(5)
                    };

                    (surface.transmission.kt * self.trace_ray(&refracted_ray, depth - 1), fresnel)
                } else {
                    (Vector3::zeros(), 0.)
                };

                if fresnel > 0.1 {
                    return emittance + indirect_lightning + fresnel * reflection + (1. - fresnel) * refraction;
                } else {
                    return emittance + indirect_lightning + reflection + refraction;
                }
            }
        }
    }

    // #[allow(dead_code)]
    pub fn cast_ray(
        &self,
        ray: &Ray,
        depth: u32,
        near_clipping_range: f64,
        far_clipping_range: f64,
    ) -> Vector3<f64> {

        match self.get_closest_hit(
            &ray,
            near_clipping_range,
            far_clipping_range,
        ) {
            None => Vector3::<f64>::zeros(),
            Some((intersection_point, obj)) => {
                if depth == 0 {
                    return Vector3::zeros();
                }
         
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
                    
                    self.cast_ray(&shadow_ray, depth - 1, EPSILON, light_distance);

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
                    let reflected_ray = Ray::new(intersection_point + (normal * EPSILON), reflected_dir);
                    self.cast_ray(&reflected_ray, depth - 1, self.camera.near_clipping_range, self.camera.far_clipping_range)
                };

                let (refraction, fresnel) = {
                    let (n_air, n_glass): (f64, f64) = (1., 1.5);
                    
                    let going_into = normal.dot(&ray.direction) < 0.;
                    let relative_normal = if going_into { normal } else { -normal };
                    let cos_theta = -relative_normal.dot(&ray.direction);

                    let n_ratio: f64 = if going_into {
                        n_air / n_glass
                    } else {
                        n_glass / n_air
                    };

                    let sin_theta_sqr = 1. - cos_theta.powi(2);
                    let sin_theta2_sqr = n_ratio.powi(2) * sin_theta_sqr;
                    let cos_theta2_sqr = 1. - sin_theta2_sqr;

                    let refracted_ray = Ray::new(
                        intersection_point - (relative_normal * EPSILON),
                        ray.direction * n_ratio
                            + relative_normal * (n_ratio * cos_theta - cos_theta2_sqr.sqrt()),
                    );
                    
                    // Schlick's Fresnel approximation
                    let fresnel = {                        
                        let r0 = ((n_glass - n_air) / (n_glass + n_air)).powi(2);
                        r0 + (1. - r0) * (1. - cos_theta2_sqr.sqrt()).powi(5)
                    };

                    (self.cast_ray(&refracted_ray, depth - 1, self.camera.near_clipping_range, self.camera.far_clipping_range), fresnel)                    
                };

                if fresnel > 0.1 {
                    return (surface.ambient.ka * ambient)
                    + (surface.diffuse.kd * diffuse)
                    + (surface.specular.ks * specular)
                    + (fresnel * surface.reflection.kr * reflection)
                    + ((1. - fresnel) * surface.transmission.kt * refraction);
                } else {
                    return (surface.ambient.ka * ambient)
                    + (surface.diffuse.kd * diffuse)
                    + (surface.specular.ks * specular)
                    + (surface.reflection.kr * reflection)
                    + (surface.transmission.kt * refraction);
                }

            }
        }
    }
}
