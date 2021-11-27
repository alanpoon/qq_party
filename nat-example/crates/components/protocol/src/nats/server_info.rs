use serde::{Serialize, Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize,PartialEq)]
pub struct ServerInfo {
    /// The unique identifier of the NATS server.
    pub server_id: String,
    /// Generated Server Name.
    pub server_name: String,
    /// The host specified in the cluster parameter/options.
    pub host: String,
    /// The port number specified in the cluster parameter/options.
    pub port: u16,
    /// The version of the NATS server.
    pub version: String,
    /// If this is set, then the server should try to authenticate upon
    /// connect.
    pub auth_required: bool,
    /// If this is set, then the server must authenticate using TLS.
    pub tls_required: bool,
    /// Maximum payload size that the server will accept.
    pub max_payload: i32,
    /// The protocol version in use.
    pub proto: i8,
    /// The server-assigned client ID. This may change during reconnection.
    pub client_id: u64,
    /// The version of golang the NATS server was built with.
    pub go: String,
    /// The nonce used for nkeys.
    pub nonce: String,
    /// A list of server urls that a client can connect to.
    pub connect_urls: Vec<String>,
    /// The client IP as known by the server.
    pub client_ip: String,
}

impl ServerInfo {
  pub fn parse(s: &str) -> Option<ServerInfo> {
      let mut obj = json::parse(s).ok()?;
      Some(ServerInfo {
          server_id: obj["server_id"].take_string()?,
          server_name: obj["server_name"].take_string().unwrap_or_default(),
          host: obj["host"].take_string()?,
          port: obj["port"].as_u16()?,
          version: obj["version"].take_string()?,
          auth_required: obj["auth_required"].as_bool().unwrap_or(false),
          tls_required: obj["tls_required"].as_bool().unwrap_or(false),
          max_payload: obj["max_payload"].as_i32()?,
          proto: obj["proto"].as_i8()?,
          client_id: obj["client_id"].as_u64()?,
          go: obj["go"].take_string()?,
          nonce: obj["nonce"].take_string().unwrap_or_default(),
          connect_urls: obj["connect_urls"]
              .members_mut()
              .filter_map(|m| m.take_string())
              .collect(),
          client_ip: obj["client_ip"].take_string().unwrap_or_default(),
      })
  }
}
