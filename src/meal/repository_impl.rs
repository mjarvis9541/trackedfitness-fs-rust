use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};

use uuid::Uuid;

use super::model::Meal;
use crate::component::select::SelectUuidName;
use crate::error::{handle_sqlx_contraint_error, Result};
use crate::food::model::Nutrition;
use crate::util::database::Filter;
use crate::util::server::normalize_whitespace;

use super::model::MealBase;

impl FromRow<'_, PgRow> for Meal {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by_id: row.try_get("created_by_id")?,
            updated_by_id: row.try_get("updated_by_id")?,
            food_count: row.try_get("food_count")?,
            username: row.try_get("username")?,
            nutrition: Nutrition::from_row(&row).unwrap_or_default(),
        })
    }
}

impl MealBase {
    const BASE_NAME: &'static str = "Meal";

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM meal WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        name: &str,
        created_by_id: Uuid,
    ) -> Result<Self> {
        let normalised_name = normalize_whitespace(&name);
        let query = sqlx::query_as!(
            Self,
            "
            INSERT INTO meal (user_id, name, created_by_id) 
            VALUES ($1, $2, $3)
            RETURNING *
            ",
            user_id,
            normalised_name,
            created_by_id
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(err, Self::BASE_NAME, "name", &["slug_key", "name_key"])
        })?;
        Ok(query)
    }
    pub async fn update(pool: &PgPool, id: Uuid, name: &str, updated_by_id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "UPDATE meal
            SET
                name = $1,
                updated_at = NOW(),
                updated_by_id = $2
            WHERE
                id = $3
            RETURNING *",
            name,
            updated_by_id,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM meal WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }
}
impl Meal {
    pub async fn get_by_id(pool: &PgPool, meal_id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as(
            "
            WITH cte_meal_food_total AS (
                SELECT
                    t1.meal_id,
                    COUNT(t2.*) AS food_count,
                    SUM(t1.quantity * t2.energy) AS energy,
                    SUM(t1.quantity * t2.protein) AS protein,
                    SUM(t1.quantity * t2.carbohydrate) AS carbohydrate,
                    SUM(t1.quantity * t2.fat) AS fat,
                    SUM(t1.quantity * t2.saturates) AS saturates,
                    SUM(t1.quantity * t2.sugars) AS sugars,
                    SUM(t1.quantity * t2.fibre) AS fibre,
                    SUM(t1.quantity * t2.salt) AS salt,
                    COALESCE(SUM(t1.quantity * t2.protein * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS protein_pct,
                    COALESCE(SUM(t1.quantity * t2.carbohydrate * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS carbohydrate_pct,
                    COALESCE(SUM(t1.quantity * t2.fat * 9) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS fat_pct
                FROM
                    meal_food t1
                    LEFT JOIN food t2 ON t2.id = t1.food_id
                GROUP BY
                    t1.meal_id
            )
            SELECT
                t1.*,
                COALESCE(t2.food_count, 0) as food_count,
                t2.energy,
                t2.protein,
                t2.carbohydrate,
                t2.fat,
                t2.saturates,
                t2.sugars,
                t2.fibre,
                t2.salt,
                t2.protein_pct,
                t2.carbohydrate_pct,
                t2.fat_pct,
                t3.username
            FROM
                meal t1
                LEFT JOIN cte_meal_food_total t2 ON t2.meal_id = t1.id
                LEFT JOIN users_user t3 ON t3.id = t1.user_id
            WHERE
                t1.id = $1
            ",
        )
        .bind(meal_id)
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, username: &str, search: &str) -> Result<i64> {
        let mut qb_count = sqlx::QueryBuilder::new(
            "
            SELECT COUNT(*)
            FROM meal t1
            LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE TRUE
            ",
        );
        qb_count.filter("t1.name", "ilike", search);
        qb_count.filter("t2.username", "=", username);
        let query = qb_count.build_query_scalar().fetch_one(pool).await?;
        Ok(query)
    }

    pub async fn filter(
        pool: &PgPool,
        username: &str,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
                "
                WITH cte_meal_total AS (
                    SELECT
                        t1.meal_id,
                        COUNT(t2.*) AS food_count,
                        SUM(t1.quantity * t2.energy) AS energy,
                        SUM(t1.quantity * t2.protein) AS protein,
                        SUM(t1.quantity * t2.carbohydrate) AS carbohydrate,
                        SUM(t1.quantity * t2.fat) AS fat,
                        SUM(t1.quantity * t2.saturates) AS saturates,
                        SUM(t1.quantity * t2.sugars) AS sugars,
                        SUM(t1.quantity * t2.fibre) AS fibre,
                        SUM(t1.quantity * t2.salt) AS salt,
                        COALESCE(SUM(t1.quantity * t2.protein * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS protein_pct,
                        COALESCE(SUM(t1.quantity * t2.carbohydrate * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS carbohydrate_pct,
                        COALESCE(SUM(t1.quantity * t2.fat * 9) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS fat_pct
                    FROM
                        meal_food t1
                        LEFT JOIN food t2 ON t2.id = t1.food_id
                    GROUP BY
                        t1.meal_id
                )
                SELECT
                    meal.id,
                    meal.user_id,
                    meal.name,
                    meal.created_at,
                    meal.updated_at,
                    meal.created_by_id,
                    meal.updated_by_id,
                    COALESCE(cte_meal_total.food_count, 0) as food_count,
                    cte_meal_total.energy,
                    cte_meal_total.protein,
                    cte_meal_total.carbohydrate,
                    cte_meal_total.fat,
                    cte_meal_total.saturates,
                    cte_meal_total.sugars,
                    cte_meal_total.fibre,
                    cte_meal_total.salt,
                    cte_meal_total.protein_pct,
                    cte_meal_total.carbohydrate_pct,
                    cte_meal_total.fat_pct,
                    users_user.username
                FROM
                    meal
                    LEFT JOIN cte_meal_total ON cte_meal_total.meal_id = meal.id
                    LEFT JOIN users_user ON users_user.id = meal.user_id
                WHERE
                    TRUE
                ",
            );
        qb.filter("users_user.username", "ilike", username);
        qb.filter("meal.name", "ilike", search);
        qb.order("meal.name", order);
        qb.paginate(size, page);
        let query = qb.build_query_as().fetch_all(pool).await?;
        Ok(query)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        let query = sqlx::query_as!(
            SelectUuidName,
            r#"
            SELECT t1.id, CONCAT(t1.name, ' (', COUNT(t2.*), ')') AS "name!"
            FROM meal t1 LEFT JOIN meal_food t2 ON t1.id = t2.meal_id
            GROUP BY t1.id ORDER BY t1.name LIMIT 1000
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}
