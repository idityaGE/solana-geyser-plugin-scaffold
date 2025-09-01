## solana-geyser-plugin-scaffold

A minimal Solana Geyser plugin scaffold written in Rust. This repository provides a drop-in example plugin you can build and load into `solana-test-validator` (or a full validator) to learn how Geyser plugins receive and process ledger events.

## What is a Geyser plugin?

The Solana Geyser plugin interface lets external libraries receive real-time notifications from a running Solana validator about block, transaction, and account updates. Plugins are dynamic libraries (.so) loaded by the validator and called via a C ABI to process or forward ledger events.

## This repo

It has two implementations of the Geyser plugin:

- `master` branch: a minimal example plugin.
- `logger_impl` branch: with an external logger file.

## Quickstart — build and run locally

Prerequisites:

- Rust toolchain (stable) and `cargo`.
- Solana `solana-test-validator` available on PATH (from Solana toolchain/CLI).
- linux or macOS or WSL (Windows Subsystem for Linux)

1. Build the plugin (release):

```bash
cargo build --release
```

2. Confirm the produced library exists (default path in config):

```bash
ls -l target/release/libsolana_geyser_plugin_scaffold.so
```

3. Start the test validator with the plugin config from the repo root:

> Note: Make sure you have correct path to the built library in `libpath`.

```bash
solana-test-validator --geyser-plugin-config geyser-plugin.json
```

If you get errors that the plugin cannot be found, open `geyser-plugin.json` and set `libpath` to the absolute path of the library file you built.

4. Produce some activity (in a second terminal) so the plugin receives events. For example:

```bash
# use the local validator's RPC
solana airdrop 1
solana transfer --allow-unfunded-recipient <RECIPIENT_PUBKEY> 0.001
```

5. Inspect plugin output:

You can also check the validator logs in `test-ledger/validator.log` for plugin load messages.

To follow validator logs in real time:

```bash
tail -f test-ledger/validator.log
```

## References

- [Anza Docs on Geyser Plugin](https://docs.anza.xyz/validator/geyser)

### Examples Plugin Implementations

- [A PostgreSQL Plugin](https://github.com/solana-labs/solana-accountsdb-plugin-postgres)
- [A Plugin Sending to a gRPC Service](https://github.com/ckamm/solana-accountsdb-connector)
- [A RabbitMQ Producer Plugin](https://github.com/holaplex/indexer-geyser-plugin)
- [A Complete Architecture Around The Geyser Plugin](https://github.com/holaplex/indexer)
- [A Kafka Producer Plugin](https://github.com/Blockdaemon/solana-accountsdb-plugin-kafka)
- [An Amazon SQS Plugin](https://github.com/rpcpool/solana-accountsdb-sqs)
- [A Google BigTable Plugin](https://github.com/lijunwangs/solana-accountsdb-plugin-bigtable)
- [A Creative Way To Use The Geyser Plugin](https://github.com/clockwork-xyz/clockwork)
