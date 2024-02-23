use crate::color::RGBColor;

use super::ExportError;

/// Returns a vector of bytes representing ppm image with binary data
///
/// ## Arguments
/// * `rgb_data` - a 1D vector or slice of RGB colored pixels in the image
/// * `width` - width of image
/// * `height` - height of image
pub fn rgb_to_binary_ppm(
    rgb_data: &[RGBColor],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, ExportError> {
    // Check if we actually have enough data
    if width * height > rgb_data.len() {
        return Err(ExportError::SizeExceedsData(width, height, rgb_data.len()));
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

/// Returns a vector of bytes representing ppm image with ascii data (useful for debugging)
///
/// ## Arguments
/// * `rgb_data` - a 1D vector or slice of RGB colored pixels in the image
/// * `width` - width of image
/// * `height` - height of image
pub fn rgb_to_ascii_ppm(
    rgb_data: &[RGBColor],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, ExportError> {
    // Check if we actually have enough data
    if width * height > rgb_data.len() {
        return Err(ExportError::SizeExceedsData(width, height, rgb_data.len()));
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
            "{} {} {}",
            resized_color.r(),
            resized_color.g(),
            resized_color.b()
        );
        output.push_str(&color_tuple);
    }

    Ok(output.bytes().collect())
}
