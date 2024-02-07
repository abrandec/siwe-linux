pub mod pam;
//pub mod wc;
pub mod utils;
pub mod tests;

use pam_bindings::conv::{Inner, Conv};
use std::ffi::{CStr, CString};

// implement pam_start from libpam
