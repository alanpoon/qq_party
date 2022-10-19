#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasmbus_rpc::{core::LinkDefinition, provider::HostBridge};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use wasmbus_rpc::provider::prelude::*;
use wasmbus_rpc::provider::ProviderTransport;
use wasmbus_rpc::actor::prelude::Context;
use std::thread::sleep;
use std::{collections::HashMap, time::Duration, time::Instant};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use tokio::sync::RwLock;

use lazy_static::lazy_static; // 1.4.0
#[allow(unused)]
const CAPABILITY_ID: &str = "wasmcloud:thread";

const OP_START_THREAD: &str = "StartThread";
const OP_STOP_THREAD: &str = "StopThread";
const OP_HANDLE_THREAD: &str = "HandleThread";

#[derive(Clone, Provider)]
#[services(Thread)]
pub struct ThreadProvider {
  actors: Arc<RwLock<HashMap<String, ThreadPool>>>,
}
impl Default for ThreadProvider{
  fn default()->Self{
    ThreadProvider{
      actors:Arc::new(RwLock::new(HashMap::new())),
    }
  }
}
/// use default implementations of provider message handlers
impl ProviderDispatch for ThreadProvider {}
/// we don't need to override put_link, delete_link, or shutdown
#[async_trait]
impl ProviderHandler for ThreadProvider {
  async fn put_link(&self, ld: &LinkDefinition) -> Result<bool, RpcError> {
    let thread_pool = ThreadPool::new(ld.clone(),get_host_bridge());
    let mut update_map = self.actors.write().await;
    update_map.insert(ld.actor_id.to_string(), thread_pool);
    Ok(true)
  }
}
struct Inner {
  bridge: &'static HostBridge,
}
struct ThreadPool{
  linkdefs: LinkDefinition,
  threads: HashMap<String,(bool,u64)>,//bool:run or not, u64: last_update
  inner: Arc<RwLock<Inner>>,
}
impl ThreadPool{
  pub fn new(ld:LinkDefinition,bridge:&'static HostBridge)->Self{
    if env_logger::try_init().is_err() {}
    ThreadPool{
      linkdefs:ld,
      threads: HashMap::new(),
      inner:Arc::new(RwLock::new(Inner{bridge})),
    }
  }
}
lazy_static! {
  static ref TIME: Arc<Mutex<HashMap<String,u64>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[async_trait]
impl Thread for ThreadProvider {
    async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
      let start = Instant::now();
      //let MAP = Arc::new(Mutex::new(HashMap::new()));
      let mut actors = self.actors.clone();
      let ctxr = ctx.clone();
      let start_thread_request_c = start_thread_request.clone();

      tokio::spawn(async move{
          let mut thread_actor = actors.write().await;
          let v = ctxr.actor.clone();          
          let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
          let utc: DateTime<Utc> = Utc::now();
          let time_stamp = utc.timestamp_millis() as u64;
          thread_pool.threads.insert(start_thread_request_c.game_id.clone(),(true,time_stamp));
          let ld = thread_pool.linkdefs.clone();
          let inner = thread_pool.inner.clone();
          drop(thread_actor);
          loop{
            let mut sleep_interval_cal = None;
            {
              let mut thread_actor = actors.write().await;
              let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
              if let Some((v,last_update)) = thread_pool.threads.get_mut(&start_thread_request_c.game_id.clone()){
                if *v{
                  //drop(thread_pool);
                  let utc: DateTime<Utc> = Utc::now();
                  let time_stamp = utc.timestamp_millis() as u64;
                  if time_stamp - *last_update > start_thread_request_c.sleep_interval as u64  {
                    *last_update = time_stamp;
                  }else{
                    sleep_interval_cal = Some(time_stamp - *last_update);
                  }
                }else{
                  drop(thread_actor);
                  break;
                }
              }
            }
            if let Some(sleep_interval) = sleep_interval_cal{
              sleep(Duration::from_millis(sleep_interval));
              continue
            }
            let m = StartThreadRequest{
              game_id: start_thread_request_c.game_id.clone(),
              elapsed: start.elapsed().as_secs() as u32,
              timestamp: time_stamp,
              sleep_interval: start_thread_request_c.sleep_interval,
              subject: start_thread_request_c.subject.clone(),
            };
            // let time_c = TIME.clone();
            // time_update(time_c,start_thread_request_c.game_id.clone(),time_stamp);
            let read_guard = inner.read().await;
            let bridge = read_guard.bridge;
            let tx = ProviderTransport::new_with_timeout(&ld, Some(bridge), Some(std::time::Duration::new(2,0)));
            let ctx = Context::default();
            let actor = ThreadSender::via(tx);
            match actor.handle_request(&ctx, &m).await {
              Err(RpcError::Timeout(_)) => {
                info!(
                      "actor {} req  timed out: returning 503",
                      &ld.actor_id,
                  );
              }
              Ok(resp) => {
                // info!(
                //       "http response received from actor {}",
                //       &ld.actor_id
                //   );
              }
              Err(e) => {
                info!(
                      "actor {} responded with error {}",
                      &ld.actor_id,
                      e.to_string()
                  );
              }
            }
          }
      });
      Ok(StartThreadResponse{})
    }
    async fn handle_request(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse>{
      Ok(StartThreadResponse{})
    }
    async fn now(&self, ctx: &Context, request: &StartThreadRequest) -> RpcResult<u64>{
      let time_c = TIME.clone();
      let mut guard = match time_c.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          poisoned.into_inner()
        },
      };
      let mut b =0;
      if let Some(t)= guard.get(&request.game_id){
        b = *t;
      }
      Ok(b)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // handle lattice control messages and forward rpc to the provider dispatch
  // returns when provider receives a shutdown control message
  provider_main(ThreadProvider::default(),None)?;

  eprintln!("Thread provider exiting");
  Ok(())
}
fn time_update(time:Arc<Mutex<HashMap<String,u64>>>,game_id:String,time_stamp:u64){
  let mut guard = match time.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };

  if let Some(t)= guard.get_mut(&game_id){
    *t = time_stamp;
  }
}