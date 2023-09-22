use crate::color::RGBColor;

pub fn linear_to_gamma_space(image_data: &mut [RGBColor]) {
    for color in image_data {
        color.linear_to_gamma();
    }
}
