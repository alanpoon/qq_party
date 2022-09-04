use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::nalgebra::Vector2;
use bevy_math::{Vec2};
use bevy_log::info;
mod trail;
pub mod physics;
pub use physics::*;
pub mod scoring;
use crate::time_interface;
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

pub fn set_state_chasetarget_npc(mut npc_query: Query<(&NPCId,&Position,&mut ChaseTargetId),Without<BallId>>,
  mut ball_query:Query<(&BallId,&Position,&mut LastNPC)>){    
    // for (npc_id,npc_pos,mut chase_target_id) in npc_query.iter_mut(){
    //   let speed:Option<u8> = match npc_id.sprite_enum{
    //     0=>{
    //       Some(70)
    //     }
    //     1=>{
    //       None
    //     }
    //     2=>{
    //       Some(70)
    //     }
    //     _=>{
    //       None
    //     }
    //   };
    //   if let Some(s) = speed{
    //     for (ball_id,pos,mut last_npc) in ball_query.iter_mut(){
    //       if pos.0.distance(npc_pos.0)<50.0{
    //         *chase_target_id = ChaseTargetId(ball_id.0,s);
    //        // *last_npc = LastNPC(npc_id.id,None);
    //       }
    //     }
    //   }
    // }
}