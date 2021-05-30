extern crate nats;
use std::error::Error;
use std::{thread, time};
fn check_schedule(nc: nats::Connection){
  let subject = "wasmdome.public.arena.schedule";
  println!("Listening on '{}'", subject);
  for msg in nc.subscribe(&subject).unwrap(){
    msg.respond(b"22");
  }
}
fn main(){
  let nc = match nats::connect("127.0.0.1:4222") {
      Err(_e) => {
          println!("Couldn't connect to the lattice. Is NATS running?");
          
      }
      Ok(v) => {
        check_schedule(v);
      },
  };
}