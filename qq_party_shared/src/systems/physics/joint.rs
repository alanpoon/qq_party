use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::nalgebra::Vector2;
use crate::*;
use bevy_log::info;
use bevy_math::{Vec2};
use crate::systems::nalgebra::Point2;

pub fn set_state_chasetarget_npc2(mut cmd:Commands,mut npc_query: Query<(Entity,&NPCId,&Position),(Without<BallId>,Without<ChaseTargetId2>)>,
mut ball_query:Query<(Entity,&BallId,&Position)>){    
  for (npc_e,npc_id,npc_pos) in npc_query.iter_mut(){
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
      for (ball_e,ball_id,pos) in ball_query.iter_mut(){
        if pos.0.distance(npc_pos.0)<50.0{
          cmd.entity(npc_e).insert(ChaseTargetId2(ball_id.0,Some(ball_e)));
        }
      }
    }
  }
}
pub fn spawn_hierachy(
  mut cmd: Commands,
  mut npc_query: Query<(Entity,&NPCId,&Position,&ChaseTargetId2),Changed<ChaseTargetId2>>,
  mut ball_query:Query<(Entity,&BallId,&mut LastNPC)>
) {
  for (npc_e,npc_id,npc_pos, chase_target_id) in npc_query.iter_mut(){
    if chase_target_id.0 !=0{
      for (ball_e,ball_id,mut last_npc) in ball_query.iter_mut(){
        if chase_target_id.0 == ball_id.0{
          if let Some(ln)=last_npc.1{
            cmd.entity(npc_e).insert(Parent(ln));
          }else{
            cmd.entity(npc_e).insert(Parent(ball_e));
          }
          *last_npc = LastNPC(npc_id.id,Some(npc_e));
          break;
        }
      }
    }
  }
}
pub fn spawn_joint(
  mut cmd: Commands,
  mut npc_query: Query<(Entity,&NPCId,&Position,&mut Velocity,&Parent)>,
  mut position_query: Query<&Position>,
){
  for (npc_e,npc_id,npc_pos,mut v,parent) in npc_query.iter_mut(){
    
    if let Ok(pos) = position_query.get(parent.0) {
      let unit_vec = (pos.0-npc_pos.0).normalize_or_zero();   
      let dist =  pos.0.distance_squared(npc_pos.0);
      let mut factor = 70.0;
      if dist<400.0{
        factor =0.0;
      }
      v.0.x = unit_vec.x *factor *unit_vec.length_recip();
      v.0.y = unit_vec.y  *factor *unit_vec.length_recip();
    }
  }
}
// fn rotate(
//   mut commands: Commands,
//   time: Res<Time>,
//   mut parents_query: Query<(Entity, &Children), With<Sprite>>,
//   mut transform_query: Query<&mut Transform, With<Sprite>>,
// ) {
//
//if let Ok(mut transform) = transform_query.get_mut(*child) {



