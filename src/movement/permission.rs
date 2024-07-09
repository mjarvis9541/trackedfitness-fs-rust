use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::{Movement, MovementQuery, MovementWithLatestWeight};

impl Movement {
    pub async fn can_create(request_user: &RequestUser) -> Result<()> {
        if request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}

impl MovementQuery {
    pub fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}

impl MovementWithLatestWeight {
    pub fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
