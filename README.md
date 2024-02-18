# timelock-poc-for-cw20
Time-lock poc for disbursement of cw20 allocations (ie, for team/devs) that unlock over time

**This Repo is Proof-Of-Concept ONLY and has not been reviewed, audited, or otherwise tested. It was drafted to provide a skeleton example of a timelock contract example for SEI utilizing cosmwasm based contracts in Rust.**

While it is possible to update this contract and deploy it for use of the timelock feature, **this code is provided AS-IS** and is **NOT** production ready.

**USE AT YOUR OWN RISK.**


## Build

To build:

```cargo build```

```cargo wasm```

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```