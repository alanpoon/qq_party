#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasmbus_rpc::{core::LinkDefinition, provider::HostBridge};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use wasmbus_rpc::provider::prelude::*;
use wasmbus_rpc::provider::ProviderTransport;
use wasmbus_rpc::actor::prelude::Context;
use wasmbus_rpc::core::{HealthCheckRequest,ActorSender,Actor};
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
    if let Some(sleep_interval) = &ld.values.get("SLEEP_INTERVAL") {
      let start = Instant::now();
      //let MAP = Arc::new(Mutex::new(HashMap::new()));
      let mut actors = self.actors.clone();
      let actor_id = ld.actor_id.clone();
      if let Ok(sleep_interval) = sleep_interval.parse::<u64>(){
        tokio::spawn(async move{
          
          let mut thread_actor = actors.write().await;    
          let mut thread_pool = (*thread_actor).get_mut(&actor_id).unwrap();
          let utc: DateTime<Utc> = Utc::now();
          let time_stamp = utc.timestamp_millis() as u64;
          thread_pool.threads.insert(actor_id.clone(),(true,time_stamp,vec![]));
          let ld = thread_pool.linkdefs.clone();
          let inner = thread_pool.inner.clone();
          drop(thread_actor);
          loop{
            let mut bridge_guard = None;
            {
              let read_guard = inner.read().await;
              bridge_guard = Some(read_guard.bridge);
            }
            if let Some(bridge) = bridge_guard{
              sleep(Duration::from_millis(3000));
              let tx = ProviderTransport::new_with_timeout(&ld, Some(bridge), Some(std::time::Duration::new(2,0)));
              let ctx = Context::default();
              let actor = ActorSender::via(tx);
              match actor.health_request(&ctx, &HealthCheckRequest{}).await {
                Ok(res)=>{
                  if res.healthy{
                    break;
                  }
                },
                Err(er)=>{}
              }
            }
          }
          loop{
            let mut sleep_interval_cal = None;
            let mut bridge_guard = None;
            {
              let read_guard = inner.read().await;
              bridge_guard = Some(read_guard.bridge);
              let mut thread_actor = actors.write().await;
              let mut thread_pool = (*thread_actor).get_mut(&actor_id).unwrap();
              if let Some((v,last_update,_conseq_failures)) = thread_pool.threads.get_mut(&actor_id){
                if *v{
                  sleep_interval_cal = Some(sleep_interval);
                }else{
                  drop(thread_actor);
                  break;
                  //wash ctl stop actor_id
                  //wash ctl start actor_id
                }
              }
            }
            if let Some(sleep_interval) = sleep_interval_cal{
              //eprintln!("sleep_interval {:?} game_id {:?} sleep_interval_req {:?}",sleep_interval,start_thread_request_c.game_id.clone(),start_thread_request_c.sleep_interval);
              sleep(Duration::from_millis(sleep_interval));
              //continue
            }
            let utc: DateTime<Utc> = Utc::now();
            let time_stamp = utc.timestamp_millis() as u64;
            
            if let Some(bridge) = bridge_guard{
              let tx = ProviderTransport::new_with_timeout(&ld, Some(bridge), Some(std::time::Duration::new(2,0)));
              let ctx = Context::default();
              let actor = ThreadSender::via(tx);
              match actor.tick(&ctx, &time_stamp).await {
                Err(RpcError::Timeout(_)) => {
                  eprintln!(
                        "actor {} req  timed out: returning 503",
                        &ld.actor_id,
                    );
                }
                Ok(resp) => {
                }
                Err(e) => {
                  let mut thread_actor = actors.write().await;
                  let mut thread_pool = (*thread_actor).get_mut(&actor_id).unwrap();
                  if let Some((_,_,conseq_failures)) = thread_pool.threads.get_mut(&actor_id){
                    let mut within_range =0;
                    for c in conseq_failures.iter(){
                      if time_stamp - c <7000{
                        within_range+=1;
                      }
                    }
                    if within_range>4{
                      eprintln!("@@@@breaking");
                      break;
                    }else{
                      conseq_failures.push(time_stamp.clone());
                    }
                  }
                  eprintln!(
                        "actor {} responded with error {}",
                        &ld.actor_id,
                        e.to_string()
                    );
                }
              }
            }else{
            }
          }
        });
      }
      
    }
 
    
    Ok(true)
  }
}

struct Inner {
  bridge: &'static HostBridge,
}
struct ThreadPool{
  linkdefs: LinkDefinition,
  threads: HashMap<String,(bool,u64,Vec<u64>)>,//bool:run or not, u64: last_update,Vec<u64>: vec of failure timestamp.
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

#[async_trait]
impl Thread for ThreadProvider {
    async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
      let start = Instant::now();
      let mut actors = self.actors.clone();
      let ctxr = ctx.clone();
      let start_thread_request_c = start_thread_request.clone();

      tokio::spawn(async move{
          let mut thread_actor = actors.write().await;
          let v = ctxr.actor.clone();          
          let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
          let utc: DateTime<Utc> = Utc::now();
          let time_stamp = utc.timestamp_millis() as u64;
          thread_pool.threads.insert(start_thread_request_c.game_id.clone(),(true,time_stamp,vec![]));
          let ld = thread_pool.linkdefs.clone();
          let inner = thread_pool.inner.clone();
          drop(thread_actor);
          loop{
            let mut sleep_interval_cal = None;
            let mut bridge_guard = None;
            {
              let read_guard = inner.read().await;
              bridge_guard = Some(read_guard.bridge);
              let mut thread_actor = actors.write().await;
              let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
              if let Some((v,last_update,_conseq_failures)) = thread_pool.threads.get_mut(&start_thread_request_c.game_id.clone()){
                if *v{
                  sleep_interval_cal = Some(start_thread_request_c.sleep_interval as u64);
                }else{
                  drop(thread_actor);
                  break;
                  //wash ctl stop actor_id
                  //wash ctl start actor_id
                }
              }
            }
            if let Some(sleep_interval) = sleep_interval_cal{
              //eprintln!("sleep_interval {:?} game_id {:?} sleep_interval_req {:?}",sleep_interval,start_thread_request_c.game_id.clone(),start_thread_request_c.sleep_interval);
              sleep(Duration::from_millis(sleep_interval));
              //continue
            }
            let utc: DateTime<Utc> = Utc::now();
            let time_stamp = utc.timestamp_millis() as u64;
            
            if let Some(bridge) = bridge_guard{
              let tx = ProviderTransport::new_with_timeout(&ld, Some(bridge), Some(std::time::Duration::new(2,0)));
              let ctx = Context::default();
              let actor = ThreadSender::via(tx);
              match actor.tick(&ctx, &time_stamp).await {
                Err(RpcError::Timeout(_)) => {
                  eprintln!(
                        "actor {} req  timed out: returning 503",
                        &ld.actor_id,
                    );
                }
                Ok(resp) => {
                }
                Err(e) => {
                  let mut thread_actor = actors.write().await;
                  let mut thread_pool = (*thread_actor).get_mut(&ctxr.actor.clone().unwrap()).unwrap();
                  if let Some((_,_,conseq_failures)) = thread_pool.threads.get_mut(&start_thread_request_c.game_id.clone()){
                    let mut within_range =0;
                    for c in conseq_failures.iter(){
                      if time_stamp - c <7000{
                        within_range+=1;
                      }
                    }
                    if within_range>4{
                      eprintln!("@@@@breaking");
                      break;
                    }else{
                      conseq_failures.push(time_stamp.clone());
                    }
                  }
                  eprintln!(
                        "actor {} responded with error {}",
                        &ld.actor_id,
                        e.to_string()
                    );
                }
              }
            }else{
              eprintln!("no bridge game_id {:?}",start_thread_request_c.game_id.clone())
            }
            
          }
      });
      Ok(StartThreadResponse{})
    }
    async fn tick(&self, ctx: &Context, start_thread_request: &u64) -> RpcResult<u32>{
      Ok(0)
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