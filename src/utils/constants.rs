use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String = {
        dotenv().ok(); // loading env vars from the file
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    pub static ref SMTP_USERNAME: String = {
        env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set")
    };

    pub static ref SMTP_PASSWORD: String = {
        env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set")
    };

    pub static ref SMTP_HOST: String = {
        env::var("SMTP_HOST").expect("SMTP_HOST must be set")
    };

    pub static ref OTP_SECRET: String = {
        env::var("OTP_SECRET").expect("OTP_SECRET must be set")
    };

    pub static ref JWT_SECRET: String = {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    };

    pub static ref PASS_RESET_LINK: String = {
        env::var("PASS_RESET_LINK").expect("PASS_RESET_LINK must be set")
    };

    pub static ref ALLOWED_ORIGINS: Vec<String> = {
        dotenv().ok();
        match env::var("ALLOWED_ORIGINS") {
            Ok(allowed_origins_env) => {
                allowed_origins_env
                    .split(',')
                    .filter_map(|origin| {
                        let trimmed_origin = origin.trim();
                        if !trimmed_origin.is_empty() {
                            Some(trimmed_origin.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            Err(_) => {
                // If ALLOWED_ORIGINS is not set, it defaults to an empty list
                println!("Warning: ALLOWED_ORIGINS variable not set. Defaulting to an empty list.");
                Vec::new()
            }
        }
    };
}
