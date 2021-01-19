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
fn ray_background_rainbow_color(ray: Ray) -> Color {
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

// Whether a ray passes through a sphere
//
// If it does not hit, it returns -1. If it does, it returns distance t that must be travelled along the given ray to reach the sphere.
//
// A sphere at origin 0, 0, 0 is the collection of points where x^2+y^2+z^2=R^2
//
// Expressed in vector form where P is a point and C is the center of the sphere: (P−C)⋅(P−C)=r^2
//
// Using a ray P(t)=A+tb instead of a point, where A is its origin, b its direction and t the
// distance along that direction, the expression becomes (P(t)−C)⋅(P(t)−C)=r^2
//
// Expanding this equation and moving everything to the left yields: (t^2)b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
//
// The function below checks whether t > 0 in (t^2)b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    // Compute (A-C)
    let oc = ray.origin - center;
    // Compute b⋅b
    let a = ray.direction.len_squared();
    // Compute b⋅(A−C)
    let half_b = oc.dot(ray.direction);
    // Compute (A−C)⋅(A−C)−r^2
    let c = oc.len_squared() - radius * radius;
    // Compute discriminant
    let discriminant = half_b * half_b - a * c;
    // Check whether discriminant < 0
    match discriminant < 0. {
        true => -1.,
        false => (-half_b - f64::sqrt(discriminant)) / a,
    }
}

fn ray_color(ray: Ray) -> Color {
    // Return a surface normal visualization if the ray hits the sphere, and the background color if it does not
    let circle_center = Point3::new(0., 0., -1.);
    let t = hit_sphere(circle_center, 0.5, ray);
    if t > 0. {
        let surface_normal = (ray.at(t) - circle_center).unit_vector();
        return 0.5 * (surface_normal + Color::new(1., 1., 1.));
    }
    ray_background_color(ray)
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
