use http::request::Parts;

use uuid::Uuid;

use super::email_service::EmailService;
use super::model::RequestUser;
use super::token::{JwtManager, TokenType};
use crate::config::get_config;
use crate::error::{Error, Result};

pub fn extract_user_from_request() -> Result<Option<RequestUser>> {
    let parts = leptos::use_context::<Parts>().ok_or(Error::InternalServer)?;
    let request_user = parts.extensions.get::<RequestUser>().cloned();
    Ok(request_user)
}

pub fn get_request_user() -> Result<RequestUser> {
    let parts = leptos::use_context::<Parts>().ok_or(Error::InternalServer)?;
    let request_user = parts
        .extensions
        .get::<RequestUser>()
        .cloned()
        .ok_or(Error::Unauthorized)?;
    request_user
        .is_active
        .then(|| ())
        .ok_or(Error::Unauthorized)?;
    Ok(request_user)
}

pub fn extract_superuser_from_request() -> Result<RequestUser> {
    let parts = leptos::use_context::<Parts>().ok_or(Error::InternalServer)?;
    let request_user = parts
        .extensions
        .get::<RequestUser>()
        .cloned()
        .ok_or(Error::Unauthorized)?;
    request_user.is_active.then(|| ()).ok_or(Error::Forbidden)?;
    request_user
        .is_superuser
        .then(|| ())
        .ok_or(Error::Forbidden)?;
    Ok(request_user)
}

pub struct AuthService;

impl AuthService {
    fn generate_link(token: &str, token_type: TokenType) -> String {
        let config = get_config();
        match token_type {
            TokenType::Activation => format!("{}/signup/confirm?token={}", config.domain, token),
            TokenType::PasswordReset => {
                format!("{}/password-reset/confirm?token={}", config.domain, token)
            }
            TokenType::EmailChange => format!("{}/confirm-email?token={}", config.domain, token),
            _ => String::new(),
        }
    }

    pub async fn send_activation_email(user_id: Uuid, name: &str, email: &str) -> Result<()> {
        let token = JwtManager::create_token(user_id, "", TokenType::Activation)?;
        let confirmation_link = Self::generate_link(&token, TokenType::Activation);
        EmailService::send_activation_email(&name, email, &confirmation_link).await?;
        Ok(())
    }

    pub async fn send_password_reset_email(user_id: Uuid, name: &str, email: &str) -> Result<()> {
        let token = JwtManager::create_token(user_id, "", TokenType::PasswordReset)?;
        let confirmation_link = Self::generate_link(&token, TokenType::PasswordReset);
        EmailService::send_reset_password_email(&name, email, &confirmation_link).await?;
        Ok(())
    }

    pub async fn send_email_change_email(user_id: Uuid, name: &str, new_email: &str) -> Result<()> {
        let token = JwtManager::create_token(user_id, new_email, TokenType::EmailChange)?;
        let confirmation_link = Self::generate_link(&token, TokenType::EmailChange);
        EmailService::send_email_confirmation(name, new_email, &confirmation_link).await?;
        Ok(())
    }
}
