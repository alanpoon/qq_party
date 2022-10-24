use bevy::prelude::*;
use crate::info_::info_;
use qq_party_shared::*;
use crate::bevy_wasmcloud_time;
pub mod publish;
use bevy_rapier2d::prelude::*;

pub fn sys_time_debug(t:Res<bevy_wasmcloud_time::Time>,
  balls_without_rigid:Query<(&BallId,&Position,&Velocity,&QQVelocity)>,
  rapier_parameters: Res<RapierConfiguration>,
 ){
  for (ball_id,pos,rv,vel) in balls_without_rigid.iter(){
    info_(format!("ball_id {:?} pos {:?} rv {:?} Velocity {:?}",ball_id,pos,rv.linvel,vel));
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