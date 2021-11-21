#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasmbus_rpc::{core::LinkDefinition, provider::HostBridge, RpcError};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use wasmbus_rpc::provider::prelude::*;
use wasmbus_rpc::provider::ProviderTransport;
use std::thread::sleep;
use std::{collections::HashMap, time::Duration, time::Instant};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use tokio::sync::RwLock;
use futures::executor::block_on;

use lazy_static::lazy_static; // 1.4.0
const TURN_DELAY_MILLIS_DEFAULT: u64 = 2000;
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
  threads: HashMap<String,bool>,
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
  static ref MAP: Arc<Mutex<HashMap<String,bool>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[async_trait]
impl Thread for ThreadProvider {
    async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
      let start = Instant::now();
      info!("inside thread");
      //let MAP = Arc::new(Mutex::new(HashMap::new()));
      let mut actors = self.actors.clone();
      let ctxr = ctx.clone();
      let start_thread_request_c = start_thread_request.clone();

      tokio::spawn(async move{
          info!("inside thread2");
          let mut thread_actor = actors.write().await;
          let v = ctxr.actor.clone();
          info!("inside thread2 v: {:?}",v);
          let thread_poolx = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap());
          info!("inside thread2 thread_poolx is some: {:?}",thread_poolx.is_some());
          
          let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
          info!("before insert ");
          thread_pool.threads.insert(start_thread_request_c.game_id.clone(),true);
          let ld = thread_pool.linkdefs.clone();
          let inner = thread_pool.inner.clone();
          drop(thread_actor);
          loop{
            info!("1elapsed {:?}",start.elapsed().as_secs());
            let mut thread_actor = actors.read().await;
            let mut thread_pool = (*thread_actor).get(&ctxr.actor.clone().unwrap()).unwrap();
            info!("elapsed {:?} thread_pool.threads len {:?}",start.elapsed().as_secs(),thread_pool.threads.len());
            if let Some(v) = thread_pool.threads.get(&start_thread_request_c.game_id.clone()){
              info!("v {:?}",v);
              if *v{
                //drop(thread_pool);
                info!("after drop");
                sleep(Duration::from_millis(TURN_DELAY_MILLIS_DEFAULT));
                let local: DateTime<Local> = Local::now();
                let m = StartThreadRequest{
                  game_id: start_thread_request_c.game_id.clone(),
                  elapsed: TURN_DELAY_MILLIS_DEFAULT as u32,
                  timestamp: local.timestamp_millis() as u64,
                };
                let read_guard = inner.read().await;
                let bridge = read_guard.bridge;
                info!("before ProviderTransport");
                let tx = ProviderTransport::new_with_timeout(&ld, Some(bridge), Some(std::time::Duration::new(2,0)));
                info!("after ProviderTransport");
                let ctx = wasmbus_rpc::Context::default();
                let actor = ThreadSender::via(tx);
                info!("via");
                match actor.handle_request(&ctx, &m).await {
                  Err(RpcError::Timeout(_)) => {
                    info!(
                          "actor {} req  timed out: returning 503",
                          &ld.actor_id,
                      );
                  }
                  Ok(resp) => {
                    info!(
                          "http response received from actor {}",
                          &ld.actor_id
                      );
                  }
                  Err(e) => {
                    info!(
                          "actor {} responded with error {}",
                          &ld.actor_id,
                          e.to_string()
                      );
                  }
                }
                info!("end");
              }else{
                drop(thread_actor);
                break;
              }
            }else{
              break;
            }
          }
      });
      Ok(StartThreadResponse{})
    }
    async fn handle_request(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse>{
      Ok(StartThreadResponse{})
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // handle lattice control messages and forward rpc to the provider dispatch
  // returns when provider receives a shutdown control message
  provider_main(ThreadProvider::default())?;

  eprintln!("Thread provider exiting");
  Ok(())
}