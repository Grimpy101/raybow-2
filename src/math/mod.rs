use std::f32::consts::PI;

use glam::Vec3A;
use rand::{rngs::ThreadRng, Rng};
use rand_xoshiro::Xoshiro256Plus;

/// Generate random normal variable with Box-Muller Transform
///
/// Warning: This can return INF!!!
pub fn random_normal_number(rng: &mut Xoshiro256Plus) -> f32 {
    // This is a fast (but not precise) RNG implementation
    //let mut rng = Xoshiro256Plus::from_rng(thread_rng()).expect("Could not retrieve RNG");

    let u1 = rng.gen::<f32>();
    let u2 = rng.gen::<f32>();

    let sqrt_part = (-2.0 * u1.ln()).sqrt();
    let cos_part = (2.0 * PI * u2).sin();

    sqrt_part * cos_part
}

pub fn random_vec3_on_unit_disk(rng: &mut Xoshiro256Plus) -> Vec3A {
    let r = rng.gen::<f32>().sqrt();
    let phi = 2.0 * PI * rng.gen::<f32>();
    let x = r * phi.cos();
    let y = r * phi.sin();
    Vec3A::new(x, y, 0.0)
}

/// Calculates a random vector on unit sphere
///
/// ## Parameters
/// * `rng` - random number generator
pub fn random_vec3_on_unit_sphere(rng: &mut Xoshiro256Plus) -> Vec3A {
    // Uses dropped coordinates method for sampling on n-sphere
    // We need to protect against infinite result!!!
    let x = random_normal_number(rng);
    let y = random_normal_number(rng);
    let z = random_normal_number(rng);
    let w = random_normal_number(rng);

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

    Vec3A::new(norm_x, norm_y, norm_z)
}

/// Creates a random vector with components in provided range
///
/// ## Parameters
/// `min` - lower bound of the range
/// `max` - upper bound of the range
pub fn random_vec3_in_range(min: f32, max: f32, rng: &mut ThreadRng) -> Vec3A {
    // This is a fast (but not precise) RNG implementation
    //let mut rng = Xoshiro256Plus::from_rng(thread_rng()).expect("Could not retrieve RNG");
    //let mut rng = thread_rng();

    let diff = max - min;

    Vec3A::new(
        min + rng.gen::<f32>() * diff,
        min + rng.gen::<f32>() * diff,
        min + rng.gen::<f32>() * diff,
    )
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
pub fn refract_vec3(vector: Vec3A, normal: Vec3A, k: f32) -> Vec3A {
    let cos_theta = (-vector).dot(normal).min(1.0);
    let refracted_perpendicular = k * (vector + cos_theta * normal);
    let refracted_parallel =
        -((1.0 - refracted_perpendicular.dot(refracted_perpendicular)).abs()).sqrt() * normal;
    refracted_perpendicular + refracted_parallel
}

/// Returns a new vector that is a reflection of the `vector` over the `normal`
///
/// ## Parameters
/// * `vector` - vector to reflect
/// * `normal` - vector to reflect over
pub fn reflect_vec3(vector: Vec3A, normal: Vec3A) -> Vec3A {
    vector - 2.0 * vector.dot(normal) * normal
}

/// Creates a random vector with components in range `[0.0, 1.0]`
pub fn uniform_random_vec3(rng: &mut Xoshiro256Plus) -> Vec3A {
    Vec3A::new(rng.gen(), rng.gen(), rng.gen())
}

/// Checks if vector is near zero in all components
pub fn is_vec3_near_zero(vector: Vec3A) -> bool {
    let threshold = 1e-8;
    vector.x < threshold && vector.y < threshold && vector.z < threshold
}

pub fn is_invalid_vec3(vector: Vec3A) -> bool {
    vector.x.is_nan() || vector.y.is_nan() || vector.z.is_nan()
}
