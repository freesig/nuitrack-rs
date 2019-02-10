extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

const NUI_SDK_DIR: &'static str = "NUI_SDK_DIR";

fn main() {
    if cfg!(not(target_os="linux")) { panic!("Only linux os is supported"); }
    let mut nui_sdk_dir = match env::var(NUI_SDK_DIR) {
        Ok(dir) => dir,
        Err(env::VarError::NotPresent) => panic!("Please set the NUI_SDK_DIR environment variable 
        to the path of nuitrack SDK root.\n
        ie. /home/user/nuitrack"),
        Err(e) => panic!("Failed to read NUI_SDK_DIR environment variable: {}", e),
    };

    nui_sdk_dir = format!("{}/{}", nui_sdk_dir, "Nuitrack");


    let include_path = format!("{}/{}", nui_sdk_dir, "include");
    let middleware_path = format!("{}/{}", include_path, "middleware");

    #[cfg(target_arch = "x86")]
    let library_path = format!("{}/lib/{}", nui_sdk_dir, "linux_arm"); 
    #[cfg(target_arch = "x86_64")]
    let library_path = format!("{}/lib/{}", nui_sdk_dir, "linux64"); 

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
    let mut lib_path = out_dir.clone();
    lib_path.push("libnui.a");
    if !lib_path.exists() {
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")
            .warnings(false)
            .include(include_path.clone())
            .include(middleware_path.clone())
            .include("nui-helpers")
            .file("nui-helpers/helper.cpp")
            .try_compile("libnui.a")
            .expect("Failed to build helper library");
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=nui");
    println!("cargo:rustc-link-search=nuitrack");
    println!("cargo:rustc-link-search={}", library_path);
    println!("cargo:rustc-link-lib=dylib=nuitrack");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let mut binding_path = out_dir.clone();
    binding_path.push("nui_bindings.rs");
    if !binding_path.exists() {
        let bindings = bindgen::Builder::default()
            .header("nui-helpers/simple.hpp")
            .header("nui-helpers/helper.hpp")
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++14")
            .clang_arg(format!("-I{}", include_path))
            .clang_arg(format!("-I{}", middleware_path))
            .enable_cxx_namespaces()
            .whitelist_type("RustResult")
            .whitelist_function("nui_init")
            .whitelist_function("nui_set_rotation")
            .whitelist_function("nui_run")
            .whitelist_function("nui_update")
            .whitelist_function("nui_release")
            .whitelist_function("register_skeleton_closure")
            .whitelist_function("register_depth_closure")
            .whitelist_function("register_color_closure")
            .whitelist_function("register_user_closure")
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").expect("bad path"));
        bindings
            .write_to_file(out_path.join("nui_bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
