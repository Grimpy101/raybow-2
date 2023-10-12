use std::ops::Mul;

use super::{bivector3::Bivector3, vector3::Vector3};

/// A tool for rotating vectors
///
/// Comes from Geometric Algebra, equivalent to Quaternions
pub struct Rotor3 {
    scalar: f32,
    bivector: Bivector3,
}

impl Rotor3 {
    /// Creates a rotor from scalar part and bivector part
    ///
    /// Rarely used, other methods for creating rotors are
    /// more convenient
    pub fn new(scalar: f32, bivector: Bivector3) -> Self {
        Self { scalar, bivector }
    }

    /// Creates a rotation that rotates first vector to second
    ///
    /// ## Parameters
    /// * `vector_from` - starting vector, should be a unit vector
    /// * `vector_to` - vector after rotation, should be a unit vector
    pub fn from_vectors(vector_from: Vector3, vector_to: Vector3) -> Self {
        let scalar = 1.0 + vector_from.dot(&vector_to);
        let minus_bivector = Bivector3::wedge(vector_to, vector_from);

        let rotor = Self {
            scalar,
            bivector: minus_bivector,
        };

        rotor.normalize()
    }

    /// Create rotor from plane and angle
    pub fn from_plane_angle(angle: f32, plane: Bivector3) -> Self {
        let half_angle = angle / 2.0;
        let sin_half_angle = half_angle.sin();
        let cos_half_angle = half_angle.cos();
        let bivector = -sin_half_angle * plane;

        Self {
            scalar: cos_half_angle,
            bivector,
        }
    }

    /// Calculates length of rotor
    pub fn length(&self) -> f32 {
        let sqrt_length = self.scalar * self.scalar
            + self.bivector.xy * self.bivector.xy
            + self.bivector.yz * self.bivector.yz
            + self.bivector.zx * self.bivector.zx;
        sqrt_length.sqrt()
    }

    /// Returns a normalized rotor
    pub fn normalize(&self) -> Self {
        let l = self.length();
        Self {
            scalar: self.scalar / l,
            bivector: self.bivector / l,
        }
    }

    pub fn rotate(rotor: Rotor3, vector: Vector3) -> Vector3 {
        let p = rotor;
        let x = vector;

        let q0 = p.scalar * x.x + x.y * p.bivector.xy + x.z * p.bivector.yz;
        let q1 = p.scalar * x.y - x.x * p.bivector.xy + x.z * p.bivector.yz;
        let q2 = p.scalar * x.z - x.x * p.bivector.zx + x.y * p.bivector.yz;

        let qxyz = x.x * p.bivector.yz - x.y * p.bivector.zx + x.z * p.bivector.xy;

        let r0 = p.scalar * q0 + q1 * p.bivector.xy + q2 * p.bivector.zx + qxyz * p.bivector.yz;
        let r1 = p.scalar * q1 - q0 * p.bivector.xy - qxyz * p.bivector.zx + q2 * p.bivector.yz;
        let r2 = p.scalar * q2 + qxyz * p.bivector.xy - q0 * p.bivector.zx - q1 * p.bivector.yz;

        Vector3 {
            x: r0,
            y: r1,
            z: r2,
        }
    }
}

impl Mul<Rotor3> for Rotor3 {
    type Output = Rotor3;

    fn mul(self, rhs: Rotor3) -> Self::Output {
        let p_a = self.scalar;
        let p_b = self.bivector;
        let q_a = rhs.scalar;
        let q_b = rhs.bivector;

        let pa_qa = p_a * q_a;
        let pxy_qxy = p_b.xy * q_b.xy;
        let pyz_qyz = p_b.yz * q_b.yz;
        let pzx_qzx = p_b.zx * q_b.zx;

        let pxy_qa = p_b.xy * q_a;
        let pa_qxy = p_a * q_b.xy;
        let pyz_qzx = p_b.yz * q_b.zx;
        let pzx_qyz = p_b.zx * q_b.yz;

        let pa_qyz = p_a * q_b.yz;
        let pyz_qa = p_b.yz * q_a;
        let pzx_qxy = p_b.zx * q_b.xy;
        let pxy_qzx = p_b.xy * q_b.zx;

        let pzx_qa = p_b.zx * q_a;
        let pa_qzx = p_a * q_b.zx;
        let pyz_qxy = p_b.yz * q_b.xy;
        let pxy_qyz = p_b.xy * q_b.yz;

        let scalar = pa_qa - pxy_qxy - pyz_qyz - pzx_qzx;
        let xy = pxy_qa + pa_qxy + pyz_qzx - pzx_qyz;
        let yz = pyz_qa + pa_qyz + pzx_qxy - pxy_qzx;
        let zx = pzx_qa + pa_qzx - pyz_qxy + pxy_qyz;

        Self::Output {
            scalar,
            bivector: Bivector3::new(xy, yz, zx),
        }
    }
}
