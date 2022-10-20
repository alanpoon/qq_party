use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn_fire;
use crate::client_message_handlers::target_velocity_handler::sub_map_area;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_logging::{info,error,debug};
use bevy::math::Vec2;
use wasmbus_rpc::actor::prelude::*;

pub fn _fn (map:Arc<Mutex<App>>,ball_id:BallId,velocity:QQVelocity,sprite_enum:u32){
  let guard = match map.lock() {
      Ok(guard) => guard,
      Err(poisoned) => {
        poisoned.into_inner()
      },
    };
    let mut app = guard;
    let is_running = app.world.get_resource::<IsRunning>().unwrap();
    if !is_running.0{
      return;
    }
    let mut query = app.world.query::<(Entity, &BallId,&Position,&QQVelocity)>();
    let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_,_)| {
      ball_id == _ball_id})
    .next();
    let bevy_wasmcloud_time_val = app.world.get_resource::<crate::bevy_wasmcloud_time::Time>().unwrap();
    let bevy_wasmcloud_time_val_clone = bevy_wasmcloud_time_val.clone();
    let fire_bundle = match local_ball {
      Some((entity, ball_id,position,vel)) => {
          let fire_bundle = FireBundle{fire_id:FireId(ball_id.0,ball_id.1,Some(position.0.clone())),position:position.clone(),
            velocity:QQVelocity(vel.0*2.0),
            //start:Time{elapsed:bevy_wasmcloud_time_val_clone.timestamp as f32  }
          };
          
          Some(fire_bundle)
      }
      _=>None
    };
    if let Some(fire_bundle) = fire_bundle{
      
      let sa = sub_map_area(fire_bundle.position.0.x,fire_bundle.position.0.y);
      let server_message = ServerMessage::Fire{ball_id:ball_id.clone(),velocity:fire_bundle.velocity.clone(),sprite_enum,timestamp:bevy_wasmcloud_time_val_clone.timestamp};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          let p_msg = PubMessage{
            body:b,
            reply_to: None,
            subject: format!("game_logic.{}",sa)
            };
          publish_(p_msg);
        }
        _=>{}
      }
      spawn_fire(&mut app.world,fire_bundle);
    }
}