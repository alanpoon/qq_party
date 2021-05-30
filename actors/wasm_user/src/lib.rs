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

fn handle_ping(ping: user::Ping) -> HandlerResult<user::Pong> {
  Ok(user::Pong {
      value: ping.value + 42,
  })
}
