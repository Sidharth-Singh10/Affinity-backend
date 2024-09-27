use axum::http::StatusCode;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::utils::constants::OTP_SECRET;

pub fn totp() -> Result<TOTP, StatusCode> {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        300,
        Secret::Raw(OTP_SECRET.as_bytes().to_vec())
            .to_bytes()
            .unwrap(),
    );

    match totp {
        Ok(t) => Ok(t),
        Err(_) => {
            println!("Error generating TOTP");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
