extern crate wapc_guest as guest;

use serde::{Deserialize, Serialize};
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_messaging as messaging;
use wasmcloud_actor_logging as logging;
use wasm_user_interface as user;
use guest::prelude::*;
const WASM_USER_ACTOR_CALL_ALIAS: &str = "wasm_user";

#[actor::init]
fn init() {
    // Register your message handlers here
    logging::enable_macros();
    messaging::Handlers::register_handle_message(handle_message);
}


fn handle_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
  if msg.subject.contains("ws_gateway.room_message."){
    debug!(
      "command {} received from user",
      msg.subject
    );
  }
  if msg.subject.contains("ws_gateway.join_room."){
    debug!(
      "command {} received from user",
      msg.subject
    );
  }
  Ok(())
}