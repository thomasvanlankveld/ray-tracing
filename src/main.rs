// Sys
use std::error::Error;

// Project
mod color;
mod point3;
mod ppm;
mod vec3;
use color::Color;
use ppm::write_ppm;
use vec3::Vec3Properties;

// 3rd party
extern crate conv;
use conv::*;

fn main() -> Result<(), Box<dyn Error>> {
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // Create pixel data
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width.into()]; image_height.into()];
    for (x, row) in image.iter_mut().enumerate() {
        for (y, pixel) in row.iter_mut().enumerate() {
            // RGB values as floats between 0 and 1
            let r = f64::value_from(y)? / f64::from(image_width - 1);
            let g = f64::value_from(255 - x)? / f64::from(image_height - 1);
            let b = 0.25;

            *pixel = Color::new(r, g, b);
        }
    }

    // Write image
    write_ppm(image_width, image_height, image, std::io::stdout())?;

    // Exit
    Ok(())
}
