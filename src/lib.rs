mod nui_import;
mod error_conversion;
mod errors;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;

#[derive(Debug)]
pub struct Unimplemented;
pub type SkeletonData = Unimplemented;

pub fn init() -> Result<(), NuiError> {
    unsafe{
        nui::nui_init().to_result().map(|_|())
    }
}

pub fn skeleton_data<F>(cb: F)
    -> Result<(), NuiError>
    where
    F: FnMut(SkeletonData) -> () + Send + 'static
{
    unimplemented!()
}

pub fn run() -> Result<(), NuiError> {
    unimplemented!()
}
pub fn release() -> Result<(), NuiError> {
    unimplemented!()
}
