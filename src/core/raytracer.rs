extern crate image;
extern crate rand;

use image::GenericImage;
use rand::Rng;

use math::*;
use util::*;
use scene::*;

const SAMPLES_PER_PIXEL: u32 = 1;
const MAX_DEPTH: u8 = 40;

#[derive(Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

impl Material {
    fn scatter_lambertian(
        &self,
        ray: &Ray,
        res: &IntersectionResult,
        albedo: Vec3,
    ) -> Option<(Ray, Vec3)> {
        let p = ray.point(res.t);
        let target = p + res.n + rnd_in_unit_sphere();
        Some((Ray::new(p, (target - p).normalize()), albedo))
    }

    fn scatter_metal(
        &self,
        ray: &Ray,
        res: &IntersectionResult,
        albedo: Vec3,
    ) -> Option<(Ray, Vec3)> {
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
            Material::Metal(albedo) => self.scatter_metal(ray, res, albedo),
        }
    }
}

pub struct IntersectionResult {
    pub t: f32,
    pub n: Vec3,
    pub material: Material,
}

pub struct RayTracer {
    pub scene: Scene,
    pub image: image::DynamicImage,
    pub dimensions: (u32, u32),
    fov: f32,
    iters: u32
}

impl RayTracer {
    pub fn new(dimensions: (u32, u32)) -> RayTracer {
        RayTracer {
            image: image::DynamicImage::new_rgb8(dimensions.0, dimensions.1),
            dimensions: dimensions,
            fov: 3.14 / 3.,
            scene: Scene::new(),
            iters: 0
        }
    }

    pub fn update(&mut self) {
        // send ray:
        let fov_tan = (self.fov * 0.5).tan();
        let aspect = (self.dimensions.0 as f32) / (self.dimensions.1 as f32);
        let mut rng = rand::thread_rng();

        for px in 0..self.dimensions.0 {
            for py in 0..self.dimensions.1 {
                let mut color =  rgb_to_vec3(self.image.get_pixel(px, py));
                color = igamma(color);

                let k = self.iters as f32;
                color = color * k / (k + 1.);
                let weight = 1. / ((k + 1.) * (SAMPLES_PER_PIXEL as f32));

                for _ in 0..SAMPLES_PER_PIXEL {
                    let xr = (px as f32) + rng.gen_range(0., 1.);
                    let yr = (py as f32) + rng.gen_range(0., 1.);
                    let x = (2. * xr) / (self.dimensions.0 as f32) - 1.;
                    let y = (2. * yr) / (self.dimensions.1 as f32) - 1.;

                    let origin = self.scene.camera.position;
                    let dir = Vec3::new(x * fov_tan * aspect, y * fov_tan, 1.).normalize();
                    let ray = Ray::new(origin, dir);
                    // TODO: orientation

                    let result_color = self.render(ray, 0);
                    color = color + result_color * weight;
                }

                color = gamma(color);
                self.image.put_pixel(px, py, vec3_to_rgb(color));
            }
        }

        self.iters = self.iters + 1;
    }

    fn render(&self, ray: Ray, depth: u8) -> Vec3 {
        let min_t = if depth > 0 { 0.001 } else { 0. };
        let intersection = self.scene.intersect(ray, min_t);
        if let Some(ref result) = intersection {
            let scatter = result.material.scatter(&ray, &result);
            if depth < MAX_DEPTH {
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
