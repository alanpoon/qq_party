extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_http_server as http;
use wasm_user_interface as user;
use wasm_user_interface as room;
use gateway_interface as gateway;
const WASM_USER_ACTOR_CALL_ALIAS: &str = "wasm_user";
const ROOM_ACTOR_CALL_ALIAS: &str = "room";
const WS_GATEWAY_ACTOR_CALL_ALIAS: &str = "ws_gateway";
#[actor::init]
fn init() {
    logging::enable_macros();
    http::Handlers::register_handle_request(handle_request);
    user::Handlers::register_ping(handle_ping);
    gateway::Handlers::register_gateway_publish(handle_req_from_gateway);
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
  Ok(http::Response::json(&p, 200, "OK"))
}
fn handle_ping(ping: user::Ping) -> HandlerResult<user::Pong> {
  Ok(user::Pong {
      value: ping.value + 42,
  })
}
fn handle_req_from_gateway(msg: gateway::BrokerMessage) -> HandlerResult<()> {
  let subject = msg.subject;
  if subject.contains("room"){
    let req:user::Ping = user::deserialize(&msg.body)?;
    let res:user::Pong = actor::call_actor(
      ROOM_ACTOR_CALL_ALIAS,
      "room",
      &req,
    )?;
    actor::call_actor(
      WS_GATEWAY_ACTOR_CALL_ALIAS,
      "GatewayPublish",
      &gateway::BrokerMessage {
         subject: String::from("ws_gateway.room"),
         reply_to:String::from(""),
         body: gateway::serialize(res).unwrap(),
       }
    )?;
  }
  Ok(())
}