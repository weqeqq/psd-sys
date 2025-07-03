use std::env;
use std::path::Path;
use std::process::Command;

use cmake::Config;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let is_prebuilt = match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => {
            println!("cargo:rustc-link-search=native=bin/x86_64-windows");
            println!("cargo:rustc-link-lib=dylib=cpmt");
            println!("cargo:rustc-link-lib=dylib=msvcprt");
            println!("cargo:rustc-link-lib=dylib=msvcrt");
            true
        }
        ("linux", "x86_64") => {
            println!("cargo:rustc-link-search=native=bin/x86_64-linux");
            println!("cargo:rustc-link-lib=dylib=stdc++");
            true
        }
        _ => false,
    };
    if is_prebuilt {
        println!("cargo:rustc-link-lib=static=psd");
        println!("cargo:rustc-link-lib=static=image");
        println!("cargo:rustc-link-lib=static=turbojpeg");
        println!("cargo:rustc-link-lib=static=jpeg");
        println!("cargo:rustc-link-lib=static=wuffs");
        return;
    }
    println!("cargo:warning=building");
    let output_dir = env::var("OUT_DIR").unwrap();
    let source_dir = Path::new(&output_dir).join("psd-cpp");

    if !source_dir.exists() {
        let status = Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                "https://github.com/weqeqq/psd-cpp",
                source_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run git clone");
        assert!(status.success(), "git clone failed");
    }
    let destination = Config::new(&source_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        destination.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=psd");
    println!("cargo:rustc-link-lib=static=image");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rerun-if-changed=build.rs");
}
