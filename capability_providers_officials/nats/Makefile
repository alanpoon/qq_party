# capability-provider/nats/Makefile

# short name that will be used for the executable name
PROJECT  = nats-messaging
CAPABILITY_ID = wasmcloud:messaging
VENDOR   = "wasmCloud"
#VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
VERSION = "0.11.5"
# During development iterations, REVISION should be incremented each build
REVISION = 0
oci_url  = $(shell if [  -z "$REMOTE" ]; then echo localhost:5000/v2/$(PROJECT):$(VERSION); else echo $(REMOTE):5000/v2/$(PROJECT):$(VERSION);fi )
#oci_url  = localhost:5000/v2/$(PROJECT):$(VERSION)
#oci_url = registry.hub.docker/v2/repositories/rustropy/game-provider:0.0.1
#include ../../build/makefiles/provider.mk

test:
	cargo test -- --nocapture

push:
	echo "oci_url" $(REMOTE)
	wash reg push --insecure $(oci_url) built/nats_messaging.par.gz
