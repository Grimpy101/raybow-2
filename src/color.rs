use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

/// RGB color structure. Handles operations with colors.
///
/// Components should be on the interval `[0.0, 1.0]`,
/// but this is not enforced and larger/smaller values can be expected.
/// To handle these cases, use the `clamp` method.
#[derive(Clone, Copy)]
pub struct RGBColor {
    r: f32,
    g: f32,
    b: f32,
}

impl RGBColor {
    /// Creates new color from RED, GREEN and BLUE components
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Returns the representation of color black (0.0, 0.0, 0.0)
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Returns the representation of color white (1.0, 1.0, 1.0)
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    /// Clamps values of components to the interval [0.0, 1.0]
    pub fn clamp(&mut self) {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
    }

    /// Does a simple gamma 2 transformation (square root) on itself
    pub fn linear_to_gamma(&mut self) {
        self.r = self.r.sqrt();
        self.g = self.g.sqrt();
        self.b = self.b.sqrt();
    }

    /// Returns the RED component
    pub fn r(&self) -> f32 {
        self.r
    }

    /// Returns the GREEN component
    pub fn g(&self) -> f32 {
        self.g
    }

    /// Returns the BLUE component
    pub fn b(&self) -> f32 {
        self.b
    }

    /// Returns a linear interpolation between two colors
    ///
    /// ## Parameters
    /// * `start_color` - the left end of interpolation range
    /// * `end_color` - the right end of interpolation range
    /// * `a` - the factor of interpolation
    pub fn lerp(start_color: Self, end_color: Self, a: f32) -> Self {
        (1.0 - a) * start_color + a * end_color
    }
}

impl Debug for RGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB[{},{},{}]", self.r, self.g, self.b)
    }
}

impl Add for RGBColor {
    type Output = RGBColor;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for RGBColor {
    type Output = RGBColor;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<RGBColor> for RGBColor {
    type Output = RGBColor;

    fn mul(self, rhs: RGBColor) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for RGBColor {
    type Output = RGBColor;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<RGBColor> for f32 {
    type Output = RGBColor;

    fn mul(self, rhs: RGBColor) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<f32> for RGBColor {
    type Output = RGBColor;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Div<RGBColor> for f32 {
    type Output = RGBColor;

    fn div(self, rhs: RGBColor) -> Self::Output {
        Self::Output {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}
