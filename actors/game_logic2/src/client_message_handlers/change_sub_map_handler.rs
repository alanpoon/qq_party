use qq_party_shared::*;
use crate::info_::info_;
use super::is_running;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
pub fn _fn (map:Arc<Mutex<App>>,_game_id:String,ball_id:BallId,position:Position){
  let guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };
  let mut app = guard;
  if !is_running(&app){
    return ;
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