use std::{ops::Mul, slice::Iter};

use super::{euler_rotation::Euler, vector3::Vector3};

#[derive(Debug)]
pub struct Matrix4x4 {
    values: [f32; 16],
}

impl Matrix4x4 {
    /// Returns identity matrix
    pub fn identity() -> Self {
        Self {
            values: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Creates matrix from provided values
    ///
    /// * `values` - an array of 16 values, from left to right, top to bottom
    pub fn from_values(values: [f32; 16]) -> Self {
        Self { values }
    }

    /// Returns matrix filled with zeros
    pub fn zeros() -> Self {
        Self { values: [0.0; 16] }
    }

    /// Returns an iterator over matrix values
    ///
    /// The values are provided in continuous array, from left to right, top to bottom
    pub fn iter(&self) -> Iter<'_, f32> {
        self.values.iter()
    }

    /// Returns an arrey of values from the matrix
    ///
    /// The values are provided in continuous array, from left to right, top to bottom
    pub fn values(&self) -> &[f32; 16] {
        &self.values
    }

    /// Creates a rotation matrix from Euler rotation
    ///
    /// * `euler` - an Euler rotation struct
    pub fn from_euler_rotation(euler: Euler) -> Self {
        let sin_x = euler.x().sin();
        let sin_y = euler.y().sin();
        let sin_z = euler.z().sin();

        let cos_x = euler.x().cos();
        let cos_y = euler.y().cos();
        let cos_z = euler.z().cos();

        let values = [
            cos_y * cos_z,
            -cos_y * sin_z,
            sin_y,
            0.0,
            cos_x * sin_z + cos_z * sin_x * sin_y,
            cos_x * cos_z - sin_x * sin_y * sin_z,
            -cos_y * sin_x,
            0.0,
            sin_x * sin_z - cos_x * cos_z * sin_y,
            cos_z * sin_x + cos_x * sin_y * sin_z,
            cos_x * cos_y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ];
        Self { values }
    }

    /// Creates translation matrix
    ///
    /// * `translation` - a translation vector
    pub fn from_translation(translation: Vector3) -> Self {
        let values = [
            1.0,
            0.0,
            0.0,
            translation.x,
            0.0,
            1.0,
            0.0,
            translation.y,
            0.0,
            0.0,
            1.0,
            translation.z,
            0.0,
            0.0,
            0.0,
            1.0,
        ];
        Self { values }
    }

    /// Creates scaling matrix
    ///
    /// * `scale` - vector where components represent scaling factor in each direction
    pub fn from_scale(scale: Vector3) -> Self {
        let values = [
            scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ];
        Self { values }
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 16];

        for i in 0..4 {
            let first_index = i * 4;
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.values[first_index + k] * rhs.values[k * 4 + j];
                }
                values[i * 4 + j] = sum;
            }
        }

        Self { values }
    }
}

// -------------------------------------- //

pub struct Matrix3x3 {
    values: [f32; 9],
}

impl Matrix3x3 {
    /// Returns identity matrix
    pub fn identity() -> Self {
        Self {
            values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates matrix from provided values
    ///
    /// * `values` - an array of 9 values, from left to right, top to bottom
    pub fn from_values(values: [f32; 9]) -> Self {
        Self { values }
    }

    /// Returns matrix filled with zeros
    pub fn zeros() -> Self {
        Self { values: [0.0; 9] }
    }

    /// Returns an iterator over matrix values
    ///
    /// The values are provided in continuous array, from left to right, top to bottom
    pub fn iter(&self) -> Iter<'_, f32> {
        self.values.iter()
    }

    /// Returns an arrey of values from the matrix
    ///
    /// The values are provided in continuous array, from left to right, top to bottom
    pub fn values(&self) -> &[f32; 9] {
        &self.values
    }

    /// Creates a rotation matrix from Euler rotation
    ///
    /// * `euler` - an Euler rotation struct
    pub fn from_euler_rotation(euler: Euler) -> Self {
        let sin_x = euler.x().sin();
        let sin_y = euler.y().sin();
        let sin_z = euler.z().sin();

        let cos_x = euler.x().cos();
        let cos_y = euler.y().cos();
        let cos_z = euler.z().cos();

        let values = [
            cos_y * cos_z,
            -cos_y * sin_z,
            sin_y,
            cos_x * sin_z + cos_z * sin_x * sin_y,
            cos_x * cos_z - sin_x * sin_y * sin_z,
            -cos_y * sin_x,
            sin_x * sin_z - cos_x * cos_z * sin_y,
            cos_z * sin_x + cos_x * sin_y * sin_z,
            cos_x * cos_y,
        ];
        Self { values }
    }

    /// Creates scaling matrix
    ///
    /// * `scale` - vector where components represent scaling factor in each direction
    pub fn from_scale(scale: Vector3) -> Self {
        let values = [scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, scale.z];
        Self { values }
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 9];

        for i in 0..3 {
            let first_index = i * 3;
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self.values[first_index + k] * rhs.values[k * 3 + k];
                }
                values[i * 3 + j] = sum;
            }
        }

        Self { values }
    }
}
