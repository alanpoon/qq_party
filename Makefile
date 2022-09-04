.PHONY: build build2
build:
	(cd capability_providers/game-provider && make push)
	(cd capability_providers_officials/nats && make push)
	(cd actors/game_logic2 && make clean_wasm && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS ops/manifest.yaml
	sleep 15
	(cd actors/game_logic2 && make start-thread)
serve_ui:
	(cd nat-example && cargo make wasm-bindgen)
	(cd nat-example && nohup basic-http-server public -a 127.0.0.1:4001 >> nohup.out 2>&1 &)
build_aws_old:
	(cd capability_providers_officials/nats && make push)
	(cd actors/game_logic2 && make clean_wasm && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS ops/manifest.aws.yaml
	sleep 15
	(cd actors/game_logic2 && make start-thread)
build_aws:
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS ops/manifest.aws.yaml
	sleep 15
	(cd actors/game_logic2 && make start-thread)