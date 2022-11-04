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

use eyre::Result;
use protocol::futures::future::{ready, Ready};
use protocol::futures::{prelude::*, Stream};
use protocol::{unwrap_and_log, RawCommand, RawEvent};
use protocol::{handle_client_op,handle_server_op};
use tracing::error;
fn event_receiver(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
) -> impl Stream<Item = RawEvent> + Send + Sync + 'static + Unpin {
    rx.map(|bytes| -> Result<RawEvent> { Ok(
      handle_server_op(bytes?)?
    )})
        .filter_map(unwrap_and_log!())
}

fn command_sender(
    tx: impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + 'static + Unpin,
) -> impl Sink<RawCommand, Error = String> + Clone + Send + Sync + 'static + Unpin {
    tx.with(|command: RawCommand| -> Ready<Result<Vec<u8>, String>> {
        match handle_client_op(command){
            Ok(vec) => ready(Ok(vec)),
            Err(err) => {
                error!("{}", err);
                ready(Err(err.to_string()))
            }
        }
    })
}
