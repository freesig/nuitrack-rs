use super::nui::{self, simple::SkeletonData, simple::DepthFrame, simple::RGBFrame};
use std::ffi::c_void;
use errors::NuiError;
use error_conversion::{NuiResult, CallBackId};

#[derive(Debug)]
pub struct CallBackSkeleton {
    callback_id: CallBackId,
    callback_ptr: *mut c_void,
}

#[derive(Debug)]
pub struct CallBackDepth {
    callback_id: CallBackId,
    callback_ptr: *mut c_void,
}

#[derive(Debug)]
pub struct CallBackColor {
    callback_id: CallBackId,
    callback_ptr: *mut c_void,
}

struct ClosureWapper<T> {
    cb: Box<FnMut(T)>,
}

pub fn register_callback_closure_skeleton<F: FnMut(SkeletonData) -> () + Send + 'static>(cb: F) -> Result<CallBackSkeleton, NuiError> {
    let cbw = Box::new(ClosureWapper{
        cb: Box::new(cb),
    }); 
    unsafe {
        let callback_ptr = Box::into_raw(cbw) as *mut c_void;
        nui::register_skeleton_closure(Some(skeleton_cb_handler), callback_ptr)
            .to_result()
            .map(|id| CallBackSkeleton{callback_id: id.into(), callback_ptr} )
    }
}

extern "C" fn skeleton_cb_handler(closure: *mut c_void, n: SkeletonData) {
    let wrapper = closure as *mut ClosureWapper<SkeletonData>;
    unsafe{
        (*(*wrapper).cb)(n);
    }

}

pub fn register_callback_closure_depth<F: FnMut(DepthFrame) -> () + Send + 'static>(cb: F) -> Result<CallBackDepth, NuiError> {
    let cbw = Box::new(ClosureWapper{
        cb: Box::new(cb),
    }); 
    unsafe {
        let callback_ptr = Box::into_raw(cbw) as *mut c_void;
        nui::register_depth_closure(Some(depth_cb_handler), callback_ptr)
            .to_result()
            .map(|id| CallBackDepth{callback_id: id.into(), callback_ptr} )
    }
}

extern "C" fn depth_cb_handler(closure: *mut c_void, n: DepthFrame) {
    let wrapper = closure as *mut ClosureWapper<DepthFrame>;
    unsafe{
        (*(*wrapper).cb)(n);
    }

}

pub fn register_callback_closure_color<F: FnMut(RGBFrame) -> () + Send + 'static>(cb: F) -> Result<CallBackColor, NuiError> {
    let cbw = Box::new(ClosureWapper{
        cb: Box::new(cb),
    }); 
    unsafe {
        let callback_ptr = Box::into_raw(cbw) as *mut c_void;
        nui::register_color_closure(Some(color_cb_handler), callback_ptr)
            .to_result()
            .map(|id| CallBackColor{callback_id: id.into(), callback_ptr} )
    }
}

extern "C" fn color_cb_handler(closure: *mut c_void, n: RGBFrame) {
    let wrapper = closure as *mut ClosureWapper<RGBFrame>;
    unsafe{
        (*(*wrapper).cb)(n);
    }

}

impl Drop for CallBackSkeleton {
    fn drop(&mut self) {
        let _cb: Box<ClosureWapper<SkeletonData>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<SkeletonData>) };
    }
}

impl Drop for CallBackDepth {
    fn drop(&mut self) {
        let _cb: Box<ClosureWapper<DepthFrame>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<DepthFrame>) };
    }
}

impl Drop for CallBackColor {
    fn drop(&mut self) {
        let _cb: Box<ClosureWapper<RGBFrame>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<RGBFrame>) };
    }
}
