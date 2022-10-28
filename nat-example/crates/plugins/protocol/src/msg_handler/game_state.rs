use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn(mut cmd: &mut Commands,mut set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,ball_id:BallId){
    for (entity, qball_id,pos,vel) in set.p0().iter_mut(){
        if ball_id ==*qball_id{
            cmd.entity(entity).insert(Dash(true,vel.linvel*2.0,vel.linvel));
        }
    }   
}
pub fn _fn_spawn_or_update_ball_bundles(
    mut cmd: &mut Commands,
    mut set: &mut ParamSet<(
        Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
        Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
        Query<(Entity,&mut Transform),With<StormRingId>>,
        // also access the whole world ... why not
        //&World,
    )>,
    delta:f32,
    ball_bundles:Vec<BallBundle>
    ){
      let len = ball_bundles.len();
      let mut founds = vec![];
      for i in 0..len{
        for (e, ball_id, mut t,mut v) in set.p0().iter_mut(){
          let ball_bundle = ball_bundles.get(i).unwrap();
          if ball_bundle.ball_id.0 == ball_id.0{
            *v = ball_bundle.velocity;
            *t = ball_bundle.transform;
            info!("found {:?}transform",ball_bundle.transform);
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
            info!("ball_bundle {:?}",ball_bundle);
          let mut ball_bundle_c = ball_bundle.clone();
          //ball_bundle_c.target_velocity = TargetVelocity(Vec2::new(0.0,0.0));
          cmd.spawn_bundle(ball_bundle_c);
        }
      }
  }