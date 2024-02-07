use crate::{
    math::{matrix::Matrix4x4, vector3::Vector3},
    ray::Ray,
};

pub struct Camera {
    origin: Vector3,
    look_at: Vector3,
    up: Vector3,
    upper_left: Vector3,
    horizontal_shift: Vector3,
    vertical_shift: Vector3,

    width: f32,
    height: f32,
    vertical_fov: f32,
    focal_length: f32,
}

impl Camera {
    /// Creates a new camera
    ///
    /// ## Parameters
    /// * `width` - output image width
    /// * `height` - output image height
    /// * `focal_length` - focal length of camera
    /// * `position` - position of the camera
    /// * `vertical_fov` - vertical field of view
    pub fn new(width: u32, height: u32, focal_length: f32, vertical_fov: f32) -> Self {
        let width = width as f32;
        let height = height as f32;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let aspect_ratio = width / height;
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        let origin = Vector3::new(0.0, 0.0, 0.0);
        let look_at = Vector3::new(0.0, 0.0, -focal_length);
        let look_up = Vector3::new(0.0, 1.0, 0.0);

        let look_difference = (origin - look_at).normalize();
        let side_direction = Vector3::cross(look_up, look_difference).normalize();
        let up_direction = Vector3::cross(look_difference, side_direction);

        let viewport_side = viewport_width * side_direction;
        let viewport_up = viewport_height * (-up_direction);

        let pixel_horizontal_shift = viewport_side / width;
        let pixel_vertical_shift = viewport_up / height;

        let upper_left =
            origin - (focal_length * look_difference) - viewport_side / 2.0 - viewport_up / 2.0;
        let upper_left = upper_left + 0.5 * (pixel_horizontal_shift + pixel_vertical_shift);

        Self {
            origin,
            look_at,
            up: look_up,
            upper_left,
            horizontal_shift: pixel_horizontal_shift,
            vertical_shift: pixel_vertical_shift,
            vertical_fov,
            width,
            height,
            focal_length,
        }
    }

    /// Sets the up vector of the camera
    ///
    /// This decides how the in-camera view is rotated
    pub fn set_up_direction(&mut self, up: Vector3) {
        self.up = up;
        self.update_transforms();
    }

    /// Sets the width of the image
    pub fn set_width(&mut self, width: u32) {
        self.width = width as f32;
        self.update_transforms();
    }

    /// Sets the height of the image
    pub fn set_height(&mut self, height: u32) {
        self.height = height as f32;
        self.update_transforms();
    }

    /// Sets the vertical field of view
    pub fn set_vertical_fov(&mut self, fov: f32) {
        self.vertical_fov = fov;
        self.update_transforms();
    }

    /// Sets the focal length of camera
    pub fn set_focal_length(&mut self, focal_length: f32) {
        self.focal_length = focal_length;
        self.update_transforms();
    }

    /// Sets the position (origin) of camera
    pub fn set_position(&mut self, position: Vector3) {
        self.origin = position;
        self.update_transforms();
    }

    /// Sets the point at which the camera looks
    pub fn look_at(&mut self, look_at: Vector3) {
        self.look_at = look_at.normalize() * self.focal_length;
        self.update_transforms();
    }

    /// Transforms camera with the given transform matrix
    pub fn transform(&mut self, matrix: Matrix4x4) {
        let origin = self.origin.to_vector4().transform(&matrix);
        let look_at = self.look_at.to_vector4().transform(&matrix);
        let up = self.up.to_vector4().transform(&matrix);

        self.origin = Vector3::new(origin.x, origin.y, origin.z);
        self.look_at = Vector3::new(look_at.x, look_at.y, look_at.z);
        self.up = Vector3::new(up.x, up.y, up.z);

        self.update_transforms();
    }

    /// Updates all data for ray direction calculation
    fn update_transforms(&mut self) {
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let aspect_ratio = self.width / self.height;
        let viewport_height = 2.0 * h * self.focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        let look_difference = (self.origin - self.look_at).normalize();
        let side_direction = Vector3::cross(self.up, look_difference).normalize();
        let up_direction = Vector3::cross(look_difference, side_direction);

        let viewport_side = viewport_width * side_direction;
        let viewport_up = viewport_height * (-up_direction);

        let pixel_horizontal_shift = viewport_side / self.width;
        let pixel_vertical_shift = viewport_up / self.height;

        let upper_left = self.origin
            - (self.focal_length * look_difference)
            - viewport_side / 2.0
            - viewport_up / 2.0;
        let upper_left = upper_left + 0.5 * (pixel_horizontal_shift + pixel_vertical_shift);

        self.upper_left = upper_left;
        self.horizontal_shift = pixel_horizontal_shift;
        self.vertical_shift = pixel_vertical_shift;
    }

    /// Get in-scene location of the center of the pixel based on its image coordinates
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_pixel_center(&self, i: u32, j: u32) -> Vector3 {
        self.upper_left + (i as f32 * self.horizontal_shift) + (j as f32 * self.vertical_shift)
    }

    /// Generates in-scene random location on the pixel based on image coordinates
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_random_location_on_pixel(&self, i: u32, j: u32) -> Vector3 {
        let pixel_center = self.get_pixel_center(i, j);
        pixel_center + self.sample_pixel_square()
    }

    /// Returns a random point in the square surrounding a pixel at the origin
    pub fn sample_pixel_square(&self) -> Vector3 {
        let px = -0.5 + rand::random::<f32>();
        let py = -0.5 + rand::random::<f32>();
        px * self.horizontal_shift + py * self.vertical_shift
    }

    /// Generates a ray through the center of the pixel
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_ray_through_pixel_center(&self, i: u32, j: u32) -> Ray {
        let origin = self.origin;
        let direction = self.get_pixel_center(i, j) - self.origin;
        Ray::new(origin, direction)
    }

    /// Generates a ray throught a random point on the pixel
    ///
    /// This is useful for multisampling.
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_random_ray_through_pixel(&self, i: u32, j: u32) -> Ray {
        let origin = self.origin;
        let direction = self.get_random_location_on_pixel(i, j) - self.origin;
        Ray::new(origin, direction)
    }
}
