use chrono::prelude::*;
use rust_decimal::Decimal;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::meal_food::model::MealFoodModel;

use super::model::{Diet, DietDayQuery, DietFoodQuery, DietMealQuery};

impl Diet {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM food_log WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        date: NaiveDate,
        user_id: Uuid,
        meal_of_day_id: Uuid,
        food_id: Uuid,
        quantity: Decimal,
        created_by_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO
                food_log (
                    date,
                    user_id,
                    meal_of_day_id,
                    food_id,
                    quantity,
                    created_by_id
                )
            VALUES
                ($1, $2, $3, $4, $5, $6)
            RETURNING
                *
            ",
            date,
            user_id,
            meal_of_day_id,
            food_id,
            quantity,
            created_by_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        date: NaiveDate,
        meal_of_day_id: Uuid,
        quantity: Decimal,
        updated_by_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "UPDATE food_log
            SET
                date = $1,
                meal_of_day_id = $2,
                quantity = $3,
                updated_at = NOW(),
                updated_by_id = $4
            WHERE
                id = $5
            RETURNING *
            ",
            date,
            meal_of_day_id,
            quantity,
            updated_by_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM food_log WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }

    pub async fn all_by_ids(pool: &PgPool, ids: &[Uuid]) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM food_log WHERE id = ANY ($1)", ids)
            .fetch_all(pool)
            .await?;
        Ok(query)
    }

    pub async fn all_by_user_id_date(
        pool: &PgPool,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT * FROM food_log WHERE user_id = $1 AND date = $2",
            user_id,
            date
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }

    pub async fn all_by_user_id_date_meal(
        pool: &PgPool,
        user_id: Uuid,
        date: NaiveDate,
        meal: Uuid,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT * FROM food_log WHERE user_id = $1 AND date = $2 AND meal_of_day_id = $3",
            user_id,
            date,
            meal,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
    pub async fn bulk_create_from_previous_day(
        pool: &PgPool,
        user_id: Uuid,
        date: &NaiveDate,
        previous_day_diet_logs: &[Self],
        request_user_id: Uuid,
    ) -> Result<PgQueryResult> {
        let meal_of_day_id_list: Vec<Uuid> = previous_day_diet_logs
            .iter()
            .map(|diet| diet.meal_of_day_id)
            .collect();
        let food_id_list: Vec<Uuid> = previous_day_diet_logs
            .iter()
            .map(|diet| diet.food_id)
            .collect();
        let quantity_list: Vec<Decimal> = previous_day_diet_logs
            .iter()
            .map(|diet| diet.quantity)
            .collect();
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO
                food_log (user_id, date, meal_of_day_id, food_id, quantity, created_by_id)
            SELECT
                $1,
                $2,
                UNNEST($3::UUID[]),
                UNNEST($4::UUID[]),
                UNNEST($5::DECIMAL[]),
                $6
            "#,
            user_id,
            date,
            &meal_of_day_id_list,
            &food_id_list,
            &quantity_list,
            request_user_id
        )
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn bulk_create_from_previous_day_meal(
        pool: &PgPool,
        user_id: Uuid,
        date: NaiveDate,
        meal_of_day_id: Uuid,
        previous_day_diet_logs: &[Self],
        request_user_id: Uuid,
    ) -> Result<PgQueryResult> {
        let food_id_list: Vec<Uuid> = previous_day_diet_logs
            .iter()
            .map(|diet| diet.food_id)
            .collect();
        let quantity_list: Vec<Decimal> = previous_day_diet_logs
            .iter()
            .map(|diet| diet.quantity)
            .collect();
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO
                food_log (user_id, date, meal_of_day_id, food_id, quantity, created_by_id)
            SELECT
                $1,
                $2,
                $3,
                UNNEST($4::UUID[]),
                UNNEST($5::DECIMAL[]),
                $6
            "#,
            user_id,
            date,
            meal_of_day_id,
            &food_id_list,
            &quantity_list,
            request_user_id
        )
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn bulk_create_from_meal_food(
        pool: &PgPool,
        user_id: Uuid,
        date: NaiveDate,
        meal_of_day_id: Uuid,
        meal_food: &[MealFoodModel],
        request_user_id: Uuid,
    ) -> Result<PgQueryResult> {
        let food_id_list: Vec<Uuid> = meal_food
            .iter()
            .map(|meal_food| meal_food.food_id)
            .collect();
        let quantity_list: Vec<Decimal> = meal_food
            .iter()
            .map(|meal_food| meal_food.quantity)
            .collect();
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO
                food_log (user_id, date, meal_of_day_id, food_id, quantity, created_by_id)
            SELECT
                $1,
                $2,
                $3,
                UNNEST($4::UUID[]),
                UNNEST($5::DECIMAL[]),
                $6
            "#,
            user_id,
            date,
            meal_of_day_id,
            &food_id_list,
            &quantity_list,
            request_user_id
        )
        .execute(pool)
        .await?;
        Ok(query)
    }
}

impl DietFoodQuery {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/diet_food_total.sql", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn all_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/diet_food_total_list.sql", username, date)
            .fetch_all(pool)
            .await?;
        Ok(query)
    }
}

impl DietMealQuery {
    pub async fn all_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/diet_meal_total_list.sql", username, date)
            .fetch_all(pool)
            .await?;
        Ok(query)
    }
}

impl DietDayQuery {
    pub async fn get_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/diet_day_total.sql", username, date)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }
}