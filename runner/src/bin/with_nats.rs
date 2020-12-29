extern crate macos_host;
#[macro_use]
extern crate log;
use macos_host::common::{await_actor_count, await_provider_count, par_from_file};
use actix_rt::time::delay_for;
use provider_archive::ProviderArchive;
use std::collections::HashMap;
use std::time::Duration;
use wasmcloud_host::{Actor, HostBuilder, NativeCapability};
use wasmcloud_host::{Host, Result};

pub(crate) async fn distributed_echo() -> Result<()> {
  
    let echo = Actor::from_file("../modules/room_mgr_signed.wasm").unwrap();
    let actor_id = echo.public_key();
    let aid = actor_id.clone();
    let host_b = HostBuilder::new()
        .build();

    host_b.start().await.unwrap();
    host_b.start_actor(echo).await.unwrap();
    await_actor_count(&host_b, 1, Duration::from_millis(50), 3)
        .await
        .unwrap();

    let arc = par_from_file("../modules/libwascc_nats.par.gz").unwrap();
    let websrv = NativeCapability::from_archive(&arc, None)?; 
    let websrv_id = arc.claims().unwrap().subject;   
    host_b.start_native_capability(websrv).await.unwrap();
    
    let arc2 = par_from_file("../modules/libwascc_logging.par.gz").unwrap();
    let websrv2 = NativeCapability::from_archive(&arc2, None)?; 
    let websrv_id2 = arc2.claims().unwrap().subject;
     
    host_b.start_native_capability(websrv2).await.unwrap();
    await_provider_count(&host_b, 3_usize, Duration::from_millis(50), 3_i32)
        .await
        .unwrap();

    let mut webvalues: HashMap<String, String> = HashMap::new();
    webvalues.insert("SUBSCRIPTION".to_string(), "wasmdome.public.arena.schedule".to_string());
    webvalues.insert("URL".to_string(), "nats://127.0.0.1:4222".to_string());
    host_b
        .set_link(
            &aid,
            "wascc:messaging",
            None,
            websrv_id,
            webvalues,
        )
        .await
        .unwrap();
    let mut webvalues2: HashMap<String, String> = HashMap::new();
    host_b
        .set_link(
            &aid,
            "wascc:logging",
            None,
            websrv_id2,
            webvalues2,
        )
        .await
        .unwrap();
    //websrv.handle_call();
    delay_for(Duration::from_secs(100000)).await;
  
    Ok(())
}
#[actix_rt::main]
async fn main() -> Result<()> {
    distributed_echo().await
}