use bevy_ecs::prelude::*;
use crate::*;

pub fn score(mut cmd:&mut Commands,ball_id:u32,lastnpc_entity:Entity,
  query:&Query<(Entity,&Parent,&NPCId),Without<BallId>>,
  mut res: &mut ResMut<ScoreBoard>){
  if let Some(mut score) = res.scores.get_mut(&ball_id){
    if let Ok((npc_e,_parent,npc_id)) = query.get(lastnpc_entity){
      inner_score_next(cmd,npc_e,npc_id,query,&mut score);
    }
  }
}

fn inner_score_next(mut cmd:&mut Commands,npc_entity:Entity,npc_id:&NPCId,query:&Query<(Entity,&Parent,&NPCId),Without<BallId>>,
mut score:&mut (i16,BallLabel))->Option<()>{
  if npc_id.sprite_enum!=0{
    let ret= if let Ok((_npc_e,parent,npc_id)) = query.get(npc_entity){
      score.0+=1;
      cmd.entity(npc_entity).despawn();
      inner_score_next(cmd,parent.0,npc_id,query,&mut score)
    }else{
      None
    };
    return ret
  }
  None
}