use std::f32::consts::PI;

use rand::Rng;
use rand_xoshiro::Xoshiro256Plus;

pub mod euler_rotation;
pub mod matrix;
pub mod quaternion;
pub mod vector3;
pub mod vector4;

/// Generate random normal variable with Box-Muller Transform
///
/// Warning: This can return INF!!!
pub fn random_normal(rng: &mut Xoshiro256Plus) -> f32 {
    // This is a fast (but not precise) RNG implementation
    //let mut rng = Xoshiro256Plus::from_rng(thread_rng()).expect("Could not retrieve RNG");

    let u1 = rng.gen::<f32>();
    let u2 = rng.gen::<f32>();

    let sqrt_part = (-2.0 * u1.ln()).sqrt();
    let cos_part = (2.0 * PI * u2).sin();

    sqrt_part * cos_part
}
