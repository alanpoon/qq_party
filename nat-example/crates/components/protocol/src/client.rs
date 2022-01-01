use std::borrow::Cow;

use futures::Sink;

use crate::{RawCommand, RawEvent,Event};
use crate::nats;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
use log::*;
pub trait Client {
    fn sender(&self) -> Box<dyn Sink<RawCommand, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Event>>;
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub Cow<'static, str>);

pub type BoxClient2 = Box<dyn Client + Send + Sync + 'static>;
const BUF_CAPACITY: usize = 128 * 1024;

pub struct BoxClient{
  pub clients: Vec<Box<dyn Client + Send + Sync + 'static>>,
  pub options: nats::Options,
}
impl Default for BoxClient{
  fn default()->Self{
    BoxClient{
      clients:vec![],
      options: nats::Options::default()
    }
  }
}
impl BoxClient{
  pub fn handle_server_op(&mut self,msg:Vec<u8>)->io::Result<Option<nats::proto::ServerOp>>{
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
    let server_op:Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
    // if let Some(z) = server_op{
    //   // match z {
    //   //   // nats::proto::ServerOp::Info(server_info)=>{
    //   //   //   for url in &server_info.connect_urls {
    //   //   //     connector.add_url(url).ok();
    //   //   //   }
    //   //   //   *self.server_info.lock() = Some(server_info);
    //   //   }
        
    //   }
    Ok(server_op)
  }
  
}
pub fn handle_server_op(msg:Vec<u8>)->io::Result<nats::proto::ServerOp>{
  let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
  let server_op:Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
  info!("server_op {:?}",server_op);
  server_op.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
}
pub fn handle_client_op(client_op:nats::proto::ClientOp)->io::Result<Vec<u8>>{
  let mut bytes:Vec<u8> = vec![];
  let mut writer = BufWriter::with_capacity(BUF_CAPACITY,&mut *bytes);
  nats::proto::encode(&mut writer,client_op.clone())?;
  writer.flush();
  info!("flush {:?}",client_op);
  Ok(writer.buffer().to_vec())
}