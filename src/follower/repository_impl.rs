use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Result;
use crate::follower::model::Follower;
use crate::follower::status::FollowerStatus;
use crate::util::database::Filter;

impl Follower {
    pub async fn pending_request_count(pool: &PgPool, user_id: Uuid) -> sqlx::Result<i64> {
        sqlx::query_scalar(
            r#"
                SELECT 
                    COUNT(*)
                FROM 
                    user_follower
                WHERE 
                    user_id = $1
                    AND status = 0
                "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.*,
                followed_user.username as "username!",
                follower_user.username as "follower!"
            FROM
                user_follower t1
                LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
            WHERE
                t1.id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        follower_id: Uuid,
        status: i32,
    ) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            INSERT INTO
                user_follower (user_id, follower_id, status)
            VALUES
                ($1, $2, $3)
            ",
        )
        .bind(user_id)
        .bind(follower_id)
        .bind(status)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn request(pool: &PgPool, username: &str, request_user: &str) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            INSERT INTO
            user_follower (user_id, follower_id, status)
            SELECT
                t1.id,
                t2.id,
                0
            FROM
                users_user t1,
                users_user t2
            WHERE
                t1.username = $1
                AND t2.username = $2
            ",
        )
        .bind(username)
        .bind(request_user)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn accept(pool: &PgPool, username: &str, request_user: &str) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            UPDATE user_follower
            SET
                status = 1,
                updated_at = NOW()
            FROM
                users_user t2,
                users_user t3
            WHERE
                user_follower.user_id = t2.id
                AND user_follower.follower_id = t3.id
                AND t2.username = $1
                AND t3.username = $2
            ",
        )
        .bind(request_user)
        .bind(username)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn remove(pool: &PgPool, username: &str, request_user: &str) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            DELETE FROM user_follower USING users_user t1,
            users_user t2
            WHERE
                user_follower.user_id = t1.id
                AND user_follower.follower_id = t2.id
                AND t1.username = $1
                AND t2.username = $2
            ",
        )
        .bind(request_user)
        .bind(username)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn unfollow(pool: &PgPool, username: &str, request_user: &str) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            DELETE FROM user_follower USING users_user t1,
            users_user t2
            WHERE
                user_follower.user_id = t1.id
                AND user_follower.follower_id = t2.id
                AND t1.username = $1
                AND t2.username = $2
            ",
        )
        .bind(username)
        .bind(request_user)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        follower_id: Uuid,
        status: i32,
    ) -> sqlx::Result<u64> {
        let query = sqlx::query(
            "
            UPDATE user_follower
            SET
                user_id = $1,
                follower_id = $2,
                status = $3,
                updated_at = NOW()
            WHERE
                id = $4
            ",
        )
        .bind(user_id)
        .bind(follower_id)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }

    pub async fn count(
        pool: &PgPool,
        username: &str,
        follower: &str,
        status: &str,
    ) -> sqlx::Result<i64> {
        let mut qbc = sqlx::QueryBuilder::new(
            "
            SELECT
                COUNT(*)
            FROM
                user_follower t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN users_user t3 ON t3.id = t1.follower_id
            WHERE
                TRUE
            ",
        );
        qbc.filter("t2.username", "=", username);
        qbc.filter("t3.username", "=", follower);
        if !status.is_empty() {
            let status = status.parse::<i64>().unwrap_or(0);
            qbc.push(" AND t1.status = ");
            qbc.push_bind(status);
        };
        let count = qbc.build_query_scalar().fetch_one(pool).await?;
        Ok(count)
    }

    pub async fn filter(
        pool: &PgPool,
        username: &str,
        follower: &str,
        status: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let order_by_column = match order {
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            "updated_at" => "t1.updated_at",
            "-updated_at" => "t1.updated_at DESC",
            _ => "t1.created_at",
        };

        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.*,
                t2.username as username,
                t3.username as follower
            FROM
                user_follower t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN users_user t3 ON t3.id = t1.follower_id
            WHERE
                TRUE
                ",
        );
        qb.filter("t2.username", "=", username);
        qb.filter("t3.username", "=", follower);
        if !status.is_empty() {
            let status = status.parse::<i64>().unwrap_or(0);
            qb.push(" AND t1.status = ");
            qb.push_bind(status);
        };

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);
        let results = qb.build_query_as().fetch_all(pool).await?;
        Ok(results)
    }

    pub async fn get_user_followers(
        pool: &PgPool,
        username: &str,
        search: &str,
        status: i32,
        order_by: &str,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let order_by_column = match order_by {
            "followed_username" => "followed_user.username",
            "-followed_username" => "followed_user.username DESC",
            "follower_username" => "follower_user.username",
            "-follower_username" => "follower_user.username DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            _ => "t1.created_at",
        };

        let query = if search.is_empty() {
            sqlx::query_as!(
                Self,
                r#"
                SELECT
                    t1.*,
                    followed_user.username as "username!",
                    follower_user.username as "follower!"
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    followed_user.username = $4
                    AND t1.status = $5
                ORDER BY
                    $1
                LIMIT
                    $2
                OFFSET
                    $3
                "#,
                order_by_column,
                page_size,
                offset,
                username,
                status,
            )
            .fetch_all(pool)
            .await?
        } else {
            let query_pattern = format!("%{}%", search);
            sqlx::query_as!(
                Self,
                r#"
                SELECT
                    t1.*,
                    followed_user.username as "username!",
                    follower_user.username as "follower!"
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    followed_user.username = $4
                    AND t1.status = $5
                    AND follower_user.username ILIKE $6
                ORDER BY
                    $1
                LIMIT
                    $2
                OFFSET
                    $3
                "#,
                order_by_column,
                page_size,
                offset,
                username,
                status,
                query_pattern,
            )
            .fetch_all(pool)
            .await?
        };

        Ok(query)
    }

    pub async fn get_user_follower_count(
        pool: &PgPool,
        username: &str,
        search: &str,
        status: i32,
    ) -> Result<i64> {
        let query = if search.is_empty() {
            sqlx::query_scalar(
                r#"
                SELECT
                    COUNT(t1.*)
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    followed_user.username = $1
                    AND t1.status = $2
                "#,
            )
            .bind(username)
            .bind(status)
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_scalar(
                r#"
                SELECT
                    COUNT(t1.*)
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    followed_user.username = $1
                    AND t1.status = $2
                    AND follower_user.username = $3
                "#,
            )
            .bind(username)
            .bind(status)
            .bind(search)
            .fetch_one(pool)
            .await?
        };
        Ok(query)
    }

    pub async fn get_user_following(
        pool: &PgPool,
        username: &str,
        search: &str,
        status: i32,
        order_by: &str,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let order_by_column = match order_by {
            "followed_username" => "followed_user.username",
            "-followed_username" => "followed_user.username DESC",
            "follower_username" => "follower_user.username",
            "-follower_username" => "follower_user.username DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            _ => "t1.created_at",
        };

        let query = if search.is_empty() {
            sqlx::query_as!(
                Self,
                r#"
                SELECT
                    t1.*,
                    followed_user.username as "username!",
                    follower_user.username as "follower!"
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    follower_user.username = $4
                    AND t1.status = $5
                ORDER BY
                    $1
                LIMIT
                    $2
                OFFSET
                    $3
                "#,
                order_by_column,
                page_size,
                offset,
                username,
                status,
            )
            .fetch_all(pool)
            .await?
        } else {
            let query_pattern = format!("%{}%", search);
            sqlx::query_as!(
                Self,
                r#"
                SELECT
                    t1.*,
                    followed_user.username as "username!",
                    follower_user.username as "follower!"
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    follower_user.username = $4
                    AND t1.status = $5
                    AND followed_user.username ILIKE $6
                ORDER BY
                    $1
                LIMIT
                    $2
                OFFSET
                    $3
                "#,
                order_by_column,
                page_size,
                offset,
                username,
                status,
                query_pattern
            )
            .fetch_all(pool)
            .await?
        };

        Ok(query)
    }

    pub async fn get_user_following_count(
        pool: &PgPool,
        username: &str,
        search: &str,
        status: i32,
    ) -> Result<i64> {
        let query = if search.is_empty() {
            sqlx::query_scalar(
                r#"
                SELECT
                    COUNT(t1.*)
                FROM
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE
                    follower_user.username = $1
                    AND t1.status = $2
                "#,
            )
            .bind(username)
            .bind(status)
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_scalar(
                r#"
                SELECT 
                    COUNT(t1.*)
                FROM 
                    user_follower t1
                    LEFT JOIN users_user followed_user ON followed_user.id = t1.user_id
                    LEFT JOIN users_user follower_user ON follower_user.id = t1.follower_id
                WHERE 
                    follower_user.username = $1 
                    AND t1.status = $2
                    AND followed_user.username = $3
                "#,
            )
            .bind(username)
            .bind(status)
            .bind(search)
            .fetch_one(pool)
            .await?
        };
        Ok(query)
    }
}

impl FromRow<'_, PgRow> for Follower {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            follower_id: row.try_get("follower_id")?,
            status: FollowerStatus::from(row.try_get::<i32, _>("status")?),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            username: row.try_get("username")?,
            follower: row.try_get("follower")?,
        })
    }
}
