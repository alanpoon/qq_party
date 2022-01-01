pub struct Options{
  servers: Vec<String>,
}
impl Default for Options{
  fn default()->Self{
    Options{servers:vec![]}
  }
}