extern crate macos_host;
use macos_host::common::{await_actor_count, await_provider_count, par_from_file};
use actix_rt::time::delay_for;
use provider_archive::ProviderArchive;
use std::collections::HashMap;
use std::time::Duration;
use wasmcloud_host::{Actor, HostBuilder, NativeCapability};
use wasmcloud_host::{Host, Result};

// Start two hosts, A and B. Host A contains an actor
// and host B contains a provider. Set a link via host B's
// API and then invoke the provider's running HTTP endpoint
// to ensure the RPC link between actor and provider works
pub(crate) async fn distributed_echo() -> Result<()> {
    let web_port = 7001_u32;
    let echo = Actor::from_file("./modules/echo.wasm").unwrap();
    let actor_id = echo.public_key();
    let aid = actor_id.clone();
    //let nc2 = nats::asynk::connect("0.0.0.0:4222").await?;
    
    let host_b = HostBuilder::new()
        .build();

    host_b.start().await.unwrap();
    host_b.start_actor(echo).await.unwrap();
    await_actor_count(&host_b, 1, Duration::from_millis(50), 3)
        .await
        .unwrap();
    // ** NOTE ** - we need both hosts to be running before we start
    // so that host b will receive the claims from host a

    let arc = par_from_file("./modules/libwascc_httpsrv.par.gz").unwrap();
    let websrv = NativeCapability::from_archive(&arc, None)?; 
    let websrv_id = arc.claims().unwrap().subject;
     
    host_b.start_native_capability(websrv).await.unwrap();
    // always have to remember that "extras" is in the provider list.
    await_provider_count(&host_b, 2_usize, Duration::from_millis(50), 3_i32)
        .await
        .unwrap();

    let mut webvalues: HashMap<String, String> = HashMap::new();
    webvalues.insert("PORT".to_string(), format!("{}", web_port));
    host_b
        .set_link(
            &aid,
            "wascc:http_server",
            None,
            websrv_id,
            webvalues,
        )
        .await
        .unwrap();

    delay_for(Duration::from_secs(2000)).await;
  
    Ok(())
}
#[actix_rt::main]
async fn main() -> Result<()> {
    distributed_echo().await
}