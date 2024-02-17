use std::{ffi::CStr, str::FromStr, env};
use dotenv::dotenv;
use fast_qr::qr;
use pam_bindings::{
    constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_ON},
    conv::Conv,
    module::{PamHandle, PamHooks},
    pam_try,
};
use structopt::StructOpt;
use walletconnect_sdk;

use crate::utils::wc::{generate_uri_sym, generate_qr_unicode, generate_wc_uri};

#[derive(StructOpt)]
struct Args {
    /// Specify HTTP address.
    #[structopt(short, long, default_value = "https://relay.walletconnect.com/rpc")]
    address: String,

    /// Specify WalletConnect project ID.
    #[structopt(short, long, default_value = "")]
    project_id: String,
}

struct PamSiwe;
pam_bindings::pam_hooks!(PamSiwe);

impl PamHooks for PamSiwe {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        eprintln!("Let's authenticate!");
        let user = pam_try!(pamh.get_user(None));

        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => {
                unreachable!("No conv available");
            }
            Err(err) => {
                println!("Couldn't get pam_conv");
                return err;
            }
        };
        
        let password = pam_try!(conv.send(PAM_PROMPT_ECHO_ON, "Password:"));
        
        let expected_cstr_password = CStr::from_bytes_with_nul(b"password\0").unwrap();

        println!("\nUser: {}, password: {:?}", user, password);
        if user != "user_1" || password != Some(expected_cstr_password) {
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

