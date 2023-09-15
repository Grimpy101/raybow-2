pub struct Progress {
    min: f32,
    max: f32,
    current: f32,
    step: f32,
    threshold: f32, // A variable to mark when the progress achieved a milestone
    milestone: f32, // Relative milestone between [0.0, 1.0]
}

impl Progress {
    /// Creates a new progress tracker
    ///
    /// ## Parameters
    /// * `min` - minimal/starting value
    /// * `max` - maximal/complete value
    /// * `step` - the increment step (how much progress is made in one iteration)
    /// * `milestone` - a relative threshold when the progress should be indicated (a value between [0.0, 1.0])
    pub fn new(min: f32, max: f32, step: f32, milestone: f32) -> Self {
        Self {
            min,
            max,
            current: min,
            step,
            threshold: min,
            milestone,
        }
    }

    /// Increments the amount progress and outputs current
    /// relative progress on every milestone
    pub fn increment(&mut self) -> Option<f32> {
        self.current += self.step;
        let current_progress = self.get_progress();
        if current_progress - self.threshold >= self.milestone {
            self.threshold += self.milestone;
            return Some(current_progress);
        }
        None
    }

    /// Get current relative progress
    pub fn get_progress(&self) -> f32 {
        (self.current - self.min) / (self.max - self.min)
    }
}
