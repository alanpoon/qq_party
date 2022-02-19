use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy_ecs_wasm::prelude::{Schedule,World,Entity,Query,SystemStage,IntoSystem,Res};
use wasmcloud_interface_logging::{info,error,debug};

pub fn _fn (map:Arc<Mutex<HashMap<String,(Schedule,World)>>>,game_id:String,ball_id:BallId,target_velocity:TargetVelocity){
  let mut guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };

  if let Some((ref mut s, ref mut w))= guard.get_mut(&game_id){
    info_(format!("target_velocity_handler guarded{:?}",ball_id));
    // for (e,b) in q.iter(w){
    //   info_(format!("q.iter {:?}",b));
    //   if b==&ball_id{
    //     info_(format!("added target_velocity {:?} with {:?}",b,target_velocity));
    //     w.entity_mut(e).insert(target_velocity);
    //     break;
    //   }
    // }
    let mut query = w.query::<(Entity, &BallId)>();
    let local_ball = query.iter(w).filter(|(_, &_ball_id)| {
      info_(format!("filter {:?}",_ball_id));
      ball_id == _ball_id})
    .next();
    match local_ball {
      Some((entity, _)) => {
          w.entity_mut(entity).insert(target_velocity);
          info_(format!("target_velocity_handler can find ball_id {:?}",ball_id));
          let server_message = ServerMessage::TargetVelocity{ball_id,target_velocity};
          match serde_json::to_vec(&server_message){
            Ok(b)=>{
              let pMsg = PubMessage{
                body:b,
                reply_to: None,
                subject: "game_logic".to_owned()
                };
                publish_(pMsg);
            }
            _=>{

            }
          }
      }
      None => {
          info_(format!("target_velocity_handler cannot find ball_id {:?}",ball_id));
          // cmd.spawn_bundle(BallBundle::new(your_ball_id))
          //     .insert(target_velocity);
      }
    }
  }
 // let b = serde_json::to_vec(&a.clone())?;
  
}