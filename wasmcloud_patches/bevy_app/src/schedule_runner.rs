use crate::{
    app::{App, AppExit},
    plugin::Plugin,
};
use bevy_ecs::event::{Events, ManualEventReader};
use bevy_utils::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

/// Determines the method used to run an [`App`]'s [`Schedule`](bevy_ecs::schedule::Schedule).
///
/// It is used in the [`ScheduleRunnerSettings`].
#[derive(Copy, Clone, Debug)]
pub enum RunMode {
    /// Indicates that the [`App`]'s schedule should run repeatedly.
    Loop {
        /// The minimum [`Duration`] to wait after a [`Schedule`](bevy_ecs::schedule::Schedule)
        /// has completed before repeating. A value of [`None`] will not wait.
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

/// The configuration information for the [`ScheduleRunnerPlugin`].
///
/// It gets added as a [`Resource`](bevy_ecs::system::Resource) inside of the [`ScheduleRunnerPlugin`].
#[derive(Copy, Clone, Default)]
pub struct ScheduleRunnerSettings {
    /// Determines whether the [`Schedule`](bevy_ecs::schedule::Schedule) is run once or repeatedly.
    pub run_mode: RunMode,
}

impl ScheduleRunnerSettings {
    /// See [`RunMode::Once`].
    pub fn run_once() -> Self {
        ScheduleRunnerSettings {
            run_mode: RunMode::Once,
        }
    }

    /// See [`RunMode::Loop`].
    pub fn run_loop(wait_duration: Duration) -> Self {
        ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(wait_duration),
            },
        }
    }
}

/// Configures an [`App`] to run its [`Schedule`](bevy_ecs::schedule::Schedule) according to a given
/// [`RunMode`].
#[derive(Default)]
pub struct ScheduleRunnerPlugin;

impl Plugin for ScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
