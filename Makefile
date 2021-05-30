all: build

build:
	IMAGE_HOST_PKEY=$(shell wash claims inspect ./actors/ws_server/target/wasm32-unknown-unknown/release/ws_server_s.wasm -o json | jq ".module" | tr -d "\"")
	echo IMAGE_HOST_PKEY