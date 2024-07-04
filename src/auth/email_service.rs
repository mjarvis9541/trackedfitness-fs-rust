use lettre::message::header::ContentType;
use lettre::message::{Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use tera::Context;

use crate::config::get_config;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Failed to render template")]
    RenderError(#[from] tera::Error),
    #[error("Failed to parse mailbox content")]
    ParseError,
    #[error("Failed to build email")]
    BuildError,
    #[error("Failed to send email: {0}")]
    SendError(#[from] lettre::error::Error),
    #[error("SMTP transport error: {0}")]
    SmtpTransportError(#[from] lettre::transport::smtp::Error),
    #[error("Failed to {0}")]
    SendingError(String),
}

pub struct EmailService;

impl EmailService {
    async fn send_email(recipient: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        let config = get_config();

        let credentials = Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone());

        let smtp_client = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .expect("should be valid host")
            .credentials(credentials)
            .build();

        let email = Message::builder()
            .from(config.from_address.parse().map_err(|_| EmailError::ParseError)?)
            .to(recipient.parse().map_err(|_| EmailError::ParseError)?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                            .body("If you can't view this email, please enable HTML or click the provided link.".to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(body.to_string()),
                    ),
                )?;

        smtp_client.send(email).await.map_err(|err| {
            tracing::error!("{:?}", err);
            EmailError::SendingError(err.to_string())
        })?;

        tracing::info!("Email sent: {}, {}", recipient, subject);
        Ok(())
    }

    pub async fn send_activation_email(
        name: &str,
        recipient: &str,
        confirmation_link: &str,
    ) -> Result<(), EmailError> {
        let config = get_config();

        let mut context = Context::new();
        context.insert("name", name);
        context.insert("confirmation_link", &confirmation_link);
        context.insert("duration", "14 days");

        let body = config
            .tera
            .render(&config.template_account_activation, &context)?;
        Self::send_email(recipient, "Activate your account", &body).await
    }

    pub async fn send_reset_password_email(
        name: &str,
        recipient: &str,
        reset_link: &str,
    ) -> Result<(), EmailError> {
        let config = get_config();

        let mut context = Context::new();
        context.insert("name", name);
        context.insert("confirmation_link", reset_link);
        context.insert("duration", "24 hours");

        let body = config
            .tera
            .render(&config.template_password_reset, &context)?;
        Self::send_email(recipient, "Reset your password", &body).await
    }

    pub async fn send_email_confirmation(
        name: &str,
        recipient: &str,
        confirmation_link: &str,
    ) -> Result<(), EmailError> {
        let config = get_config();

        let mut context = Context::new();
        context.insert("name", name);
        context.insert("confirmation_link", &confirmation_link);
        context.insert("duration", "24 hours");

        let body = config
            .tera
            .render(&config.template_email_change, &context)?;
        Self::send_email(recipient, "Confirm your new email address", &body).await
    }
}
