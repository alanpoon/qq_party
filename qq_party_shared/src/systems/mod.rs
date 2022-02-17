#[cfg(feature = "non_actor")]
use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::component::Component;
use bevy_log::info;
use crate::time_interface;
use crate::{TargetVelocity,Velocity,Time,BallId,Position};

pub fn update_velocity_system(mut query: Query<(&mut Velocity, &TargetVelocity)>, time: Res<Time>) {
  //let delta = time.delta_seconds();
  let delta = 2.0;
  let speed = 2.0;

  for (mut velocity, target_velocity) in query.iter_mut() {
      velocity.0 = velocity.0 * (1.0 - delta * speed) + target_velocity.0 * (delta * speed);
  }
}
pub fn auto_target_velocity<X:time_interface::TimeInterface + Component>(mut query: Query<&mut TargetVelocity>, time: Res<X>) {
  let delta = time.delta_seconds();
  info!("delta {:?}",delta);

  for mut tv in query.iter_mut() {
      //pos.0 += vel.0 * time.delta_seconds() * 5.0;
      tv.0 +=  time.delta_seconds() * 50.0;
  }
}
pub fn update_state_position<X:time_interface::TimeInterface + Component>(mut query: Query<&mut Position,&mut Velocity>, time: Res<X>) {
  let delta = time.delta_seconds();
  for (mut pos,mut vel) in query.iter_mut() {
      //pos.0 += vel.0 * time.delta_seconds() * 5.0;
      pos.0.x +=  time.delta_seconds() * vel.0.x;
      pos.0.y +=  time.delta_seconds() * vel.0.y;
  }
}
pub fn update_state_velocity(mut query: Query<&mut Velocity,&mut TargetVelocity>){
  for (mut v,mut tv) in query.iter_mut() {
    if tv.0.x *50.0!=0.0{
      v.0.x = tv.0.x *50.0;
    }
    if tv.0.y *50.0!=0.0{
      v.0.y = tv.0.y * 50.0;
    }
    *tv = TargetVelocity(Vec2::ZERO);
  }
}