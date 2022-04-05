use engine::Engine;
use serde_yaml;
use show_image::event::VirtualKeyCode;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::error::Error;
use std::{env, fs::File};

mod camera;
mod engine;
mod light;
mod mesh;
mod objects;
mod ray;
mod scene;
mod texture_material;

use {crate::ray::*, crate::scene::*};

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let file_arg = args[1].clone();
    let cpu = args[2].parse().unwrap();

    let file = File::open(&file_arg)?;
    let scene: Scene = serde_yaml::from_reader(file)?;

    let engine = Engine::from_scene(&scene);
    assert!(engine.camera.up.dot(&engine.camera.forward) == 0.);

    let (width, height) = (engine.canvas_width, engine.canvas_height);

    let receiver = engine.stream_render(cpu, 2048);
    let mut merged_buffer = receiver.recv().unwrap();
    let mut render_count = 1.;

    // Create a window with default options and display the image.
    let window = create_window("Pathtracer", Default::default())?;
    let event_channel = window.event_channel()?;

    for single_buffer in receiver {
        let image_buffer = Engine::buffer_float_to_u8(&merged_buffer);
        window.set_image(
            "Pathtracer",
            ImageView::new(ImageInfo::rgb8(width as u32, height as u32), &image_buffer),
        )?;

        for i in 0..merged_buffer.len() {
            // Interpolate previous buffer with new data to reduce noise
            merged_buffer[i] = merged_buffer[i] * (render_count / (render_count + 1.))
                + single_buffer[i] * (1. / (render_count + 1.));
        }

        // Save image on press S, exit program on Escape
        while let Ok(event) = event_channel.try_recv() {
            if let event::WindowEvent::KeyboardInput(event) = event {
                if event.input.state.is_pressed() {
                    if event.input.key_code == Some(VirtualKeyCode::Escape) {
                        println!("Exit program");
                        return Ok(());
                    } else if event.input.key_code == Some(VirtualKeyCode::S) {
                        println!("Save into output.png");
                        Engine::save("output.png", &image_buffer, width, height).unwrap();
                    }
                }
            }
        }

        render_count += 1.;
    }

    Ok(())
}
