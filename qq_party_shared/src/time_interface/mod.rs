pub trait TimeInterface {
  fn delta_seconds(&self) -> f32;
  fn update_with_timestamp(&mut self,timestamp:u64);
}