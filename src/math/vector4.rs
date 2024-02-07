use super::matrix::Matrix4x4;

/// A 3D vector implementation with components of type f32
#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    /// Creates a new 4D vector from components x,y,z,w
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Transforms the vector with given matrix
    ///
    /// ## Parameters
    /// * `matrix` - a 4x4 transform matrix
    pub fn transform(&self, matrix: &Matrix4x4) -> Self {
        let m = matrix.values();
        let mut x = self.x * m[0] + self.y * m[1] + self.z * m[2] + self.w * m[3];
        let mut y = self.x * m[4] + self.y * m[5] + self.z * m[6] + self.w * m[7];
        let mut z = self.x * m[8] + self.y * m[9] + self.z * m[10] + self.w * m[11];
        let mut w = self.x * m[12] + self.y * m[13] + self.z * m[14] + self.w * m[15];

        x /= w;
        y /= w;
        z /= w;
        w /= w;

        Self { x, y, z, w }
    }
}
