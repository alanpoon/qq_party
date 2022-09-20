use bevy_ecs::prelude::*;
use crate::{A,Time};
use crate::info_::info_;
use qq_party_shared::*;
use crate::bevy_wasmcloud_time;
pub mod publish;
use crate::util::sub_map_area;
use qq_party_shared::*;
use bevy_rapier2d::prelude::*;

// pub fn sys(mut query: Query<&mut A>,time: Res<Time>) {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
//   for  mut a in query.iter_mut() {
//       //let n = format!("sys a >{:?}, t >{:?}",a,2);
//       //info_(n);
//       //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
//       a.position = a.position + 1;
//   }
// }
pub fn sys_time_debug(t:Res<bevy_wasmcloud_time::Time>,
  balls_without_rigid:Query<(&BallId,&Position,&RigidBodyVelocityComponent,&Velocity)>,
  rapier_parameters: Res<RapierConfiguration>,
 ){
  for (ball_id,pos,rv,vel) in balls_without_rigid.iter(){
    info_(format!("ball_id {:?} pos {:?} rv {:?} Velocity {:?} rapier_parametersscale {:?}",ball_id,pos,rv.0.linvel,vel,rapier_parameters.scale));
  }
}
pub fn sys_ball_bundle_debug(query: Query<&BallId>) {
  //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for ball_id in query.iter() {
      let n = format!("debug ballbundle a >{:?},",ball_id);
      info_(n);
  }
}
pub fn sys_health_check_despawn(mut commands: Commands,query: Query<(Entity,&BallId,&bevy_wasmcloud_time::Time)>,bevy_wasmcloud_time:Res<bevy_wasmcloud_time::Time>){
  for (entity,ball_id,bwt) in query.iter() {
    // info_(format!("bevy_wasmcloud_time.timestamp- bwt.timestamp {:?}",bevy_wasmcloud_time.timestamp- bwt.timestamp));
    if (bevy_wasmcloud_time.timestamp- bwt.timestamp) >20000{
      // info_(format!("despawn {:?}",ball_id));
      commands.entity(entity).despawn();
    }
  }
}