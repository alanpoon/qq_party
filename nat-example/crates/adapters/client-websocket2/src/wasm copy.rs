use crate::{command_sender, event_receiver};
use async_trait::async_trait;
use tracing::event;
use std::borrow::Cow;
use eyre::{Result,Report};
use lazy_static::lazy_static;
use futures::channel::mpsc::channel;
use futures::future::{Ready,ready};
use futures::prelude::*;
use crate::Client;
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub Cow<'static, str>);
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use log::*;
lazy_static! {
    static ref EVENTS: Mutex<HashMap<ClientName, Vec<Vec<u8>>>> = Mutex::new(HashMap::default());
    static ref EVENTS_CALLBACK: Mutex<HashMap<ClientName, Vec<Vec<u8>>>> = Mutex::new(HashMap::default());
}

pub struct WebSocketClient<Tx,RE> {
    client_name: ClientName,
    command_sender: Tx,
    pub url:String,
    pub events: Arc<Mutex<HashMap<ClientName,Vec<RE>>>>,
    pub events_callback: Arc<Mutex<HashMap<ClientName,Vec<RE>>>>,
}

#[async_trait]
impl<Tx,RC,RE:Clone> Client<RC,RE>  for WebSocketClient<Tx,RE>
where
    Tx: Sink<RC, Error = String> + Clone + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<RC, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<RE>> {
        let mut map = self.events.lock().unwrap();
        let events = map.get_mut(&self.client_name).unwrap();
        let result = events.clone();
        events.clear();
        events.truncate(10);
        return Some(result);
    }
}

pub async fn connect<RC:Send + Sync +'static,RE:Send + Sync +'static+Clone>(
    client_name: ClientName,
    url: String,
    command_closure: Arc<dyn Fn(RC) -> Ready<Result<Vec<u8>, String>>+ Send + Sync>,
    event_closure:Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<RE>+Send+Sync>
) -> Result<(
    WebSocketClient<impl Sink<RC, Error = String> + Clone + Send + Sync + Unpin + 'static
    ,RE>,pharos::Events<ws_stream_wasm::WsEvent>),
> {
    let mut meta = cross_websocket::connect(url.clone()).await?;
    let _client_name_c = client_name.clone();
    let evt:pharos::Events<ws_stream_wasm::WsEvent> = meta.observe_close().await.unwrap();
    let (tx, rx)= meta.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    wasm_bindgen_futures::spawn_local(
        rx_clone.map(Ok)
      .forward(tx).map(|_|{info!("zzzz");()}));
    
    let event_receiver = event_receiver(rx,event_closure);
    let mut ws_client = WebSocketClient {
        client_name: client_name.clone(),
        command_sender: command_sender(tx_clone.sink_map_err(|err| err.to_string()),command_closure),
        url:url,
        events:Arc::new(Mutex::new(HashMap::new())),
        events_callback:Arc::new(Mutex::new(HashMap::new()))
    };
    
    ws_client.events
    //EVENTS
        .lock()
        .unwrap()
        .insert(client_name.clone(), Vec::new());
    
    wasm_bindgen_futures::spawn_local(async {event_receiver.for_each(move |event| {
          ready(
            ws_client.events
                .lock()
                .unwrap()
                .get_mut(&client_name)
                .unwrap()
                .push(event)
                //.push(Event::Nats(client_name.0.to_string(),event)),
          )
      }).await;
    });
    Ok((ws_client,evt))
}
