use std::{ffi::CStr, str::FromStr, env};
use dotenv::dotenv;
use fast_qr::qr;
use pam_bindings::{
    constants::{PamFlag, PamResultCode},
    module::{PamHandle, PamHooks},
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
        dotenv().ok();
        let address = env::var("WC_ADDRESS").expect("WC_ADDRESS must be set");
        let project_id = env::var("WC_PROJECT_ID").expect("WC_PROJECT_ID must be set");
        let uri = generate_uri_sym("0d775e27722b25c0b166f28fe4c7893cfb78f340ae4ffea8629d8b538d3a7044".to_string(), "8e887df02c5686637b59c239a0adb02fb86181477625bcfd2083595533fc4477".to_string());
        let qr = generate_qr_unicode(uri);
        //let wc_uri = generate_wc_uri("7b3a9ff4317a47a87eb8451472548e3501ef116e66db15b166080c28b67da8b1".to_string());
        //let qr_code = generate_wc_qr_code("uri".to_string());

        eprintln!("CONNECTING: {}", qr);
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

