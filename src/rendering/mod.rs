use crate::color::RGBColor;

pub mod render;
pub mod renderables;

pub struct RenderResult {
    pub width: usize,
    pub height: usize,
    pub image_data: Vec<RGBColor>,
}
