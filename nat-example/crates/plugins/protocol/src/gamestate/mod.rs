use bevy::prelude::*;
use qq_party_shared::*;
pub fn spawn_or_update_ball_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &BallId,&mut Position,&mut Velocity,&mut TargetVelocity),Without<NPCId>>,
  delta:f32,
  ball_bundles:Vec<BallBundle>
  ){
    let len = ball_bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, ball_id,mut pos, mut v,mut tv) in v_query.iter_mut(){
        let ball_bundle = ball_bundles.get(i).unwrap();
        if &ball_bundle.ball_id == ball_id{
          *v = ball_bundle.velocity;
          //info!("ball_bundle.velocity.0.x {:?} delta {:?}",ball_bundle.velocity.0.x,delta);
          (*pos).0.x = ball_bundle.position.0.x+ ball_bundle.velocity.0.x *delta;
          (*pos).0.y = ball_bundle.position.0.y+ ball_bundle.velocity.0.y *delta;
          *tv = ball_bundle.target_velocity;
          founds.push(i);
          //found = true;
          break;
        }
      }
      //move to server_msg_despawn
      // if !found{
      //   cmd.entity(entity).despawn();
      // }
      //}
    }
    for (i,ball_bundle) in ball_bundles.iter().enumerate(){
      if !founds.contains(&i){
        cmd.spawn_bundle(ball_bundle.clone());
      }
    }
}
pub fn spawn_or_update_npc_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &NPCId,&mut Position,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
  delta:f32,
  bundles:Vec<NPCBundle>
  ){
    let len = bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, npc_id,mut pos, mut v,mut ct) in v_query.iter_mut(){
        let bundle = bundles.get(i).unwrap();
        if &bundle.npc_id == npc_id{
          *v = bundle.velocity;
          (*pos).0.x = bundle.position.0.x+ bundle.velocity.0.x *delta;
          (*pos).0.y = bundle.position.0.y+ bundle.velocity.0.y *delta;
          *ct = bundle.chase_target;
          founds.push(i);
          break;
        }
      }
      // if !found{
      //   cmd.entity(entity).despawn();
      // }
    }
    for (i,bundle) in bundles.iter().enumerate(){
      if !founds.contains(&i){
        cmd.spawn_bundle(bundle.clone());
      }
    }
}