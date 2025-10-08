use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let output_dir = env::var("OUT_DIR").unwrap();
    let source_dir = Path::new(&output_dir).join("psd-cpp");

    if !source_dir.exists() {
        let status = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/weqeqq/psd-cpp",
                source_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run git clone");
        assert!(status.success(), "git clone failed");
    }
    env::set_current_dir(source_dir).unwrap();
    assert!(
        Command::new("cmake")
            .args(&[
                "--preset",
                "release",
                format!("-DCMAKE_INSTALL_PREFIX={}", output_dir).as_str()
            ])
            .status()
            .unwrap()
            .success(),
        "huy"
    );
    assert!(
        Command::new("cmake")
            .args(&["--build", "--preset", "release", "--target", "install"])
            .status()
            .unwrap()
            .success(),
        "huy"
    );

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(output_dir).join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=psd");
    println!("cargo:rustc-link-lib=static=file");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "windows" {
        if target_os == "macos" {
            println!("cargo:rustc-link-lib=dylib=c++");
        }else {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
