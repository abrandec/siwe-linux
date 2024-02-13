use rand::{distributions::Alphanumeric, Rng};
use fast_qr::qr::{QRBuilder, QRCode};
// use walletconnect_sdk;

// Generate a walletconnect URI (2.0 format)
// Not a complete implementation, just enough to create a connection
// ERC-1328: WalletConnect URI Format - https://eips.ethereum.org/EIPS/eip-1328
pub fn generate_wc_uri(topic: String) -> String {
    // Generate a random 64-byte symmetric key
    let sym_key: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect();

    let uri = format!("wc:{}@2?relay-protocol=irn&symKey={:?}", topic, sym_key);
    return uri.to_string();
}

pub fn generate_uri_sym(topic: String, sym: String) -> String {
    // Generate a random 64-byte symmetric key

    let uri = format!("wc:{}@2?relay-protocol=irn&symKey={}", topic, sym);
    return uri.to_string();
}

// Generate a QR code in Unicode for a given URI
pub fn generate_qr_unicode(uri: String) -> String {
    let qr = QRBuilder::new(uri).build().unwrap();
    // Note: Will start with a newline
    return format!("\n{}", qr.to_str());
}