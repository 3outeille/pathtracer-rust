use image::ColorType;
use std::fs::File;
use image::png::PNGEncoder;
extern crate nalgebra;
use nalgebra::{Vector3};

fn write_image(filename: &str, pixels: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
    return Ok(());
}

fn main() {

    let width = 200;
    let height = 100;

    let mut pixels = vec![0; width * height * 3];

    for j in 0..height {
        for i in 0..width {

            let r = i as f32 / width as f32;
            let g = j as f32 / height as f32;
            let b = 1. as f32;

            let offset = j * width + i;
            pixels[offset * 3] = (255. * r) as u8;
            pixels[offset * 3 + 1] = (255. * g) as u8;
            pixels[offset * 3 + 2] = (255. * b) as u8;
        }
    }

    write_image("output.png", &pixels, width, height).expect("error writing image");
}