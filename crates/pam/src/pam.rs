use std::{ffi::CStr, str::FromStr};

use pam::{
    constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_ON},
    conv::Conv,
    module::{PamHandle, PamHooks},
    pam_try,
};

struct PamSiwe;
pam::pam_hooks!(PamSiwe);

impl PamHooks for PamSiwe {

}

