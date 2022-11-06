#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

// #[cfg(target_arch = "wasm32")]
mod wasm;
// #[cfg(target_arch = "wasm32")]
pub use wasm::*;

mod next_vec;
pub use next_vec::*;

use eyre::{Result,Report};
use futures::future::{ready, Ready};
use futures::{prelude::*, Stream};
use std::io::{BufReader,BufWriter};
use std::sync::Arc;
// use protocol::{RawCommand, RawEvent};
// use protocol::{handle_client_op,handle_server_op};
use tracing::error;
fn event_receiver(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
    closure: Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<Arc<dyn RE + Send + Sync>>>
) -> impl Stream<Item = Arc<dyn RE+Sync+Send>>  + 'static + Unpin {
    rx.map(move|bytes| -> Result<Arc<dyn RE + Send + Sync>> { closure(bytes)})
        .filter_map(|item|
            ready(match item {
                Ok(ok) => Some(ok),
                Err(err) => {
                    error!("{}", err);
                    None
                }
            })
        )
}

fn command_sender<RC:Send + Sync +'static>(
    tx: impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + 'static + Unpin,
    closure: Arc<dyn Fn(RC) -> Ready<Result<Vec<u8>, String>>+ Send + Sync>
) -> impl Sink<RC, Error = String> + Clone + Send + Sync + 'static + Unpin {
    tx.with(move|command: RC| -> Ready<Result<Vec<u8>, String>> {
        closure(command)
    })
}
const BUF_CAPACITY: usize = 128 * 1024;
use std::io;
// pub fn handle_server_op(msg:Vec<u8>)->io::Result<nats::proto::ServerOp>{
//     let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
//     let server_op:Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
//     server_op.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
//   }RawCommand
pub trait Client<RC,Event> {
    fn sender(&self) -> Box<dyn Sink<RC, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Event>>;
}
pub trait RE {

}
pub trait Client2<RC> {
    fn sender(&self) -> Box<dyn Sink<RC, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Arc<dyn RE + Send + Sync>>>;
}