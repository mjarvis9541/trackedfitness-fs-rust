use uuid::Uuid;

use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::{Progress, ProgressQuery};

impl ProgressQuery {
    pub async fn can_create(request_user: &RequestUser, target_user_id: Uuid) -> Result<()> {
        if target_user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}

impl Progress {
    pub async fn can_create(request_user: &RequestUser, target_user_id: Uuid) -> Result<()> {
        if target_user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
