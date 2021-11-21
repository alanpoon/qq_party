# capability-provider/httpserver-rs/Makefile

# short name that will be used for the executable name
PROJECT  = game-provider
CAPABILITY_ID = wasmcloud:thread
VENDOR   = "wasmCloud"
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
# During development iterations, REVISION should be incremented each build
REVISION = 0
#oci_url  = localhost:5000/v2/$(PROJECT):$(VERSION)
oci_url = registry.hub.docker/v2/repositories/rustropy/game-provider:0.0.1
include ../../build/makefiles/provider.mk

test:
	cargo test -- --nocapture
