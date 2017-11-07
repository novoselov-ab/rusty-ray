use math::*;
use raytracer::*;


pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult>;
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub material: Material
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

pub struct Camera {
    pub position: Vec3,
    // orientation
}

pub struct Scene {
    pub objects: Vec<Box<SceneObject>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            camera: Camera {
                position: Vec3::zero(),
            },
        }
    }

    pub fn intersect(&self, ray: Ray, min_t: f32) -> Option<IntersectionResult> {
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
}