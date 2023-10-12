use std::f32::consts::PI;

pub mod bivector3;
pub mod rotor3;
pub mod vector3;

/// Generate random normal variable with Box-Muller Transform
///
/// Warning: This can return INF!!!
pub fn random_normal() -> f32 {
    let u1 = rand::random::<f32>();
    let u2 = rand::random::<f32>();

    (-2.0 * u1.ln()).powf(0.5) * (2.0 * PI * u2).cos()
}
