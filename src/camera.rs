use crate::math::vector3::Vector3;

pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,
    origin: Vector3,
    upper_left_pixel_location: Vector3,
    horizontal_pixel_distance: Vector3,
    vertical_pixel_distance: Vector3,
}

impl Camera {
    /// Creates a new camera
    ///
    /// ## Parameters
    /// * `width` - output image width
    /// * `height` - output image height
    /// * `focal_length` - focal length of camera
    /// * `origin` - position of the camera
    pub fn new(width: u32, height: u32, focal_length: f32, origin: Vector3) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_horizontal_vector = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_vertical_vector = Vector3::new(0.0, -viewport_height, 0.0);

        let horizontal_pixel_distance = viewport_horizontal_vector / width as f32;
        let vertical_pixel_distance = viewport_vertical_vector / height as f32;

        // Upper left corner of the viewport
        let viewport_upper_left = origin
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_horizontal_vector / 2.0
            - viewport_vertical_vector / 2.0;
        // The center of the upper left pixel is half-way down and right from the corner to the next pixel
        let upper_left_pixel =
            viewport_upper_left + 0.5 * (horizontal_pixel_distance + vertical_pixel_distance);

        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            upper_left_pixel_location: upper_left_pixel,
            horizontal_pixel_distance,
            vertical_pixel_distance,
        }
    }

    /// Get in-scene location of the center of the pixel based on its image coordinates
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_pixel_center(&self, i: u32, j: u32) -> Vector3 {
        self.upper_left_pixel_location
            + (i as f32 * self.horizontal_pixel_distance)
            + (j as f32 * self.vertical_pixel_distance)
    }

    /// Get position of the camera
    pub fn origin(&self) -> Vector3 {
        self.origin
    }
}
