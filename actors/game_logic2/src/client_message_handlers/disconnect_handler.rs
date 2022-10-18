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
pub fn _fn(map:Arc<Mutex<App>>,ball_id_secret:String){
  let mut guard = match map.lock() {
      Ok(guard) => guard,
      Err(poisoned) => {
        poisoned.into_inner()
      },
    };
    let mut app = guard;
    let mut query = app.world.query::<(Entity, &BallId,&Position)>();
    let mut ball_to_despawn:Option<Entity> = None;
    match ball_id_secret.parse::<u32>(){
      Ok(ball_id)=>{
        let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_)| {
          ball_id == _ball_id.0})
        .next();
        match local_ball {
          Some((entity, ball_id,position)) => {
              //despawn
              info_(format!("..despawn ball_id{:?}",ball_id));
              ball_to_despawn = Some(entity.clone());
              let sa = sub_map_area(position.0.x.clone(),position.0.y.clone());
              let server_message = ServerMessage::Disconnect{ball_id:ball_id.0.clone()};
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
      },
      _=>{}
    }
    if let Some(entity) = ball_to_despawn{
      app.world.despawn(entity);
    }
    
}