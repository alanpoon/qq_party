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
use std::sync::Arc;
use tracing::error;

fn event_receiver2(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
    closure: Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<Vec<u8>>>
) -> impl Stream<Item = Vec<u8>>  + 'static + Unpin {
    rx.map(move|bytes| -> Result<Vec<u8>> { closure(bytes)})
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

fn command_sender2(
    tx: impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + 'static + Unpin,
    closure: Arc<dyn Fn(Vec<u8>) -> Ready<Result<Vec<u8>, String>>+ Send + Sync>
) -> impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + 'static + Unpin {
    tx.with(move|command: Vec<u8>| -> Ready<Result<Vec<u8>, String>> {
        closure(command)
    })
}

pub trait Client4 {
    fn sender(&self) -> Box<dyn Sink<Vec<u8>, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Vec<u8>>>;
}