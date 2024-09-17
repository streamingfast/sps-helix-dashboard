# Injective Pyth Oracle Prices Substreams modules

This module is a simple `injective` substreams using [injective-common](https://substreams.dev/streamingfast/injective-common/v0.2.2) modules to filter specific pyth oracle events.  

## Usage

```bash
substreams build
substreams auth
substreams gui
```

## Modules

### `injective:filtered_events`
Filters `wasm-incentives-claim_reward` events.
Default param: `injective:filtered_events: "type:wasm-incentives-claim_reward"`

### `map_claim_rewards`

