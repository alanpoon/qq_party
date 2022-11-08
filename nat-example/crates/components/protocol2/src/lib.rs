mod client_state;
pub mod event;
pub mod command;
pub use command::*;
pub use client_state::*;
pub use event::*;
use nats_lite::nats;
use std::io::{BufReader,BufWriter};
use std::io::Write;
const BUF_CAPACITY: usize = 128 * 1024;

use std::io;
pub fn handle_server_op(msg:Vec<u8>)->io::Result<nats::proto::ServerOp>{
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
    let server_op:Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
    server_op.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
  }
  pub fn handle_client_op(client_op:nats::proto::ClientOp)->io::Result<Vec<u8>>{
    let mut bytes:Vec<u8> = vec![];
    let mut writer = BufWriter::with_capacity(BUF_CAPACITY,&mut *bytes);
    nats::proto::encode(&mut writer,client_op.clone())?;
    if let Ok(_)= writer.flush(){
      
    }
    //info!("flush {:?}",client_op);
    Ok(writer.buffer().to_vec())
  }