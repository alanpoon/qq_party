use crate::{event_receiver};
use async_trait::async_trait;
use eyre::{Result};
use lazy_static::lazy_static;
use futures::channel::mpsc::channel;
use futures::future::{ready};
use futures::prelude::*;
use crate::{Client};
use std::borrow::Cow;
use std::sync::Mutex;
use std::collections::HashMap;
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub Cow<'static, str>);
use log::*;
lazy_static! {
    static ref EVENTS: Mutex<HashMap<ClientName, Vec<Vec<u8>>>> = Mutex::new(HashMap::default());
}

pub struct WebSocketClient<Tx> {
    client_name: ClientName,
    command_sender: Tx,
    pub url:String,
}

#[async_trait]
impl<Tx> Client for WebSocketClient<Tx>
where
    Tx: Sink<Vec<u8>, Error = String> + Clone + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<Vec<u8>, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Vec<u8>>> {
        let mut map = EVENTS.lock().unwrap();
        let events = map.get_mut(&self.client_name).unwrap();
        let result = events.clone();
        events.clear();
        events.truncate(10);
        return Some(result);
    }
}
pub async fn connect(
    client_name: ClientName,
    url: String,
) -> Result<(
    WebSocketClient<impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + Unpin + 'static
    >,pharos::Events<ws_stream_wasm::WsEvent>),
> {
    let mut meta = cross_websocket::connect(url.clone()).await?;
    let _client_name_c = client_name.clone();
    let evt:pharos::Events<ws_stream_wasm::WsEvent> = meta.observe_close().await.unwrap();
    let (tx, rx)= meta.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    wasm_bindgen_futures::spawn_local(rx_clone.map(Ok)
      .forward(tx).map(|_|{info!("zzzz");()}));
    
    let event_receiver = event_receiver(rx);
    let result = Ok((WebSocketClient {
        client_name: client_name.clone(),
        command_sender: tx_clone.sink_map_err(|err| err.to_string()),
        url:url,
    },evt));
    EVENTS
        .lock()
        .unwrap()
        .insert(client_name.clone(), Vec::new());
    
    wasm_bindgen_futures::spawn_local(async {event_receiver.for_each(move |event| {
          ready(
            EVENTS
                .lock()
                .unwrap()
                .get_mut(&client_name)
                .unwrap()
                .push(event)
                //.push(Event::Nats(client_name.0.to_string(),event)),
          )
      }).await;
    });
    result
}
