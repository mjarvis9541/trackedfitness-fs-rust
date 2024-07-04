use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::get_config;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TokenType {
    Auth,
    Activation,
    PasswordReset,
    EmailChange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub exp: i64,
    pub user_id: Uuid,
    pub email: String,
    pub token_type: TokenType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub exp: i64,
    pub user_id: Uuid,
    pub username: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

pub struct JwtManager;

impl JwtManager {
    pub fn generate_auth_token(
        user_id: Uuid,
        username: &str,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
    ) -> Result<String> {
        let config = get_config();

        let expiration = Utc::now()
            .checked_add_signed(TimeDelta::seconds(config.token_duration_authentication))
            .expect("valid timestamp")
            .timestamp();

        let claims = AuthToken {
            exp: expiration,
            user_id,
            username: username.to_string(),
            is_active,
            is_staff,
            is_superuser,
        };

        let token = encode(&Header::default(), &claims, &config.encoding_key)?;
        Ok(token)
    }

    pub fn validate_auth_token(token: &str) -> Result<AuthToken> {
        let config = get_config();
        let validation = Validation::default();

        let token_data = decode::<AuthToken>(token, &config.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn create_token(
        user_id: Uuid,
        email: impl Into<String>,
        token_type: TokenType,
    ) -> Result<String> {
        let config = get_config();

        let duration = match token_type {
            TokenType::Auth => config.token_duration_authentication,
            TokenType::Activation => config.token_duration_account_activation,
            TokenType::PasswordReset => config.token_duration_password_reset,
            TokenType::EmailChange => config.token_duration_email_change,
        };

        let expiration = Utc::now()
            .checked_add_signed(TimeDelta::days(duration))
            .expect("valid timestamp")
            .timestamp();

        let claims = Token {
            exp: expiration,
            user_id,
            email: email.into(),
            token_type,
        };

        let token = encode(&Header::default(), &claims, &config.encoding_key)?;
        Ok(token)
    }

    pub fn validate_token(token: &str, expected_type: TokenType) -> Result<Token> {
        let config = get_config();

        let validation = Validation::default();
        let token_data = decode::<Token>(token, &config.decoding_key, &validation)?;
        let claims = token_data.claims;

        if claims.token_type != expected_type {
            return Err(Error::InvalidToken);
        }

        Ok(claims)
    }
}
