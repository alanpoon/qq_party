.PHONY: build build2
build:
	(cd capability_providers/game-provider && make push)
	(cd actors/game_logic && make push)
	wash ctl apply CBLXC6GW777ZB4EZBVDWJ7AJHA5R4TIN7GQC32XMCDA4NIPQQTPY3SVP ops2/manifest.yaml
	sleep 30
	(cd actors/game_logic && make start-thread)
serve_ui:
	(cd nat-example && cargo make wasm-bindgen)
	(cd nat-example && nohup basic-http-server public -a 127.0.0.1:4001 >> nohup.out 2>&1 &)