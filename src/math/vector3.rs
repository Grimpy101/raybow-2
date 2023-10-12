use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::random_normal;

/// A 3D vector implementation with components of type f32
#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Creates a new 3D vector from components x,y,z
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a random vector with components in range `[0.0, 1.0]`
    pub fn random() -> Self {
        Self {
            x: rand::random::<f32>(),
            y: rand::random::<f32>(),
            z: rand::random::<f32>(),
        }
    }

    /// Checks if vector is near zero in all components
    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.x < threshold && self.y < threshold && self.z < threshold
    }

    pub fn is_invalid(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    /// Returns a new vector that is a reflection of the `vector` over the `normal`
    ///
    /// ## Parameters
    /// * `vector` - vector to reflect
    /// * `normal` - vector to reflect over
    pub fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
        vector - 2.0 * vector.dot(&normal) * normal
    }

    /// Calculates the vector representing new direction of light in the material
    /// from incoming direction of light outside the material (see Snell's law)
    ///
    /// Parameter `k` represents the ratio: `eta / eta'`, where `eta` is the
    /// refractive index of the material the light is coming from, and `eta'`
    /// is the refractive index of the material the light is entering.
    ///
    /// ### Some refractive indices (at room temperature):
    /// * vacuum = 1.0
    /// * standard air = 1.000273
    /// * water = 1.333
    /// * window glass = 1.52
    /// * diamond = 2.417
    /// * amber = 1.55
    /// * human lens = 1.386 - 1.406
    ///
    /// ## Parameters
    /// * `vector` - direction of incoming light
    /// * `normal` - normal at the contact point of the surface of the material
    /// * `k` - a ratio of refractive indices of the materials the light is going through
    pub fn refract(vector: Vector3, normal: Vector3, k: f32) -> Vector3 {
        let cos_theta = (-vector).dot(&normal).min(1.0);
        let refracted_perpendicular = k * (vector + cos_theta * normal);
        let refracted_parallel =
            -((1.0 - refracted_perpendicular.dot(&refracted_perpendicular)).abs()).sqrt() * normal;
        refracted_perpendicular + refracted_parallel
    }

    /// Creates a random vector with components in provided range
    ///
    /// ## Parameters
    /// `min` - lower bound of the range
    /// `max` - upper bound of the range
    pub fn random_in_range(min: f32, max: f32) -> Self {
        let diff = max - min;
        Self {
            x: min + rand::random::<f32>() * diff,
            y: min + rand::random::<f32>() * diff,
            z: min + rand::random::<f32>() * diff,
        }
    }

    /// Calculates a random vector on unit sphere
    pub fn random_on_unit_sphere() -> Self {
        // Uses dropped coordinates method for sampling on n-sphere
        // We need to protect agains infinite result!!!
        let x = random_normal();
        let y = random_normal();
        let z = random_normal();
        let w = random_normal();

        let norm = (x * x + y * y + z * z + w * w).sqrt();

        let mut norm_x = x / norm;
        let mut norm_y = y / norm;
        let mut norm_z = z / norm;

        // This is needed because division with infinity returns NaN
        if norm.is_infinite() {
            norm_x = 0.0;
            norm_y = 0.0;
            norm_z = 0.0;
        }

        Self {
            x: norm_x,
            y: norm_y,
            z: norm_z,
        }
    }

    /// Calculates dot product of two vectors
    ///
    /// # Parameters
    /// * `rhs` - the second vector
    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculates magnitude/length of the vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the given vector
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}
