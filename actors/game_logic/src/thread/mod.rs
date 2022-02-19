use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use crate::bevy_wasmcloud_time;
use crate::Time;
use qq_party_shared::time_interface::TimeInterface;
pub async fn thread_handle_request(map:Arc<Mutex<HashMap<String,(Schedule,World)>>>,start_thread_request: &StartThreadRequest)->RpcResult<StartThreadResponse>{
  let mut n = String::from("");
  {
    let mut guard = match map.lock() {
      Ok(guard) => guard,
      Err(poisoned) => {
        n = format!("{:?}",poisoned);
        poisoned.into_inner()
      },
    };
    if start_thread_request.subject.is_none(){
      if let Some((ref mut s, ref mut w))= guard.get_mut(&start_thread_request.game_id) {
        if let Some(mut t) = w.get_resource_mut::<Time>(){
          n = String::from("can find time");
          t.update(start_thread_request.elapsed as f32);
        }else{
          w.insert_resource(Time{elapsed:start_thread_request.elapsed as f32});
        }
        if let Some(mut t) = w.get_resource_mut::<bevy_wasmcloud_time::Time>(){
          n = String::from("can find time");
          //t.update(start_thread_request.elapsed as f32);
          t.update_with_timestamp(start_thread_request.timestamp)
        }else{
          w.insert_resource(bevy_wasmcloud_time::Time{timestamp:start_thread_request.timestamp,..Default::default()});
        }
      // /w.spawn().insert_bundle(arugio_shared::BallBundle);

      s.run_once(w);
      }else{
        n = String::from("can't find");
      }
    }else{
      let split_arr:Vec<String> = start_thread_request.game_id.split("_").map(|x|x.to_string()).collect();

      if let Some((ref mut s, ref mut w))= guard.get_mut(split_arr.get(0).unwrap()) {
        
      }
    }
    
  }
  info!("{}",n);
  Ok(StartThreadResponse{})
}