use bevy::prelude::*;
use qq_party_shared::*;
pub fn spawn_or_update_ball_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &BallId,&mut Position,&mut QQVelocity,&mut TargetVelocity),Without<NPCId>>,
  delta:f32,
  ball_bundles:Vec<BallBundle>
  ){
    let len = ball_bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, ball_id,mut pos, mut v,mut _tv) in v_query.iter_mut(){
        let ball_bundle = ball_bundles.get(i).unwrap();
        if ball_bundle.ball_id.0 == ball_id.0{
          *v = ball_bundle.velocity;
          (*pos).0.x = ball_bundle.position.0.x+ ball_bundle.velocity.0.x *delta;
          (*pos).0.y = ball_bundle.position.0.y+ ball_bundle.velocity.0.y *delta;
          //*tv = ball_bundle.target_velocity;
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
        let mut ball_bundle_c = ball_bundle.clone();
        ball_bundle_c.target_velocity = TargetVelocity(Vec2::new(0.0,0.0));
        cmd.spawn_bundle(ball_bundle_c);
      }
    }
}
pub fn spawn_or_update_npc_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &NPCId,&mut Position,&mut QQVelocity,&mut ChaseTargetId),Without<BallId>>,
  delta:f32,
  bundles:Vec<NPCBundle>
  ){
    let len = bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, npc_id,mut pos, mut v,mut _ct) in v_query.iter_mut(){
        let bundle = bundles.get(i).unwrap();
        if bundle.npc_id.id == npc_id.id{
          *v = bundle.velocity;
          (*pos).0.x = bundle.position.0.x+ bundle.velocity.0.x *delta;
          (*pos).0.y = bundle.position.0.y+ bundle.velocity.0.y *delta;
          //*ct = bundle.chase_target;
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
        let mut bundle_c = bundle.clone();
        bundle_c.chase_target = ChaseTargetId(0,0);
        cmd.spawn_bundle(bundle_c);
      }
    }
}
pub fn spawn_fire_bundle(
  mut cmd: &mut Commands,
  bundle:FireBundle
  ){
    cmd.spawn_bundle(bundle);
}
pub fn spawn_or_delete_storm_rings_bundles(
  mut cmd: &mut Commands,
  mut v_query:&mut Query<Entity,With<StormRingId>>,
  bundles:Vec<StormRingId>
  ){
    let len = bundles.len();
    if len==0{
      for e in v_query.iter(){
        cmd.entity(e).despawn_recursive();
      }
    }else{
      for storm in bundles.iter(){
        cmd.spawn().insert(storm.clone());
      }
    }
}