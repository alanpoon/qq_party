use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse};
use wasmbus_rpc::actor::prelude::*;
use crate::bevy_wasmcloud_time;
use crate::{QQTime,TimeV2};
use crate::info_::info_;
use crate::startup;
use bevy::prelude::*;
use qq_party_shared::time_interface::TimeInterface;
pub async fn thread_handle_request(map:Arc<Mutex<App>>,start_thread_request: &StartThreadRequest)->RpcResult<StartThreadResponse>{
  {
    let guard = match map.try_lock() {
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
          t.update_with_timestamp(start_thread_request.timestamp)

          //t.update(start_thread_request.elapsed as f32);
        }else{
          app.world.insert_resource(Time::default());
        }

        if let Some( _t) = app.world.get_resource_mut::<QQTime>(){
          //t.update(start_thread_request.elapsed as f32);
        }else{
          app.world.insert_resource(QQTime{elapsed:start_thread_request.elapsed as f32});
        }
        if let Some( t) = app.world.get_resource_mut::<TimeV2>(){
          //t.update(start_thread_request.elapsed as f32);
        }else{
          app.world.insert_resource(TimeV2{elapsed:HashMap::from([
            (String::from("A"),start_thread_request.elapsed as f32),
            (String::from("B"),(start_thread_request.elapsed + 1000) as f32),
            (String::from("C"),(start_thread_request.elapsed + 2000) as f32),
            (String::from("D"),(start_thread_request.elapsed + 3000) as f32),
            (String::from("scoreboard"),(start_thread_request.elapsed + 100) as f32),
            (String::from("storm_ring"),(start_thread_request.elapsed + 300) as f32),
          ])});
        }
        if let Some(mut t) = app.world.get_resource_mut::<bevy_wasmcloud_time::Time>(){
          n = String::from("can find time");
          //t.update(start_thread_request.elapsed as f32);
          t.update_with_timestamp(start_thread_request.timestamp)
        }else{
          app.world.insert_resource(bevy_wasmcloud_time::Time{timestamp:start_thread_request.timestamp,..Default::default()});
        }
      // /w.spawn().insert_bundle(arugio_shared::BallBundle);
         startup::state_update(&mut app);
         app.update();
        // drop(app);      
    }
    
  }
  //info!("{}",n);
  Ok(StartThreadResponse{})
}