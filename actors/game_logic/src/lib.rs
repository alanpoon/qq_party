extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_game as game_engine;

#[actor::init]
fn init() {
    logging::enable_macros();
    game_engine::Handlers::register_start_thread(start_thread);
}
//wash ctl call MDN3AIPQ62QAFZJCSULSCR5D2NQYARPDYK763YLG4EYZLMPKECEWIFY2 StartThread '{"game_id": "hi"}'
fn start_thread(req: game_engine::StartThreadRequest) -> HandlerResult<Vec<u8>> {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "Coercing Rust String to str")?;
  game_engine::start_thread(req)
}
