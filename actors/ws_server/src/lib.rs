extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_http_server as http;
use wasm_user_interface as user;
const WASM_USER_ACTOR_CALL_ALIAS: &str = "wasm_user";

#[actor::init]
fn init() {
    logging::enable_macros();
    http::Handlers::register_handle_request(handle_request);
}
fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
  let mut p = user::Pong{
    value: 0,
  };
  if req.path =="/echo"{
    p = actor::call_actor(
      WASM_USER_ACTOR_CALL_ALIAS,
      "Ping",
      &user::Ping { value: 11 },
    )?;
  }
  Ok(http::Response::json(&p, 200, "OK"))
}
