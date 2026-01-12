use crate::config::email::EmailConfig;
use crate::services::email_templates::{
    invite_template, invite_template_plain, password_reset_template, password_reset_template_plain,
};
use anyhow::{Context, Result};
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

/// Service for sending emails using SMTP
pub struct EmailService {
    config: EmailConfig,
    transport: Option<AsyncSmtpTransport<Tokio1Executor>>,
}

impl EmailService {
    /// Create a new EmailService instance
    pub fn new(config: EmailConfig) -> Result<Self> {
        let transport = if config.enabled {
            let mut builder =
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
                    .port(config.smtp_port);

            // Add credentials if provided
            if let (Some(username), Some(password)) = (&config.smtp_username, &config.smtp_password)
            {
                let credentials = Credentials::new(username.clone(), password.clone());
                builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
                    .context("Failed to create SMTP relay")?
                    .port(config.smtp_port)
                    .credentials(credentials);
            }

            Some(builder.build())
        } else {
            tracing::info!("Email service disabled - emails will be logged only");
            None
        };

        Ok(Self { config, transport })
    }

    /// Send an invitation email to a new user
    pub async fn send_invite(&self, to_email: &str, first_name: &str, token: &str) -> Result<()> {
        let invite_url = format!("{}/accept-invite?token={}", self.config.frontend_url, token);

        let subject = "Welcome to Time Manager - Activate Your Account";
        let html_body = invite_template(first_name, &invite_url);
        let plain_body = invite_template_plain(first_name, &invite_url);

        self.send_email(to_email, subject, &html_body, &plain_body)
            .await
    }

    /// Send a password reset email
    pub async fn send_password_reset(
        &self,
        to_email: &str,
        first_name: &str,
        token: &str,
    ) -> Result<()> {
        let reset_url = format!(
            "{}/password-reset?token={}",
            self.config.frontend_url, token
        );

        let subject = "Reset Your Time Manager Password";
        let html_body = password_reset_template(first_name, &reset_url);
        let plain_body = password_reset_template_plain(first_name, &reset_url);

        self.send_email(to_email, subject, &html_body, &plain_body)
            .await
    }

    /// Internal method to send an email
    async fn send_email(
        &self,
        to_email: &str,
        subject: &str,
        html_body: &str,
        plain_body: &str,
    ) -> Result<()> {
        // Parse from address
        let from_mailbox: Mailbox =
            format!("{} <{}>", self.config.from_name, self.config.from_email)
                .parse()
                .context("Invalid from email address")?;

        // Parse to address
        let to_mailbox: Mailbox = to_email
            .parse()
            .context("Invalid recipient email address")?;

        // Build the email message with both HTML and plain text versions
        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(plain_body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .context("Failed to build email message")?;

        // Send or log based on whether transport is enabled
        match &self.transport {
            Some(transport) => {
                transport
                    .send(email)
                    .await
                    .context("Failed to send email")?;
                tracing::info!("Email sent successfully to {}", to_email);
            }
            None => {
                tracing::info!(
                    "Email service disabled - would have sent email to {}: {}",
                    to_email,
                    subject
                );
                tracing::debug!("Email content (plain text):\n{}", plain_body);
            }
        }

        Ok(())
    }

    /// Check if email service is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_service_disabled_by_default() {
        let config = EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_username: None,
            smtp_password: None,
            from_email: "test@example.com".to_string(),
            from_name: "Test".to_string(),
            frontend_url: "http://localhost:5173".to_string(),
            enabled: false,
        };

        let service = EmailService::new(config).unwrap();
        assert!(!service.is_enabled());
    }
}
