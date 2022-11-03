use bevy::prelude::*;
use bevy::utils::Duration;
use qq_party_shared::*;
use crate::*;

pub fn hide_display_ui(mut cooldown_timer:Query<(Entity,&mut CoolDownTimer)>,time:Res<Time>,mut to_despawn:ResMut<EntityToRemove>){
    for (e,mut cd) in cooldown_timer.iter_mut(){
        if cd.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
            to_despawn.entities.insert(e);
            match serde_json::to_string(&CoolDownMessage::DisplayUI(cd.1.clone())){
                Ok(j)=>{
                    push_web_bevy_events_fn2(&j);
                }
                Err(e)=>{
                    info!("push_web_bevy_events_fn2 error {:?}",e);
                }
            }
        }
    }
    
}