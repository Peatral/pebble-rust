use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

#[derive(Deserialize)]
struct PackageJson {
    pebble: PebbleConfig,
}

#[derive(Deserialize)]
struct PebbleConfig {
    #[serde(rename = "targetPlatforms")]
    target_platforms: Vec<String>,
}

pub fn run_build() {
    println!("[RUST-BUILD] Starting Pebble build process...");

    let package_json = Path::new("package.json");

    if !package_json.exists() {
        eprintln!("[RUST-BUILD] ERROR: package.json not found!");
        exit(1);
    }

    let content = fs::read_to_string(package_json).expect("Failed to read package.json");
    let parsed: PackageJson =
        serde_json::from_str(&content).expect("Invalid package.json structure");
    let platforms = parsed.pebble.target_platforms;

    println!("[RUST-BUILD] Detected target platforms: {:?}", platforms);

    let target = "thumbv7m-none-eabi";
    let rustflags = "-C relocation-model=pie -C codegen-units=1 -C link-arg=--gc-sections -C link-arg=--build-id=sha1 -C link-arg=--emit-relocs -C debuginfo=2";

    for platform in platforms {
        println!("--------------------------------------------------");
        println!(
            "[RUST-BUILD] Compiling Rust binaries for platform: {}",
            platform.to_uppercase()
        );
        println!("--------------------------------------------------");

        let deps_dir = Path::new("target")
            .join(target)
            .join("release")
            .join("deps");

        if deps_dir.exists()
            && let Ok(entries) = fs::read_dir(&deps_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == "o" {
                    fs::remove_file(path).ok();
                }
            }
        }

        let status = Command::new("cargo")
            .arg("build")
            .args([
                "--target",
                target,
                "--release",
                "--no-default-features",
                "--features",
                &platform,
            ])
            .env("PEBBLE_PLATFORM", &platform)
            .env("RUSTFLAGS", rustflags)
            .status()
            .expect("Failed to execute cargo build");

        if !status.success() {
            eprintln!("[RUST-BUILD] cargo build failed for platform: {}", platform);
            exit(1);
        }

        let platform_obj_dir = format!("build/rust_out/{}", platform);
        fs::create_dir_all(&platform_obj_dir).expect("Failed to create platform out dir");

        let out_obj_abs = std::env::current_dir()
            .unwrap()
            .join(&platform_obj_dir)
            .join("rust_app.o");

        if let Ok(entries) = fs::read_dir(&deps_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().unwrap_or_default() == "a" {
                    let ar_status = Command::new("arm-none-eabi-ar")
                        .current_dir(&deps_dir)
                        .arg("x")
                        .arg(path.file_name().unwrap())
                        .status()
                        .expect("Failed to execute ar extraction");

                    if !ar_status.success() {
                        eprintln!("[RUST-BUILD] Archive extraction failed for: {:?}", path);
                        exit(1);
                    }
                }
            }
        }

        let mut rcgu_objects = Vec::new();
        if let Ok(entries) = fs::read_dir(&deps_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_string_lossy();
                    if file_name.ends_with(".rcgu.o") {
                        rcgu_objects.push(file_name.into_owned());
                    } else {
                        fs::remove_file(path).ok();
                    }
                }
            }
        }

        if rcgu_objects.is_empty() {
            eprintln!(
                "[RUST-BUILD] ERROR: No .rcgu.o files found in deps for {}!",
                platform
            );
            exit(1);
        }

        let ld_status = Command::new("arm-none-eabi-ld")
            .current_dir(&deps_dir)
            .arg("-r")
            .args(&rcgu_objects)
            .arg("-o")
            .arg(&out_obj_abs)
            .status()
            .expect("Failed to execute linker");

        if !ld_status.success() {
            eprintln!("[RUST-BUILD] Linking failed for platform: {}", platform);
            exit(1);
        }
    }

    println!(
        "[RUST-BUILD] All Rust platforms compiled successfully! Handing over to Pebble SDK..."
    );

    let pebble_status = Command::new("pebble")
        .arg("build")
        .status()
        .expect("Failed to execute final pebble build");

    if !pebble_status.success() {
        exit(1);
    }
}
