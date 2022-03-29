use bevy_ecs::prelude::*;
use crate::{A,Time,BallBundle};
use crate::info_::info_;
use qq_party_shared::*;

pub fn sys(mut query: Query<&mut A>,time: Res<Time>) {
  //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for  mut a in query.iter_mut() {
      let n = format!("sys a >{:?}, t >{:?}",a,2);
      info_(n);
      //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
      a.position = a.position + 1;
  }
}
pub fn spawn(mut cmd: Commands) {
  cmd.spawn_bundle(BallBundle{
    a:A{position:2}});
}
