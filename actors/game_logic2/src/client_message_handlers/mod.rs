pub mod target_velocity_handler;
pub mod welcome_handler;
pub mod change_sub_map_handler;
pub mod fire_handler;
pub mod dash_handler;
pub mod disconnect_handler;
use bevy::prelude::*;
pub fn is_running(_app:&App)->bool{
    return true;
}