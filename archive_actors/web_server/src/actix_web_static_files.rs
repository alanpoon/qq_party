pub struct Resource {
  pub data: &'static [u8],
  pub modified: u64,
  pub mime_type: &'static str,
}