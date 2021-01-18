use std::error::Error;

use crate::Color;

// View at http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
pub fn write_ppm(
    image_width: u16,
    image_height: u16,
    image: Vec<Vec<Color>>,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    // Write file header
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    // Write pixels
    for row in image {
        for pixel in row {
            write_pixel(pixel, &mut writer)?;
        }
    }

    // Exit
    Ok(())
}

fn write_pixel(pixel: Color, writer: &mut impl std::io::Write) -> Result<(), Box<dyn Error>> {
    // Convert pixel values to integers in 0 - 255 range
    let resized_pixel = [
        (255.999_f64 * pixel[0]).floor() as u8,
        (255.999_f64 * pixel[1]).floor() as u8,
        (255.999_f64 * pixel[2]).floor() as u8,
    ];

    // Write pixel
    write!(
        writer,
        "{} {} {}\n",
        resized_pixel[0], resized_pixel[1], resized_pixel[2]
    )?;

    // Exit
    Ok(())
}
