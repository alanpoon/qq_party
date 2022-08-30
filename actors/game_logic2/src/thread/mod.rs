use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use crate::bevy_wasmcloud_time;
use crate::Time;
use crate::info_::info_;
use bevy_app::App;
use bevy_ecs::prelude::*;
use qq_party_shared::time_interface::TimeInterface;
pub async fn thread_handle_request(map:Arc<Mutex<App>>,start_thread_request: &StartThreadRequest)->RpcResult<StartThreadResponse>{
  let mut n = String::from("thread_handle_request");
  {
    let mut guard = match map.try_lock() {
      Ok(guard) => Ok(guard),
      Err(poisoned) => {
        n = format!("{:?}",poisoned);
        info_(format!("poisoned{}",n));
        Err(n.clone())
      },
    };
    if let Err(_)= guard{
      return Ok(StartThreadResponse{});
    } 
    if start_thread_request.subject.is_none(){
        let mut app = guard.unwrap();
        if let Some(mut t) = app.world.get_resource_mut::<Time>(){
          n = String::from("can find time");
          //t.update(start_thread_request.elapsed as f32);
        }else{
          app.world.insert_resource(Time{elapsed:start_thread_request.elapsed as f32});
        }
        if let Some(mut t) = app.world.get_resource_mut::<bevy_wasmcloud_time::Time>(){
          n = String::from("can find time");
          //t.update(start_thread_request.elapsed as f32);
          t.update_with_timestamp(start_thread_request.timestamp)
        }else{
          app.world.insert_resource(bevy_wasmcloud_time::Time{timestamp:start_thread_request.timestamp,..Default::default()});
        }
      // /w.spawn().insert_bundle(arugio_shared::BallBundle);
         app.update();
        // drop(app);      
    }else{
      let split_arr:Vec<String> = start_thread_request.game_id.split("_").map(|x|x.to_string()).collect();
    }
    
  }
  //info!("{}",n);
  Ok(StartThreadResponse{})
}