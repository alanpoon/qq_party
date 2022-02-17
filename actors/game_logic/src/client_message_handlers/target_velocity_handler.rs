use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use wasmcloud_interface_logging::{info,error,debug};

pub fn _fn (map:Arc<Mutex<HashMap<String,(Schedule,World)>>>,game_id:String,ball_id:BallId,target_velocity:TargetVelocity){
  let mut guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };
  if let Some((ref mut s, ref mut w))= guard.get_mut(&game_id){
    if let Some(mut tv) = w.get_resource_mut::<TargetVelocity>(){
      *tv = target_velocity;
    }else{
      w.insert_resource(target_velocity);
    }
  }
 // let b = serde_json::to_vec(&a.clone())?;
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
    _=>{}
  }
}