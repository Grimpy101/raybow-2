use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::{rngs::ThreadRng, Rng};
use rand_xoshiro::Xoshiro256Plus;

use super::{matrix::Matrix3x3, random_normal, vector4::Vector4};

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
    pub fn random_in_range(min: f32, max: f32, rng: &mut ThreadRng) -> Self {
        // This is a fast (but not precise) RNG implementation
        //let mut rng = Xoshiro256Plus::from_rng(thread_rng()).expect("Could not retrieve RNG");
        //let mut rng = thread_rng();

        let diff = max - min;
        Self {
            x: min + rng.gen::<f32>() * diff,
            y: min + rng.gen::<f32>() * diff,
            z: min + rng.gen::<f32>() * diff,
        }
    }

    /// Calculates a random vector on unit sphere
    ///
    /// ## Parameters
    /// * `rng` - random number generator
    pub fn random_on_unit_sphere(rng: &mut Xoshiro256Plus) -> Self {
        // Uses dropped coordinates method for sampling on n-sphere
        // We need to protect against infinite result!!!
        let x = random_normal(rng);
        let y = random_normal(rng);
        let z = random_normal(rng);
        let w = random_normal(rng);

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

    /// Returns cross product of two vectors
    ///
    /// ## Parameters
    /// * `vector1` - First vector
    /// * `vector2` - Second vector
    pub fn cross(vector1: Vector3, vector2: Vector3) -> Self {
        let x = vector1.y * vector2.z - vector1.z * vector2.y;
        let y = vector1.z * vector2.x - vector1.x * vector2.z;
        let z = vector1.x * vector2.y - vector1.y * vector2.x;
        Self { x, y, z }
    }

    /// Transforms the vector with given matrix
    ///
    /// ## Parameters
    /// * `matrix` - a 3x3 transformation matrix
    pub fn transform(&self, matrix: Matrix3x3) -> Self {
        let m = matrix.values();
        let x = self.x * m[0] + self.y * m[1] + self.z * m[2];
        let y = self.x * m[3] + self.y * m[4] + self.z * m[5];
        let z = self.x * m[6] + self.y * m[7] + self.z * m[8];

        Self { x, y, z }
    }

    /// A helper function to quickly convert to 4-D vector
    ///
    /// Sets 1.0 as the last coordinate
    pub fn to_vector4(&self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 1.0)
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
