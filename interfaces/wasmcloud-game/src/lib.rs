mod generated;
#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
pub use generated::*;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub fn start_thread(request: StartThreadRequest) -> HandlerResult<StartThreadResponse> {
    Host::default().start_thread(request)
}
#[cfg(feature = "guest")]
pub fn stop_thread(request: StartThreadRequest) -> HandlerResult<StartThreadResponse> {
    Host::default().stop_thread(request)
}
