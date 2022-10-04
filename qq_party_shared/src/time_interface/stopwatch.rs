use bevy_ecs::{component::Component};
use std::time::Duration;
#[derive(Component, Clone, Debug, Default)]
pub struct Stopwatch {
    elapsed: Duration,
    paused: bool,
}

impl Stopwatch {
 
    pub fn new() -> Self {
        Default::default()
    }
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    #[inline]
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    #[inline]
    pub fn set_elapsed(&mut self, time: Duration) {
        self.elapsed = time;
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        if !self.paused() {
            self.elapsed += delta;
        }
        self
    }

    #[inline]
    pub fn pause(&mut self) {
        self.paused = true;
    }

    #[inline]
    pub fn unpause(&mut self) {
        self.paused = false;
    }

    #[inline]
    pub fn paused(&self) -> bool {
        self.paused
    }

    #[inline]
    pub fn reset(&mut self) {
        self.elapsed = Default::default();
    }
}
