use super::model::WorkoutBase;
use crate::auth::model::{RequestUser, User};
use crate::error::{Error, Result};

impl WorkoutBase {
    pub async fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_create(target_user: &User, user: &RequestUser) -> Result<()> {
        let _target = target_user;
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_delete(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
