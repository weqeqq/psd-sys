use std::env;
use std::path::Path;
use std::process::Command;

use cmake::Config;

fn main() {
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
