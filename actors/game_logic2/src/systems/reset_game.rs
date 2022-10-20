use qq_party_shared::*;
use bevy::prelude::*;
use bevy::utils::Duration;
use crate::messaging_::publish_;
use crate::info_::info_;
use crate::startup::ResetGameTimer;
use wasmcloud_interface_messaging::{PubMessage};
pub fn reset_game_timer_system(
    mut cmd: Commands,
    mut ball_query: Query<(Entity,&BallId)>,
    mut npc_query: Query<(Entity,&NPCId)>,
    mut storm_query: Query<(Entity,&StormRingId)>,
    mut timer_query: Query<&mut ResetGameTimer>,
    mut res: ResMut<ScoreBoard>,
    time:Res<Time>){
    for mut timer in timer_query.iter_mut(){
        if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
            for (e,_) in ball_query.iter(){
                cmd.entity(e).despawn();
            }
            for (e,_) in npc_query.iter(){
                cmd.entity(e).despawn();
            }
            for (e,_) in storm_query.iter(){
                cmd.entity(e).despawn();
            }
            
            *res = ScoreBoard::default();
            let server_message = ServerMessage::ResetGame{};
            info_(format!("reset game"));
            match rmp_serde::to_vec(&server_message){
              Ok(b)=>{
                let p_msg = PubMessage{
                  body:b,
                  reply_to: None,
                  subject: String::from("game_logic.reset")
                };
                publish_(p_msg);
              }
              _=>{}
            }
        }
    }
}