This tutorial focuses on running a Relay Chain and joining a local parachain to it.

It requires to run one validator with Alice

Then join another validator to the concensus with Bob

first clone latest polkadot-sdk 

then run

```bash
cargo build --release
```

Then you have to download the local chain spec that is located in the docs.

see:

https://docs.substrate.io/tutorials/build-a-parachain/prepare-a-local-relay-chain/

```bash
curl -o ./target/tmp/raw-local-chainspec.json https://docs.substrate.io/assets/tutorials/relay-chain-specs/raw-local-chainspec.json
```

> I needed to do --insecure-validator-i-know-what-i-do because of M3? or the version has a problem, still to be determined.


```bash
./target/release/polkadot \
--alice \
--validator \
--base-path ./target/tmp/relay/alice \
--chain ./target/tmp/raw-local-chainspec.json \
--port 30334 \
--rpc-port 9945 \
--insecure-validator-i-know-what-i-do
```

how to run bob.

```bash
./target/release/polkadot \
--bob \
--validator \
--base-path ./target/tmp/relay/bob \
--chain ./target/tmp/raw-local-chainspec.json \
--port 30334 \
--rpc-port 9945 \
--insecure-validator-i-know-what-i-do
```