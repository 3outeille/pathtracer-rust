
use image::ColorType;
use std::fs::File;
use image::png::PNGEncoder;

pub(crate) fn write_image(filename: &str, pixels: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
    return Ok(());
}