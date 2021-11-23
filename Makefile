.PHONY: build build2
build:
	(cd capability_providers/game-provider && make push)
	(cd actors/game_logic && make push)
	wash ctl apply $(shell wash ctl get hosts -o json | jq -r ".hosts[0].id") ops2/manifest.yaml
	(cd actors/game_logic && make start-thread)
build2:
	cargo build --target wasm32-unknown-unknown