extern crate rand;

//#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::{glutin, Surface};
use image::GenericImage;
use rand::Rng;
use time::PreciseTime;
use std::rc::*;
use std::cmp::*;

mod support;
mod math;

use math::*;

const IMAGE_SIZE: (u32, u32) = (1024, 512);

trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

struct Sphere {
    origin: Vec3,
    radius: f32,
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let to_center = self.origin - ray.origin;
        let to_nearest = to_center.dot(ray.dir);
        if to_nearest < 0. {
            None
        } else {
            let d2 = to_center.square_length() - to_nearest * to_nearest;
            let r2 = self.radius * self.radius;
            if d2 > r2 {
                None
            } else {
                let tt = (r2 - d2).sqrt();
                let t0 = to_nearest + tt;
                let t1 = to_nearest - tt;
                let mut t = t0.min(t1);
                if t < 0. {
                    t = t0.max(t1);
                }
                if t < 0. {
                    None
                } else {
                    Some(t)
                }
            }
        }
    }
}

struct Camera {
    position: Vec3,
    // orinetation
}

struct Scene {
    objects: Vec<Box<SceneObject>>,
    camera: Camera
}

struct IntersectionResult {
    t: f32,
    material: i32,
}

impl Scene {
    fn new() -> Scene {
        Scene { objects: Vec::new(), camera: Camera { position: Vec3::zero() } }
    }

    fn intersect(&self, ray: Ray) -> Option<IntersectionResult> {
        let mut result: Option<IntersectionResult> = None;
        for object in &self.objects {
            if let Some(t) = object.intersect(&ray) {
                let ok = match result {
                    Some(ref r) => r.t > t,
                    None => true,
                };
                if ok {
                    result = Some(IntersectionResult { t: t, material: 0 })
                }
            }
        }
        result
    }
}

struct RayTracer {
    image: image::DynamicImage,
    dimensions: (u32, u32),
    fov: f32,
    scene: Scene,
}

impl RayTracer {
    fn new(dimensions: (u32, u32)) -> RayTracer {
        RayTracer {
            image: image::DynamicImage::new_rgb8(dimensions.0, dimensions.1),
            dimensions: dimensions,
            fov: 3.14 / 3.,
            scene: Scene::new(),
        }
    }

    fn update(&mut self) {

        // send ray:
        let fov_tan = (self.fov * 0.5).tan();
        let aspect = (self.dimensions.0 as f32) / (self.dimensions.1 as f32);
        
        for px in 0..self.dimensions.0 {
            for py in 0..self.dimensions.1 {
                let x = (2. * (px as f32)) / (self.dimensions.0 as f32) - 1.;
                let y = (2. * (py as f32)) / (self.dimensions.1 as f32) - 1.;

                let origin = self.scene.camera.position;
                let dir = Vec3::new(x * fov_tan * aspect, y * fov_tan, 1.).normalize();
                let ray = Ray::new(origin, dir);
                // TODO: orientation

                let intersection = self.scene.intersect(ray);
                if let Some(ref result) = intersection {
                    println!("{}", result.t);
                    self.image.put_pixel(px, py, image::Rgba([(result.t * 10.) as u8, 255, 255, 255]));
                }
            }
        }

        // let mut rng = rand::thread_rng();
        // let d = self.dimensions;
        // for _ in 1..10 {
        //     self.image.put_pixel(
        //         rng.gen_range(0, d.0),
        //         rng.gen_range(0, d.1),
        //         image::Rgba([255, 255, 255, 255]),
        //     );
        // }
    }
}

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
    rt.scene.objects.push(Box::new(Sphere { origin: Vec3::new(0., 0., 5.), radius: 1. } ));
    rt.scene.objects.push(Box::new(Sphere { origin: Vec3::new(5., 0., 10.), radius: 1. } ));
    rt.scene.objects.push(Box::new(Sphere { origin: Vec3::new(0., -5., 7.), radius: 1. } ));

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
