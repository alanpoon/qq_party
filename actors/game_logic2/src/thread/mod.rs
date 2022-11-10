use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse};
use wasmbus_rpc::actor::prelude::*;
use crate::{TimeV2};
use crate::info_::info_;
use crate::startup;
use bevy::prelude::*;
pub async fn thread_handle_request(map:Arc<Mutex<App>>,start_thread_request: &u64)->RpcResult<u32>{
  {
    let guard = match map.try_lock() {
      Ok(guard) => Ok(guard),
      Err(poisoned) => {
        Err(String::from(""))
      },
    };
    if let Err(_)= guard{
      return Ok(0);
    } 
    // if start_thread_request.subject.is_none(){
        let mut app = guard.unwrap();
        if let Some(mut t) = app.world.get_resource_mut::<Time>(){
          t.update_with_timestamp(start_thread_request.clone())
        }else{
          app.world.insert_resource(Time::default());
        }
        if let Some( t) = app.world.get_resource_mut::<TimeV2>(){
          //t.update(start_thread_request.elapsed as f32);
        }else{
          app.world.insert_resource(TimeV2{elapsed:HashMap::from([
            (String::from("A"),0.0),
            (String::from("B"),1000.0),
            (String::from("C"),2000.0),
            (String::from("D"),3000.0),
            (String::from("scoreboard"),100.0),
            (String::from("storm_ring"),300.0),
          ])});
        }

         startup::state_update(&mut app);
         app.update();
    //}
    
  }
  //info!("{}",n);
  Ok(0)
}