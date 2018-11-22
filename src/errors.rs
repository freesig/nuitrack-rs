use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NuiError {
    Failed(String),
}

impl Error for NuiError {}

impl fmt::Display for NuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NuiError::*;
        match self {
            Failed(msg) => write!(f, "Nui API call failed: {}", msg),
        }
    }
}


