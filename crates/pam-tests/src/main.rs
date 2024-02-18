use pam_siwe::misc::Client;
fn main() {
    let mut client = Client::start("siwe-auth").expect("Failed to start PAM client");
}