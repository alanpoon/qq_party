start server
cd runner & make

send nats command
cd runner & cargo run --bin msg

https://docs.nats.io/nats-tools/nsc/basics

bash ops/compile.sh ws_gateway,ws_server,wasm_user


nats-account-server -c ops/nas_not.conf
nohup nats-server -c ops/gateway.conf -V -D nohup.out 2>&1 &
nats-server -c ops/gateway.conf -V -D
# nats-server -V -D
# nats-server -c ops/websocket_leaf2.conf -V -D
# nats-server -c ops/websocket_leaf3.conf -V -D
# nats-server -c ops/websocket.conf -V -D

RUST_LOG=crane=trace,info,debug wasmcloud -m ops/manifest.yaml --rpc-jwt ~/.nsc/nats/O/accounts/A/users/admin.jwt --rpc-seed SUAL3RJ5MZK7X3XENQ2A573JFCUAMI45KBSOAFRIBWUP6I4VSJOUVP7LGQ

RUST_BACKTRACE=1 RUST_LOG=crane=trace,info,debug wasmcloud -m ops/manifest.yaml --rpc-jwt eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJqdGkiOiJETzNRQ01WS1VXSUpGUFE2WVBRUlpYNldWS1hQQjJFT1hXT0hWWlE2RE5DNUZMQk1ZVldBIiwiaWF0IjoxNjIyMjk1MzQ0LCJpc3MiOiJBQVA3VVpQTkVFM0xEVFdTSVBBRDZESFZJWDRIS0JUREdQTjU2NVRXWUdDUlZERlJJUVZOWERBQiIsIm5hbWUiOiJhZG1pbiIsInN1YiI6IlVCQkRFT1laNkRUSzY1VjZYWkVGNVlPMkNWWVVCWldCTzJJMkZUQTRCNTJKRlRGMlBBU0czS1lMIiwibmF0cyI6eyJwdWIiOnt9LCJzdWIiOnt9LCJzdWJzIjotMSwiZGF0YSI6LTEsInBheWxvYWQiOi0xLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.w_evRL2p3u-McwjntNTGuP_-0UPQgr4ZdGVBUZyz-nAU7MhNQAQ8Qh2gf6UBvmKioAnL4B_lQtOQOT0GpGowCA --rpc-seed SUAL3RJ5MZK7X3XENQ2A573JFCUAMI45KBSOAFRIBWUP6I4VSJOUVP7LGQ --rpc-host 0.0.0.0 --rpc-port 4222

RUST_BACKTRACE=1 RUST_LOG=crane=trace,info,debug rust-gdb --args ../wasmcloud/target/debug/wasmcloud -m ops/manifest2.yaml --rpc-jwt eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJqdGkiOiJETzNRQ01WS1VXSUpGUFE2WVBRUlpYNldWS1hQQjJFT1hXT0hWWlE2RE5DNUZMQk1ZVldBIiwiaWF0IjoxNjIyMjk1MzQ0LCJpc3MiOiJBQVA3VVpQTkVFM0xEVFdTSVBBRDZESFZJWDRIS0JUREdQTjU2NVRXWUdDUlZERlJJUVZOWERBQiIsIm5hbWUiOiJhZG1pbiIsInN1YiI6IlVCQkRFT1laNkRUSzY1VjZYWkVGNVlPMkNWWVVCWldCTzJJMkZUQTRCNTJKRlRGMlBBU0czS1lMIiwibmF0cyI6eyJwdWIiOnt9LCJzdWIiOnt9LCJzdWJzIjotMSwiZGF0YSI6LTEsInBheWxvYWQiOi0xLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.w_evRL2p3u-McwjntNTGuP_-0UPQgr4ZdGVBUZyz-nAU7MhNQAQ8Qh2gf6UBvmKioAnL4B_lQtOQOT0GpGowCA --rpc-seed SUAL3RJ5MZK7X3XENQ2A573JFCUAMI45KBSOAFRIBWUP6I4VSJOUVP7LGQ --rpc-host 0.0.0.0 --rpc-port 4222

../wasmcloud/target/debug/wasmcloud

RUST_BACKTRACE=1 RUST_LOG=info,debug wasmcloud -m ops/manifest2.yaml --rpc-jwt eyJ0eXAiOiJKV1QiLCJhbGciOiJlZDI1NTE5LW5rZXkifQ.eyJqdGkiOiJETzNRQ01WS1VXSUpGUFE2WVBRUlpYNldWS1hQQjJFT1hXT0hWWlE2RE5DNUZMQk1ZVldBIiwiaWF0IjoxNjIyMjk1MzQ0LCJpc3MiOiJBQVA3VVpQTkVFM0xEVFdTSVBBRDZESFZJWDRIS0JUREdQTjU2NVRXWUdDUlZERlJJUVZOWERBQiIsIm5hbWUiOiJhZG1pbiIsInN1YiI6IlVCQkRFT1laNkRUSzY1VjZYWkVGNVlPMkNWWVVCWldCTzJJMkZUQTRCNTJKRlRGMlBBU0czS1lMIiwibmF0cyI6eyJwdWIiOnt9LCJzdWIiOnt9LCJzdWJzIjotMSwiZGF0YSI6LTEsInBheWxvYWQiOi0xLCJ0eXBlIjoidXNlciIsInZlcnNpb24iOjJ9fQ.w_evRL2p3u-McwjntNTGuP_-0UPQgr4ZdGVBUZyz-nAU7MhNQAQ8Qh2gf6UBvmKioAnL4B_lQtOQOT0GpGowCA --rpc-seed SUAL3RJ5MZK7X3XENQ2A573JFCUAMI45KBSOAFRIBWUP6I4VSJOUVP7LGQ --rpc-host 0.0.0.0 --rpc-port 4222

OAQH7NZJAVZP2XAMOQONTOIFHASYAHB44XLVH76WFYBE5C3MPHGXZIKG

curl http://localhost:8080/echo

curl http://localhost:8081/echo2

ws_server
wash ctl call MCSYADSEYARGJ3KVWIMIMLLOOPTA7NJN7IUM7WZXFWJ3GXIWJMXDKRXA GatewayPublish {"BrokerMessage":"hi",}

wasm_user
RUST_LOG=crane=trace,info,debug wash ctl call MCJTMRY252TORMJVVBGT4XWJRJNFCVGJPOLXBY6A7GSCMVY6KVLA6CLB Ping '{"value": 2}'

wash ctl call MCJTMRY252TORMJVVBGT4XWJRJNFCVGJPOLXBY6A7GSCMVY6KVLA6CLB Ping '{"value": 2}'

wash ctl call MDN3AIPQ62QAFZJCSULSCR5D2NQYARPDYK763YLG4EYZLMPKECEWIFY2 StartThreadRequest '{"game_id": "hi"}'

codesign -fs gdb_codesign "$(which gdb)"
