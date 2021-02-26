#[macro_use]
extern crate log;
extern crate wapc_guest as guest;
use actor_core as actorcore;
use actor_messaging as messaging;
use actor_logging as logging;
use guest::prelude::*;
use guest::HandlerResult;
extern crate qqparty_protocol as protocol;
use protocol::*;
#[macro_use]
mod macros;
#[no_mangle]
pub fn wapc_init() {
    actorcore::Handlers::register_health_request(health);
    messaging::Handlers::register_request(handle_message);
    messaging::Handlers::register_deliver_message(handle_deliver_message);
 
}
fn handle_log(log_req:logging::WriteLogRequest)->HandlerResult<()>{ 
  logging::default().write_log(log_req);
  Ok(())
}

fn handle_message(subject:String, msg:Vec<u8>, timeout: i64) -> HandlerResult<messaging::BrokerMessage> {
  foo_info!(
      "Received broker message on '{}'",
      "echo".to_string(),
      subject.clone()
  );
  let resp = actor_core::deserialize::<messaging::BrokerMessage>(msg.as_ref()).unwrap();
  let published_response = add_match(subject.clone(),resp.reply_to.clone());
  Ok(messaging::BrokerMessage{
    subject: subject.clone(),
    reply_to: resp.reply_to,
    body: bincode::serialize(&published_response?).unwrap()
  })
}
fn handle_deliver_message(m:messaging::BrokerMessage)->HandlerResult<messaging::BrokerMessage>{
  foo_info!(
    "Delivered broker message on '{:?}'",
    "echo".to_string(),
    m.clone()
  );
  let mut k = m;
  k.body = vec![];
  Ok(k)
}
fn health(_h: actorcore::HealthCheckRequest) -> HandlerResult<actorcore::HealthCheckResponse> {
  Ok(actorcore::HealthCheckResponse::healthy())
}
fn add_match(subject: String, reply_to: String) -> HandlerResult<messaging::PublishResponse> {
  info!("Scheduling new match");
  let sm = MatchScheduleEntry{
    max_actors: 2,
    board_height: 3,
    board_width: 4,
    max_turns: 4,
  };
  
  messaging::default().publish(&subject, Some(&reply_to), &serde_json::to_vec(&sm)?)
}