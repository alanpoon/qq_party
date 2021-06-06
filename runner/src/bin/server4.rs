extern crate nats;
use std::error::Error;
use std::{thread, time};
fn check_schedule(nc: nats::Connection){
  let subject = "ws_gateway.room";
  println!("Listening on '{}'", subject);
  for msg in nc.subscribe(&subject).unwrap(){
    //let client = nats::connect("127.0.0.1:4222").unwrap();
    //let rply = msg.reply.unwrap();
    println!("reply {:?}",msg);
    //client.publish_with_reply_or_headers(&rply, None, None, &msg.data);
  } 
}
fn main(){
  let nc = match nats::Options::with_user_pass("admin", "secret")
            .connect("127.0.0.1:4222") {
      Err(_e) => {
          println!("Couldn't connect to the lattice. Is NATS running?");
          
      }
      Ok(v) => {
        check_schedule(v);
      },
  };
}