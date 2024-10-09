use crate::utils::constants::HMAC_SECRET;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;

fn create_hmac(token: &str) -> String {
    let hmac_secret_bytes = HMAC_SECRET.as_bytes();

    let mut mac =
        Hmac::<Sha256>::new_from_slice(hmac_secret_bytes).expect("HMAC can take key of any size");
    mac.update(token.as_bytes());
    let result = mac.finalize();
    general_purpose::STANDARD_NO_PAD.encode(result.into_bytes())
}

pub fn generate_secure_token() -> (String, String) {
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let hmac = create_hmac(&token);
    (token, hmac)
}

pub fn verify_token(token: &str, stored_hmac: &str) -> bool {
    let calculated_hmac = create_hmac(token);
    calculated_hmac == stored_hmac
}
