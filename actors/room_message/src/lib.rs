extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use gateway_interface as gateway;
use room_interface as room;

const WS_GATEWAY_ACTOR_CALL_ALIAS: &str = "ws_gateway";

#[actor::init]
fn init() {
  room::Handlers::register_message_request(handle_room_message);
}

fn handle_room_message(req: room::MessageRequest)-> HandlerResult<()> {
  actor::call_actor(
    WS_GATEWAY_ACTOR_CALL_ALIAS,
    "GatewayPublish",
    &gateway::BrokerMessage {
       subject: format!("ws_gateway.room_message.{}",req.room_number),
       reply_to:String::from(""),
       body: gateway::serialize(req).unwrap(),
    }
  )?;
  Ok(())
}