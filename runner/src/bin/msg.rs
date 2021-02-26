extern crate nats;
use std::error::Error;
fn check_schedule(nc: nats::Connection){
  let res = nc.request_timeout(
      "wasmdome.public.arena.schedule",
      "hello",
      std::time::Duration::from_millis(2500),
  );
  match res {
      Err(e) => {
          println!("Error requesting schedule from the lattice: {}", e);
      }
      _ => {
        println!("{:?}",res);
      },
  };
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