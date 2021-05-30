start server
cd runner & make

send nats command
cd runner & cargo run --bin msg

https://docs.nats.io/nats-tools/nsc/basics

nats-account-server -operator ~/.nsc/nats/O/O.jwt
nats-server -V -D
RUST_LOG=crane=trace,info,debug wasmcloud -m ops/manifest.yaml --allow-live-updates


nats-server -c ops/gateway.conf -V -D
RUST_LOG=crane=trace,info,debug wasmcloud -m ops/manifest.yaml --allow-live-updates --control-credsfile ~/.nkeys/creds/O/A/admin.creds


curl http://localhost:8080/echo