#[macro_use]
extern crate log;
extern crate wapc_guest as guest;
use actor_core as actorcore;
use actor_messaging as messaging;
use guest::prelude::*;
use guest::HandlerResult;
extern crate qqparty_protocol as protocol;
use protocol::*;

#[no_mangle]
pub fn wapc_init() {
    actorcore::Handlers::register_health_request(health);
    messaging::Handlers::register_request(handle_message);
}

fn handle_message(subject:String, msg:Vec<u8>, timeout: i64) -> HandlerResult<messaging::BrokerMessage> {
    info!(
        "Received broker message on '{}'",
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