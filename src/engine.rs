
use image::ColorType;
use std::fs::File;
use image::png::PNGEncoder;
use {crate::scene::*, crate::utils::*, crate::ray::*};
pub struct Engine;

impl Engine {
    
    pub fn render_scene(&self, scene: Scene) -> Vec<u8> {
        let mut pixels = vec![0; scene.canvas_width * scene.canvas_height * 3];

        for j in 0..scene.canvas_height {
            for i in 0..scene.canvas_width {
                
                let u = (i as f32 * scene.camera.viewport_width) / (scene.canvas_width - 1) as f32;
                let v = (j as f32 * scene.camera.viewport_height) / (scene.canvas_height - 1) as f32;
                
                let target = scene.camera.top_left_start + u * scene.camera.right - v * scene.camera.up;
                let ray = Ray::new(scene.camera.origin, (target - scene.camera.origin).normalize());

                if let (intersect_point, Some(min_obj)) = scene.cast_ray(&ray, scene.camera.near_clipping_range, scene.camera.far_clipping_range) {

                    let pixel_color = scene.get_color_ray(&intersect_point, &min_obj, &ray,0);     
                    
                    let offset = j * scene.canvas_width + i;
                    pixels[offset * 3] = (255.0 * pixel_color.x)  as u8;
                    pixels[offset * 3 + 1] = (255.0 * pixel_color.y) as u8;
                    pixels[offset * 3 + 2] = (255.0 * pixel_color.z) as u8;
                }
            }

        }

        return pixels;
    }

    pub fn render_scene_realtime(&self) {
        todo!();
    }

    pub fn save_scene(&self, filename: &str, pixels: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
        return Ok(());
    }
}