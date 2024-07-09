use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::{Error, Result};

use crate::auth::privacy_level::PrivacyLevel;
use crate::follower::status::FollowerStatus;
use crate::user::model::UserQuery;
use crate::user_block::model::UserBlockStatus;

impl UserQuery {
    pub async fn get(pool: &PgPool, username: &str, request_user_id: Uuid) -> Result<Self> {
        let query: Option<UserQuery> = sqlx::query_as(
            r#"
            SELECT
                false as can_view,
                t1.name,
                t1.username,
                CASE WHEN t1.id = $2 THEN true ELSE false END as is_self,
                t1.privacy_level,
                t2.status AS follower_status,
                t3.blocked_status AS blocked_status,
                (SELECT COUNT(*) FROM user_follower WHERE user_id = t1.id AND status = 1) as follower_count,
                (SELECT COUNT(*) FROM user_follower WHERE follower_id = t1.id AND status = 1) as following_count
            FROM
                users_user t1
                LEFT JOIN user_follower t2 ON t2.user_id = t1.id AND t2.follower_id = $2
                LEFT JOIN user_block t3 ON t3.blocker_id = t1.id AND t3.blocked_id = $2
            WHERE
                t1.username = $1
            "#,)
            .bind(username,)
            .bind(request_user_id).fetch_optional(pool).await?;

        let Some(mut user) = query else {
            return Err(Error::Other("User not found".into()));
        };

        if user.blocked_status == UserBlockStatus::Blocked {
            return Err(Error::Forbidden);
        }
        if user.is_self || user.follower_status == FollowerStatus::Accepted {
            user.can_view = true;
        }
        Ok(user)
    }
}

impl FromRow<'_, PgRow> for UserQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let privacy_level: i32 = row.try_get("privacy_level").unwrap_or(-1);
        let follower_status: i32 = row.try_get("follower_status").unwrap_or(-1);
        let blocked_status: i32 = row.try_get("blocked_status").unwrap_or(-1);
        Ok(Self {
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            is_self: row.try_get("is_self")?,
            privacy_level: PrivacyLevel::from(privacy_level),
            blocked_status: UserBlockStatus::from(blocked_status),
            follower_status: FollowerStatus::from(follower_status),
            follower_count: row.try_get("follower_count")?,
            following_count: row.try_get("following_count")?,
            can_view: false,
        })
    }
}
