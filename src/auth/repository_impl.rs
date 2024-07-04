use rand::{thread_rng, Rng};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::auth::model::{User, UserRelation};
use crate::auth::privacy_level::PrivacyLevel;
use crate::component::select::SelectUuidName;
use crate::error::{handle_sqlx_contraint_error, Error, Result};
use crate::util::server::{normalize_whitespace, slugify};

use super::model::RequestUser;

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            email: row.try_get("email")?,
            email_verified: row.try_get("email_verified")?,
            is_active: row.try_get("is_active")?,
            is_staff: row.try_get("is_staff")?,
            is_superuser: row.try_get("is_superuser")?,
            privacy_level: PrivacyLevel::from(row.try_get::<i32, _>("privacy_level")?),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            last_login: row.try_get("last_login")?,
        })
    }
}

impl User {
    const BASE_NAME: &'static str = "User";

    pub fn remove_sensitive_info(mut self) -> Self {
        self.password = String::new();
        self.email = String::new();
        self
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM users_user WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM users_user WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT * FROM users_user WHERE username = $1",
            username
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        username: &str,
        password: &str,
        email: &str,
        email_verified: bool,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
        privacy_level: i32,
    ) -> Result<Self> {
        let name = normalize_whitespace(name);
        let hashed_password = bcrypt::hash(password, 8)?;
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO
                users_user (
                    name,
                    username,
                    password,
                    email,
                    email_verified,
                    is_active,
                    is_staff,
                    is_superuser,
                    privacy_level
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                *
            "#,
            name,
            username,
            hashed_password,
            email,
            email_verified,
            is_active,
            is_staff,
            is_superuser,
            privacy_level
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(
                err,
                Self::BASE_NAME,
                "email_key",
                &["email_key", "username_key"],
            )
        })?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: &str,
        username: &str,
        email: Option<String>,
        email_verified: Option<bool>,
        is_active: Option<bool>,
        is_staff: Option<bool>,
        is_superuser: Option<bool>,
        privacy_level: i32,
    ) -> Result<Self> {
        let name = normalize_whitespace(name);
        let username = slugify(username);
        sqlx::query_as!(
            Self,
            r#"
            UPDATE users_user
            SET
                name = $1,
                username = $2,
                email = $3,
                email_verified = $4,
                is_active = $5,
                is_staff = $6,
                is_superuser = $7,
                privacy_level = $8,
                updated_at = NOW()
            WHERE
                id = $9
            RETURNING
                *
            "#,
            name,
            username,
            email,
            email_verified,
            is_active,
            is_staff,
            is_superuser,
            privacy_level,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Error::from)
    }

    pub async fn update_email(pool: &PgPool, user_id: Uuid, email: &str) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE users_user
            SET email = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
            email,
            user_id,
        )
        .fetch_one(pool)
        .await
        .map_err(Error::from)
    }

    pub async fn update_password(pool: &PgPool, user_id: Uuid, new_password: &str) -> Result<Self> {
        let hashed_password = bcrypt::hash(new_password, 8)?;
        sqlx::query_as!(
            Self,
            r#"
            UPDATE users_user
            SET password = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
            hashed_password,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(Error::from)
    }

    pub async fn activate(pool: &PgPool, user_id: Uuid) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE users_user
            SET email_verified = true, is_active = true, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(Error::from)
    }

    pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool> {
        let query = sqlx::query!(
            r#"SELECT EXISTS (SELECT 1 FROM users_user WHERE username = $1) AS "exists!""#,
            username,
        )
        .fetch_one(pool)
        .await?;
        let query = query.exists;
        Ok(query)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        sqlx::query_as!(
            SelectUuidName,
            r#"SELECT id, username as name FROM users_user ORDER BY username LIMIT 1000"#
        )
        .fetch_all(pool)
        .await
        .map_err(Error::from)
    }

    pub fn generate_username_from_email(email: &str) -> Result<String> {
        let base_username = email.split('@').next().unwrap_or("user");
        let url_safe_username = slugify(&base_username);
        let mut rng = thread_rng();
        let random_number: u32 = rng.gen_range(1000..9999);
        let username = format!("{}{}", url_safe_username, random_number).to_lowercase();
        Ok(username)
    }

    pub async fn check_view_permission(
        pool: &PgPool,
        user: &RequestUser,
        username: &str,
    ) -> Result<()> {
        if user.is_superuser {
            return Ok(());
        }
        if user.username == username {
            return Ok(());
        }
        let can_view: bool = sqlx::query_scalar(
            "
                SELECT
                    EXISTS (
                        SELECT
                            1
                        FROM
                            users_user t1
                            LEFT JOIN user_block t2 ON t2.blocker_id = t1.id
                            AND t2.blocked_id = $1
                            AND t2.blocked_status = 1
                            LEFT JOIN user_follower t3 ON t3.user_id = t1.id
                            AND t3.follower_id = $1
                            AND t3.status = 1
                        WHERE
                            t1.username = $2
                            AND t2.id IS NULL
                            AND (
                                t3.id IS NOT NULL
                                AND t1.privacy_level = 2
                                OR t1.privacy_level = 1
                                OR t1.privacy_level = 0
                            )
                            AND t1.id <> $1
                    ) AS record_exists
            ",
        )
        .bind(user.id)
        .bind(username)
        .fetch_one(pool)
        .await?;
        if !can_view {
            return Err(Error::Forbidden);
        }
        Ok(())
    }

    pub async fn check_view_permission_by_user_id(
        pool: &PgPool,
        user: &RequestUser,
        target_user_id: Uuid,
    ) -> Result<()> {
        if user.is_superuser {
            return Ok(());
        }
        if user.id == target_user_id {
            return Ok(());
        }
        let can_view: bool = sqlx::query_scalar(
            "
                SELECT
                    EXISTS (
                        SELECT
                            1
                        FROM
                            users_user t1
                            LEFT JOIN user_block t2 ON t2.blocker_id = t1.id
                            AND t2.blocked_id = $1
                            AND t2.blocked_status = 1
                            LEFT JOIN user_follower t3 ON t3.user_id = t1.id
                            AND t3.follower_id = $1
                            AND t3.status = 1
                        WHERE
                            t1.id = $2
                            AND t2.id IS NULL
                            AND (
                                t3.id IS NOT NULL
                                AND t1.privacy_level = 2
                                OR t1.privacy_level = 1
                                OR t1.privacy_level = 0
                            )
                            AND t1.id <> $1
                    ) AS record_exists
            ",
        )
        .bind(user.id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await?;
        if !can_view {
            return Err(Error::Forbidden);
        }
        Ok(())
    }

    pub async fn admin_create(
        pool: &PgPool,
        name: &str,
        username: &str,
        password: &str,
        email: &str,
        email_verified: bool,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
        privacy_level: i32,
    ) -> Result<Self> {
        let hashed_password = bcrypt::hash(password, 8)?;
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO users_user (name, username, password, email, email_verified, is_active, is_staff, is_superuser, privacy_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            name,
            username,
            hashed_password,
            email,
            email_verified,
            is_active,
            is_staff,
            is_superuser,
            privacy_level
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn admin_update(
        pool: &PgPool,
        id: Uuid,
        name: &str,
        username: &str,
        email: &str,
        email_verified: bool,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
        privacy_level: i32,
    ) -> Result<Self> {
        let url_safe_username = slugify(username);
        let query = sqlx::query_as!(
            Self,
            "
            UPDATE users_user
            SET
                name = $1,
                username = $2,
                email = $3,
                email_verified = $4,
                is_active = $5,
                is_staff = $6,
                is_superuser = $7,
                privacy_level = $8,
                updated_at = NOW()
            WHERE
                id = $9
            RETURNING
                *
            ",
            name,
            url_safe_username,
            email,
            email_verified,
            is_active,
            is_staff,
            is_superuser,
            privacy_level,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}

impl UserRelation {
    pub async fn get_by_id(pool: &PgPool, user_id: Uuid, target_user_id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.id,
                t1.name,
                t1.username,
                t1.privacy_level AS privacy_level,
                t2.status AS follower_status,
                t3.blocked_status AS blocked_status
            FROM
                users_user t1
                LEFT JOIN user_follower t2 ON t2.user_id = t1.id AND t2.follower_id = $2
                LEFT JOIN user_block t3 ON t3.blocker_id = t1.id AND t3.blocked_id = $2
            WHERE
                t1.id = $1
            "#,
            user_id,
            target_user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}
