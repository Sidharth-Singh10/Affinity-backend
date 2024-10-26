use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use crate::utils::constants::{SMTP_HOST, SMTP_PASSWORD, SMTP_USERNAME};

pub struct Email {
    to: String,
    email_body: String,
}

impl Email {
    pub fn new(to: String, email_body: String) -> Self {
        Email { to, email_body }
    }

    fn new_transport(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, lettre::transport::smtp::Error> {
        let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(SMTP_HOST.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        Ok(transport)
    }

    pub async fn send_email(&self, subject: String) -> Result<(), Box<dyn std::error::Error>> {
        // let html_template = self.render_template(template_name)?;

        let email = Message::builder()
            .from("no-reply@affinity.com".parse()?)
            .to(self.to.parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(self.email_body.clone())?;

        let transport = self.new_transport()?;

        transport.send(email).await?;
        Ok(())
    }
}
