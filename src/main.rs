// Sys
use std::error::Error;

// Project
mod color;
mod point3;
mod vec3;
use color::Color;
use point3::Point3;
use vec3::Vec3Properties;

// 3rd party
extern crate conv;
use conv::*;

// Todo: Pixels into struct?

fn main() -> Result<(), Box<dyn Error>> {
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    let color = Color::new(6., 2., 3.);
    let point = Point3::new(3., 4., 5.);
    let _sum = color + point;
    // println!("{}", _sum);

    // Create pixel data
    let mut image = vec![vec![[0u8; 3]; image_width.into()]; image_height.into()];
    for (x, row) in image.iter_mut().enumerate() {
        for (y, pixel) in row.iter_mut().enumerate() {
            // RGB values as floats between 0 and 1
            let r = f64::value_from(y)? / f64::from(image_width - 1);
            let g = f64::value_from(255 - x)? / f64::from(image_height - 1);
            let b = 0.25;

            pixel[0] = (255.999_f64 * r).floor() as u8;
            pixel[1] = (255.999_f64 * g).floor() as u8;
            pixel[2] = (255.999_f64 * b).floor() as u8;
        }
    }

    // Write image
    write_ppm(image_width, image_height, image, std::io::stdout());

    // Exit
    Ok(())
}

// View at http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
fn write_ppm(
    image_width: u16,
    image_height: u16,
    image: Vec<Vec<[u8; 3]>>,
    mut writer: impl std::io::Write,
) {
    // Write file header
    if let Err(e) = writeln!(writer, "P3\n{} {}\n255", image_width, image_height) {
        println!("Writing error: {}", e.to_string());
    }

    // Write pixels
    for row in image {
        for pixel in row {
            if let Err(e) = write!(writer, "{} {} {}\n", pixel[0], pixel[1], pixel[2]) {
                println!("Writing error: {}", e.to_string());
            }
        }
    }
}
