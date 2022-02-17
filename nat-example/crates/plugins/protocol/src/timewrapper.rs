use qq_party_shared::time_interface::TimeInterface;
use bevy::prelude::*;
#[derive(Component,Default,Debug)]
pub struct TimeWrapper{
  pub time:Time,
}
impl TimeInterface for TimeWrapper{
  fn delta_seconds(&self)->f32{
    self.time.delta_seconds()
  }
  fn update_with_timestamp(&mut self, timestamp: u64){
    self.time.update();
  }
}
pub fn into_timewrapper(time: Res<Time>,mut time_wrapper: ResMut<TimeWrapper>){
  time_wrapper.time = (*time).clone();
}