use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::{Food, FoodQuery};

impl Food {
    pub async fn can_create(request_user: &RequestUser) -> Result<()> {
        if request_user.is_active {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if self.created_by_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if self.created_by_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }
}

impl FoodQuery {
    pub fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
