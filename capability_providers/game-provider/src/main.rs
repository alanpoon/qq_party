#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasmbus_rpc::{core::LinkDefinition, provider::HostBridge, RpcError};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver};
use wasmbus_rpc::provider::prelude::*;
use std::thread::sleep;
use std::{collections::HashMap, time::Duration, time::Instant};
use std::sync::{Arc, Mutex, RwLock};
use chrono::prelude::*;
use lazy_static::lazy_static; // 1.4.0
const TURN_DELAY_MILLIS_DEFAULT: u64 = 2000;
#[allow(unused)]
const CAPABILITY_ID: &str = "wasmcloud:game";

const OP_START_THREAD: &str = "StartThread";
const OP_STOP_THREAD: &str = "StopThread";
const OP_HANDLE_THREAD: &str = "HandleThread";

struct Inner {
  bridge: &'static HostBridge,
}
#[derive(Default, Clone, Provider)]
#[services(Thread)]
pub struct ThreadProvider {
  inner: Arc<RwLock<Inner>>,
}
/// use default implementations of provider message handlers
impl ProviderDispatch for ThreadProvider {}
/// we don't need to override put_link, delete_link, or shutdown
impl ProviderHandler for ThreadProvider {
  async fn put_link(&self, ld: &LinkDefinition) -> Result<bool, RpcError> {
    
  }
}

lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,bool>>> = Arc::new(Mutex::new(HashMap::new()));
}
#[async_trait]
impl Thread for ThreadProvider {
    async fn start_thread(&self, _ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
      let start = Instant::now();
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
                let m = StartThreadRequest{
                  game_id: start_thread_request.game_id.clone(),
                  elapsed: Some(TURN_DELAY_MILLIS_DEFAULT as f32),
                  timestamp: Some(local.timestamp_millis()),
                };
                // if let Ok(buf) = serialize(m){
                //   let module_lock = (*module).read().unwrap();
                //   lock.dispatch(
                //     &module_lock,
                //     OP_HANDLE_THREAD,
                //     &buf,
                //   );
                // }

                let read_guard = self.inner.read().await;
                let bridge = read_guard.bridge;
                let response = match Self::send_actor(linkdefs, req, bridge, timeout).await
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!(
                            "sending HttpRequest to actor ({} {}): {}",
                            &method,
                            &path.as_str(),
                            e
                        );
                        HttpResponse {
                            status_code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            ..Default::default()
                        }
                    }
                };
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
      let m = StartThreadResponse{};
      Ok(m)
    }
}