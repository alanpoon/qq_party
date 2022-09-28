use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn_fire;
use crate::client_message_handlers::target_velocity_handler::sub_map_area;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use bevy_app::App;
use std::sync::{Arc, Mutex};
use bevy_ecs::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use bevy_math::Vec2;
use wasmbus_rpc::actor::prelude::*;

pub fn _fn2 (map:Arc<Mutex<App>>,ball_id:BallId,velocity:Velocity,sprite_enum:u32)-> RpcResult<()>{
  info_(format!("firing from "));  
  let mut guard = match map.lock() {
      Ok(guard) => guard,
      Err(poisoned) => {
        poisoned.into_inner()
      },
    };
    let mut app = guard;
    let mut query = app.world.query::<(Entity, &BallId,&Position,&Velocity)>();
    let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_,_)| {
      ball_id == _ball_id})
    .next();
    let bevy_wasmcloud_time_val = app.world.get_resource::<crate::bevy_wasmcloud_time::Time>().unwrap();
    let bevy_wasmcloud_time_val_clone = bevy_wasmcloud_time_val.clone();
    let fire_bundle = match local_ball {
      Some((entity, ball_id,position,velocity)) => {
          let fire_bundle = FireBundle{fire_id:FireId(sprite_enum),owner:ball_id.clone(),position:position.clone(),
            velocity:Velocity(velocity.0*1.3),start:Time{elapsed:bevy_wasmcloud_time_val_clone.timestamp as f32}};
          
          Some(fire_bundle)
      }
      _=>None
    };
    if let Some(fire_bundle) = fire_bundle{
      let sa = sub_map_area(fire_bundle.position.0.x,fire_bundle.position.0.y);
      let server_message = ServerMessage::Fire{ball_id:ball_id.clone(),velocity:velocity.clone(),sprite_enum,timestamp:bevy_wasmcloud_time_val_clone.timestamp};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          let pMsg = PubMessage{
            body:b,
            reply_to: None,
            subject: format!("game_logic.{}",sa)
            };
          publish_(pMsg);
        }
        _=>{}
      }
      spawn_fire(&mut app.world,fire_bundle);
    }
    Ok(())
}