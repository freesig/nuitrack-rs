use super::nui::{self, simple::SkeletonData, simple::DepthFrame, simple::RGBFrame};
use std::ffi::c_void;
use errors::NuiError;
use error_conversion::{NuiResult, CallBackId};
use std::marker::PhantomData;
use std::panic::{catch_unwind, UnwindSafe};

pub enum CallBackType {
    Skeleton,
    Depth,
    Color,
}

pub struct CallBack<T> {
    _callback_id: CallBackId,
    callback_ptr: *mut c_void,
    /// Gives drop the type to use
    callback_type: CallBackType,
    _phantom: PhantomData<T>,
}

struct ClosureWapper<T> {
    cb: Box<FnMut(T)>,
}

extern "C" fn cb_handler<T: UnwindSafe>(closure: *mut c_void, n: T) {
    catch_unwind(|| safe_handle(closure, n))
        .unwrap_or_else(|p| {
            eprintln!("User callback has panicked with: {:?}", p);
            unsafe {
                nui::nui_release()
                    .to_result()
                    .map(|_| ())
                    .unwrap_or_else(|e| eprintln!("Failed to release nui with: {}", e));
            }
        });
}

fn safe_handle<T: UnwindSafe>(closure: *mut c_void, n: T) {
    let wrapper = closure as *mut ClosureWapper<T>;
    unsafe{
        (*(*wrapper).cb)(n);
    }
}

// Needed because the FFI needs seperate function addresses to call
extern "C" fn skeleton_handler(closure: *mut c_void, n: SkeletonData) { cb_handler(closure, n) }
extern "C" fn depth_handler(closure: *mut c_void, n: DepthFrame) { cb_handler(closure, n) }
extern "C" fn color_handler(closure: *mut c_void, n: RGBFrame) { cb_handler(closure, n) }

impl<T> ClosureWapper<T> {
    fn ptr<F: FnMut(T) -> () + Send + 'static>(cb: F) -> *mut c_void {
        let cbw = Box::new(ClosureWapper{
            cb: Box::new(cb),
        });
        Box::into_raw(cbw) as *mut c_void
    }
}

impl CallBack<SkeletonData> {
    pub fn new<F: FnMut(SkeletonData) -> () + Send + 'static>(cb: F) -> Result<Self, NuiError> {
        let callback_ptr = ClosureWapper::ptr(cb);
        unsafe {
            nui::register_skeleton_closure(Some(skeleton_handler), callback_ptr)
                .to_result()
                .map(|id| CallBack{_callback_id: id.into(), callback_ptr, callback_type: CallBackType::Skeleton, _phantom: PhantomData} )
        }
    }
}

impl CallBack<DepthFrame> {
    pub fn new<F: FnMut(DepthFrame) -> () + Send + 'static>(cb: F) -> Result<Self, NuiError> {
        let callback_ptr = ClosureWapper::ptr(cb);
        unsafe {
            nui::register_depth_closure(Some(depth_handler), callback_ptr)
                .to_result()
                .map(|id| CallBack{_callback_id: id.into(), callback_ptr, callback_type: CallBackType::Depth, _phantom: PhantomData} )
        }
    }
}

impl CallBack<RGBFrame> {
    pub fn new<F: FnMut(RGBFrame) -> () + Send + 'static>(cb: F) -> Result<Self, NuiError> {
        let callback_ptr = ClosureWapper::ptr(cb);
        unsafe {
            nui::register_color_closure(Some(color_handler), callback_ptr)
                .to_result()
                .map(|id| CallBack{_callback_id: id.into(), callback_ptr, callback_type: CallBackType::Color, _phantom: PhantomData} )
        }
    }
}

impl <T> Drop for CallBack<T> {
    fn drop(&mut self) {
        use self::CallBackType::*;
        match self.callback_type {
            Skeleton => {
                let _cb: Box<ClosureWapper<SkeletonData>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<SkeletonData>) };
            },
            Depth  => {
                let _cb: Box<ClosureWapper<DepthFrame>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<DepthFrame>) };
            },
            Color => {
                let _cb: Box<ClosureWapper<RGBFrame>> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper<RGBFrame>) };
            },
        }
    }
}
