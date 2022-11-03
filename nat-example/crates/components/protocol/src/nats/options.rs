pub struct Options{
  pub servers: Vec<String>,
}
impl Default for Options{
  fn default()->Self{
    Options{servers:vec![]}
  }
}