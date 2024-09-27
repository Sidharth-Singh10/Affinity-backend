use crate::configs::email_config::Email;
use axum::{http::StatusCode, response::IntoResponse};
use handlebars::Handlebars;

pub struct EmailOTP {
    user_name: String,
    otp: String,
    to: String,
}

impl EmailOTP {
    pub fn new(user_name: String, otp: String, to: String) -> Self {
        EmailOTP { user_name, otp, to }
    }

    pub async fn send_otp(&self) -> impl IntoResponse {
        let mut handlebars = Handlebars::new();

        // Register the template with Handlebars
        if handlebars
            .register_template_file("email_template", "src/utils/hbs/email_verification.hbs")
            .is_err()
        {
            // Return 500 Internal Server Error if template registration fails
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        // Data to inject into the template
        let data = serde_json::json!({
            "user_name": self.user_name,
            "otp": self.otp,
            "email": self.to,
        });

        // Render the email body
        let email_body = match handlebars.render("email_template", &data) {
            Ok(body) => body,
            Err(_) => {
                // Return 500 Internal Server Error if rendering the template fails
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        // Email configuration
        let email = Email::new(self.to.clone(), email_body);

        // Try to send the email
        match email.send_email("OTP Verification".to_string()).await {
            Ok(_) => Ok(StatusCode::OK.into_response()),
            Err(_) => {
                // Return 500 Internal Server Error if sending the email fails
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
