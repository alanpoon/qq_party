use crate::bevy_wasmcloud_time;
use bevy_ecs_wasm::prelude::{Commands,Entity,Schedule,World,Query,SystemStage,IntoSystem,Res,ResMut};
use crate::{A,Time};
use crate::info_::info_;
pub mod publish;
use qq_party_shared::*;
pub fn sys_bevy_wasmcloud_time(time: Res<bevy_wasmcloud_time::Time>,elapsed_time:ResMut<Time>) {
  let n = format!("bevy_wasmcloud_time sys t >{:?}",*time);
  info_(n);
}

pub fn sys(mut query: Query<&mut A>,time: Res<Time>) {
  //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for  mut a in query.iter_mut() {
      let n = format!("sys a >{:?}, t >{:?}",a,2);
      info_(n);
      //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
      a.position = a.position + 1;
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
    info_(format!("bevy_wasmcloud_time.timestamp- bwt.timestamp {:?}",bevy_wasmcloud_time.timestamp- bwt.timestamp));
    if (bevy_wasmcloud_time.timestamp- bwt.timestamp) >20000{
      info_(format!("despawn {:?}",ball_id));
      commands.entity(entity).despawn();
    }
  }
}