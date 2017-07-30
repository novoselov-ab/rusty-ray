extern crate rand;

//#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::{glutin, Surface};
use image::GenericImage;
use rand::Rng;
use time::PreciseTime;

mod support;
mod math;

use math::*;

const IMAGE_SIZE: u32 = 512;



struct RayTracer {
    image: image::DynamicImage,
    dimensions: (u32, u32),
}

impl RayTracer {
    fn new(dimensions: (u32, u32)) -> RayTracer {
        RayTracer {
            image: image::DynamicImage::new_rgb8(dimensions.0, dimensions.1),
            dimensions: dimensions,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let d = self.dimensions;
        for _ in 1..10 {
            self.image.put_pixel(
                rng.gen_range(0, d.0),
                rng.gen_range(0, d.1),
                image::Rgba([255, 255, 255, 255]),
            );
        }
    }
}

fn main() {
    // Building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(IMAGE_SIZE, IMAGE_SIZE)
        .with_title("Rusty Ray");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building a texture with "OpenGL" drawn on it
    let mut rt = RayTracer::new((IMAGE_SIZE, IMAGE_SIZE));

    // the main loop
    support::start_loop(|| {

        //
        rt.update();

        // drawing a frame
        let start = PreciseTime::now();
        {
            let target = display.draw();

            let image =
                glium::texture::RawImage2d::from_raw_rgb(rt.image.raw_pixels(), rt.dimensions);

            let opengl_texture = glium::Texture2d::new(&display, image).unwrap();

            opengl_texture
                .as_surface()
                .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);

            target.finish().unwrap();
        }
        //println!("{} seconds", start.to(PreciseTime::now()));

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    _ => (),
                }
            }
            _ => (),
        });

        action
    });
}
