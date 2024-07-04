use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::{Brand, BrandQuery};

impl Brand {
    pub fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_create(user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, user: &RequestUser) -> Result<()> {
        user.is_superuser().await
    }

    pub async fn can_delete(&self, user: &RequestUser) -> Result<()> {
        user.is_superuser().await
    }
}

impl BrandQuery {
    pub fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
