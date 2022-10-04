use super::{timer::Timer,TimeInterface};
use bevy_ecs::prelude::*;
use std::time::Duration;
#[derive(Component, Clone, Debug)]
pub struct DamageCountdown  {
    pub main_timer: Timer,
}
impl DamageCountdown {
    pub fn new() -> Self {
        Self {
            main_timer: Timer::from_seconds(5.0, true),
        }
    }
}

impl Default for DamageCountdown {
    fn default() -> Self {
        Self::new()
    }
}
