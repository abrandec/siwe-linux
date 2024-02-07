use std::{ffi::CStr, str::FromStr};

use pam_bindings::{
    constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_ON},
    conv::Conv,
    module::{PamHandle, PamHooks},
    pam_try,
};

struct PamSiwe;
pam_bindings::pam_hooks!(PamSiwe);

impl PamHooks for PamSiwe {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        eprintln!("Let's authenticate!");
        let user = pam_try!(pamh.get_user(None));
        
        if user != "siwe_user" {
            return PamResultCode::PAM_AUTH_ERR;
        }
        return PamResultCode::PAM_SUCCESS;
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("set credentials");
        return PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        return PamResultCode::PAM_SUCCESS
    }
}

