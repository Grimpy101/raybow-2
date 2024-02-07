use std::ops::Mul;

use super::vector3::Vector3;

/// A rotation quaternion implementation.
///
/// Components are x, y, z, w, so that *xi + yj + zk + w = q*
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        let x = axis.x * half_angle.sin();
        let y = axis.y * half_angle.sin();
        let z = axis.z * half_angle.sin();
        let w = half_angle.cos();

        Self { x, y, z, w }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        let lx = self.x;
        let ly = self.y;
        let lz = self.z;
        let lw = self.w;

        let rx = rhs.x;
        let ry = rhs.y;
        let rz = rhs.z;
        let rw = rhs.w;

        let x = lw * rx + lx * rw + ly * rz - lz * ry;
        let y = lw * ry + ly * rw - lx * rz + lz * rx;
        let z = lw * rz + lz * rw + lx * ry - ly * rx;
        let w = lw * rw - lx * rx - ly * ry - lz * rz;

        Self::Output { x, y, z, w }
    }
}
