/// Handles the calculations regarding intervals of real numbers
pub struct Interval {
    min: f32,
    max: f32,
}

impl Default for Interval {
    /// Creates a new interval with infinite length
    fn default() -> Self {
        Self {
            min: -f32::INFINITY,
            max: f32::INFINITY,
        }
    }
}

impl Interval {
    /// Creates a new interval
    ///
    /// ## Parameters
    /// * `min` - lowest value of the interval
    /// * `max` - highest value of the interval
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    /// Returns true if `x` is on the interval, including both bounds
    ///
    /// ## Parameters
    /// * `x` - value to check for
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns true if `x` is on the interval, excluding both bounds
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    /// Returns the upper bound of the interval
    pub fn max(&self) -> f32 {
        self.max
    }

    /// Returns the lower bound of the interval
    pub fn min(&self) -> f32 {
        self.min
    }
}
