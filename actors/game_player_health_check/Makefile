PROJECT  = game_player_health_check
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:messaging wasmcloud:builtin:logging
#CLAIMS   = wasmcloud:thread wasmcloud:messaging
# registry url for our actor
REG_URL  = $(shell if [  -z "$(REMOTE)" ]; then echo localhost:5000/v2/$(PROJECT):$(VERSION); else echo $(REMOTE):5000/v2/$(PROJECT):$(VERSION);fi )
DEFAULT_REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)

#REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

include ../../build/makefiles/actor.mk

ACTOR_ID=$(shell make --silent actor_id )
CALL_ALIAS = game_player_health_check
# messaging provider id from wasmcloud.azurecr.io/nats
# If you built it from source, get the provider id with `make inspect` in the source folder
MESSAGING_PROVIDER_ID = VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7
link:
	# link to messaging provider
	wash ctl link put $(shell make actor_id | tail -1) \
		$(MESSAGING_PROVIDER_ID) wasmcloud:messaging \
		'URI=nats://localhost:4222' 'SUBSCRIPTION=demo.echo'
# @wash call -o json $(ACTOR_ID) --cluster-seed SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E Thread.StartThread \
# 		'{"game_id":"qq_db_kv1","timestamp":0,"elapsed":0,"sleep_interval":100}'
start-thread:
	echo actor_id $(ACTOR_ID)
	@wash call -o json $(ACTOR_ID) --cluster-seed SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E Thread.StartThread \
		'{"game_id":"player_health_check","timestamp":2,"elapsed":0,"sleep_interval":30000}'
