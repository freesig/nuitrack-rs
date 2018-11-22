use super::nui::{self, simple::SkeletonData};
use std::ffi::c_void;
use errors::NuiError;
use error_conversion::{NuiResult, CallBackId};

pub struct CallBack {
    callback_id: CallBackId,
    callback_ptr: *mut c_void,
}

struct ClosureWapper {
    cb: Box<FnMut(SkeletonData)>,
}

pub fn register_callback_closure<F: FnMut(SkeletonData) -> () + Send + 'static>(cb: F) -> Result<CallBack, NuiError> {
    let cbw = Box::new(ClosureWapper{
        cb: Box::new(cb),
    }); 
    unsafe {
        let callback_ptr = Box::into_raw(cbw) as *mut c_void;
        nui::register_closure(Some(cb_handler), callback_ptr)
            .to_result()
            .map(|id| CallBack{callback_id: id.into(), callback_ptr} )
    }
}


extern "C" fn cb_handler(closure: *mut c_void, n: SkeletonData) {
    let wrapper = closure as *mut ClosureWapper;
    unsafe{
        (*(*wrapper).cb)(n);
    }

}

impl Drop for CallBack {
    fn drop(&mut self) {
        let _cb: Box<ClosureWapper> = unsafe { Box::from_raw(self.callback_ptr as *mut ClosureWapper) };
    }
}
