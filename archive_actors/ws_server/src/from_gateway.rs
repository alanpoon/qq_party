use wasm_user_interface as user;
use gateway_interface as gateway;
use wasmcloud_actor_core as actor;
const ROOM_GIFT_ACTOR_CALL_ALIAS: &str = "room_gift";
const INGAME_BEAN_ACTOR_CALL_ALIAS: &str = "ingame_bean";
use wapc_guest::HandlerResult;
use wasmcloud_actor_logging as logging;
use log::{debug, error, info, warn};
use convert_case::{Case, Casing};
fn remove_whitespace(s: &mut String) {
  s.retain(|c| !c.is_whitespace());
}
//wash ctl call MAZLL727FH5HDKM6UWP7R2N2RPV5Z4WL5M7ECTF6E2Z635Q3DBVGZBIL GatewayPublish '{"subject":"ws_gateway.room_gift.gift_request","replyTo":"","body":[131,167,103,105,102,116,95,105,100,32,171,114,111,111,109,95,110,117,109,98,101,114,162,50,51,167,117,115,101,114,95,105,100,32]}'
//wash ctl call MAZLL727FH5HDKM6UWP7R2N2RPV5Z4WL5M7ECTF6E2Z635Q3DBVGZBIL GatewayPublish '{"subject":"ws_gateway.room_gift.gift_request","replyTo":"","body":{"room_number":"2","gift_id":32,"user_id":32}}'
pub fn handle_req_from_gateway(msg: gateway::BrokerMessage) -> HandlerResult<Vec<u8>> {
  let subject = msg.subject.clone();
  logging::default().write_log("LOGGING_ACTORINFO", "info", &subject)?;
  let args = msg.subject.split(".");
  let args_list:Vec<&str> = args.collect();
  let actor_alias =args_list.get(1).ok_or("no element")?;
  let cmd = args_list.get(2).ok_or("no element")?;
  let mut n_cmd:String = cmd.to_case(Case::Title);
  remove_whitespace(&mut n_cmd);
  logging::default().write_log("LOGGING_ACTORINFO", "info", &n_cmd)?;
  let cmd_split = cmd.split(".");
  logging::default().write_log("LOGGING_ACTORINFO", "info", &subject)?;
  let res_bytes = wapc_guest::host_call("default", actor_alias, &n_cmd, &msg.body)?;
  let j = format!("{:?}",res_bytes);
  logging::default().write_log("LOGGING_ACTORINFO", "res_bytes", &j)?;
  Ok(res_bytes)
}
// pub fn json_str_to_msgpack_bytes(payload: String) -> HandlerResult<Vec<u8>> {
//   println!("payload {:?}",payload);
//   let json: serde_json::value::Value = serde_json::from_str(&payload)?;
//   println!("json {:?}",json);
//   let payload = serdeconv::to_msgpack_vec(&json)?;
//   Ok(payload)
// }