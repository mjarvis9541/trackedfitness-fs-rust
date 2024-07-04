use super::model::Diet;
use crate::auth::model::RequestUser;
use crate::error::{Error, Result};
use uuid::Uuid;

impl Diet {
    pub fn can_create(request_user: &RequestUser, user_id: Uuid) -> Result<()> {
        if user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }
}
