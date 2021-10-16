use log::{debug, error, info,trace};
use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_BIND_ACTOR, OP_HEALTH_REQUEST, OP_REMOVE_ACTOR};
use std::{collections::HashMap, time::Duration, time::Instant};
use wasmcloud_actor_core::{deserialize, serialize, CapabilityConfiguration, HealthCheckResponse};
use wasmcloud_provider_core as codec;
use std::error::Error;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::sleep;
use chrono::prelude::*;

#[cfg(not(feature = "static_plugin"))]
use wasmcloud_provider_core::capability_provider;
#[allow(unused)]
const CAPABILITY_ID: &str = "wasmcloud:game";

const OP_START_THREAD: &str = "StartThread";
const OP_STOP_THREAD: &str = "StopThread";
const OP_HANDLE_THREAD: &str = "HandleThread";

const TURN_DELAY_MILLIS_DEFAULT: u64 = 2000;

#[cfg(not(feature = "static_plugin"))]
capability_provider!(GameProvider, GameProvider::new);
use lazy_static::lazy_static; // 1.4.0

lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,bool>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone)]
pub struct GameProvider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    module: Arc<RwLock<String>>,
}

impl GameProvider{
  fn spawn_server(&self,start_thread_request:wasmcloud_game::StartThreadRequest)->Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
    let start = Instant::now();
    info!("start thread");
    let dispatcher = self.dispatcher.clone();
    let module = self.module.clone();
    //let MAP = Arc::new(Mutex::new(HashMap::new()));
    std::thread::spawn(move || {
      // some work here
      let mut map = MAP.clone();
      let mut m = map.lock().unwrap();
      m.insert(start_thread_request.game_id.clone(),true);
      drop(m);
      
      loop{
        let mut map = MAP.clone();
        let m = map.lock().unwrap();
        info!("elapsed {:?}",start.elapsed().as_secs());
        if let Some(v) = m.get(&start_thread_request.game_id.clone()){
          if *v{
            drop(m);
            sleep(Duration::from_millis(TURN_DELAY_MILLIS_DEFAULT));
            let resp = {
              let lock = (*dispatcher).read().unwrap();
              let local: DateTime<Local> = Local::now();
              let m = wasmcloud_game::StartThreadRequest{
                game_id: start_thread_request.game_id.clone(),
                elapsed: Some(TURN_DELAY_MILLIS_DEFAULT as f32),
                timestamp: Some(local.timestamp_millis()),
              };
              if let Ok(buf) = serialize(m){
                let module_lock = (*module).read().unwrap();
                lock.dispatch(
                  &module_lock,
                  OP_HANDLE_THREAD,
                  &buf,
                );
              }
            };
          }else{
            drop(m);
            break;
          }
        }else{
          drop(m);
          break;
        }
      }
    });
    let m = wasmcloud_game::StartThreadResponse{};
    serialize(m)
  }
  fn stop_server(&self,start_thread_request:wasmcloud_game::StartThreadRequest)->Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
    let mut map = MAP.clone();
    let mut m = map.lock().unwrap();
    m.remove(&start_thread_request.game_id).ok_or("game_id not in hashmap")?;
    let m = wasmcloud_game::StartThreadResponse{};
    serialize(m)
  }
  pub fn new() -> Self {
    Self::default()
  }
}
impl Default for GameProvider {
  fn default() -> Self {
      if env_logger::try_init().is_err() {}
      GameProvider {
          dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
          module: Arc::new(RwLock::new(String::from(""))),
      }
  }
}
impl CapabilityProvider for GameProvider {
  /// Accepts the dispatcher provided by the wasmCloud host runtime
  fn configure_dispatch(
      &self,
      dispatcher: Box<dyn Dispatcher>,
  ) -> Result<(), Box<dyn Error + Sync + Send>> {
      info!("2zzDispatcher configured.");

      let mut lock = self.dispatcher.write().unwrap();
      *lock = dispatcher;

      Ok(())
  }

  /// Handles an invocation from the host runtime
  fn handle_call(
      &self,
      actor: &str,
      op: &str,
      msg: &[u8],
  ) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
      trace!("Handling operation `{}` from `{}`", op, actor);

      match op {
          OP_BIND_ACTOR if actor == "system" => {
              let cfgvals = deserialize::<CapabilityConfiguration>(msg)?;
              let mut lock = self.module.write().unwrap();
              *lock = cfgvals.module;
              //self.spawn_server(&deserialize(msg)?);
              Ok(vec![])
          }
          OP_REMOVE_ACTOR if actor == "system" => {
              let cfgvals = deserialize::<CapabilityConfiguration>(msg)?;
              info!("Removing actor configuration for {}", cfgvals.module);
              Ok(vec![])
          }
          OP_HEALTH_REQUEST if actor == "system" => Ok(serialize(HealthCheckResponse {
              healthy: true,
              message: "".to_string(),
          })?),
          OP_START_THREAD =>{
              info!("OP_START_THREAD  {:?}", msg);
              let start_thread_req = deserialize::<wasmcloud_game::StartThreadRequest>(msg)?;
              self.spawn_server(start_thread_req)
          },
          OP_STOP_THREAD =>{
            info!("OP_STOP_THREAD  {:?}", msg);
            let start_thread_req = deserialize::<wasmcloud_game::StartThreadRequest>(msg)?;
            self.stop_server(start_thread_req)
          },
          _ => Err("bad dispatch".into()),
      }
  }

  fn stop(&self) {
    
  }
}