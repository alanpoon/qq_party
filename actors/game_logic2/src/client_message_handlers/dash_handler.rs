use qq_party_shared::*;
use crate::messaging_::publish_;
use crate::client_message_handlers::target_velocity_handler::sub_map_area;
use wasmcloud_interface_messaging::{PubMessage};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::is_running;
use std::sync::{Arc, Mutex};
pub fn _fn(map:Arc<Mutex<App>>,ball_id:BallId){
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
    let mut query = app.world.query::<(Entity, &BallId,&Velocity,&Transform)>();
    let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_,_)| {
      ball_id == _ball_id})
    .next();

    match local_ball {
      Some((entity, ball_id,vel,transform)) => {
          let sa = sub_map_area(transform.translation.x,transform.translation.y);
          let ball_id_c = ball_id.clone();
          let vel_c = vel.clone();
          app.world.entity_mut(entity).insert(Dash(true,vel_c.linvel*2.0,vel_c.linvel));
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
}