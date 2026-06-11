use std::{env, fs};
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let home = env::var("HOME").expect("HOME environment variable not set");

    let supported_platforms = [
        ("aplite", "CARGO_FEATURE_APLITE"),
        ("basalt", "CARGO_FEATURE_BASALT"),
        ("chalk", "CARGO_FEATURE_CHALK"),
        ("diorite", "CARGO_FEATURE_DIORITE"),
        ("emery", "CARGO_FEATURE_EMERY"),
        ("flint", "CARGO_FEATURE_FLINT"),
        ("gabbro", "CARGO_FEATURE_GABBRO"),
    ];

    let enabled_platforms: Vec<&str> = supported_platforms
        .iter()
        .filter_map(|(name, env_var)| env::var(env_var).ok().map(|_| *name))
        .collect();

    if enabled_platforms.is_empty() {
        panic!("At least one platform feature (e.g., 'emery', 'chalk') must be enabled.");
    }

    for platform in enabled_platforms {
        let sdk_base = PathBuf::from(&home)
            .join(".pebble-sdk/SDKs/current/sdk-core/pebble")
            .join(platform);

        let include_dir = sdk_base.join("include");
        let pebble_h = include_dir.join("pebble.h");

        if !pebble_h.exists() {
            panic!("Could not find pebble.h for {} at {}", platform, pebble_h.display());
        }

        // Create isolated dummy directories for each platform to avoid thread/build collisions
        let dummy_dir = out_dir.join(format!("dummy_{}", platform));
        let dummy_include_dir = dummy_dir.join("include");
        let dummy_src_dir = dummy_include_dir.join("src");
        fs::create_dir_all(&dummy_src_dir).unwrap();

        fs::write(dummy_include_dir.join("message_keys.auto.h"), "// Stub\n").unwrap();
        fs::write(dummy_src_dir.join("resource_ids.auto.h"), "// Stub\n").unwrap();

        println!("cargo:rerun-if-changed={}", pebble_h.display());

        let bindings = bindgen::Builder::default()
            .header_contents(
                &format!("{}_wrapper.h", platform),
                &format!(
                    "#include <stdint.h>\n\
                     typedef int32_t time_t;\n\
                     #include \"{}\"",
                    pebble_h.display()
                )
            )
            .use_core()
            .ctypes_prefix("cty")
            .clang_arg(format!("-I{}", include_dir.display()))
            .clang_arg(format!("-I{}", dummy_include_dir.display()))
            .clang_arg("--target=arm-none-eabi")
            .clang_arg("-Wno-macro-redefined")
            .clang_arg("-D_TIME_H_")
            .rustified_enum(".*")
            .generate()
            .unwrap_or_else(|_| panic!("Unable to generate bindings for {}", platform));

        // Write to a platform-specific filename!
        let output_file = out_dir.join(format!("bindings_{}.rs", platform));
        bindings.write_to_file(output_file).expect("Couldn't write bindings!");
    }
}