#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(target_arch = "wasm32")]
use ws_stream_wasm;
use pharos;
use thiserror::Error;

pub struct WebSocketClient<Tx, Rx,Sx> where Sx:pharos::Observable<ws_stream_wasm::WsEvent> {
    tx: Tx,
    rx: Rx,
    meta: Sx,
}

impl<Tx, Rx,Sx> WebSocketClient<Tx, Rx,Sx> where Sx:pharos::Observable<ws_stream_wasm::WsEvent>{
    pub fn split(self) -> (Tx, Rx) {
        (self.tx, self.rx)
    }
    pub async fn observe_close(&mut self)->Result<pharos::Events<ws_stream_wasm::WsEvent>,<Sx as pharos::Observable<ws_stream_wasm::WsEvent>>::Error>{
      self.meta.observe(pharos::Filter::Pointer( ws_stream_wasm::WsEvent::is_closed ).into()).await
    }
}

#[non_exhaustive]
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SinkError {
    // TODO: Refine
    #[error("send failed {0}")]
    Send(String),
}
