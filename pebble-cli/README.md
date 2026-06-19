# pebble-cli

A CLI helper tool and build orchestrator for compiling Pebble Rust applications. It compiles the project for all targeted watch platforms, extracts/merges Rust dependencies, and packages the app using the Pebble SDK.

The main command provided is `cargo pebble build`.

## How It Works

When you run `cargo pebble build` inside a Pebble Rust project:
1. **Reads Configuration**: Parses the `pebble.targetPlatforms` list from the project's `package.json` (e.g., `["aplite", "basalt", "diorite", "chalk", "emery", "gabbro"]`).
2. **Compiles Rust Code**: For each target platform:
   - Cleans the build dependencies folder of old object binaries.
   - Runs `cargo build --target thumbv7m-none-eabi --release` for the platform feature (e.g., `--features basalt`).
   - Inject environment variable `PEBBLE_PLATFORM` and custom relocatable flags (`RUSTFLAGS`) so the compiler generates relocation-model-compatible position independent executable (PIE) binaries.
3. **Extracts Static Libraries**: Unpacks the static library archives (`.a`) compiled by Cargo using `arm-none-eabi-ar x`.
4. **Merges Objects**: Merges all compiled compilation units (`.rcgu.o`) into a single output object file `build/rust_out/<platform>/rust_app.o` using `arm-none-eabi-ld -r`.
5. **Packages App**: Invokes `pebble build`, handing over compilation to the Pebble SDK build script (`wscript`), which links the Rust object code and packages it into a Pebble bundle (`.pbw`).

## Installation

To install `cargo-pebble` command globally, run the following from the workspace root:

```bash
cargo install --path pebble-cli
```

Ensure that the Pebble SDK CLI (`pebble`) is installed and available in your `PATH`.

## Command Usage

```bash
# Compile and build the Pebble project for all target platforms
cargo pebble build
```

## License

This package is licensed under the MIT License. See [LICENSE-MIT](../LICENSE-MIT) at the workspace root for details.
