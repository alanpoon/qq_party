.PHONY: build
build:
	(cd actors/game_logic && make push2)
	(cd capability_providers/game-provider && make push)
	wash ctl apply $(shell wash ctl get hosts -o json | jq -r ".hosts[0].id") ops2/manifest.yaml
	(cd actors/game_logic && make start-thread)