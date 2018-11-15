extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

const NUI_INCLUDE_PATH: &'static str = "/home/tom/devbox/nuitrack/Nuitrack/include";
const NUI_MIDDLE_INCLUDE_PATH: &'static str =
    "/home/tom/devbox/nuitrack/Nuitrack/include/middleware";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
    let mut lib_path = out_dir.clone();
    lib_path.push("libnui.a");
    if !lib_path.exists() {
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")
            .warnings(false)
            .include(NUI_INCLUDE_PATH)
            .include(NUI_MIDDLE_INCLUDE_PATH)
            .include("nui-helpers")
            .file("nui-helpers/helper.cpp")
            .try_compile("libnui.a")
            .expect("Failed to build helper library");
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=nui");
    println!("cargo:rustc-link-search=nuitrack");
    println!("cargo:rustc-link-search=/home/tom/devbox/nuitrack/Nuitrack/lib/linux64");
    println!("cargo:rustc-link-lib=dylib=nuitrack");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let mut binding_path = out_dir.clone();
    binding_path.push("nui_bindings.rs");
    if !binding_path.exists() {
        let bindings = bindgen::Builder::default()
            .header("nui-helpers/helper.hpp")
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++14")
            .clang_arg(format!("-I{}", NUI_INCLUDE_PATH))
            .clang_arg(format!("-I{}", NUI_MIDDLE_INCLUDE_PATH))
            .enable_cxx_namespaces()
            .whitelist_type("RustResult")
            .opaque_type("RHandTracker")
            .whitelist_type("RHandTracker")
            .whitelist_function("nui_init")
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
        bindings
            .write_to_file(out_path.join("nui_bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
