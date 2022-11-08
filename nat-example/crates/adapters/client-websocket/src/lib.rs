#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

mod next_vec;
pub use next_vec::*;

use eyre::{Result};
use futures::future::{ready};
use futures::{prelude::*, Stream};
use tracing::error;

fn event_receiver(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
) -> impl Stream<Item = Vec<u8>>  + 'static + Unpin {
    rx
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

pub trait Client {
    fn sender(&self) -> Box<dyn Sink<Vec<u8>, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Vec<u8>>>;
}