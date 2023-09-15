use crate::math::vector3::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    /// Creates a new ray
    ///
    /// ## Parameters
    /// * `origin` - where the ray starts
    /// * `direction` - direction of the ray
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    /// Retrieves direction of the ray
    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    /// Calculates 3D position based on how far along the ray we are
    ///
    /// ## Parameters
    /// * `t` - represents distance along the ray
    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + t * self.direction
    }
}
