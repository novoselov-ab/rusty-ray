extern crate rand;

//#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::{glutin, Surface};
use image::GenericImage;
use rand::Rng;
use time::PreciseTime;
use std::cmp::*;

mod support;
mod math;

use math::*;

const IMAGE_SIZE: (u32, u32) = (800, 400);
const SAMPLES_PER_PIXEL: u32 = 100;

fn gamma(v: Vec3) -> Vec3 {
    Vec3::new(v.x.sqrt(),v.y.sqrt(),v.z.sqrt())
}

fn rgba(r: f32, g: f32, b: f32, a: f32) -> image::Rgba<u8> {
    image::Rgba([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ])
}

fn rgb(r: f32, g: f32, b: f32) -> image::Rgba<u8> {
    rgba(r, g, b, 1.0)
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let x = a as f32;
    let y = b as f32;
    ((x + (y - x) * t) as u8)
}

fn lerp_rgba(im0: image::Rgba<u8>, im1: image::Rgba<u8>, x: f32) -> image::Rgba<u8> {
    image::Rgba([
        lerp(im0[0], im1[0], x),
        lerp(im0[1], im1[1], x),
        lerp(im0[2], im1[2], x),
        lerp(im0[3], im1[3], x)
    ])
}

#[derive(Clone)]
enum Material {
    Lambertian(Vec3),
    Metal(Vec3)
}

impl Material {
    fn scatter_lambertian(&self, ray: &Ray, res: &IntersectionResult, albedo: Vec3) -> Option<(Ray, Vec3)> {
        let p = ray.point(res.t);
        let target = p + res.n + rnd_in_unit_sphere();
        Some((Ray::new(p, (target - p).normalize()), albedo))
    }

    fn scatter_metal(&self, ray: &Ray, res: &IntersectionResult, albedo: Vec3) -> Option<(Ray, Vec3)> {
        let reflected = ray.dir.reflect(res.n);
        if reflected.dot(res.n) > 0. {
            Some((Ray::new(ray.point(res.t), reflected.normalize()), albedo))
        } else {
            None
        }
    }

    pub fn scatter(&self, ray: &Ray, res: &IntersectionResult) -> Option<(Ray, Vec3)> {
        match *self {
            Material::Lambertian(albedo) => self.scatter_lambertian(ray, res, albedo),
            Material::Metal(albedo) => self.scatter_metal(ray, res, albedo)
        }
    }
}

fn rnd_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
        if p.square_length() <= 1. {
            break;
        }
    }
    p
}

struct IntersectionResult {
    t: f32,
    n: Vec3,
    material: Material,
}

trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult>;
}

struct Sphere {
    origin: Vec3,
    radius: f32,
    material: Material
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult> {
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
                    let normal = (ray.point(t) - self.origin).normalize();
                    Some(IntersectionResult { t: t, n: normal, material: self.material.clone()})
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
    camera: Camera,
}

impl Scene {
    fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            camera: Camera {
                position: Vec3::zero(),
            },
        }
    }

    fn intersect(&self, ray: Ray, min_t: f32) -> Option<IntersectionResult> {
        let mut best_result: Option<IntersectionResult> = None;
        for object in &self.objects {
            if let Some(result) = object.intersect(&ray) {
                let ok = match best_result {
                    Some(ref r) => r.t > result.t,
                    None => true
                };
                if ok && result.t > min_t {
                    best_result = Some(result)
                }
            }
        }
        best_result
    }

    fn render(&self, ray: Ray, depth: u8) -> Vec3 {
        let min_t = if depth > 0 { 0.001 } else { 0. };
        let intersection = self.intersect(ray, min_t);
        if let Some(ref result) = intersection {
            let scatter = result.material.scatter(&ray, &result);
            if depth < 50 {
                if let Some((r, attennuation)) = scatter {
                    return self.render(r, depth + 1) * attennuation
                }
            }
            Vec3::new(0., 0., 0.)
        } else {
            Vec3::new(1.0, 1.0, 1.0)
        }        
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
        let mut rng = rand::thread_rng();

        for px in 0..self.dimensions.0 {
            for py in 0..self.dimensions.1 {
                let mut color = Vec3::new(0., 0., 0.);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let xr = (px as f32) + rng.gen_range(0., 1.);
                    let yr = (py as f32) + rng.gen_range(0., 1.);
                    let x = (2. * xr) / (self.dimensions.0 as f32) - 1.;
                    let y = (2. * yr) / (self.dimensions.1 as f32) - 1.;

                    let origin = self.scene.camera.position;
                    let dir = Vec3::new(x * fov_tan * aspect, y * fov_tan, 1.).normalize();
                    let ray = Ray::new(origin, dir);
                    // TODO: orientation

                    color = color + self.scene.render(ray, 0);
                }

                color = color / (SAMPLES_PER_PIXEL as f32);
                color = gamma(color);

                self.image.put_pixel(
                    px,
                    py,
                    rgb(color.x, color.y, color.z)
                );
                
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
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(0., 0., 10.),
        radius: 1.,
        material: Material::Lambertian(Vec3::new(0.7, 0.2, 0.2))
    })); 
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(5., 0., 10.),
        radius: 1.,
        material: Material::Metal(Vec3::new(0.1, 0.1, 1.0))
    })); 
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(-5., 0., 10.),
        radius: 1.,
        material: Material::Metal(Vec3::new(0.8, 0.8, 0.7))
    })); 
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(0., -201., 10.),
        radius: 200.,
        material: Material::Lambertian(Vec3::new(0.3, 0.2, 0.0))
    }));
    /*
    rt.scene.objects.push(Box::new(Sphere {
        origin: Vec3::new(5., 0., 7.),
        radius: 3.,
        material: Material::Lambertian(0.2)
    }));*/

    let mut frames = 0;

    // the main loop
    support::start_loop(|| {
        //
        if frames == 0 {
            rt.update();
        }

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
