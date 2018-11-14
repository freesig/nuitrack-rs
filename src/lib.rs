mod nui_import;

use self::nui_import as nui;
use std::io;
use nui::RustResult;


pub fn initialize() -> Result<(), io::Error> {
    unsafe {
        from_c_result(nui::nui_init())
    }
}

fn from_c_result(result: RustResult) -> Result<(), io::Error> {
    match result.tag {
        0 => Ok(()),
        1 => Err(io::Error::new(io::ErrorKind::Other, "failed to init nui")),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown Error")),
    }
}
