use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::nalgebra::Vector2;
use crate::*;
use bevy_log::info;
use bevy_math::{Vec2};
use crate::systems::nalgebra::Point2;

pub fn score(mut cmd:&mut Commands,ball_id:u32,lastnpc_entity:Entity,
  query:&Query<(Entity,&Parent,&NPCId),Without<BallId>>,
  mut res: &mut ResMut<ScoreBoard>){
  if let Some(mut score) = res.scores.get_mut(&ball_id){
    if let Ok((npc_e,parent,npc_id)) = query.get(lastnpc_entity){
      inner_score_next(cmd,npc_e,npc_id,query,&mut score);
    }
  }
}

fn inner_score_next(mut cmd:&mut Commands,npc_entity:Entity,npc_id:&NPCId,query:&Query<(Entity,&Parent,&NPCId),Without<BallId>>,
mut score:&mut i16)->Option<()>{
  if npc_id.sprite_enum!=0{
    let ret= if let Ok((npc_e,parent,npc_id)) = query.get(npc_entity){
      *score+=1;
      cmd.entity(npc_entity).despawn();
      inner_score_next(cmd,parent.0,npc_id,query,&mut score)
    }else{
      None
    };
    return ret
  }
  None
}