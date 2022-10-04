use super::stopwatch::Stopwatch;
use bevy_ecs::{component::Component};
use std::time::Duration;

/// Tracks elapsed time. Enters the finished state once `duration` is reached.
///
/// Non repeating timers will stop tracking and stay in the finished state until reset.
/// Repeating timers will only be in the finished state on each tick `duration` is reached or
/// exceeded, and can still be reset at any given point.
///
/// Paused timers will not have elapsed time increased.
#[derive(Component, Clone, Debug, Default)]
pub struct Timer {
    stopwatch: Stopwatch,
    duration: Duration,
    repeating: bool,
    finished: bool,
    times_finished: u32,
}

impl Timer {
    /// Creates a new timer with a given duration.
    ///
    /// See also [`Timer::from_seconds`](Timer::from_seconds).
    pub fn new(duration: Duration, repeating: bool) -> Self {
        Self {
            duration,
            repeating,
            ..Default::default()
        }
    }
    
    pub fn from_seconds(duration: f32, repeating: bool) -> Self {
        Self {
            duration: Duration::from_secs_f32(duration),
            repeating,
            ..Default::default()
        }
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }

    #[inline]
    pub fn just_finished(&self) -> bool {
        self.times_finished > 0
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    #[inline]
    pub fn elapsed_secs(&self) -> f32 {
        self.stopwatch.elapsed_secs()
    }

    #[inline]
    pub fn set_elapsed(&mut self, time: Duration) {
        self.stopwatch.set_elapsed(time);
    }

    #[inline]
    pub fn duration(&self) -> Duration {
        self.duration
    }

    #[inline]
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    #[inline]
    pub fn repeating(&self) -> bool {
        self.repeating
    }

    #[inline]
    pub fn set_repeating(&mut self, repeating: bool) {
        if !self.repeating && repeating && self.finished {
            self.stopwatch.reset();
            self.finished = self.just_finished();
        }
        self.repeating = repeating
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        if self.paused() {
            return self;
        }

        if !self.repeating() && self.finished() {
            self.times_finished = 0;
            return self;
        }

        self.stopwatch.tick(delta);
        self.finished = self.elapsed() >= self.duration();

        if self.finished() {
            if self.repeating() {
                self.times_finished =
                    (self.elapsed().as_nanos() / self.duration().as_nanos()) as u32;
                // Duration does not have a modulo
                self.set_elapsed(self.elapsed() - self.duration() * self.times_finished);
            } else {
                self.times_finished = 1;
                self.set_elapsed(self.duration());
            }
        } else {
            self.times_finished = 0;
        }

        self
    }

    #[inline]
    pub fn pause(&mut self) {
        self.stopwatch.pause();
    }

    #[inline]
    pub fn unpause(&mut self) {
        self.stopwatch.unpause();
    }

    #[inline]
    pub fn paused(&self) -> bool {
        self.stopwatch.paused()
    }

    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.finished = false;
        self.times_finished = 0;
    }
    #[inline]
    pub fn percent(&self) -> f32 {
        self.elapsed().as_secs_f32() / self.duration().as_secs_f32()
    }

    #[inline]
    pub fn percent_left(&self) -> f32 {
        1.0 - self.percent()
    }

    #[inline]
    pub fn times_finished(&self) -> u32 {
        self.times_finished
    }
}