use qq_party_shared::time_interface::TimeInterface;

#[derive(Debug,Clone,Default)]
pub struct Time{
  pub delta_seconds: f32,
  pub timestamp: u64,
  pub startup:u64,
  pub last_update: u64,
}
impl TimeInterface for Time{
  fn delta_seconds(&self) -> f32{
    self.delta_seconds
  }
  fn update_with_timestamp(&mut self,timestamp:u64){
    if self.startup==0{
      self.startup = timestamp;
    }
    if self.last_update!=0{
      self.delta_seconds = (timestamp - self.last_update) as f32 / 1000.0;
    }
    self.last_update = timestamp;
    self.timestamp = timestamp;
  }
}