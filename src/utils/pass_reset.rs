use crate::configs::email_config::Email;
use axum::{http::StatusCode, response::IntoResponse};
use handlebars::Handlebars;

pub struct PassReset {
    user_name: String,
    reset_link: String,
    to: String,
}

impl PassReset {
    pub fn new(user_name: String, reset_link: String, to: String) -> Self {
        PassReset {
            user_name,
            reset_link,
            to,
        }
    }

    pub async fn send_pass_reset(&self) -> impl IntoResponse {
        let mut handlebars = Handlebars::new();

        // Register the template with Handlebars
        if handlebars
            .register_template_file("email_template", "src/utils/hbs/pass_reset.hbs")
            .is_err()
        {
            // Return 500 Internal Server Error if template registration fails
            eprintln!("lauda");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        // Data to inject into the template
        let data = serde_json::json!({
            "user_name": self.user_name,
            "reset_link": self.reset_link,
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
        match email.send_email("reset_link".to_string()).await {
            Ok(_) => Ok(StatusCode::OK.into_response()),
            Err(_) => {
                // Return 500 Internal Server Error if sending the email fails
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
