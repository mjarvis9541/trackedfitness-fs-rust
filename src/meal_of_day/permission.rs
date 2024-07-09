use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::MealOfDay;

impl MealOfDay {
    pub async fn can_create(user: &RequestUser) -> Result<()> {
        user.is_superuser().await
    }

    pub async fn can_update(&self, user: &RequestUser) -> Result<()> {
        user.is_superuser().await
    }

    pub async fn can_delete(&self, user: &RequestUser) -> Result<()> {
        user.is_superuser().await
    }
}

impl MealOfDay {
    pub async fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
