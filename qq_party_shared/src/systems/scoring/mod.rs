use bevy::prelude::*;
use crate::*;

use super::entity_to_remove;

pub fn score(cmd:&mut Commands,ball_id:u32,lastnpc_entity:Entity,
  query:&Query<(Entity,&QQParent,&NPCId),Without<BallId>>,
  res: &mut ResMut<ScoreBoard>,mut to_despawn:&mut ResMut<entity_to_remove::EntityToRemove>){
  if let Some(mut score) = res.scores.get_mut(&ball_id){
    if let Ok((npc_e,_parent,npc_id)) = query.get(lastnpc_entity){
      inner_score_next(cmd,npc_e,npc_id,query,&mut score,&mut to_despawn);
    }
  }
}

fn inner_score_next(cmd:&mut Commands,npc_entity:Entity,npc_id:&NPCId,query:&Query<(Entity,&QQParent,&NPCId),Without<BallId>>,
mut score:&mut (i16,BallLabel),mut to_despawn:&mut ResMut<entity_to_remove::EntityToRemove>)->Option<()>{
  if npc_id.sprite_enum!=0{
    let ret= if let Ok((_npc_e,parent,npc_id)) = query.get(npc_entity){
      score.0+=1;
      //cmd.entity(npc_entity).despawn();
      to_despawn.entities.insert(npc_entity);
      inner_score_next(cmd,parent.0,npc_id,query,&mut score,&mut to_despawn)
    }else{
      info!("wierld  npc{:?} npc_id{:?}",npc_entity,npc_id);
      None
    };
    return ret
  }
  None
}