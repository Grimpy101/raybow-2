use crate::color::RGBColor;

pub mod render;

pub struct RenderResult {
    width: u32,
    height: u32,
    image_data: Vec<RGBColor>,
}

impl RenderResult {
    pub fn new(width: u32, height: u32, image_data: Vec<RGBColor>) -> Self {
        Self {
            width,
            height,
            image_data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn heigth(&self) -> u32 {
        self.height
    }

    pub fn image_data(&self) -> &Vec<RGBColor> {
        &self.image_data
    }
}
