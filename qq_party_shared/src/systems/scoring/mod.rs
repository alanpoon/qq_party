use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::nalgebra::Vector2;
use crate::*;
use bevy_log::info;
use bevy_math::{Vec2};
use crate::systems::nalgebra::Point2;

pub fn score(mut cmd:&mut Commands,ball_id:u32,npc_entity:Entity,
  query:&Query<(Entity,&Parent)>,
  mut res: &mut ResMut<ScoreBoard>){
  if let Some(mut score) = res.scores.get_mut(&ball_id){
    if let Ok((npc_e,parent)) = query.get(npc_entity){
      inner_score_next(cmd,npc_e,query,&mut score);
    }
  }
}

fn inner_score_next(mut cmd:&mut Commands,npc_entity:Entity,query:&Query<(Entity,&Parent)>,mut score:&mut i16)->Option<()>{
  *score+=1;
  if let Ok((npc_e,parent)) = query.get(npc_entity){
    inner_score_next(cmd,parent.0,query,&mut score)
  }else{
    None
  }
}