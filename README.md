# Floresta-ffi

This repository contains the code to generate FFI bindings for [Floresta](https://github.com/getfloresta/Floresta), a Bitcoin Utreexo full node. The bindings are generated using [UniFFI](https://github.com/mozilla/uniffi-rs).

## Supported targets

| Language   | Platform          | Published Package                                   | Build Instructions              |
|------------|-------------------|-----------------------------------------------------|---------------------------------|
| Kotlin     | Android           | `org.getfloresta:floresta-android`                   | [floresta-android](floresta-android/README.md) |
| Python     | Linux, macOS      | (source only)                                        | See below                       |

## Repository structure

```
floresta-ffi/
├── floresta-ffi/        # Rust crate with UniFFI bindings
│   ├── src/             # Rust source + UDL definition
│   └── Cargo.toml       # Rust dependencies
├── floresta-android/    # Android library project (Gradle)
│   ├── lib/             # Android library module
│   └── scripts/         # Cross-compilation build scripts
└── README.md
```

## How to generate the bindings

If you want to generate the bindings and use it, you can use the [just command runner](https://github.com/casey/just) to generate the code. The `justfile` in the `floresta-ffi/` directory contains the commands to generate the bindings for the supported languages.

```bash
cd floresta-ffi
just gen-<your-language>
```

Where `<your-language>` is the language you want to generate the bindings for. For example, to generate the bindings for Python:

```bash
cd floresta-ffi
just gen-python
```

This will build the shared library and generate glue code to use the shared library. The generated code will be in the `floresta-ffi/generated/<language>` folder.

## Android

See [floresta-android/README.md](floresta-android/README.md) for Android build and usage instructions.

## Python example

To use it in Python, you need the generated bindings and the shared-object in the same folder. Then you can start `florestad` with:

```python
from floresta import Florestad

daemon = Florestad()
daemon.start()

# do something

# at the end you need to stop the daemon
daemon.stop()
```

After you start it, you'll have a JSON-RPC server and an Electrum server running on the default ports. You may use them to communicate with the daemon, see your balance, send transactions, etc.

## Customizing the daemon

There's a `Config` object that may be passed to the `Florestad` constructor. It has the following fields:

- `datadir` (str) - Required: path to the data directory where chain and wallet data will be stored
- `network` (Network) - Required: the Bitcoin network to run on (Bitcoin, Signet, Testnet, Regtest, Testnet4)
- `assume_valid` (AssumeValidArg) - Required: which blocks are assumed to have valid scripts
- `cfilters` (bool) - Whether to build and store compact block filters
- `filters_start_height` (int) - Block height to start downloading compact filters from
- `log_to_stdout` (bool) - Whether to write logs to stdout
- `log_to_file` (bool) - Whether to write logs to a file
- `wallet_xpub` (list[str]) - SLIP-132-encoded extended public keys to watch
- `wallet_descriptor` (list[str]) - Output descriptors to watch
- `config_file` (str) - Path to a TOML configuration file
- `proxy` (str) - SOCKS5 proxy for outgoing connections
- `connect` (list[str]) - Nodes to connect to exclusively
- `json_rpc_address` (str) - Address for the JSON-RPC server to listen on
- `zmq_address` (str) - Address for the ZMQ server (requires zmq-server feature)
- `electrum_address` (str) - Address for the Electrum server to listen on
- `enable_electrum_tls` (bool) - Whether to enable the Electrum TLS server
- `electrum_address_tls` (str) - Address for the Electrum TLS server
- `tls_key_path` (str) - Path to TLS private key file
- `tls_cert_path` (str) - Path to TLS certificate file
- `generate_cert` (bool) - Whether to generate a self-signed TLS certificate
- `allow_v1_fallback` (bool) - Whether to allow v1 transport fallback
- `assume_utreexo` (bool) - Enable assume-utreexo mode
- `assumeutreexo_value` (AssumeUtreexoValue) - Custom Utreexo accumulator state
- `backfill` (bool) - Whether to backfill skipped blocks
- `debug` (bool) - Enable debug logging
- `disable_dns_seeds` (bool) - Disable DNS seed nodes
- `user_agent` (str) - User agent string advertised to peers

To set them, create a `Config` object and pass it to the `Florestad` constructor:

```python
from floresta import Florestad, Config, Network, AssumeValidArg

config = Config(
    datadir="/path/to/data",
    network=Network.Bitcoin,
    assume_valid=AssumeValidArg.Hardcoded(),
    cfilters=True,
    log_to_stdout=True,
)
daemon = Florestad.from_config(config)
daemon.start()
```

## License

MIT