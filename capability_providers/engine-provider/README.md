# Assembly Mechs: Beyond WasmDome - Game Engine Capability Provider

This capability provider is responsible for managing the core game execution, and can be run offline with a _local lattice_ or can be run anywhere with lattices comprised of _leaf nodes_.

## Running

To run the capability provider, you can simply fire it up inside a lattice-enabled waSCC host binary and point said binary at the following manifest:

```yaml
---
actors: []
capabilities:
    - path: /path/to/wasmdome/engine-provider/target/release/libengine_provider.so
bindings: []
```

With this yaml file you can run wascc-host:

```terminal
wascc-host --manifest ./wasmdome-provider.yaml
```

## Relevant Subjects

With the introduction of this capability provider, there are a couple of changes to the subjects previously used by wasmdome.

* `wasmdome.arena.control` - This is the subject on which this provider listens for control messages in order to start a new match.
* `wasmdome.arena.events` - Arena-wide events. Monitor this subject to be told when actors connect to the arena lobby, actors disconnect, and matches begin and end.
* `wasmdome.match.{}.events` - The event stream for any given match.


## Starting a Match (Testing)

To start a match manually without using the `wasmdome` CLI, you can publish a message on `wasmdome.arena.control` with the `nats-pub` sample tool, as shown below (modify the parameters and actor list for your scenario):

```
❯ ~/go/src/github.com/nats-io/nats.go/examples/nats-pub/nats-pub -s 127.0.0.1 wasmdome.arena.control '{"StartMatch":{"match_id":"abc123", "actors":["MCPBZXJDXCWRJAOPHXCBGVU55BAKCQSNUXUQRKRLI6RWYRFJW7W64JH4"], "board_height": 8, "board_width": 8, "max_turns": 10, "aps_per_turn": 4}}'
```
