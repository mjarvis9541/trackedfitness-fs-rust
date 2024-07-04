use leptos::ServerFnError;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Error;
use crate::util::database::Filter;

use super::model::{UserBlock, UserBlockStatus};

impl FromRow<'_, PgRow> for UserBlock {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            blocker_id: row.try_get("blocker_id")?,
            blocked_id: row.try_get("blocked_id")?,
            blocked_status: UserBlockStatus::from(row.try_get::<i32, _>("blocked_status")?),
            blocked_at: row.try_get("blocked_at")?,
            unblocked_at: row.try_get("unblocked_at")?,
            blocker_username: row.try_get("blocker_username")?,
            blocked_username: row.try_get("blocked_username")?,
        })
    }
}

impl UserBlock {
    pub async fn try_get_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        sqlx::query_as(
            "
            SELECT
                t1.*,
                t2.username as blocker_username,
                t3.username as blocked_username
            FROM
                user_block t1
                LEFT JOIN users_user t2 ON t2.id = t1.blocker_id
                LEFT JOIN users_user t3 ON t3.id = t1.blocked_id
            WHERE 
                t1.id = $1
            ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_object_or_404(pool: &PgPool, id: Uuid) -> Result<Self, ServerFnError> {
        let object = Self::try_get_by_id(pool, id)
            .await?
            .ok_or(Error::NotFound)?;
        Ok(object)
    }

    pub async fn create(
        pool: &PgPool,
        blocker_id: Uuid,
        blocked_id: Uuid,
        blocked_status: i32,
    ) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            INSERT INTO user_block (blocker_id, blocked_id, blocked_status)
            VALUES ($1, $2, $3)
            ",
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .bind(blocked_status)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        blocker_id: Uuid,
        blocked_id: Uuid,
        blocked_status: i32,
    ) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            UPDATE user_block 
            SET
                blocker_id = $1,
                blocked_id = $2,
                blocked_status = $3,
                unblocked_at = NOW()
            WHERE id = $4
            ",
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .bind(blocked_status)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn update_by_username_pair(
        pool: &PgPool,
        blocker_username: &str,
        blocked_username: &str,
        blocked_status: i32,
    ) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            UPDATE user_block t1
            SET
                blocked_status = $3,
                unblocked_at = NOW()
            FROM
                users_user t2,
                users_user t3
            WHERE
                blocker_id = t2.id
                AND blocked_id = t3.id
                AND t2.username = $1
                AND t3.username = $2
            ",
        )
        .bind(blocker_username)
        .bind(blocked_username)
        .bind(blocked_status)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn count(
        pool: &PgPool,
        blocker: &str,
        blocked: &str,
        status: &str,
    ) -> sqlx::Result<i64> {
        let mut qb_scalar = sqlx::QueryBuilder::new(
            "
            SELECT COUNT(t1.*) 
            FROM user_block t1
            LEFT JOIN users_user t2 ON t2.id = t1.blocker_id
            LEFT JOIN users_user t3 ON t3.id = t1.blocked_id
            WHERE TRUE
            ",
        );
        qb_scalar.filter("t2.username", "=", blocker);
        qb_scalar.filter("t3.username", "ilike", blocked);
        if !status.is_empty() {
            let status = status.parse::<i64>().unwrap_or_default();
            qb_scalar.push(" AND t1.blocked_status = ");
            qb_scalar.push_bind(status);
        };
        qb_scalar.build_query_scalar().fetch_one(pool).await
    }

    pub async fn filter(
        pool: &PgPool,
        blocker: &str,
        blocked: &str,
        status: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.*,
                t2.username as blocker_username,
                t3.username as blocked_username
            FROM
                user_block t1
                LEFT JOIN users_user t2 ON t2.id = t1.blocker_id
                LEFT JOIN users_user t3 ON t3.id = t1.blocked_id
            WHERE
                TRUE
            ",
        );
        qb.filter("t2.username", "=", blocker);
        qb.filter("t3.username", "ilike", blocked);
        if !status.is_empty() {
            let status = status.parse::<i64>().unwrap_or_default();
            qb.push(" AND t1.blocked_status = ");
            qb.push_bind(status);
        };
        qb.order("t2.username", order);
        qb.paginate(size, page);
        qb.build_query_as().fetch_all(pool).await
    }
}
