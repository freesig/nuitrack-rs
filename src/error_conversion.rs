use super::nui_import::root::{Value, RustResult};
use super::nui_import::root::simple::SkeletonData;
use super::errors::NuiError;
use std::ffi::CStr;
use std::os::raw::c_char;

pub trait NuiResult {
    type Item;
    fn to_result(self) -> Result<Self::Item, NuiError>;
}

const EMPTY_TYPE: i32 = 0;
const CALL_BACK_TYPE: i32 = 1;
const SKELETON_DATA_TYPE: i32 = 2;

pub type CallBackId = u64;

pub enum CData{
    SkeletonData(SkeletonData),
    CallBackId(CallBackId),
    Empty,
}

impl NuiResult for RustResult {
    type Item = CData;
    fn to_result(self) -> Result<Self::Item, NuiError>{
        unsafe {
            match self {
                RustResult { 
                    tag: EMPTY_TYPE, 
                    ..
                } => Ok(CData::Empty),
                RustResult { 
                    tag: CALL_BACK_TYPE, 
                    value: Value{ callback_id },
                } => Ok(CData::CallBackId(callback_id)),
                RustResult { 
                    tag: SKELETON_DATA_TYPE, 
                    value: Value{ skeleton_data },
                } => Ok(CData::SkeletonData(skeleton_data)),
                RustResult {
                    tag: _,
                    value: Value{ error_msg },
                } => Err(NuiError::Failed(
                        CStr::from_ptr((& error_msg[0]) as *const c_char).to_string_lossy().into_owned(),
                        )),
            }
        }
    }
}

impl From<CData> for CallBackId {
    fn from(data: CData) -> CallBackId {
        match data {
            CData::CallBackId(id) => id,
            _ => panic!("Type conversion failure"),
        }
    }
}
