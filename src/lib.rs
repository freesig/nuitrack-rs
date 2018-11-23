mod nui_import;
mod error_conversion;
mod errors;
mod callbacks;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;
use nui::simple::SkeletonData;
use callbacks::CallBack;

#[derive(Debug)]
pub struct Unimplemented;

pub fn init() -> Result<(), NuiError> {
    unsafe{
        nui::nui_init().to_result().map(|_|())
    }
}

pub fn skeleton_data<F>(cb: F)
    -> Result<CallBack, NuiError>
    where
    F: FnMut(SkeletonData) -> () + Send + 'static
{
    callbacks::register_callback_closure(cb)
}

pub fn run() -> Result<(), NuiError> {
    unsafe{
        nui::nui_run().to_result().map(|_|())
    }
}

pub fn update() -> Result<(), NuiError> {
    unsafe{
        nui::nui_update().to_result().map(|_|())
    }
}

pub fn release() -> Result<(), NuiError> {
    unsafe{
        nui::nui_release().to_result().map(|_|())
    }
}
