extern crate image;
extern crate rand;

use math::*;
use rand::Rng;


pub fn gamma(v: Vec3) -> Vec3 {
    Vec3::new(v.x.sqrt(), v.y.sqrt(), v.z.sqrt())
}

pub fn igamma(v: Vec3) -> Vec3 {
    Vec3::new(v.x * v.x, v.y * v.y, v.z * v.z)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> image::Rgba<u8> {
    image::Rgba([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ])
}

pub fn rgb(r: f32, g: f32, b: f32) -> image::Rgba<u8> {
    rgba(r, g, b, 1.0)
}

pub fn vec3_to_rgb(v: Vec3) -> image::Rgba<u8> {
    rgb(v.x, v.y, v.z)
}

pub fn rgb_to_vec3(p: image::Rgba<u8>) -> Vec3 {
    Vec3::new((p[0] as f32) / 255.0, (p[1] as f32) / 255.0, (p[2] as f32) / 255.0)
}

pub fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let x = a as f32;
    let y = b as f32;
    ((x + (y - x) * t) as u8)
}

pub fn lerp_rgba(im0: image::Rgba<u8>, im1: image::Rgba<u8>, x: f32) -> image::Rgba<u8> {
    image::Rgba([
        lerp(im0[0], im1[0], x),
        lerp(im0[1], im1[1], x),
        lerp(im0[2], im1[2], x),
        lerp(im0[3], im1[3], x),
    ])
}

pub fn rnd_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
        );
        if p.square_length() <= 1. {
            break;
        }
    }
    p
}
