mod nui_import;
mod error_conversion;
mod errors;
mod callbacks;
mod data;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;
use std::marker::PhantomData;
pub use nui::tdv::nuitrack::Color3;
pub use nui::simple::{SkeletonData, DepthFrame, RGBFrame};
pub use callbacks::CallBack;

pub struct Nui<T> {
    state: T,
    callbacks: Vec<CallBackHolder>,
}

unsafe impl<T> Send for Nui<T> {}

pub struct State<T> {
    phantom: PhantomData<T>,
}
pub struct Initialized {
    clean_up: fn() -> (),
}
pub struct Running;
pub struct Offline;

enum CallBackHolder {
    Skeleton(CallBack<SkeletonData>),
    Depth(CallBack<DepthFrame>),
    Color(CallBack<RGBFrame>),
}

pub fn init() -> Result<Nui<Initialized>, NuiError> {
    Nui::<Offline>::new()
}

impl Nui<Offline> {
    pub fn new() -> Result<Nui<Initialized>, NuiError> {
        unsafe{
            nui::nui_init()
                .to_result()
                .map(|_|Nui{state: Initialized{clean_up: release_nui}, callbacks: Vec::new()})
        }
    }
}

impl Nui<Initialized> {
    pub fn skeleton_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(SkeletonData) -> () + Send + 'static
        {
            CallBack::<SkeletonData>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Skeleton(cbw)))

        }

    pub fn depth_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(DepthFrame) -> () + Send + 'static
        {
            CallBack::<DepthFrame>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Depth(cbw)))
        }

    pub fn color_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(RGBFrame) -> () + Send + 'static
        {
            CallBack::<RGBFrame>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Color(cbw))) 
        }

    pub fn run(mut self) -> Result<Nui<Running>, NuiError> {
        unsafe{
            fn none(){};
            self.state.clean_up = none;
            nui::nui_run()
                .to_result()
                .map(|_|Nui{state: Running{}, callbacks: self.callbacks})
        }
    }
}

impl Nui<Running> {
    pub fn update(&self) -> Result<(), NuiError> {
        unsafe{
            nui::nui_update().to_result().map(|_|())
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        release_nui();
    }
}

impl Drop for Initialized {
    fn drop(&mut self) {
        (self.clean_up)();
    }
}

fn release_nui() {
    unsafe{
        match nui::nui_release().to_result() {
            Ok(_) => (),
            Err(e) => eprintln!("Error releasing nuitrack: {}", e),
        }
    }
}
