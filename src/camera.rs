use glam::{Mat4, Vec3A, Vec4, Vec4Swizzles};
use rand::Rng;
use rand_xoshiro::Xoshiro256Plus;

use crate::{math::random_on_unit_disk, ray::Ray};

pub struct Camera {
    origin: Vec3A,
    look_at: Vec3A,
    up: Vec3A,
    upper_left: Vec3A,
    horizontal_shift: Vec3A,
    vertical_shift: Vec3A,

    width: f32,
    height: f32,
    vertical_fov: f32,

    dof_angle: f32,
    dof_distance: f32,
    dof_disk_horizontal: Vec3A,
    dof_disk_vertical: Vec3A,
}

impl Default for Camera {
    fn default() -> Self {
        let width = 256.0;
        let height = 256.0;
        let vertical_fov = 60.0f32;
        let origin = Vec3A::new(0.0, 0.0, 0.0);
        let look_at = Vec3A::new(0.0, 0.0, -1.0);
        let up = Vec3A::new(0.0, 1.0, 0.0);
        let upper_left = Vec3A::new(0.0, 0.0, 0.0);
        let pixel_horizontal_shift = Vec3A::new(0.0, 0.0, 0.0);
        let pixel_vertical_shift = Vec3A::new(0.0, 0.0, 0.0);
        let dof_angle = 0.0;
        let dof_distance = 1.0;
        let dof_disk_horizontal = Vec3A::new(0.0, 0.0, 0.0);
        let dof_disk_vertical = Vec3A::new(0.0, 0.0, 0.0);

        let mut camera = Self {
            origin,
            look_at,
            up,
            upper_left,
            horizontal_shift: pixel_horizontal_shift,
            vertical_shift: pixel_vertical_shift,
            width,
            height,
            vertical_fov,
            dof_angle,
            dof_distance,
            dof_disk_horizontal,
            dof_disk_vertical,
        };
        camera.update_transforms();
        camera
    }
}

impl Camera {
    /// Creates a new camera
    ///
    /// ## Parameters
    /// * `width` - output image width
    /// * `height` - output image height
    /// * `vertical_fov` - vertical field of view (zoom)
    /// * `dof_distance` - distance of depth of field
    /// * `dof_cone_angle` - size of the depth of field
    pub fn new(
        width: usize,
        height: usize,
        vertical_fov: f32,
        dof_distance: f32,
        dof_cone_angle: f32,
    ) -> Self {
        let width = width as f32;
        let height = height as f32;

        if height == 0.0 || width == 0.0 {
            panic!("Width or height of camera is 0.0!");
        }

        let origin = Vec3A::new(0.0, 0.0, 0.0);
        let look_at = Vec3A::new(0.0, 0.0, -1.0);
        let look_up = Vec3A::new(0.0, 1.0, 0.0);

        let upper_left = Vec3A::new(0.0, 0.0, 0.0);
        let horizontal_shift = Vec3A::new(0.0, 0.0, 0.0);
        let vertical_shift = Vec3A::new(0.0, 0.0, 0.0);

        let dof_disk_horizontal = Vec3A::new(0.0, 0.0, 0.0);
        let dof_disk_vertical = Vec3A::new(0.0, 0.0, 0.0);

        let mut camera = Self {
            origin,
            look_at,
            up: look_up,
            upper_left,
            horizontal_shift,
            vertical_shift,
            vertical_fov,
            width,
            height,
            dof_angle: dof_cone_angle,
            dof_distance,
            dof_disk_horizontal,
            dof_disk_vertical,
        };
        camera.update_transforms();
        camera
    }

    /// Sets the up vector of the camera
    ///
    /// This decides how the in-camera view is rotated
    pub fn set_up_direction(&mut self, up: Vec3A) {
        self.up = up;
        self.update_transforms();
    }

    /// Sets the width of the image
    pub fn set_width(&mut self, width: usize) {
        self.width = width as f32;
        self.update_transforms();
    }

    /// Sets the height of the image
    pub fn set_height(&mut self, height: usize) {
        self.height = height as f32;
        self.update_transforms();
    }

    /// Sets the vertical field of view
    pub fn set_vertical_fov(&mut self, fov: f32) {
        self.vertical_fov = fov;
        self.update_transforms();
    }

    /// Sets the position (origin) of camera
    pub fn set_position(&mut self, position: Vec3A) {
        self.origin = position;
        self.update_transforms();
    }

    pub fn set_defocus(&mut self, dof_distance: f32, dof_cone_angle: f32) {
        self.dof_distance = dof_distance;
        self.dof_angle = dof_cone_angle;
        self.update_transforms();
    }

    /// Sets the point at which the camera looks
    pub fn look_at(&mut self, look_at: Vec3A) {
        self.look_at = look_at.normalize();
        self.update_transforms();
    }

    /// Transforms camera with the given transform matrix
    pub fn transform(&mut self, matrix: Mat4) {
        let origin: Vec4 = self.origin.extend(1.0);
        let look_at: Vec4 = self.look_at.extend(1.0);
        let up: Vec4 = self.up.extend(1.0);
        let transformed_origin = matrix * origin;
        let transformed_look_at = matrix * look_at;
        let transformed_up = matrix * up;

        self.origin = transformed_origin.xyz().into();
        self.look_at = transformed_look_at.xyz().into();
        self.up = transformed_up.xyz().into();

        self.update_transforms();
    }

    /// Updates all data for ray direction calculation
    fn update_transforms(&mut self) {
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let aspect_ratio = self.width / self.height;
        let viewport_height = 2.0 * h * self.dof_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let look_difference = (self.origin - self.look_at).normalize();
        let side_direction = self.up.cross(look_difference).normalize();
        let up_direction = look_difference.cross(side_direction);

        let viewport_side = viewport_width * side_direction;
        let viewport_up = viewport_height * (-up_direction);

        let pixel_horizontal_shift = viewport_side / self.width;
        let pixel_vertical_shift = viewport_up / self.height;

        let upper_left = self.origin
            - (look_difference * self.dof_distance)
            - viewport_side / 2.0
            - viewport_up / 2.0;
        let upper_left = upper_left + 0.5 * (pixel_horizontal_shift + pixel_vertical_shift);

        // Depth of field
        let depth_of_field_radius = self.dof_distance * (self.dof_angle / 2.0).to_radians().tan();
        let defocus_disk_horizontal = side_direction * depth_of_field_radius;
        let defocus_disk_vertical = up_direction * depth_of_field_radius;

        self.upper_left = upper_left;
        self.horizontal_shift = pixel_horizontal_shift;
        self.vertical_shift = pixel_vertical_shift;
        self.dof_disk_horizontal = defocus_disk_horizontal;
        self.dof_disk_vertical = defocus_disk_vertical;
    }

    /// Get in-scene location of the center of the pixel based on its image coordinates
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_pixel_center(&self, i: usize, j: usize) -> Vec3A {
        self.upper_left + (i as f32 * self.horizontal_shift) + (j as f32 * self.vertical_shift)
    }

    /// Generates in-scene random location on the pixel based on image coordinates
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    /// * `rng` - instance of a random value generator
    pub fn get_random_location_on_pixel(
        &self,
        i: usize,
        j: usize,
        rng: &mut Xoshiro256Plus,
    ) -> Vec3A {
        let pixel_center = self.get_pixel_center(i, j);
        pixel_center + self.sample_pixel_square(rng)
    }

    /// Returns a random point in the square surrounding a pixel at the origin
    ///
    /// ## Parameters
    /// * `rng` - instance of a random value generator
    pub fn sample_pixel_square(&self, rng: &mut Xoshiro256Plus) -> Vec3A {
        let px = -0.5 + rng.gen::<f32>();
        let py = -0.5 + rng.gen::<f32>();
        px * self.horizontal_shift + py * self.vertical_shift
    }

    /// Generates a ray through the center of the pixel
    ///
    /// ## Parameters
    /// * `i` - horizontal image location of the pixel
    /// * `j` - vertical image location of the pixel
    pub fn get_ray_through_pixel_center(&self, i: usize, j: usize) -> Ray {
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
    /// * `rng` - an instance of random value generator
    pub fn get_random_ray_through_pixel(
        &self,
        i: usize,
        j: usize,
        rng: &mut Xoshiro256Plus,
    ) -> Ray {
        let origin = if self.dof_angle <= 0.0 {
            self.origin
        } else {
            // This creates depth of field.
            // We set ray origin as a random point on a disk in the camera origin.
            // Since the projection plane is the same as the DOF plane,
            // the rays hit "correctly" only in that region, making everything
            // else blurry.
            let p = random_on_unit_disk(rng);
            self.origin + (p.x * self.dof_disk_horizontal) + (p.y * self.dof_disk_vertical)
        };
        let direction = self.get_random_location_on_pixel(i, j, rng) - origin;
        Ray::new(origin, direction)
    }
}
