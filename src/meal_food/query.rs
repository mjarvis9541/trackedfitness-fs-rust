use rust_decimal::Decimal;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::diet::model::Diet;
use crate::error::{Error, Result};
use crate::food::model::Nutrition;

use super::model::{MealFood, MealFoodModel};

impl FromRow<'_, PgRow> for MealFoodModel {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            food_id: row.try_get("food_id")?,
            quantity: row.try_get("quantity")?,
        })
    }
}

impl FromRow<'_, PgRow> for MealFood {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            meal_id: row.try_get("meal_id")?,
            food_id: row.try_get("food_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by_id: row.try_get("created_by_id")?,
            updated_by_id: row.try_get("updated_by_id")?,
            food_name: row.try_get("food_name")?,
            food_slug: row.try_get("food_slug")?,
            brand_name: row.try_get("brand_name")?,
            brand_slug: row.try_get("brand_slug")?,
            data_value: row.try_get("data_value")?,
            data_measurement: row.try_get("data_measurement")?,
            nutrition: Nutrition::from_row(row).unwrap_or_default(),
        })
    }
}

impl MealFood {
    pub async fn try_get(pool: &PgPool, meal_food_id: Uuid) -> Result<Option<Self>> {
        Ok(sqlx::query_as(
            "
            SELECT
                t1.id,
                t1.meal_id,
                t1.food_id,
                t1.created_at,
                t1.updated_at,
                t1.created_by_id,
                t1.updated_by_id,
                t2.name AS food_name,
                t2.slug AS food_slug,
                t3.name AS brand_name,
                t3.slug AS brand_slug,
                t1.quantity,
                t1.quantity * t2.data_value AS data_value,
                t2.data_measurement,
                t1.quantity * t2.energy AS energy,
                t1.quantity * t2.protein AS protein,
                t1.quantity * t2.carbohydrate AS carbohydrate,
                t1.quantity * t2.fat AS fat,
                t1.quantity * t2.saturates AS saturates,
                t1.quantity * t2.sugars AS sugars,
                t1.quantity * t2.fibre AS fibre,
                t1.quantity * t2.salt AS salt,
                COALESCE(t1.quantity * t2.protein * 4 / NULLIF(t2.energy, 0) * 100, 0) AS protein_pct,
                COALESCE(t1.quantity * t2.carbohydrate * 4 / NULLIF(t2.energy, 0) * 100, 0) AS carbohydrate_pct,
                COALESCE(t1.quantity * t2.fat * 9 / NULLIF(t2.energy, 0) * 100, 0) AS fat_pct
            FROM
                meal_food t1
                LEFT JOIN food t2 ON t1.food_id = t2.id
                LEFT JOIN food_brand t3 ON t2.brand_id = t3.id
            WHERE
                t1.id = $1
            ",
        )
        .bind(meal_food_id)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn get_object_or_404(pool: &PgPool, id: Uuid) -> Result<Self> {
        Self::try_get(pool, id)
            .await?
            .ok_or_else(|| Error::NotFound)
    }

    pub async fn create_and_return_meal_id(
        pool: &PgPool,
        meal_id: Uuid,
        food_id: Uuid,
        quantity: Decimal,
        created_by_id: Uuid,
    ) -> Result<Uuid> {
        let query = sqlx::query_scalar!(
            r#"
            INSERT INTO
                meal_food (
                    meal_id,
                    food_id,
                    quantity,
                    created_by_id
                )
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                meal_id
            "#,
            meal_id,
            food_id,
            quantity,
            created_by_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update_and_return_meal_id(
        pool: &PgPool,
        meal_food_id: Uuid,
        quantity: Decimal,
        updated_by_id: Uuid,
    ) -> Result<Uuid> {
        let query = sqlx::query_scalar!(
            r#"
            UPDATE
                meal_food
            SET
                quantity = $1,
                updated_at = NOW(),
                updated_by_id = $2
            WHERE
                id = $3
            RETURNING
                meal_id
            "#,
            quantity,
            updated_by_id,
            meal_food_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64> {
        Ok(sqlx::query!("DELETE FROM meal_food WHERE id = $1", id)
            .execute(pool)
            .await?
            .rows_affected())
    }

    pub async fn all_by_meal_id(pool: &PgPool, meal_id: Uuid) -> Result<Vec<Self>> {
        let query = sqlx::query_as(
                "
                    SELECT
                    t1.id,
                    -- t4.user_id,
                    t1.meal_id,
                    t1.food_id,
                    t1.created_at,
                    t1.updated_at,
                    t1.created_by_id,
                    t1.updated_by_id,
                    -- food
                    t2.name AS food_name,
                    t2.slug AS food_slug,
                    -- brand
                    t3.name AS brand_name,
                    t3.slug AS brand_slug,
                    -- food
                    t1.quantity,
                    t1.quantity * t2.data_value AS data_value,
                    t2.data_measurement,
                    t1.quantity * t2.energy AS energy,
                    t1.quantity * t2.protein AS protein,
                    t1.quantity * t2.carbohydrate AS carbohydrate,
                    t1.quantity * t2.fat AS fat,
                    t1.quantity * t2.saturates AS saturates,
                    t1.quantity * t2.sugars AS sugars,
                    t1.quantity * t2.fibre AS fibre,
                    t1.quantity * t2.salt AS salt,
                    COALESCE(t1.quantity * t2.protein * 4 / NULLIF(t2.energy, 0) * 100, 0) AS protein_pct,
                    COALESCE(t1.quantity * t2.carbohydrate * 4 / NULLIF(t2.energy, 0) * 100, 0) AS carbohydrate_pct,
                    COALESCE(t1.quantity * t2.fat * 9 / NULLIF(t2.energy, 0) * 100, 0) AS fat_pct
                FROM
                    meal_food t1
                    LEFT JOIN food t2 ON t1.food_id = t2.id
                    LEFT JOIN food_brand t3 ON t2.brand_id = t3.id
                WHERE
                    t1.meal_id = $1
                LIMIT 100
                ",
            ).bind(meal_id).fetch_all(pool).await?;
        Ok(query)
    }
}

impl MealFoodModel {
    pub async fn all_by_meal_id(pool: &PgPool, meal_id: Uuid) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM meal_food WHERE meal_id = $1")
            .bind(meal_id)
            .fetch_all(pool)
            .await?)
    }

    pub async fn bulk_create_from_diet(
        pool: &PgPool,
        meal_id: Uuid,
        diet_items: &[Diet],
        request_user_id: Uuid,
    ) -> sqlx::Result<u64> {
        let food_id_list: Vec<Uuid> = diet_items.iter().map(|obj| obj.food_id).collect();
        let quantity_list: Vec<Decimal> = diet_items.iter().map(|obj| obj.quantity).collect();
        Ok(sqlx::query(
            r#"
            INSERT INTO
                meal_food (meal_id, food_id, quantity, created_by_id)
            SELECT
                $1,
                UNNEST($2::UUID[]),
                UNNEST($3::DECIMAL[]),
                $4
            "#,
        )
        .bind(meal_id)
        .bind(&food_id_list)
        .bind(&quantity_list)
        .bind(request_user_id)
        .execute(pool)
        .await?
        .rows_affected())
    }
}
