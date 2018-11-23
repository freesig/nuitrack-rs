mod nui_import;
mod error_conversion;
mod errors;
mod callbacks;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;
use nui::simple::{SkeletonData, DepthFrame, RGBFrame};
use callbacks::{CallBackSkeleton, CallBackDepth, CallBackColor};

#[derive(Debug)]
pub struct Unimplemented;

pub fn init() -> Result<(), NuiError> {
    unsafe{
        nui::nui_init().to_result().map(|_|())
    }
}

pub fn skeleton_data<F>(cb: F)
    -> Result<CallBackSkeleton, NuiError>
    where
    F: FnMut(SkeletonData) -> () + Send + 'static
{
    callbacks::register_callback_closure_skeleton(cb)
}

pub fn depth_data<F>(cb: F)
    -> Result<CallBackDepth, NuiError>
    where
    F: FnMut(DepthFrame) -> () + Send + 'static
{
    callbacks::register_callback_closure_depth(cb)
}

pub fn color_data<F>(cb: F)
    -> Result<CallBackColor, NuiError>
    where
    F: FnMut(RGBFrame) -> () + Send + 'static
{
    callbacks::register_callback_closure_color(cb)
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
