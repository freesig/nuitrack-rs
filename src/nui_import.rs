#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/nui_bindings.rs"));

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RHandTrackerWrapper {
    pub result: root::RustResult,
    pub r_hand_tracker: root::RHandTracker,
}

extern "C" {
    pub fn create_hand_tracker() -> RHandTrackerWrapper;
}

extern "C" {
    pub fn hand_tracker_callback(hand_tracker: [u64; 2], hand_callback: extern "C" fn(root::RHandTrackerDataPtr) -> ()) -> root::RustResult;
}

extern "C" {
    pub fn to_raw(ptr: root::RHandTrackerDataPtr) -> root::RustResult;
}
