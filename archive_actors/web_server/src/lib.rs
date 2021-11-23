extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_http_server as http;
use std::collections::HashMap;
mod actix_web_static_files;
use lazy_static::lazy_static; // 1.4.0
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
lazy_static! {
  static ref FILES: HashMap<&'static str, actix_web_static_files::Resource> =
      generate();
}
#[actor::init]
fn init() {
    logging::enable_macros();
    http::Handlers::register_handle_request(handle_request);
}
fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
  let (_,path) = req.path.split_at(1);
  logging::default().write_log("LOGGING_ACTORINFO", "http", path)?;
  let resource = FILES.get(&path as &str).ok_or("no file")?;
  let res = http::Response{
    body:resource.data.to_vec(),
    header: HashMap::new(),
    status: "OK".to_string(),
    status_code: 200,
  };
  Ok(res)
}
