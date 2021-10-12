extern crate wapc_guest as guest;

use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;
use wasm_user_interface as user;
use guest::prelude::*;
const WS_SERVER_ACTOR_CALL_ALIAS: &str = "ws_server";

#[actor::init]
fn init() {
    // Register your message handlers here
    user::Handlers::register_ping(handle_ping);
}
//wash ctl call MCXF2PXO54H4TPUZLCTLIAYHV2XL4UX6O7MK2FORICIAKDPIRONFFKVH Ping '{"value": 2}'
fn handle_ping(ping: user::Ping) -> HandlerResult<user::Pong> {
  Ok(user::Pong {
      value: ping.value + 42,
  })
}
