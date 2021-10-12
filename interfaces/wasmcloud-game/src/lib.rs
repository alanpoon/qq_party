mod generated;
#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
pub use generated::*;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub fn start_thread(request: StartThreadRequest) -> HandlerResult<Vec<u8>> {
  let host = default();
  host.start_thread(request);
  Ok(Vec::new()) // TODO: Provide implementation.
}
