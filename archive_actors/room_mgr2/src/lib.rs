#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
mod kv;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_messaging as messaging;
use room_interface as room;
const MSG_LINK: &str = "default";
const INIT_SUBJECT: &str = "";
const WS_GATEWAY_ACTOR_CALL_ALIAS: &str = "ws_gateway";

#[actor::init]
fn init() {
  //room::Handlers::register_join_room_request(handle_join_room_request);
  actor::Handlers::register_health_request(health);  
  messaging::Handlers::register_handle_message(handle_join_room_request2);
  logging::enable_macros();
}
fn health(_h: actor::HealthCheckRequest) -> HandlerResult<actor::HealthCheckResponse> {
  // info!(
  //   "HealthCheckRequest"
  // );
  // let rooms = kv::get_rooms()?;
  // info!(
  //   "HealthCheckRequest {:?}",rooms.clone()
  // );
  // for room in rooms.into_iter(){
  //   let user_ids = kv::get_room_members(room.clone())?;
  //   info!(
  //     "command {:?} get_room_members",
  //     user_ids.clone()
  //   );
  //   let _ = messaging::host(MSG_LINK)
  //     .publish(
  //         String::from(INIT_SUBJECT),
  //         format!("public.users-connected.{}",room),
  //         serde_json::to_vec(&json!({
  //           "user_ids": user_ids
  //         })).unwrap(),
  //     )
  //     .map(|_| true);
  // }
  Ok(actor::HealthCheckResponse::healthy())
}
// fn handle_join_room_request(msg: messaging::BrokerMessage)-> HandlerResult<()> {
//   debug!(
//     "command {} received from user",
//     msg.subject
//   );
//   if msg.subject.contains("join_room"){
//     let v: room::JoinRoomRequest = serde_json::from_str(std::str::from_utf8(&msg.body).unwrap())?;
//     kv::join_room(v.room_number, v.user_id)?;

//     let _ = messaging::host(MSG_LINK)
//       .publish(
//           String::from(INIT_SUBJECT),
//           format!("public.user-connected.{}",v.room_number),
//           serde_json::to_vec(&json!({
//             "user_id": v.user_id
//           })).unwrap(),
//       )
//       .map(|_| true);
//   }else if msg.subject.contains("disconnect"){
//     let v: room::JoinRoomRequest = serde_json::from_str(std::str::from_utf8(&msg.body).unwrap())?;
//     let _ = messaging::host(MSG_LINK)
//       .publish(
//           String::from(INIT_SUBJECT),
//           format!("public.disconnect.{}",v.room_number),
//           serde_json::to_vec(&json!({
//             "user_id": v.user_id
//           })).unwrap(),
//       )
//       .map(|_| true);
//   }
//   Ok(())
// }

fn handle_join_room_request2(msg: messaging::BrokerMessage)-> HandlerResult<()> {
  info!(
    "command {} received from user",
    msg.subject
  );
  if msg.subject.contains("host_create_room"){
    let v: room::HostCreateRoomRequest = serde_json::from_str(std::str::from_utf8(&msg.body).unwrap())?;
    kv::add_room(v.room_number_s, v.description)?;
  } else if msg.subject.contains("join_room"){
    let v: room::JoinRoomRequest = serde_json::from_str(std::str::from_utf8(&msg.body).unwrap())?;
    //let u = serde_json::to_string(v.user)?;
    kv::join_room(v.room_number_s, serde_json::to_string(&v.user)?)?;
  } else if msg.subject.contains("leave_room"){
    let v: room::LeaveRoomRequest = serde_json::from_str(std::str::from_utf8(&msg.body).unwrap())?;
    kv::leave_room(v.room_number_s, serde_json::to_string(&v.user)?)?;
  }
  Ok(())
}