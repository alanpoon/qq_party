use wasmcloud_interface_messaging::PubMessage;
use qq_party_shared::*;
use crate::info_::info_;
use crate::Time;
use crate::bevy_wasmcloud_time;
use crate::messaging_::publish_;
use bevy_ecs_wasm::prelude::{Query,Res,ResMut};
pub fn sys_publish_game_state(mut elapsed_time:ResMut<Time>,bevy_wasmcloud_time_val:Res<bevy_wasmcloud_time::Time>,query: Query<(&BallId,&Position,&Velocity,&TargetVelocity)>) {
  if (*elapsed_time).elapsed > 5.0{
    (*elapsed_time).elapsed = 0.0;
    let mut ball_bundles =vec![];
    for (ball_id,position,velocity,target_velocity) in query.iter(){
      ball_bundles.push(BallBundle{ball_id:ball_id.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()});
    }
    info_(format!("publish gamestate {:?}",ball_bundles.clone()));

    let channel_message_back = ServerMessage::GameState{ball_bundles,timestamp:(*bevy_wasmcloud_time_val).timestamp};

    match serde_json::to_vec(&channel_message_back){
      Ok(b)=>{
        let pMsg = PubMessage{
          body:b,
          reply_to: None,
          subject: format!("game_logic")
        };
        publish_(pMsg);
      }
      _=>{}
    }
    return
  }
  info_(format!("bevy_wasmcloud_time delta_seconds {:?} {:?}",(*bevy_wasmcloud_time_val).delta_seconds, (*elapsed_time).elapsed));
  (*elapsed_time).elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
}