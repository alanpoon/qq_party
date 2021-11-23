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
  logging::enable_macros();
  room::Handlers::register_gift_request(handle_room_gift);
}

fn handle_room_gift(req: room::GiftRequest)-> HandlerResult<room::GiftResponse>{
  // actor::call_actor(
  //   WS_GATEWAY_ACTOR_CALL_ALIAS,
  //   "GatewayPublish",
  //   &gateway::BrokerMessage {
  //      subject: format!("ws_gateway.room_gift.gift_request.{}",req.room_number),
  //      reply_to:String::from(""),
  //      body: gateway::serialize(req).unwrap(),
  //   }
  // )?;
  let k = format!("{:?}",req.gift_id);
  logging::default().write_log("LOGGING_ACTORINFO", "gift_id", &k)?;
  let res = room::GiftResponse{
    gift_id: req.gift_id,
    user_id: req.user_id,
  };
  let k2 = format!("{:?}",res.gift_id);
  logging::default().write_log("LOGGING_ACTORINFO", "res gift_id", &k2)?;
  Ok(res)
}