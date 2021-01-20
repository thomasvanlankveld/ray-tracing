// Sys
use std::error::Error;

// Project
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod point3;
mod ppm;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use point3::Point3;
use ppm::write_ppm;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

// 3rd party
extern crate conv;
extern crate rand;
use conv::*;

#[allow(dead_code)]
fn ray_background_color_rainbow(ray: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction);

    // Generate color by scaling x and y of the unit direction from domain -1, 1 to range 0, 1.
    Color::new(
        unit_direction.y / 2. + 0.5,
        0.5 - unit_direction.x / 2.,
        0.25,
    )
}

fn ray_background_color(ray: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction);

    // Scale the y factor of the unit direction from domain -1, 1 to range 0, 1
    let t = 0.5 * (unit_direction.y + 1.);

    // Scale from domain t: 0, 1 to color: white, blue
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

// Return a surface normal visualization if the ray hits the sphere, and the background color if it does not
fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::new();

    if world.hit(ray, 0., f64::INFINITY, &mut record) {
        return 0.5 * (record.normal + Color::new(1., 1., 1.));
    }

    ray_background_color(ray)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height: u16 = (f64::from(image_width) / aspect_ratio).floor() as u16;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    // world.add(Box::new(Sphere::new(Point3::new(-1., 0., -2.), 0.5)));
    // world.add(Box::new(Sphere::new(Point3::new(-2., 0., -3.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let camera = Camera::new();

    // Create pixel data
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width.into()]; image_height.into()];
    for (y, row) in image.iter_mut().rev().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            // Per pixel, compute a number of randomly sampled values and add them all to the pixel
            for _ in 0..(samples_per_pixel - 1) {
                let u = (f64::value_from(x)? + random::<f64>()) / f64::value_from(image_width - 1)?;
                let v =
                    (f64::value_from(y)? + random::<f64>()) / f64::value_from(image_height - 1)?;
                let ray = camera.get_ray(u, v);
                *pixel += ray_color(ray, &world);
            }
            // Get average value of all samples by dividing by number of samples
            *pixel /= f64::from(samples_per_pixel);
        }
    }

    // Write image
    write_ppm(image_width, image_height, image, std::io::stdout())?;

    // Exit
    Ok(())
}
