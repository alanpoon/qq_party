#[cfg(feature = "non_actor")]
use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::component::Component;
use bevy_math::{Vec2};
use bevy_log::info;
use crate::time_interface;
use crate::{TargetVelocity,Velocity,Time,BallId,Position};

pub fn update_state_position<X:time_interface::TimeInterface + Component>(mut query: Query<(&mut Position,&mut Velocity)>, time: Res<X>) {
  let delta = time.delta_seconds();
  for (mut pos,mut vel) in query.iter_mut() {
      //pos.0 += vel.0 * time.delta_seconds() * 5.0;
    if (pos.0.x<=20.0 && vel.0.x >0.0) || (pos.0.x>=3820.0 && vel.0.x <0.0) || (pos.0.x>=20.0 && pos.0.x <= 3820.0){
      pos.0.x +=  delta * vel.0.x;
    }
    if (pos.0.y<=20.0 && vel.0.y >0.0) || (pos.0.y>=3820.0 && vel.0.y <0.0) || (pos.0.y>=20.0 && pos.0.y <= 3820.0){
      pos.0.y +=  delta * vel.0.y;
    }
  }
}
pub fn update_state_velocity(mut query: Query<(&mut Velocity,&mut TargetVelocity)>){
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