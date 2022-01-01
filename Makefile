.PHONY: build build2
build:
	(cd capability_providers/game-provider && make push)
	(cd capability_providers_officials/nats && make push)
	(cd actors/game_logic && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS ops/manifest.yaml
	sleep 34
	(cd actors/game_logic && make start-thread)
serve_ui:
	(cd nat-example && cargo make wasm-bindgen)
	(cd nat-example && nohup basic-http-server public -a 127.0.0.1:4001 >> nohup.out 2>&1 &)