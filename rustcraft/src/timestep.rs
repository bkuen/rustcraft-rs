//! Type representing a time step in the game loop

use std::ops::{Add, Sub};

/// TimeStep
///
/// A `TimeStep` represents the current frame time
/// of the game loop.
#[derive(Copy, Clone)]
pub struct TimeStep(pub f32);

impl Add<f32> for TimeStep {
    type Output = TimeStep;

    fn add(self, rhs: f32) -> Self::Output {
        TimeStep(self.0 + rhs)
    }
}

impl Sub<f32> for TimeStep {
    type Output = TimeStep;

    fn sub(self, rhs: f32) -> Self::Output {
        TimeStep(self.0 - rhs)
    }
}

impl TimeStep {
    /// Creates a new time step
    pub fn new(time_step: f32) -> Self {
        Self(time_step)
    }

    /// Returns the time step in seconds
    pub fn seconds(&self) -> f32 {
        self.0
    }

    /// Returns the time step in milliseconds
    pub fn milliseconds(&self) -> f32 {
        self.0 * 1000.0
    }
}