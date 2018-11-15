mod nui_import;
mod error_conversion;

use self::nui_import::root as nui;
use self::error_conversion::NuiResult;
use std::io;

pub fn initialize() -> Result<(), io::Error> {
    unsafe { nui::nui_init().to_result() }
}

pub fn create_hand_tracker() -> Result<(), io::Error> {
    unsafe {
        nui_import::create_hand_tracker().to_result()
    }
}

/*
fn hand_callback(&mut nui::RHandTrackerData) {
}
*/
