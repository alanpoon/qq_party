pub mod target_velocity_handler;
pub mod welcome_handler;
pub mod change_sub_map_handler;
pub mod fire_handler;
pub mod dash_handler;
pub mod disconnect_handler;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn is_running(app:&App)->bool{
    if let Some(st) = app.world.get_resource::<StateTransformer>(){
        match st.1{
            QQState::Running|QQState::StopNotification=>{
                return true;
            }
            _=>{}
        }
    }
    return false;
}