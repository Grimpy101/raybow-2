use std::{error::Error, fmt::Display};

pub mod ppm;

/// Errors in image generation
#[derive(Debug)]
pub enum ExportError {
    /// First two parameters are width and height,
    /// then actual data size
    SizeExceedsData(u32, u32, usize),
}

impl Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExportError::SizeExceedsData(width, height, size) => {
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

impl Error for ExportError {}
