use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use crate::bevy_wasmcloud_time;
use crate::startup::IsRunning;
pub fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,position:Position){
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
  let mut query = app.world.query::<(Entity, &BallId)>();
  let local_ball = query.iter(&app.world).filter(|(_, &_ball_id)| {
    ball_id == _ball_id})
  .next();
  match local_ball {
    Some((entity, _)) => {
        app.world.entity_mut(entity).insert(position);
    }
    None => {
        info_(format!("target_velocity_handler cannot find ball_id {:?}",ball_id));
    }
  }
}