use bevy::prelude::*;
use crate::*;
use crate::systems::entity_to_remove;
pub fn set_state_chasetarget_npc2(mut cmd:Commands,mut npc_query: Query<(Entity,&NPCId,&Transform),(Without<BallId>,Without<ChaseTargetId2>)>,
mut ball_query:Query<(Entity,&BallId,&Transform,&mut LastNPC)>,
query_scoring:Query<(Entity,&QQParent,&NPCId),Without<BallId>>,
mut res:ResMut<ScoreBoard>,
mut to_despawn:ResMut<entity_to_remove::EntityToRemove>){    
  for (ball_e,ball_id,t,mut last_npc) in ball_query.iter_mut(){
    let mut is_near_crate= false;
    for (npc_e,npc_id,npc_t) in npc_query.iter_mut(){
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
      if let Some(s) = speed{
        if t.translation.distance(npc_t.translation)<50.0 && !last_npc.2{
          cmd.entity(npc_e).insert(ChaseTargetId2(ball_id.0,Some(ball_e),s));
        }
      }else if is_crate{
        if let Some(last_npc_e) = last_npc.1{
          if t.translation.distance(npc_t.translation)<25.0{
            crate::systems::scoring::score(&mut cmd,ball_id.0,last_npc_e,&query_scoring,&mut res,&mut to_despawn);
            *last_npc = LastNPC(0,None,true);
            is_near_crate = true;
          }
        }
      }
    }
    if is_near_crate{
      *last_npc = LastNPC(last_npc.0,last_npc.1,true);
    }else{
      *last_npc = LastNPC(last_npc.0,last_npc.1,false);
    }
  }
}
pub fn spawn_hierachy(
  mut cmd: Commands,
  mut npc_query: Query<(Entity,&NPCId,&ChaseTargetId2),Changed<ChaseTargetId2>>,
  mut ball_query:Query<(Entity,&BallId,&mut LastNPC)>
) {
  for (npc_e,npc_id, chase_target_id) in npc_query.iter_mut(){
    if chase_target_id.0 !=0{
      for (ball_e,ball_id,mut last_npc) in ball_query.iter_mut(){
        if chase_target_id.0 == ball_id.0 && !last_npc.2{
          if let Some(ln)=last_npc.1{
            cmd.entity(npc_e).insert(QQParent(ln));
          }else{
            cmd.entity(npc_e).insert(QQParent(ball_e));
          }
          if npc_id.sprite_enum!=0{ //not snake
            *last_npc = LastNPC(npc_id.id,Some(npc_e),last_npc.2);
          }
          break;
        }
      }
    }
  }
}
pub fn spawn_joint(
  mut npc_query: Query<(Entity,&NPCId,&Transform,&mut Velocity,&QQParent,&ChaseTargetId2)>,
  position_query: Query<&Transform,Or<(With<BallId>,With<NPCId>)>>,
  last_npc_query:Query<(Entity,&NPCId,&QQParent)>,
  mut ball_query:Query<(&BallId,&mut LastNPC)>,
  mut to_despawn: ResMut<entity_to_remove::EntityToRemove>,
){
  for (npc_e,npc_id,npc_t,mut v,parent,chase_target) in npc_query.iter_mut(){
    if let Ok(t) = position_query.get(parent.0) {
      let unit_vec = (t.translation-npc_t.translation).normalize_or_zero();   
      let dist =  t.translation.distance_squared(npc_t.translation);
      let mut factor = 70.0;
      if dist<300.0{
        factor =0.0;
        if npc_id.sprite_enum == 0{
          for (ball_id,mut last_npc) in ball_query.iter_mut(){
            if ball_id.0 == chase_target.0 && !last_npc.2{
              if last_npc.0!=0 && last_npc.1.is_some(){
                if let Ok((ln_e,_npc_id_,ln_parent))=last_npc_query.get(last_npc.1.unwrap()){
                  *last_npc = LastNPC(last_npc.0,Some(ln_parent.0),last_npc.2);
                  //*last_npc.0 = ln_parent.0;
                  //cmd.entity(ln_e).despawn(); //despawn last npc
                  (*to_despawn).entities.insert(ln_e);
                }
              }
              break;
            }
          }
          (*to_despawn).entities.insert(npc_e);
          //cmd.entity(npc_e).despawn(); //despawn snake
        }
      }
      v.linvel.x = unit_vec.x *factor *unit_vec.length_recip();
      v.linvel.y = unit_vec.y  *factor *unit_vec.length_recip();
      
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