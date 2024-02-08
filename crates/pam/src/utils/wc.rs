use rand::{distributions::Alphanumeric, Rng};
// use walletconnect_sdk;

// Generate a walletconnect URI (2.0 format)
// Not a complete implementation, just enough to create a connection
// ERC-1328: WalletConnect URI Format - https://eips.ethereum.org/EIPS/eip-1328
pub fn generate_wc_uri(topic: String) -> String {
    // wc:{TOPIC}@2?relay-protocol=irn&symKey={SYM_KEY}
    let sym_key: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect();

    let uri = format!("wc:{}@2?relay-protocol=irn&symKey={:?}", topic, sym_key);
    return uri.to_string();
}