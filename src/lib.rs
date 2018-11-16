mod nui_import;
mod error_conversion;

use self::nui_import::root as nui;
use self::error_conversion::{NuiResult, CData};
use std::io;
use nui::{RHandData, RHandTrackerDataPtr, RHandTracker};
use self::nui_import::RHandTrackerWrapper;

pub fn initialize() -> Result<(), io::Error> {
    unsafe { nui::nui_init().to_result().map(|_|()) }
}

pub fn create_hand_tracker() -> Result<RHandTracker, io::Error> {
    unsafe {
        nui_import::create_hand_tracker()
            .to_result()
            .map(|d| 
                 match d {
                     Some(CData::HandTracker(h)) => h,
                     _ => unreachable!(),
                 })
    }
}

pub fn add_hand_tracker_cb(ht: RHandTrackerWrapper, user_callback: extern"C" fn(data: RHandTrackerDataPtr) -> ()) -> Result<u64, io::Error> {
    unsafe {
        nui_import::hand_tracker_callback(ht.r_hand_tracker, user_callback)
            .to_result()
            .map(|d| 
                 match d {
                     Some(CData::CallBackId(h)) => h,
                     _ => unreachable!(),
                 })
    }
}

pub fn to_hand_data(data: RHandTrackerDataPtr) -> Result<RHandData, io::Error> {
    unsafe {
        nui_import::to_raw(data).to_result()
            .map(|d| 
                 match d {
                     Some(CData::HandData(h)) => h,
                     _ => unreachable!(),
                 })
    }
}
