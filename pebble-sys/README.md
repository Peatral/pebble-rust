# pebble-sys

Raw FFI bindings to the Pebble SDK generated automatically using `bindgen`.

This crate provides low-level C bindings for the Pebble OS, allowing Rust to interface with the Pebble SDK functions, structs, and variables. It is intended to be used via the safe wrapper [`pebble-rs`](../pebble-rs), but can also be used directly for low-level or custom API integration.

## How It Works

During compilation, the [`build.rs`](build.rs) script:
1. Locates the Pebble SDK configuration from the modern path `~/.local/share/pebble-sdk/SDKs/current/sdk-core/pebble/` (falling back to legacy `~/.pebble-sdk/`).
2. Inspects which platform features are enabled (e.g., `aplite`, `basalt`, `chalk`, `diorite`, `emery`, `flint`, `gabbro`).
3. Generates platform-specific bindings (`bindings_<platform>.rs`) in the build output (`OUT_DIR`).
4. Re-exposes the active bindings module dynamically in [`lib.rs`](src/lib.rs) using `cfg_if!`.

## Platform Features

You must enable at least one feature corresponding to the Pebble hardware platform you are compiling for:
- `aplite` (Pebble Classic / Pebble Steel)
- `basalt` (Pebble Time / Pebble Time Steel)
- `chalk` (Pebble Time Round)
- `diorite` (Pebble 2 SE / Pebble 2 HR)

These watches were released under Core Devices:
- `emery` (Pebble Time 2)
- `flint` (Pebble 2 Duo)
- `gabbro` (Pebble Round 2)

Example in `Cargo.toml`:
```toml
[dependencies]
pebble-sys = { version = "0.1.0", features = ["basalt"] }
```

## Requirements

- Pebble SDK installed and configured at `~/.local/share/pebble-sdk/` (or legacy `~/.pebble-sdk/`).
- Clang / LLVM installed on the host system to run `bindgen`.
