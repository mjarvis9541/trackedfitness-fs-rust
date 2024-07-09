use chrono::prelude::*;
use rust_decimal::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;

use super::model::{Profile, ProfileImage, ProfileQuery};

impl Profile {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self,
             "SELECT t1.* FROM user_profile t1 LEFT JOIN users_user t2 ON t2.id = t1.user_id WHERE t2.username = $1", username).fetch_optional(pool).await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        sex: &str,
        height: Decimal,
        date_of_birth: NaiveDate,
        activity_level: &str,
        fitness_goal: &str,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO user_profile (
                    user_id,
                    sex,
                    height,
                    date_of_birth,
                    activity_level,
                    fitness_goal,
                    created_by_id
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            ",
            user_id,
            sex,
            height,
            date_of_birth,
            activity_level,
            fitness_goal,
            request_user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        sex: &str,
        height: Decimal,
        date_of_birth: NaiveDate,
        activity_level: &str,
        fitness_goal: &str,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "UPDATE user_profile
            SET
                sex = $1,
                height = $2,
                date_of_birth = $3,
                activity_level = $4,
                fitness_goal = $5,
                updated_at = NOW(),
                updated_by_id = $6
            WHERE
                id = $7
            RETURNING *
            ",
            sex,
            height,
            date_of_birth,
            activity_level,
            fitness_goal,
            request_user_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM user_profile WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}

impl ProfileQuery {
    pub fn remove_sensitive_info(mut self) -> Self {
        self.date_of_birth = NaiveDate::default();
        self
    }

    pub async fn get_latest_by_username(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.*,
                t2.username as "username!",
                t3.weight_kg as "latest_weight?",
                t3.date as "latest_weight_date?"
            FROM
                user_profile t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN progress t3 ON t3.user_id = t1.user_id
                AND t3.date = (
                    SELECT
                        MAX(date)
                    FROM
                        progress
                    WHERE
                        user_id = t1.user_id
                        AND weight_kg IS NOT NULL
                        AND date <= $2
                )
            WHERE
                t2.username = $1
            "#,
            username,
            date
        )
        .fetch_optional(pool)
        .await?;

        Ok(query)
    }
}

impl ProfileImage {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<ProfileImage>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT 
                t1.user_id,
                t1.image_location
            FROM user_profile t1 
            LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE t2.username = $1
            ",
            username
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete_profile_image(
        pool: &PgPool,
        user_id: Uuid,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "UPDATE user_profile SET image_location = NULL, updated_at = NOW(), updated_by_id = $1 WHERE user_id = $2 RETURNING user_id, image_location", 
            user_id, request_user_id)
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update_profile_image(
        pool: &PgPool,
        user_id: Uuid,
        image_location: &str,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "
            UPDATE user_profile
            SET
                image_location = $1,
                updated_at = NOW(),
                updated_by_id = $2
            WHERE
                user_id = $3
            RETURNING
                user_id,
                image_location
            ",
            image_location,
            request_user_id,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}
