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
	cargo build --target wasm32-unknown-unknown --release
	wash claims sign target/wasm32-unknown-unknown/release/game_logic.wasm -d target/wasm32-unknown-unknown/release/game_logic_s.wasm -c wasmcloud:game --name "game_logic" -a "game_logic"

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
	@$(CARGO) build --target wasm32-unknown-unknown --release
	wash claims sign target/wasm32-unknown-unknown/release/game_logic.wasm -d target/wasm32-unknown-unknown/release/game_logic_s.wasm -c wasmcloud:game -l --name "game_logic" -a "game_logic"