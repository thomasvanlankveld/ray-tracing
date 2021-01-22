// Sys
use std::io::Write;
use std::{error::Error, rc::Rc};

// Project
mod camera;
mod color;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod point3;
mod ppm;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use color::Color;
use dielectric::Dielectric;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use lambertian::Lambertian;
use metal::Metal;
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

#[allow(dead_code)]
fn ray_background_color(ray: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction);

    // Scale the y factor of the unit direction from domain -1, 1 to range 0, 1
    let t = 0.5 * (unit_direction.y + 1.);

    // Scale from domain t: 0, 1 to color: white, blue
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

// Return a surface normal visualization if the ray hits the sphere, and the background color if it does not
fn ray_color(ray: Ray, world: &dyn Hittable, depth: i64) -> Color {
    let mut record = HitRecord::new();

    if depth <= 0 {
        return Color::new(0., 0., 0.);
    };

    // We set t_min to slightly above 0, so we don't get values below 0 from floating point rounding errors (this fixes shadow acne)
    if world.hit(ray, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::nowhere();
        let mut attenuation = Color::new(0., 0., 0.);
        return if record
            .material
            .scatter(ray, &record, &mut attenuation, &mut scattered)
        {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Color::new(0., 0., 0.)
        };
    };

    ray_background_color(ray)
}

fn controlled_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let blue_ball_material = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let ball_material = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    // let metal_material = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let yellow_metal_material = Metal::new(Color::new(0.8, 0.6, 0.2), 0.1);
    let refracting_material = Dielectric::new(1.5);

    let ground = Sphere::new(Point3::new(0., -100.5, -1.), 100., Rc::new(ground_material));
    let left = Sphere::new(Point3::new(-1., 0., -1.), 0.5, Rc::new(refracting_material));
    let left_internal = Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        Rc::new(refracting_material),
    );
    let center = Sphere::new(Point3::new(0., 0., -1.), 0.5, Rc::new(blue_ball_material));
    let right = Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Rc::new(yellow_metal_material),
    );

    world.add(Box::new(left));
    world.add(Box::new(left_internal));
    world.add(Box::new(center));
    world.add(Box::new(right));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1., 0., -2.),
    //     0.5,
    //     Rc::new(Lambertian::new(Color::new(0.3, 0.3, 0.7))),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-2., 0., -3.),
    //     0.5,
    //     Rc::new(Lambertian::new(Color::new(0.3, 0.7, 0.3))),
    // )));
    world.add(Box::new(ground));

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height: u16 = (f64::from(image_width) / aspect_ratio).floor() as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = controlled_scene();

    // Camera
    let look_from = Point3::new(3., 3., 2.);
    let look_at = Point3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let vertical_field_of_view = 20.;
    let focus_distance = (look_from - look_at).len();
    let aperture = 2.;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vertical_field_of_view,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    // Create pixel data
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width.into()]; image_height.into()];
    for (y, row) in image.iter_mut().rev().enumerate() {
        writeln!(
            std::io::stderr(),
            "Scanlines remaining: {}",
            image_height - y as u16,
        )?;
        for (x, pixel) in row.iter_mut().enumerate() {
            // Per pixel, compute a number of randomly sampled values and add them all to the pixel
            for _ in 0..(samples_per_pixel - 1) {
                let u = (f64::value_from(x)? + random::<f64>()) / f64::value_from(image_width - 1)?;
                let v =
                    (f64::value_from(y)? + random::<f64>()) / f64::value_from(image_height - 1)?;
                let ray = camera.get_ray(u, v);
                *pixel += ray_color(ray, &world, max_depth);
            }

            // Get average value of all samples and apply gamma correction
            let scale = 1. / f64::from(samples_per_pixel);
            pixel.x = f64::sqrt(scale * pixel.x);
            pixel.y = f64::sqrt(scale * pixel.y);
            pixel.z = f64::sqrt(scale * pixel.z);
        }
    }

    // Write image
    write_ppm(image_width, image_height, image, std::io::stdout())?;

    // Exit
    Ok(())
}
