// command_closure: Arc<dyn Fn(RC) -> Ready<Result<Vec<u8>, String>>+ Send + Sync>,
//     event_closure:Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<Arc<dyn RE +Send+Sync>>>

use std::sync::Arc;
use protocol;
use futures::future::{ready,Ready};
use eyre::{Result,Report};
use client_websocket::{RE,RC};
// pub fn handle_server_op(msg:Vec<u8>)->io::Result<nats::proto::ServerOp>{
//     let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
//     let server_op:Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
//     server_op.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
//   }
#[derive(Clone)]
pub struct Command{

}

#[derive(Clone)]
pub struct Event{

}
impl RE for Event{

}
pub static command_closure:Arc<dyn Fn(Command) -> Ready<Result<Vec<u8>, String>>+ Send + Sync> = Arc::new(|rc|{
ready(Ok(vec![]))
});

pub static event_closure:Arc<dyn Fn(Result<Vec<u8>,Report>) -> Result<Arc<Event>>+ Send + Sync> = Arc::new(|rc|{
    Ok(Arc::new(Event{}))
});