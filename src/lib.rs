mod nui_import;
mod error_conversion;
mod errors;
mod callbacks;
mod data;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;
pub use nui::tdv::nuitrack::Color3;
pub use nui::simple::{SkeletonData, DepthFrame, RGBFrame};
pub use callbacks::CallBack;

pub fn init() -> Result<(), NuiError> {
    unsafe{
        nui::nui_init().to_result().map(|_|())
    }
}

pub fn skeleton_data<F>(cb: F)
    -> Result<CallBack<SkeletonData>, NuiError>
    where
    F: FnMut(SkeletonData) -> () + Send + 'static
{
    CallBack::<SkeletonData>::new(cb)

}

pub fn depth_data<F>(cb: F)
    -> Result<CallBack<DepthFrame>, NuiError>
    where
    F: FnMut(DepthFrame) -> () + Send + 'static
{
    CallBack::<DepthFrame>::new(cb)
}

pub fn color_data<F>(cb: F)
    -> Result<CallBack<RGBFrame>, NuiError>
    where
    F: FnMut(RGBFrame) -> () + Send + 'static
{
    CallBack::<RGBFrame>::new(cb)
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
