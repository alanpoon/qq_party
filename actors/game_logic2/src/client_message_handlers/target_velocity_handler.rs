use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{PubMessage};
use std::sync::{Arc, Mutex};
use super::is_running;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub fn _fn (map:Arc<Mutex<App>>,_game_id:String,ball_id:BallId,target_velocity:TargetVelocity){
  let  guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };
  let mut app = guard;
  if !is_running(&app){
    return ;
  }

  let mut velocity_query= app.world.query::<(&BallId,&Transform,&mut Velocity)>();
  for (gball_id,transform,mut velocity) in velocity_query.iter_mut(&mut app.world){
    if gball_id.0 ==ball_id.0{
      let sa = sub_map_area(transform.translation.x,transform.translation.y);
      update::target_velocity::velocity(&mut velocity, target_velocity.clone());
      info_(format!("pos x: {:?} y:{:?} vel {:?}",transform.translation.x,transform.translation.y,velocity.linvel));

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
      break;
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