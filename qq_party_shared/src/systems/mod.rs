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
pub fn update_state_velocity_npc(mut npc_query: Query<(&Position,&mut Velocity,&ChaseTargetId),(With<NPCId>,Without<BallId>)>,
  ball_query:Query<(&BallId,&Position,&Velocity)>){
  for (npc_pos,mut v,chase_target_id) in npc_query.iter_mut(){
    if chase_target_id.0 !=0{
      for (ball_id,pos,velocity) in ball_query.iter(){
        if chase_target_id.0 == ball_id.0{
          if chase_target_id.1 ==0{
            let unit_vec = (pos.0+2.0*velocity.0-npc_pos.0).normalize_or_zero();    
            v.0.x = unit_vec.x *chase_target_id.1 as f32 *unit_vec.length_recip();
            v.0.y = unit_vec.y *chase_target_id.1 as f32 *unit_vec.length_recip();
            break;
          }else{
            
          }
        }
      }
    }
  }
}
