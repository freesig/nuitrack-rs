#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/nui_bindings.rs"));

#[cfg(target_os = "macos")]
include!(concat!("../nui-helpers", "/nui_bindings_mac.rs"));

