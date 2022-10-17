use qq_party_shared::*;
use bevy_rapier2d::prelude::*;
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
use crate::bevy_wasmcloud_time;
pub fn _fn(map:Arc<Mutex<App>>,ball_id:BallId)-> RpcResult<()>{
  let mut guard = match map.lock() {
      Ok(guard) => guard,
      Err(poisoned) => {
        poisoned.into_inner()
      },
    };
    let mut app = guard;
    let mut query = app.world.query::<(Entity, &BallId,&QQVelocity,&Position)>();
    let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_,_)| {
      ball_id == _ball_id})
    .next();
    match local_ball {
      Some((entity, ball_id,vel,position)) => {
          let sa = sub_map_area(position.0.x,position.0.y);
          let ball_id_c = ball_id.clone();
          let vel_c = vel.clone();
          app.world.entity_mut(entity).insert(Dash(true,vel_c.0*2.0,vel_c.0));
          app.world.entity_mut(entity).insert(DashTimer(Timer::from_seconds(1.0, false)));
          
          let server_message = ServerMessage::Dash{ball_id:ball_id_c};
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
      }
      _=>{}
    }
    
    Ok(())
}