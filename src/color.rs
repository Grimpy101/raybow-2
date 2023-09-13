use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

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
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<RGBColor> for f32 {
    type Output = RGBColor;

    fn div(self, rhs: RGBColor) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}
