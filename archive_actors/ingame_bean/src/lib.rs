extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use gateway_interface as gateway;
use ingame_interface as ingame;
const WS_GATEWAY_ACTOR_CALL_ALIAS: &str = "ws_gateway";

#[actor::init]
fn init() {
  ingame::Handlers::register_spend_bean_request(handle_spend_bean);
}

fn handle_spend_bean(req: ingame::SpendBeanRequest)-> HandlerResult<ingame::SpendBeanResponse>{
  let res = ingame::SpendBeanResponse{
    error_code:0,
  };
  Ok(res)
}