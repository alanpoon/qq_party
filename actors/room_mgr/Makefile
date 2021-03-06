# Copyright 2015-2019 Capital One Services, LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all bench build check clean doc test update

all: build

bench:
	@$(CARGO) bench

build:
	@$(CARGO) build
	wash claims sign target/wasm32-unknown-unknown/debug/room_mgr.wasm -o target/wasm32-unknown-unknown/debug/room_mgr_signed.wasm -g -l --name "room_mgr"

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

test: build
	@$(CARGO) test

update:
	@$(CARGO) update

release:
	@$(CARGO) build --release
	wash claims sign target/wasm32-unknown-unknown/release/room_mgr.wasm -o target/wasm32-unknown-unknown/release/room_mgr_signed.wasm -c wasm:messaging --name "room_mgr"
	
