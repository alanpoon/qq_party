use bevy_ecs::system::ResMut;
use bevy_utils::{Duration, Instant};

/// Tracks elapsed time since the last update and since the App has started
#[derive(Debug, Clone)]
pub struct Time {
    elapsed: f32,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            elapsed: f32,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        
    }

    pub(crate) fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = instant - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.last_update = Some(instant);
    }

    /// The delta between the current tick and last tick as a [`Duration`]
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// The delta between the current and last tick as [`f32`] seconds
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }

    /// The [`Instant`] the app was started
    #[inline]
    pub fn startup(&self) -> Instant {
        self.startup
    }

    /// The ['Instant'] when [`Time::update`] was last called, if it exists
    #[inline]
    pub fn last_update(&self) -> Option<Instant> {
        self.last_update
    }

    pub fn time_since_startup(&self) -> Duration {
        Instant::now() - self.startup
    }
}

pub(crate) fn time_system(mut time: ResMut<Time>) {
    time.update();
}