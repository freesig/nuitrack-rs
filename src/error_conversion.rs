use super::nui_import::root::{Value, Nothing, RustResult, RHandTracker, RHandData};
use super::nui_import::RHandTrackerWrapper;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::io;

pub trait NuiResult {
    type Item;
    fn to_result(self) -> Result<Self::Item, io::Error>;
}

pub enum CData{
    HandData(RHandData),
    HandTracker(RHandTracker),
    CallBackId(u64),
}

impl NuiResult for RustResult {
    type Item = Option<CData>;
    fn to_result(self) -> Result<Self::Item, io::Error>{
        unsafe {
            match self {
                RustResult { 
                    tag: 0, 
                    value: Value{ hand_data },
                } => Ok(Some(CData::HandData(hand_data))),
                RustResult { 
                    tag: 0, 
                    value: Value{ callback_id },
                } => Ok(Some(CData::CallBackId(callback_id))),
                RustResult { tag: 0, .. } => Ok(None),
                RustResult {
                    tag: 1,
                    value: Value{ error_msg },
                } => Err(io::Error::new(
                        io::ErrorKind::Other,
                        CStr::from_ptr((& error_msg[0]) as *const c_char).to_string_lossy().into_owned(),
                        )),
                _ => unreachable!(),
            }
        }
    }
}

impl NuiResult for RHandTrackerWrapper {
    type Item = Option<CData>;
    fn to_result(self) -> Result<Self::Item, io::Error> {
        unsafe {
            match self {
                RHandTrackerWrapper { 
                    result: RustResult { tag: 0, ..},
                    r_hand_tracker
                } => Ok(Some(CData::HandTracker(r_hand_tracker))),
                RHandTrackerWrapper { result: RustResult{tag: 0, ..}, .. } => Ok(None),
                RHandTrackerWrapper{ 
                    result: RustResult {
                        tag: 1,
                        value: Value{ error_msg },
                    },
                    ..
                } => Err(io::Error::new(
                        io::ErrorKind::Other,
                        CStr::from_ptr((& error_msg[0]) as *const c_char).to_string_lossy().into_owned(),
                        )),
                _ => unreachable!(),
            }
        }
    }
}


impl Default for RustResult {
    fn default() -> RustResult { RustResult{ tag: 1, value: Value{ empty: Nothing{ _address: 0} } } }
}
