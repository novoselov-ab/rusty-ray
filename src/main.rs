extern crate rand;

//#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::{glutin, Surface};
//use time::PreciseTime;

mod support;
mod math;
mod core;

use math::*;
use core::*;


const IMAGE_SIZE: (u32, u32) = (400, 200);


fn main() {
    // Building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(IMAGE_SIZE.0, IMAGE_SIZE.1)
        .with_title("Rusty Ray");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building a texture with "OpenGL" drawn on it
    let mut rt = RayTracer::new(IMAGE_SIZE);

    // scene setup
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(0., 0., 5.),
        radius: 1.,
        material: Material::Lambertian(Vec3::new(0.9, 0.2, 0.2)),
    }));
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(3., 0., 5.),
        radius: 1.,
        material: Material::Metal(Vec3::new(0.1, 0.1, 1.0)),
    }));
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(-3., 0., 5.),
        radius: 1.,
        material: Material::Metal(Vec3::new(0.8, 0.8, 0.7)),
    }));
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(1.1, -0.8, 2.5),
        radius: 0.2,
        material: Material::Metal(Vec3::new(0.4, 0.8, 0.7)),
    }));
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(0., -201., 5.),
        radius: 200.,
        material: Material::Lambertian(Vec3::new(0.1, 0.2, 0.1)),
    }));

    let mut frames = 0;

    // the main loop
    support::start_loop(|| {
        // for now update only once
        if frames >= 0 {
            rt.update();
        }

        // drawing a frame
        //let start = PreciseTime::now();
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
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => action = support::Action::Stop,
                _ => (),
            },
            _ => (),
        });

        frames += 1;

        action
    });
}
