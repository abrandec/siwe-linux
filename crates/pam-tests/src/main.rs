use std::any::Any;

use pam_siwe::misc::Client;

fn main() {  
    let mut client = Client::start("siwe-auth").expect("Unable to start PAM client");
    client
        .conversation_mut()
        .set_credentials("user_1", "password");
}