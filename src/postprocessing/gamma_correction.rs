use crate::color::RGBColor;

/// Transform image data from linear to gamma space.
///
/// Render data is by default in linear space.
/// This postprocessing step transforms it into gamma space,
/// so the final image can be viewed by external programs
/// correctly. By default, power of 2 is used for conversion.
pub fn linear_to_gamma_space(image_data: &mut [RGBColor]) {
    for color in image_data {
        color.linear_to_gamma();
    }
}
