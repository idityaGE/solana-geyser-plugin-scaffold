## solana-geyser-plugin-scaffold

A minimal Solana Geyser plugin scaffold written in Rust. This repository provides a drop-in example plugin you can build and load into `solana-test-validator` (or a full validator) to learn how Geyser plugins receive and process ledger events.

## What is a Geyser plugin?

The Solana Geyser plugin interface lets external libraries receive real-time notifications from a running Solana validator about block, transaction, and account updates. Plugins are dynamic libraries (.so) loaded by the validator and called via a C ABI to process or forward ledger events.

## This repo

- Source: Rust plugin scaffold in `src/`.
- Example plugin config: `geyser-plugin.json` (points to the built library and contains plugin-specific configuration).

By default the included `geyser-plugin.json` points at (example):

```
{
	"libpath": "/home/idityage/github_repos/solana-geyser-plugin-scaffold/target/release/libsolana_geyser_plugin_scaffold.so",
	"config": {
		"output_file": "/tmp/geyser_plugin.txt"
	}
}
```

Important: replace the `libpath` value with the absolute path to the library file produced by your build. Example library filenames by OS are described below.

## Quickstart — build and run locally

Prerequisites:

- Rust toolchain (stable) and `cargo`.
- Solana `solana-test-validator` available on PATH (from Solana toolchain).

1. Build the plugin (release):

```bash
cargo build --release
```

2. Confirm the produced library exists (default path in config):

```bash
ls -l target/release/libsolana_geyser_plugin_scaffold.so
```

3. Start the test validator with the plugin config from the repo root:

```bash
solana-test-validator --geyser-plugin-config geyser-plugin.json
```

The validator will load the plugin `.so` and the plugin will use the `config` section from `geyser-plugin.json`

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

## Troubleshooting

- Plugin not loaded / validator errors:
  - Check `libpath` in `geyser-plugin.json` is absolute and the `.so` exists.
  - Ensure the `.so` is readable by the user running the validator.
- Nothing appears in the plugin output file:
  - Confirm the validator started without errors and processed some slots/transactions.
- Building fails:
  - Ensure you have a current Rust toolchain and linker setup for building shared libraries on Linux.

## Library file names by OS

- Linux: the build will typically produce a `.so` shared library (for example `target/release/libsolana_geyser_plugin_scaffold.so`). Use that file path in `libpath`.
- macOS: Rust will produce a `.dylib` dynamic library. Update `libpath` to point to the generated `.dylib` file.
- Windows: validators expect Unix-style shared libraries; building and testing a Geyser plugin on native Windows is uncommon. Use WSL (Windows Subsystem for Linux) to build a `.so` and run `solana-test-validator` from WSL if you need local testing on Windows.

## Development notes

- The Rust code for the plugin lives in `src/`. The entrypoints follow the Solana Geyser plugin ABI and the scaffold writes simple, human-readable messages to the configured `output_file` for received callbacks.
- Small changes to the ABI or function signatures must match the validator's expectations. See Solana docs for Geyser plugin ABI and callback semantics.

If you'd like, I can also:

- Add a small script `scripts/run-local.sh` that builds and launches `solana-test-validator` with the config.
- Add a minimal `CONTRIBUTING.md` or expand the README with examples of parsing the plugin output.
