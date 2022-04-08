use crate::{
    app::{App, AppExit},
    plugin::Plugin,
    ManualEventReader,
};
use bevy_ecs::event::Events;
use bevy_utils::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

/// Determines the method used to run an [App]'s  [`Schedule`](bevy_ecs::schedule::Schedule).
#[derive(Copy, Clone, Debug)]
pub enum RunMode {
    /// Indicates that the [`App`]'s schedule should run repeatedly.
    Loop {
        /// Minimum duration to wait after a schedule has completed before repeating.
        /// A value of [`None`] will not wait.
        wait: Option<Duration>,
    },
    /// Indicates that the [`App`]'s schedule should run only once.
    Once,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Loop { wait: None }
    }
}

/// Configuration information for [`ScheduleRunnerPlugin`].
#[derive(Copy, Clone, Default)]
pub struct ScheduleRunnerSettings {
    /// Determines whether the [`Schedule`](bevy_ecs::schedule::Schedule) is run once or repeatedly.
    pub run_mode: RunMode,
}

impl ScheduleRunnerSettings {
    /// [`RunMode::Once`]
    pub fn run_once() -> Self {
        ScheduleRunnerSettings {
            run_mode: RunMode::Once,
        }
    }

    /// [`RunMode::Loop`]    
    pub fn run_loop(wait_duration: Duration) -> Self {
        ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(wait_duration),
            },
        }
    }
}

/// Configures an `App` to run its [`Schedule`](bevy_ecs::schedule::Schedule) according to a given
/// [`RunMode`]
#[derive(Default)]
pub struct ScheduleRunnerPlugin;

impl Plugin for ScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        // let settings = app
        //     .world
        //     .get_resource_or_insert_with(ScheduleRunnerSettings::default)
        //     .to_owned();
    }
}
