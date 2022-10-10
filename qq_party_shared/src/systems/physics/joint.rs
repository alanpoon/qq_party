use bevy::prelude::*;
use crate::*;

pub fn set_state_chasetarget_npc2(mut cmd:Commands,mut npc_query: Query<(Entity,&NPCId,&Position),(Without<BallId>,Without<ChaseTargetId2>)>,
mut ball_query:Query<(Entity,&BallId,&Position,&mut LastNPC)>,
query_scoring:Query<(Entity,&QQParent,&NPCId),Without<BallId>>,mut res:ResMut<ScoreBoard>){    
  for (npc_e,npc_id,npc_pos) in npc_query.iter_mut(){
    let mut is_crate = false;
    let speed:Option<u8> = match npc_id.sprite_enum{
      0=>{
        Some(70)
      }
      1=>{
        is_crate = true;
        None
      }
      2=>{
        Some(70)
      }
      _=>{
        None
      }
    };
    
    for (ball_e,ball_id,pos,mut last_npc) in ball_query.iter_mut(){
      if let Some(s) = speed{
        if pos.0.distance(npc_pos.0)<50.0{
          cmd.entity(npc_e).insert(ChaseTargetId2(ball_id.0,Some(ball_e),s));
        }
      }else if is_crate{
        if let Some(last_npc_e) = last_npc.1{ 
          if pos.0.distance(npc_pos.0)<25.0{
            crate::systems::scoring::score(&mut cmd,ball_id.0,last_npc_e,&query_scoring,&mut res);
            *last_npc = LastNPC(0,None);
          }
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
  for (npc_e,npc_id,_npc_pos, chase_target_id) in npc_query.iter_mut(){
    if chase_target_id.0 !=0{
      for (ball_e,ball_id,mut last_npc) in ball_query.iter_mut(){
        if chase_target_id.0 == ball_id.0{
          if let Some(ln)=last_npc.1{
            cmd.entity(npc_e).insert(QQParent(ln));
          }else{
            cmd.entity(npc_e).insert(QQParent(ball_e));
          }
          if npc_id.sprite_enum!=0{ //not snake
            *last_npc = LastNPC(npc_id.id,Some(npc_e));
          }
          break;
        }
      }
    }
  }
}
pub fn spawn_joint(
  mut cmd: Commands,
  mut npc_query: Query<(Entity,&NPCId,&Position,&mut QQVelocity,&QQParent,&ChaseTargetId2)>,
  position_query: Query<&Position>,
  last_npc_query:Query<(Entity,&NPCId,&QQParent)>,
  mut ball_query:Query<(&BallId,&mut LastNPC)>
){
  for (npc_e,npc_id,npc_pos,mut v,parent,chase_target) in npc_query.iter_mut(){
    if let Ok(pos) = position_query.get(parent.0) {
      let unit_vec = (pos.0-npc_pos.0).normalize_or_zero();   
      let dist =  pos.0.distance_squared(npc_pos.0);
      let mut factor = 70.0;
      if dist<300.0{
        factor =0.0;
        if npc_id.sprite_enum == 0{
          for (ball_id,mut last_npc) in ball_query.iter_mut(){
            if ball_id.0 == chase_target.0{
              if last_npc.0!=0 && last_npc.1.is_some(){
                if let Ok((ln_e,_npc_id_,ln_parent))=last_npc_query.get(last_npc.1.unwrap()){
                  *last_npc = LastNPC(last_npc.0,Some(ln_parent.0));
                  //*last_npc.0 = ln_parent.0;
                  cmd.entity(ln_e).despawn(); //despawn last npc
                }
              }
              break;
            }
          }
          cmd.entity(npc_e).despawn(); //despawn snake
        }
      }
      v.0.x = unit_vec.x *factor *unit_vec.length_recip();
      v.0.y = unit_vec.y  *factor *unit_vec.length_recip();
      
    }
    // if let Some(ball_entity) = chase_target.1{
    //   if let Ok((ball_id,last_npc))= ball_query.get(ball_entity){
    //     if last_npc.1.is_none(){
    //       cmd.entity(npc_e).despawn();
    //     }
    //   }
    // }
  }
}