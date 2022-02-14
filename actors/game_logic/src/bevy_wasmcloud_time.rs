use qq_party_shared::time_interface::TimeInterface;

#[derive(Debug,Clone,Default)]
pub struct Time{
  pub delta_seconds: f32,
  pub timestamp: u64,
}
impl TimeInterface for Time{
  fn delta_seconds(&self) -> f32{
    self.delta_seconds
  }
  fn update_with_timestamp(&mut self,timestamp:u64){
    self.timestamp = timestamp;
  }
}