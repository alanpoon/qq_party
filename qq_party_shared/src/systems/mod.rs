use bevy_ecs::prelude::*;
use bevy_math::{Vec2};
pub mod physics;
pub use physics::*;
pub mod scoring;
use crate::*;

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