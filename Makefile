.PHONY: build build2
build:
	(cd capability_providers/game-provider && make push)
	(cd actors/game_logic && make push)
	wash ctl apply NAXNT4ASSO4ENYZGKYU5DMIKCTPM72O5Q7RJCRUYKF2YGALBHVEU3PET ops2/manifest.yaml --ctl-seed SCABKLLO4OZAT4WERZ2BC4NDFHNUJO6WOGHVG4JLKPFUVJLHAP4WQWWSJ4
	(cd actors/game_logic && make start-thread)
serve_ui:
	(cd nat-example && cargo make wasm-bindgen)
	(cd nat-example && nohup basic-http-server public -a 127.0.0.1:4001 >> nohup.out 2>&1 &)