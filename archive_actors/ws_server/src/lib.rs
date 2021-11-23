extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_http_server as http;
use wasm_user_interface as user;
use wasm_user_interface as room;
use gateway_interface as gateway;
const WASM_USER_ACTOR_CALL_ALIAS: &str = "wasm_user";
const WS_GATEWAY_ACTOR_CALL_ALIAS: &str = "ws_gateway";
mod from_gateway;
use from_gateway::handle_req_from_gateway;
#[actor::init]
fn init() {
    logging::enable_macros();
    http::Handlers::register_handle_request(handle_request);
    user::Handlers::register_ping(handle_ping);
    gateway::Handlers::register_gateway_publish(handle_req_from_gateway);
}
fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "Coercing Rust String to str")?;
  let mut p = user::Pong{
    value: 0,
  };
  if req.path ==String::from("/echo"){
    p = actor::call_actor(
      WASM_USER_ACTOR_CALL_ALIAS,
      "Ping",
      &user::Ping { value: 11 },
    )?;
    actor::call_actor(
      WS_GATEWAY_ACTOR_CALL_ALIAS,
      "GatewayPublish",
      &gateway::BrokerMessage {
         subject: String::from("ws_gateway.room"),
         reply_to:String::from(""),
         body: gateway::serialize(user::Ping { value: 11 }).unwrap(),
      }
    )?;
  }
  if req.path ==String::from("/gift"){
    //ecs::start(String::from("special"));
  }

  Ok(http::Response::json(&p, 200, "OK"))
}
fn handle_ping(ping: user::Ping) -> HandlerResult<user::Pong> {
  Ok(user::Pong {
      value: ping.value + 42,
  })
}
