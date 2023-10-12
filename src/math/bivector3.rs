use std::ops::{Div, Mul};

use super::vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Bivector3 {
    pub xy: f32,
    pub yz: f32,
    pub zx: f32,
}

impl Bivector3 {
    pub fn new(xy: f32, yz: f32, zx: f32) -> Self {
        Self { xy, yz, zx }
    }

    pub fn wedge(vector1: Vector3, vector2: Vector3) -> Self {
        Self {
            xy: vector1.x * vector2.y - vector1.y * vector2.x,
            yz: vector1.y * vector2.z - vector1.z * vector2.y,
            zx: vector1.z * vector2.x - vector1.x * vector2.z,
        }
    }

    pub fn length(&self) -> f32 {
        let sqrt_length = self.xy * self.xy + self.yz * self.yz + self.zx * self.zx;
        sqrt_length.sqrt()
    }

    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {
            xy: self.xy / l,
            yz: self.yz / l,
            zx: self.zx / l,
        }
    }
}

impl Div<f32> for Bivector3 {
    type Output = Bivector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            xy: self.xy / rhs,
            yz: self.yz / rhs,
            zx: self.zx / rhs,
        }
    }
}

impl Mul<Bivector3> for f32 {
    type Output = Bivector3;

    fn mul(self, rhs: Bivector3) -> Self::Output {
        Self::Output {
            xy: self * rhs.xy,
            yz: self * rhs.yz,
            zx: self * rhs.zx,
        }
    }
}
