extern crate nats;
extern crate nkeys;
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
  let seed = "SUAL3RJ5MZK7X3XENQ2A573JFCUAMI45KBSOAFRIBWUP6I4VSJOUVP7LGQ";
  //let seed = "SAALYEWSBBZQYHTWJJF53ZQPDW33NHNHLTZBTK4ZC2QXMXBXIL3HUH5ZCY";
  let kp = nkeys::KeyPair::from_seed(seed).unwrap();

  fn load_jwt() -> std::io::Result<String> {
      Ok(String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJqdGkiOiJETzNRQ01WS1VXSUpGUFE2WVBRUlpYNldWS1hQQjJFT1hXT0hWWlE2RE5DNUZMQk1ZVldBIiwiaWF0IjoxNjIyMjk1MzQ0LCJpc3MiOiJBQVA3VVpQTkVFM0xEVFdTSVBBRDZESFZJWDRIS0JUREdQTjU2NVRXWUdDUlZERlJJUVZOWERBQiIsIm5hbWUiOiJhZG1pbiIsInN1YiI6IlVCQkRFT1laNkRUSzY1VjZYWkVGNVlPMkNWWVVCWldCTzJJMkZUQTRCNTJKRlRGMlBBU0czS1lMIiwibmF0cyI6eyJwdWIiOnt9LCJzdWIiOnt9LCJzdWJzIjotMSwiZGF0YSI6LTEsInBheWxvYWQiOi0xLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.w_evRL2p3u-McwjntNTGuP_-0UPQgr4ZdGVBUZyz-nAU7MhNQAQ8Qh2gf6UBvmKioAnL4B_lQtOQOT0GpGowCA"))
  }

  let nc = match nats::Options::with_jwt(load_jwt, move |nonce| kp.sign(nonce).unwrap())
            .connect("0.0.0.0:4222") {
      Err(_e) => {
          println!("Couldn't connect to the lattice. Is NATS running?");
          println!("err {:?}",_e);
      }
      Ok(v) => {
        check_schedule(v);
      },
  };
}