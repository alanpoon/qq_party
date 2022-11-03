use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;

pub fn _fn_spawn_or_update_ball_bundles(
    cmd: &mut Commands,
    set: &mut ParamSet<(
        Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
        Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
        Query<(Entity,&mut Transform),With<StormRingId>>,
        // also access the whole world ... why not
        //&World,
    )>,
    _delta:f32,
    ball_bundles:Vec<BallBundle>
    ){
      let len = ball_bundles.len();
      let mut founds = vec![];
      for i in 0..len{
        for (_e, ball_id, mut t,mut v) in set.p0().iter_mut(){
          let ball_bundle = ball_bundles.get(i).unwrap();
          if ball_bundle.ball_id.0 == ball_id.0{
            *v = ball_bundle.velocity;
            *t = ball_bundle.transform;
            founds.push(i);
            //found = true;
            break;
          }
        }

      }
      for (i,ball_bundle) in ball_bundles.iter().enumerate(){
        if !founds.contains(&i){
          cmd.spawn_bundle(ball_bundle.clone());
        }
      }
  }
pub fn _fn_spawn_or_update_npc_bundles(
  cmd: &mut Commands,
  set: &mut ParamSet<(
      Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
      Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
      Query<(Entity,&mut Transform),With<StormRingId>>,
      // also access the whole world ... why not
      //&World,
  )>,
  _delta:f32,
  bundles:Vec<NPCBundle>
  ){
    let len = bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, npc_id,mut t, mut v,mut _ct) in set.p1().iter_mut(){
        let bundle = bundles.get(i).unwrap();
        if bundle.npc_id.id == npc_id.id{
          *v = bundle.velocity;
          *t = bundle.transform;
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