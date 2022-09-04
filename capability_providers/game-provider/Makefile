# capability-provider/httpserver-rs/Makefile

# short name that will be used for the executable name
PROJECT  = game-provider
CAPABILITY_ID = wasmcloud:thread
VENDOR   = "wasmCloud"
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
# During development iterations, REVISION should be incremented each build
REVISION = 0
oci_url  = $(shell if [  -z "$REMOTE" ]; then echo localhost:5000/v2/$(PROJECT):$(VERSION); else echo $(REMOTE):5000/v2/$(PROJECT):$(VERSION);fi )
#oci_url = registry.hub.docker/v2/repositories/rustropy/game-provider:0.0.1
include ../../build/makefiles/provider.mk

test:
	cargo test -- --nocapture

bui:
	/usr/local/bin/docker run --userns host -e PKG_CONFIG_ALLOW_CROSS=1 --rm --user 71002896:345931250 -e XARGO_HOME=/xargo -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e USER=alan.poon -e CROSS_RUNNER= -v /Users/alan.poon/.xargo:/xargo:Z -v /Users/alan.poon/.cargo:/cargo:Z -v /cargo/bin -v /Users/alan.poon/Documents/rust/wascc/qq_party/capability_providers/game-provider://Users/alan.poon/Documents/rust/wascc/qq_party/capability_providers/game-provider:Z -v /Users/alan.poon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu:/rust:Z,ro -v /Users/alan.poon/Documents/rust/wascc/qq_party/capability_providers/game-provider/target:/target:Z -w /Users/alan.poon/Documents/rust/wascc/qq_party/capability_providers/game-provider -i -t wasmcloud/cross:aarch64-unknown-linux-gnu sh -c PATH=$PATH:/rust/bin cargo build --release -vv --target aarch64-unknown-linux-gnu
par_insert_linux:
	echo par_insert_linux.........
	wash par insert --arch x86_64-linux  --binary build_linux/x86_64-linux.bin build/game-provider.par.gz
par_insert_linux2:
	XDG_CACHE_HOME=${HOME}/.cache cross build --release -vv --target aarch64-unknown-linux-gnu
	cross build --release -vv --target aarch64-unknown-linux-gnu
#wash par insert --arch aarch64-linux  --binary aarch64-unknown-linux-gnu/release/ build/game-provider.par.gz

push2: $(par_insert_linux)
	echo "oci_url" $(REMOTE)
	wash reg push --insecure $(oci_url) build/game-provider.par.gz
