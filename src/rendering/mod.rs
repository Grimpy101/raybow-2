use crate::color::RGBColor;

pub mod render;
pub mod renderables;

pub struct RenderResult {
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<RGBColor>,
}
