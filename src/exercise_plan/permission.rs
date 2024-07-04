use uuid::Uuid;

use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::ExercisePlan;

impl ExercisePlan {
    pub async fn can_create(request_user: &RequestUser, target_user_id: Uuid) -> Result<()> {
        if target_user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }
}
