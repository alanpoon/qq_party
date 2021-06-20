#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_messaging as messaging;
use wasm_user_interface as user;
use gateway_interface as gateway;
const WS_SERVER_ACTOR_CALL_ALIAS: &str = "ws_server";
const MSG_LINK: &str = "default";
const INIT_SUBJECT: &str = "";
#[actor::init]
fn init() {
    logging::enable_macros();
    messaging::Handlers::register_handle_message(handle_message);
    gateway::Handlers::register_gateway_publish(handle_req_to_gateway);
    http::Handlers::register_handle_request(handle_request);
    //
}
fn handle_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
  actor::call_actor(
    WS_SERVER_ACTOR_CALL_ALIAS,
    "GatewayPublish",
    &gateway::BrokerMessage {
      subject: msg.subject,
      reply_to: msg.reply_to,
      body: msg.body
    },
  )?;  
  Ok(())
}
fn handle_req_to_gateway(msg: gateway::BrokerMessage) -> HandlerResult<()> {
  let _ = messaging::host(MSG_LINK)
    .publish(
        String::from(INIT_SUBJECT),
        msg.subject,
        msg.body
    )
    .map(|_| true);
  Ok(())
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "Coercing Rust String to str")?;
  let p = json!({
    "name": "hll",
    "birth_year": "j"
  });
  if req.path =="/echo2"{
    /*let p = actor::call_actor(
      WS_SERVER_ACTOR_CALL_ALIAS,
      "Ping",
      &user::Ping { value: 11 },
    )?;*/
    publish();
  }
  Ok(http::Response::json(&p, 200, "OK"))
}
fn publish() -> () {
  let _ = messaging::host(MSG_LINK)
  .publish(
      INIT_SUBJECT.to_string(),
      "ws_gateway.room".to_string(),
      serde_json::to_vec(&json!({
        "name": "hll",
        "birth_year": "j"
      }))
      .unwrap(),
  )
  .map(|_| true);
}