use leptos::*;

use chrono::{TimeDelta, Utc};
use http::request::Parts;
use http::{header, HeaderMap};
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

impl AuthToken {
    pub fn validate(token: &str) -> Result<Self> {
        let config = get_config();
        let validation = Validation::default();
        let token_data = decode::<Self>(token, &config.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn validate_from_headers(headers: &HeaderMap) -> Option<Self> {
        let config = get_config();
        headers.get(header::COOKIE).and_then(|x| {
            x.to_str().ok().and_then(|cookie_str| {
                cookie_str
                    .split("; ")
                    .find(|&x| x.starts_with(&config.auth_cookie_name))
                    .and_then(|cookie| cookie.split('=').last())
                    .and_then(|token| Self::validate(token).ok())
            })
        })
    }

    pub fn get_user() -> Result<Self> {
        let req = use_context::<Parts>().ok_or(Error::InternalServer)?;
        let token = Self::validate_from_headers(&req.headers).ok_or(Error::Unauthorized)?;
        Ok(token)
    }

    pub fn get_superuser() -> Result<Self> {
        let req = use_context::<Parts>().ok_or(Error::InternalServer)?;
        let token = Self::validate_from_headers(&req.headers).ok_or(Error::Unauthorized)?;
        if token.is_superuser {
            Ok(token)
        } else {
            Err(Error::Unauthorized)
        }
    }
}
