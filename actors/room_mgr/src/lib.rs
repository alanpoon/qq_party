#[macro_use]
extern crate log;
extern crate wapc_guest as guest;
extern crate wasmcloud_actor_core as core;
extern crate wasmcloud_actor_messaging as messaging;
extern crate wasmcloud_actor_logging as logging;
extern crate wasmcloud_actor_telnet as telnet;
use telnet::TelnetResult;
use guest::prelude::*;
use guest::HandlerResult;
extern crate qqparty_protocol as protocol;
use protocol::*;
#[macro_use]
mod macros;
#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    messaging::Handlers::register_handle_message(handle_message);
    logging::enable_macros();
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
}

fn handle_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
  info!(target: "GETLOG","echo");
  let subject = msg.subject.clone();
  let replyTo = msg.reply_to.clone();
  add_match(subject.clone(),replyTo.clone());
  Ok(())
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
  Ok(core::HealthCheckResponse::healthy())
}
fn add_match(subject: String, reply_to: String) -> HandlerResult<messaging::PublishResponse> {
  info!("Scheduling new match");
  let sm = MatchScheduleEntry{
    max_actors: 2,
    board_height: 3,
    board_width: 4,
    max_turns: 4,
  };
  messaging::default().publish(reply_to, "no_reply".to_string(), serde_json::to_vec(&sm)?)
}