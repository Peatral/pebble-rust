# pebble-build

A library utility designed for use inside a Pebble application's `build.rs` script.

`pebble-build` automates the generation of Rust constants for message keys and resource IDs defined in the Pebble application's `package.json` file. This allows Rust applications to refer to resources (like fonts, images, and keys) using compile-time checked type constants.

## How to Use

1. Add `pebble-build` to your `[build-dependencies]` in `Cargo.toml`:
   ```toml
   [build-dependencies]
   pebble-build = { path = "../../pebble-build" } # Adjust path accordingly
   ```

2. Create a file named [`build.rs`](../examples/hello-world/build.rs) in the root of your application directory (alongside `Cargo.toml` and `package.json`):
   ```rust
   fn main() {
       pebble_build::build();
   }
   ```

3. Use the [`pebble-rs`](../pebble-rs) macros inside your application code to import the constants:
   ```rust
   // Auto-imports RESOURCE_ID_* constants
   include_resource_ids!();

   // Auto-imports MESSAGE_KEY_* constants
   include_message_keys!();
   ```

## How It Works

When `pebble_build::build()` is run during cargo compilation:
1. It reads the application's local `package.json` file.
2. It identifies the target Pebble platform (using the `PEBBLE_PLATFORM` environment variable or the active `CARGO_FEATURE_<PLATFORM>` feature flags).
3. It filters media resources that target the active platform.
4. It parses:
   - `pebble.messageKeys` list -> generates a module mapping names to their index identifiers.
   - `pebble.resources.media` list -> generates a module mapping resource names to resource IDs.
5. It writes `message_keys.rs` and `resource_ids.rs` inside the compilation output directory (`OUT_DIR`).
6. The `pebble-rs` macros `include_message_keys!` and `include_resource_ids!` include these generated files into your application's crate structure.

## License

This package is licensed under the MIT License. See [LICENSE-MIT](../LICENSE-MIT) at the workspace root for details.
