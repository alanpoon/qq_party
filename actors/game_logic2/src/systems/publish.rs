use wasmcloud_interface_messaging::PubMessage;
use bevy_ecs::prelude::*;
use qq_party_shared::*;
use crate::info_::info_;
use crate::Time;
use crate::bevy_wasmcloud_time;
use crate::messaging_::publish_;
pub fn sys_publish_game_state(mut elapsed_time:ResMut<Time>,bevy_wasmcloud_time_val:Res<bevy_wasmcloud_time::Time>,
  query: Query<(&BallId,&Position,&Velocity,&TargetVelocity)>,
  npc_query: Query<(&NPCId,&Position,&Velocity,&ChaseTargetId)>) {
  if (*elapsed_time).elapsed > 5.0{
    (*elapsed_time).elapsed = 0.0;
    let mut ball_bundles =vec![];
    let mut npc_bundles = vec![];
    for (ball_id,position,velocity,target_velocity) in query.iter(){
      ball_bundles.push(BallBundle{ball_id:ball_id.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()});
    }
    for (npc_id,position,velocity,chase_target) in npc_query.iter(){
      npc_bundles.push(NPCBundle{npc_id:npc_id.clone(),position:position.clone(),velocity:velocity.clone(),chase_target:chase_target.clone()});
    }
    let channel_message_back = ServerMessage::GameState{ball_bundles,npc_bundles:npc_bundles,timestamp:(*bevy_wasmcloud_time_val).timestamp};

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
  // info_(format!("bevy_wasmcloud_time delta_seconds {:?} {:?}",(*bevy_wasmcloud_time_val).delta_seconds, (*elapsed_time).elapsed));
  (*elapsed_time).elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
}