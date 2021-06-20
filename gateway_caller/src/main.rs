use std::env;
extern crate serdeconv;
extern crate serde_json;
use gateway_interface as gateway;
fn main() {
    let args:Vec<String> = env::args().collect();
    let arg1 = args.get(1).unwrap().to_string();
    let b = json_str_to_msgpack_bytes(arg1).unwrap();
    let n = gateway::BrokerMessage{
      subject: String::from(""),
      reply_to: String::from(""),
      body: b,
    };
    let b = serde_json::to_string(&n).unwrap();
    println!("res {:?}",b);
}
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

pub fn json_str_to_msgpack_bytes(payload: String) -> Result<Vec<u8>> {
  println!("payload {:?}",payload);
  let json: serde_json::value::Value = serde_json::from_str(&payload)?;
  println!("json {:?}",json);
  let payload = serdeconv::to_msgpack_vec(&json)?;
  Ok(payload)
}