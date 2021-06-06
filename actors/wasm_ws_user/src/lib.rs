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
fn handle_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
  // TODO: handle request to obtain access token

  if msg.subject == INBOUND_SUBJECT {
      handle_inbound_message(msg)
  } else if msg.subject.starts_with(BACKEND_SUBJECT_PREFIX) {
      handle_outbound_message(msg)
  } else {
      Err(format!("Unrecognized subject {}", msg.subject).into())
  }
}