//! Type representing a time step in the game loop

use std::ops::{Add, Sub};

/// TimeStep
///
/// A `TimeStep` represents the current frame time
/// of the game loop.
pub struct TimeStep(pub f64);

impl Add<f64> for TimeStep {
    type Output = TimeStep;

    fn add(self, rhs: f64) -> Self::Output {
        TimeStep(self.0 + rhs)
    }
}

impl Sub<f64> for TimeStep {
    type Output = TimeStep;

    fn sub(self, rhs: f64) -> Self::Output {
        TimeStep(self.0 - rhs)
    }
}

impl TimeStep {
    /// Creates a new time step
    fn new(time_step: f64) -> Self {
        Self(time_step)
    }

    /// Returns the time step in seconds
    fn seconds(&self) -> f64 {
        self.0
    }

    /// Returns the time step in milliseconds
    fn milliseconds(&self) -> f64 {
        self.0
    }
}