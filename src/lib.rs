mod nui_import;
mod error_conversion;
mod errors;

use errors::NuiError;

#[derive(Debug)]
pub struct Unimplemented;
pub type SkeletonData = Unimplemented;

pub fn init() -> Result<(), NuiError> {
    unimplemented!()
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
