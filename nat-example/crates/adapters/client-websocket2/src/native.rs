use async_trait::async_trait;

use eyre::{Report,Result};
use futures::future::{ready, Ready};
use futures::channel::mpsc::channel;
use crate::Client;
use futures::{prelude::*, Sink, Stream};
use std::sync::Arc;
use crate::{command_sender, event_receiver, NextVec,RC,RE};

pub struct WebSocketClient<Tx, Rx> {
    command_sender: Tx,
    event_receiver: Rx,
}

#[async_trait]
impl<Tx, Rx,RC,Event> Client<RC,Event> for WebSocketClient<Tx, Rx>
where
    Tx: Sink<RC, Error = String> + Clone + Send + Sync + Unpin + 'static,
    Rx: Stream<Item = Event> + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<RC, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Event>> {
        futures_lite::future::block_on(NextVec(&mut self.event_receiver))
    }
}

pub async fn connect<RC:Send + Sync +'static,RE:Send + Sync +'static>(
    url: String,
    command_closure: Arc<dyn Fn(RC) -> Ready<Result<Vec<u8>, String>>+ Send + Sync>,
    event_closure:Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<RE>+Send+Sync>
) -> Result<
    WebSocketClient<
        impl Sink<RC, Error = String> + Clone + Send + Sync + Unpin + 'static,
        impl Stream<Item = RE> + Send + Sync + Unpin + 'static,
    >,
> {
    let (tx, rx) = cross_websocket::connect(url).await?.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    tokio::spawn(rx_clone.map(Ok).forward(tx));

    Ok(WebSocketClient {
        command_sender: command_sender(tx_clone.sink_map_err(|err| err.to_string()),command_closure),
        event_receiver: event_receiver(rx,event_closure),
    })
}
