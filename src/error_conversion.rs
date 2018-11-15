use super::nui::{self, Value, Nothing, RustResult};
use super::nui_import::RHandTrackerWrapper;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::io;

pub trait NuiResult {
    type Item;
    fn to_result(self) -> Result<Self::Item, io::Error>;
}

impl NuiResult for RustResult {
    type Item = ();
    fn to_result(self) -> Result<Self::Item, io::Error>{
        unsafe {
            match self {
                RustResult { tag: 0, .. } => Ok(()),
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
    type Item = ();
    fn to_result(self) -> Result<Self::Item, io::Error> {
        unsafe {
            match self {
                RHandTrackerWrapper { result: RustResult{tag: 0, ..}, .. } => Ok(()),
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
