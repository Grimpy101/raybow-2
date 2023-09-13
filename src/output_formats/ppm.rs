use std::{error::Error, fmt::Display};

use crate::color::RGBColor;

/// Errors in PPM image generation
#[derive(Debug)]
pub enum PPMError {
    /// First two parameters are width and height,
    /// then actual data size
    SizeExceedsData(u32, u32, usize),
}

impl Display for PPMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            PPMError::SizeExceedsData(width, height, size) => {
                format!(
                    "Promised size ({}x{}={}) exceeds the actual data ({})",
                    width,
                    height,
                    width * height,
                    size
                )
            }
        };
        write!(f, "PPMError: {}", message)
    }
}

impl Error for PPMError {}

/// Returns a vector of bytes representing ppm image with binary data
///
/// ## Arguments
/// * `rgb_data` - a 1D vector or slice of RGB colored pixels in the image
/// * `width` - width of image
/// * `height` - height of image
pub fn rgb_to_binary_ppm(
    rgb_data: &[RGBColor],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, PPMError> {
    // Check if we actually have enough data
    if width * height > rgb_data.len() as u32 {
        return Err(PPMError::SizeExceedsData(width, height, rgb_data.len()));
    }

    let mut header: Vec<u8> = format!("P6\n{} {}\n{}\n", width, height, 255)
        .bytes()
        .collect();

    let mut output = Vec::new();
    output.append(&mut header);

    for color in rgb_data.iter() {
        let mut modified_color = *color;
        modified_color.clamp();
        let resized_color = modified_color * 255.0;
        output.push(resized_color.r() as u8);
        output.push(resized_color.g() as u8);
        output.push(resized_color.b() as u8);
    }

    Ok(output)
}

/// Returns a vector of bytes representing ppm image with ascii data
///
/// ## Arguments
/// * `rgb_data` - a 1D vector or slice of RGB colored pixels in the image
/// * `width` - width of image
/// * `height` - height of image
pub fn rgb_to_ascii_ppm(
    rgb_data: &[RGBColor],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, PPMError> {
    // Check if we actually have enough data
    if width * height > rgb_data.len() as u32 {
        return Err(PPMError::SizeExceedsData(width, height, rgb_data.len()));
    }

    let header = format!("P3\n{} {}\n{}\n", width, height, 255);

    let mut output = header;

    for (i, color) in rgb_data.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        let mut modified_color = *color;
        modified_color.clamp();
        let resized_color = modified_color * 255.0;
        let color_tuple = format!(
            "{} {} {}\n",
            resized_color.r(),
            resized_color.g(),
            resized_color.b()
        );
        output.push_str(&color_tuple);
    }

    Ok(output.bytes().collect())
}
