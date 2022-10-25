
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
pub mod physics;
pub use physics::*;
pub mod scoring;
pub mod entity_to_remove;
pub use entity_to_remove::*;
use crate::*;

pub fn update_physics_velocity(mut query: Query<(&mut Velocity,&mut TargetVelocity),Changed<TargetVelocity>>){
  for (mut v,tv) in query.iter_mut() {

      let f = if tv.0.x * tv.0.x+tv.0.y * tv.0.y>=2.0{
        1.0
      } else{
        std::f32::consts::SQRT_2
      };
      v.linvel.x = tv.0.x *50.0 * f;
      v.linvel.y = tv.0.y * 50.0 * f;
    // if tv.0.x *50.0!=0.0{
    //   v.linvel.x = tv.0.x *50.0;
    // }
    // if tv.0.y *50.0!=0.0{
    //   v.linvel.y = tv.0.y * 50.0;
    // }
    
    //*tv = TargetVelocity(Vec2::ZERO);
  }
}

pub fn update_state_velocity(mut query: Query<(&Velocity,&mut QQVelocity)>){
  for (v,mut qv) in query.iter_mut() {
    *qv = QQVelocity(v.linvel.into())
  }
}
pub fn add_physics_velocity(mut cmd:Commands,mut query: Query<(Entity,&QQVelocity),Without<Velocity>>){
  for (e,v) in query.iter_mut() {
    cmd.entity(e).insert(Velocity::linear([v.0.x,v.0.y].into()));
  }
}