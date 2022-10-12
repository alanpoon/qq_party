use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use crate::bevy_wasmcloud_time;
pub fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,target_velocity:TargetVelocity){
  let mut guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };
  let mut app = guard;
  let mut query = app.world.query::<(Entity, &BallId,&Position)>();
  let bevy_wasmcloud_time_val = app.world.get_resource_mut::<bevy_wasmcloud_time::Time>().unwrap();
  let bevy_wasmcloud_time_val_clone = bevy_wasmcloud_time_val.clone();
  let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_)| {
    ball_id == _ball_id})
  .next();
  match local_ball {
    Some((entity, _,position)) => {
        let sa = sub_map_area(position.0.x,position.0.y);
        app.world.entity_mut(entity).insert(target_velocity);
        app.world.entity_mut(entity).insert(bevy_wasmcloud_time_val_clone);
        let server_message = ServerMessage::TargetVelocity{ball_id,target_velocity};
        match rmp_serde::to_vec(&server_message){
          Ok(b)=>{
            let p_msg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic.{}",sa)
              };
              publish_(p_msg);
            
          }
          _=>{

          }
        }
    }
    None => {
        info_(format!("target_velocity_handler cannot find ball_id {:?}",ball_id));
    }
  }
  
}
pub fn sub_map_area(x:f32,y:f32) ->String{
  let mut sub_map = String::from("C");
  if x > 1900.0 && y <1900.0{
    sub_map = String::from("D");
  }else if x > 1900.0 && y >= 1900.0{
    sub_map = String::from("B");
  }else if x <= 1900.0 && y >= 1900.0{
    sub_map = String::from("A");
  }
  sub_map
}