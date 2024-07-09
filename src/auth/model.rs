use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::error::{Error, Result};

use super::privacy_level::PrivacyLevel;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub privacy_level: PrivacyLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl User {
    pub fn is_authenticated(&self) -> Result<()> {
        self.is_active.then(|| ()).ok_or(Error::Unauthorized)
    }

    pub fn is_staff(&self) -> Result<()> {
        self.is_active.then(|| ()).ok_or(Error::Unauthorized)
    }

    pub fn is_superuser(&self) -> Result<()> {
        self.is_superuser.then(|| ()).ok_or(Error::Unauthorized)
    }

    pub fn ensure_account_active(&self) -> Result<()> {
        self.is_active.then(|| ()).ok_or(Error::InvalidCredentials)
    }

    pub fn ensure_email_verified(&self) -> Result<()> {
        self.email_verified
            .then(|| ())
            .ok_or(Error::InvalidCredentials)
    }

    pub fn ensure_account_not_active(&self) -> Result<()> {
        (!self.is_active)
            .then(|| ())
            .ok_or_else(|| Error::Other("This account has already been activated.".into()))
    }

    pub fn ensure_email_not_verified(&self) -> Result<()> {
        (!self.email_verified)
            .then(|| ())
            .ok_or_else(|| Error::Other("This email address has already been verified.".into()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RequestUser {
    pub id: Uuid,
    pub username: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRelation {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub privacy_level: i32,
    pub follower_status: Option<i32>,
    pub blocked_status: Option<i32>,
}
