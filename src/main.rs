// Sys
use std::error::Error;

// Project
mod color;
mod point3;
mod ppm;
mod ray;
mod vec3;
use color::Color;
use point3::Point3;
use ppm::write_ppm;
use ray::Ray;
use vec3::Vec3;

// 3rd party
extern crate conv;
use conv::*;

#[allow(dead_code)]
fn ray_rainbow_color(ray: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction);
    Color::new(
        unit_direction.y / 2. + 0.5,
        0.5 - unit_direction.x / 2.,
        0.25,
    )
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height: u16 = (f64::from(image_width) / aspect_ratio).floor() as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Create pixel data
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width.into()]; image_height.into()];
    for (y, row) in image.iter_mut().rev().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            // Construct ray for pixel
            let u = f64::value_from(x)? / f64::value_from(image_width - 1)?;
            let v = f64::value_from(y)? / f64::value_from(image_height - 1)?;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);

            // Get pixel for ray
            *pixel = ray_color(ray);
        }
    }

    // Write image
    write_ppm(image_width, image_height, image, std::io::stdout())?;

    // Exit
    Ok(())
}
