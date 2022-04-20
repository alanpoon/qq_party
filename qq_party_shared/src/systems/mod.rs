// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::*;
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::*;
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::component::Component;
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::nalgebra::Vector2;
use bevy_math::{Vec2};
use bevy_log::info;
mod trail;
pub mod physics;
pub use physics::*;
use crate::time_interface;
use crate::{TargetVelocity,Velocity,Time,BallId,Position,ChaseTargetId,NPCId};

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
pub fn update_state_velocity_npc(mut npc_query: Query<(&Position,&mut Velocity,&ChaseTargetId),(With<NPCId>,Without<BallId>)>,
  ball_query:Query<(&BallId,&Position,&Velocity)>){
    //info!("update_state_velocity_npc");
    for (npc_pos,mut v,chase_target_id) in npc_query.iter_mut(){
      if chase_target_id.0 !=0{
        for (ball_id,pos,velocity) in ball_query.iter(){
          if chase_target_id.0 == ball_id.0{
            //let unit_vec = (pos.0+4.0*velocity.0-npc_pos.0).normalize_or_zero();
            let unit_vec = (pos.0+2.0*velocity.0-npc_pos.0).normalize_or_zero();
            //let unit_vec = (pos.0-npc_pos.0).normalize_or_zero();
            //info!("unit_vec{:?} {:?}",unit_vec,1.0/unit_vec.x);
            // v.0.x = unit_vec.x *chase_target_id.1 as f32 * 1.0/unit_vec.x;
            // v.0.y = unit_vec.y *chase_target_id.1 as f32 * 1.0/unit_vec.y;
            v.0.x = unit_vec.x *chase_target_id.1 as f32 *unit_vec.length_recip();
            v.0.y = unit_vec.y *chase_target_id.1 as f32 *unit_vec.length_recip();
            break;
          }
        }
      }
    }
}
pub fn set_state_chasetarget_npc(mut npc_query: Query<(&NPCId,&Position,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
  ball_query:Query<(&BallId,&Position)>){
    //info!("set_state_chasetarget_npc");

    for (npc_id,npc_pos,mut v,mut chase_target_id) in npc_query.iter_mut(){
      //info!("set_state_chasetarget_npc npc_query");

      let speed:Option<u8> = match npc_id.sprite_enum{
        0=>{
          Some(70)
        }
        1=>{
          None
        }
        2=>{
          Some(70)
        }
        _=>{
          None
        }
      };
      if let Some(s) = speed{
        for (ball_id,pos) in ball_query.iter(){
          if pos.0.distance(npc_pos.0)<50.0{
            *chase_target_id = ChaseTargetId(ball_id.0,s);
            //info!("set_state_chasetarget_npc npc_query found");
          }
        }
      } 
    }
}