# examples/actor/echo-messaging

PROJECT  = game_logic
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:messaging wasmcloud:builtin:logging wasmcloud:thread
#CLAIMS   = wasmcloud:thread wasmcloud:messaging
# registry url for our actor
REG_URL  = localhost:$(reg_host)/v2/$(PROJECT):$(VERSION)
#REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

include ../../build/makefiles/actor.mk

ACTOR_ID=$(shell make actor_id 2>/dev/null | tail -1)
CALL_ALIAS = game_logic
# messaging provider id from wasmcloud.azurecr.io/nats
# If you built it from source, get the provider id with `make inspect` in the source folder
MESSAGING_PROVIDER_ID = VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7
THREAD_PROVIDER_ID = VAEUPY7Y42M5FZSQIABI4FVK5SVNTU2SREZNIUQLGAHMAP4O4WVMZ3KB
link:
	# link to messaging provider
	wash ctl link put $(shell make actor_id | tail -1) \
		$(MESSAGING_PROVIDER_ID) wasmcloud:messaging \
		'URI=nats://localhost:4222' 'SUBSCRIPTION=demo.echo'
	wash ctl link put $(shell make actor_id | tail -1) \
		$(THREAD_PROVIDER_ID) wasmcloud:thread

start-thread:
	@wash call -o json $(ACTOR_ID) --cluster-seed SCABKLLO4OZAT4WERZ2BC4NDFHNUJO6WOGHVG4JLKPFUVJLHAP4WQWWSJ4 Thread.StartThread \
		'{"game_id":"hello","timestamp":2,"elapsed":0}'
start-thread2:
	@wash call -o json $(ACTOR_ID) Thread.StartThread \
		'{"game_id":"hello","timestamp":2,"elapsed":0}'