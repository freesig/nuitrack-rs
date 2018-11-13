extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

const NUI_HEADER_PATH: &'static str = "home/tom/devbox/nuitrack/Nuitrack/include/nuitrack/Nuitrack.h";

fn main() {
    cc::Build::new()
        .include(NUI_HEADER_PATH)
        .include("nui-helpers/helper.hpp")
        .file("nui-helpers/helper.cpp")
        .cpp(true)
        .compile("libnui.a");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=nuitrack");
    println!("cargo:rustc-link-lib=static=nui");
    let bindings = bindgen::Builder::default()
        .header(NUI_HEADER_PATH)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .whitelist_type("RustResult")
        .whitelist_function("nui_init")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
    bindings
        .write_to_file(out_path.join("nui_bindings.rs"))
        .expect("Couldn't write bindings!");
}
