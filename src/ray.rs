use glam::Vec3A;

pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
}

impl Ray {
    /// Creates a new ray
    ///
    /// ## Parameters
    /// * `origin` - where the ray starts
    /// * `direction` - direction of the ray
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction }
    }

    /// Retrieves direction of the ray
    pub fn direction(&self) -> Vec3A {
        self.direction
    }

    /// Retrieves origin of the ray
    pub fn origin(&self) -> Vec3A {
        self.origin
    }

    /// Calculates 3D position based on how far along the ray we are
    ///
    /// ## Parameters
    /// * `t` - represents distance along the ray
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}
