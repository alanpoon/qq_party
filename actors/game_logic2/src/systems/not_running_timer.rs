use qq_party_shared::*;
use bevy::prelude::*;
use bevy::utils::Duration;
use crate::messaging_::publish_;
use crate::info_::info_;
use crate::startup::{RunningTimer,NotRunningTimer,IsRunning};
use wasmcloud_interface_messaging::{PubMessage};
pub fn system(
    mut cmd: Commands,
    mut timer_query: Query<(Entity,&mut NotRunningTimer)>,
    mut is_running: ResMut<IsRunning>,
    time:Res<Time>){
    if !(*is_running).0{
      for (e,mut timer) in timer_query.iter_mut(){
          if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
              (*is_running).0 = true;
              cmd.spawn().insert(RunningTimer(Timer::new(Duration::new(20,0),false)));
              cmd.entity(e).despawn();
              //info_(format!("running game"));
              if let Ok(npc)=crate::startup::npc::spawn_npc_bundles_sync(){
                cmd.spawn_batch(npc_bundles);
              }
              cmd.spawn().insert(crate::startup::storm_ring::spawn_storm_ring(3400.0,3400.0,80));
              let server_message = ServerMessage::StartGameTiming{next_start_game_timing:Some(StartGameTiming())};
              match rmp_serde::to_vec(&server_message){
                Ok(b)=>{
                  let p_msg = PubMessage{
                    body:b,
                    reply_to: None,
                    subject: String::from("game_logic.start")
                  };
                  publish_(p_msg);
                }
                _=>{}
              }
          }
      }
  }

}