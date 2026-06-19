# Pebble Rust Workspace

A modern Rust workspace for building Pebble smartwatch applications. This repository replaces the older monolithic `pebble-rust` crate with a modular set of tools and libraries to compile, bind, and package Pebble apps using modern Rust.

## Workspace Architecture

This workspace contains four core crates:

1. **[pebble-sys](./pebble-sys)**: Raw, low-level FFI bindings generated automatically using `bindgen` from the Pebble SDK. It exposes the raw C functions, types, and constants.
2. **[pebble-rs](./pebble-rs)**: A safe, idiomatic, high-level wrapper around the raw FFI bindings. It integrates Rust features (like allocator integration, safe types, delegates, and macros) to simplify Pebble development.
3. **[pebble-build](./pebble-build)**: A cargo build script helper library. It parses `package.json` at compile-time to generate safe Rust bindings for message keys and resource IDs in the output directory.
4. **[pebble-cli](./pebble-cli)**: The build orchestrator providing the `cargo pebble` command. It automates compiling Rust binaries for Pebble's various hardware platforms, extracting intermediate objects, and linking them into the final Pebble application bundle.

---

## Prerequisites

To compile Rust applications for Pebble, you need:

1. **Pebble SDK**: Ensure the Pebble SDK is installed and active on your system. The SDK is typically located under the modern path `~/.local/share/pebble-sdk/` (or legacy `~/.pebble-sdk/`) with the current active SDK headers located at:
   `~/.local/share/pebble-sdk/SDKs/current/sdk-core/pebble/`
2. **GNU Arm Toolchain**: Ensure `arm-none-eabi` utilities (like `arm-none-eabi-ar` and `arm-none-eabi-ld`) are installed and available in your shell's `PATH`. These utilities are usually included with the Pebble SDK installation.
3. **Rust target**: Install the ARM Cortex-M3 target via rustup:
   ```bash
   rustup target add thumbv7m-none-eabi
   ```
4. **Clang / LLVM**: Required for `bindgen` to parse Pebble SDK headers.

---

## Installation

Install the Pebble Rust build orchestrator (`cargo-pebble` CLI) from the workspace root:

```bash
cargo install --path pebble-cli
```

This installs the binary to your Cargo binary directory, allowing you to use `cargo pebble` anywhere.

---

## Getting Started

Check out the [examples](./examples) folder to see how to structure your projects. A standard Pebble Rust project has:
- A `Cargo.toml` with `crate-type = ["staticlib"]` depending on `pebble-rs` and `pebble-sys`.
- A `build.rs` invoking `pebble-build` to generate asset and communication key bindings.
- A Pebble configuration file `package.json` specifying targeted platforms, resources, and message keys.
- A Pebble build script `wscript` configured to link the generated Rust object files.

### Compiling an Example

Navigate into one of the examples (e.g., `examples/hello-world`):

```bash
cd examples/hello-world
cargo pebble build
```

The CLI compiles the project for all targets specified in `package.json` and runs the final `pebble build` task. You can then run/install the application on an emulator or watch:

```bash
pebble install --emulator basalt
```

---

## Licensing

Only the high-level wrapper crate [`pebble-rs`](./pebble-rs) is double-licensed under:
- **GPLv3** ([`LICENSE-GPLv3`](./LICENSE-GPLv3))
- **BSD 3-Clause** ([`LICENSE-BSD-3.0`](./LICENSE-BSD-3.0))

Derivatives of `pebble-rs` should comply with both. Other packages in this workspace may have different licensing terms.

---

## Credits & Acknowledgements

* **[RoccoDev](https://github.com/roccodev)**: The original maintainer and creator of the `pebble-rust` project, who designed the initial bindings and wrappers.
* **[Eva van Houten](https://github.com/evavh)**: For extensively cleaning up and improving the build scripts to make the Rust application compile and run seamlessly on Pebble emulators/devices.
* **[andars](https://github.com/andars)**: For their work on the original [pebble.rs](https://github.com/andars/pebble.rs) project, which served as a basis for some files in this library.

