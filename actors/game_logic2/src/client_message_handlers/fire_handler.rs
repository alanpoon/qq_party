use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn_fire;
use crate::client_message_handlers::target_velocity_handler::sub_map_area;
use wasmcloud_interface_messaging::{PubMessage};
use bevy::prelude::*;
use super::is_running;
use bevy_rapier2d::prelude::*;
use std::sync::{Arc, Mutex};

pub fn _fn (map:Arc<Mutex<App>>,ball_id:BallId,_velocity:QQVelocity,sprite_enum:u32){
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
    let mut query = app.world.query::<(Entity, &BallId,&Transform,&Velocity)>();
    let local_ball = query.iter(&app.world).filter(|(_, &_ball_id,_,_)| {
      ball_id == _ball_id})
    .next();
    let fire_bundle = match local_ball {
      Some((_, ball_id,t,vel)) => {
          let fire_bundle = FireBundle{fire_id:FireId(ball_id.0,ball_id.1,Some([t.translation.x,t.translation.y].into())),
            transform:Transform::from_xyz(t.translation.x, t.translation.y, 3.0),
            global_transform:GlobalTransform::identity(),
            rigid_body:RigidBody::Dynamic,
            velocity:Velocity { linvel: vel.linvel *2.0, ..Default::default() },
          };
          
          Some(fire_bundle)
      }
      _=>None
    };
    if let Some(fire_bundle) = fire_bundle{
      
      let sa = sub_map_area(fire_bundle.transform.translation.x,fire_bundle.transform.translation.y);
      spawn_fire(&mut app.world,fire_bundle.clone());
      let server_message = ServerMessage::Fire{ball_id:ball_id.clone(),velocity:
        QQVelocity(Vec2::new(fire_bundle.velocity.linvel.x,fire_bundle.velocity.linvel.y)),
        sprite_enum};
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
}