use chrono::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::summary::model::UserDaySummary;
use crate::util::database::Filter;

use super::model::{DietTarget, DietTargetInput, DietTargetQuery};

impl DietTarget {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM diet_target WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT t1.* FROM diet_target t1 
            LEFT JOIN users_user t2 ON t2.id = t1.user_id 
            WHERE t2.username = $1 AND t1.date = $2",
            username,
            date
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        data: DietTargetInput,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO diet_target (
                user_id,
                date,
                weight,
                energy,
                fat,
                saturates,
                carbohydrate,
                sugars,
                fibre,
                protein,
                salt,
                created_by_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING * 
            ",
            data.user_id,
            data.date,
            data.weight,
            data.energy,
            data.fat,
            data.saturates,
            data.carbohydrate,
            data.sugars,
            data.fibre,
            data.protein,
            data.salt,
            request_user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        data: DietTargetInput,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            DietTarget,
            "
            UPDATE diet_target
            SET
                weight = $1,
                energy = $2,
                fat = $3,
                saturates = $4,
                carbohydrate = $5,
                sugars = $6,
                fibre = $7,
                protein = $8,
                salt = $9,
                updated_at = NOW(),
                updated_by_id = $10
            WHERE id = $11
            RETURNING *
            ",
            data.weight,
            data.energy,
            data.fat,
            data.saturates,
            data.carbohydrate,
            data.sugars,
            data.fibre,
            data.protein,
            data.salt,
            request_user_id,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM diet_target WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn bulk_create_update(
        pool: &PgPool,
        data: DietTargetInput,
        date_list: &[NaiveDate],
        request_user_id: Uuid,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO diet_target (
                user_id,
                date,
                weight,
                energy,
                fat,
                saturates,
                carbohydrate,
                sugars,
                fibre,
                protein,
                salt,
                created_by_id
            )
            SELECT
                $1::UUID,
                unnest($2::DATE[]),
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12
            ON CONFLICT (user_id, date)
            DO UPDATE SET
                weight = EXCLUDED.weight,
                energy = EXCLUDED.energy,
                fat = EXCLUDED.fat,
                saturates = EXCLUDED.saturates,
                carbohydrate = EXCLUDED.carbohydrate,
                sugars = EXCLUDED.sugars,
                fibre = EXCLUDED.fibre,
                protein = EXCLUDED.protein,
                salt = EXCLUDED.salt,
                updated_by_id = EXCLUDED.created_by_id,
                updated_at = NOW()
            RETURNING *
            "#,
            data.user_id,
            date_list,
            data.weight,
            data.energy,
            data.fat,
            data.saturates,
            data.carbohydrate,
            data.sugars,
            data.fibre,
            data.protein,
            data.salt,
            request_user_id,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}

impl DietTargetQuery {
    pub async fn get_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.id,
                t1.user_id,
                t1.date,
                t1.weight,
                t1.energy,
                t1.fat,
                t1.saturates,
                t1.carbohydrate,
                t1.sugars,
                t1.fibre,
                t1.protein,
                t1.salt,
                t1.created_at,
                t1.updated_at,
                t1.created_by_id,
                t1.updated_by_id,
                t2.username,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "protein_pct!",
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "carbohydrate_pct!",
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS "fat_pct!",
                COALESCE(t1.energy / NULLIF(t1.weight, 0), 0) AS "energy_per_kg!",
                COALESCE(t1.protein / NULLIF(t1.weight, 0), 0) AS "protein_per_kg!",
                COALESCE(t1.carbohydrate / NULLIF(t1.weight, 0), 0) AS "carbohydrate_per_kg!",
                COALESCE(t1.fat / NULLIF(t1.weight, 0), 0) AS "fat_per_kg!"
            FROM
                diet_target t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE
                t2.username = $1
                AND t1.date = $2
            LIMIT 1
            "#,
            username,
            date,
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn get_latest_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.id,
                t1.user_id,
                t1.date,
                t1.weight,
                t1.energy,
                t1.fat,
                t1.saturates,
                t1.carbohydrate,
                t1.sugars,
                t1.fibre,
                t1.protein,
                t1.salt,
                t1.created_at,
                t1.updated_at,
                t1.created_by_id,
                t1.updated_by_id,
                t2.username,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "protein_pct!",
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "carbohydrate_pct!",
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS "fat_pct!",
                COALESCE(t1.energy / NULLIF(t1.weight, 0), 0) AS "energy_per_kg!",
                COALESCE(t1.protein / NULLIF(t1.weight, 0), 0) AS "protein_per_kg!",
                COALESCE(t1.carbohydrate / NULLIF(t1.weight, 0), 0) AS "carbohydrate_per_kg!",
                COALESCE(t1.fat / NULLIF(t1.weight, 0), 0) AS "fat_per_kg!"
            FROM
                diet_target t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE
                t2.username = $1
                AND t1.date <= $2
            ORDER BY
                t1.date DESC
            LIMIT 1
            "#,
            username,
            date,
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count_by_username(
        pool: &PgPool,
        username: &str,
        search: &str,
    ) -> sqlx::Result<i64> {
        let _search = search;
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                COUNT(t1.*)
            FROM
                diet_target t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE
                t2.username =
            ",
        );
        qb.push_bind(username);

        qb.build_query_scalar().fetch_one(pool).await
    }

    pub async fn filter_by_username(
        pool: &PgPool,
        username: &str,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<UserDaySummary>> {
        let order_by_column = match order {
            "date" => "t1.date",
            "-date" => "t1.date DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            "updated_at" => "t1.updated_at",
            "-updated_at" => "t1.updated_at DESC",
            _ => "t1.date DESC",
        };
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.*,
                t1.energy::DECIMAL as energy,
                t2.username,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS protein_pct,
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS carbohydrate_pct,
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS fat_pct,
                COALESCE(t1.energy / NULLIF(t1.weight, 0), 0) AS energy_per_kg,
                COALESCE(t1.protein / NULLIF(t1.weight, 0), 0) AS protein_per_kg,
                COALESCE(t1.carbohydrate / NULLIF(t1.weight, 0), 0) AS carbohydrate_per_kg,
                COALESCE(t1.fat / NULLIF(t1.weight, 0), 0) AS fat_per_kg,
                true as actual
            FROM
                diet_target t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE
                t2.username = 
            ",
        );
        qb.push_bind(username);

        if !search.is_empty() && search.len() == 4 {
            let search = search.parse::<i32>().unwrap_or_default();
            qb.push(" AND EXTRACT(YEAR FROM t1.date) = ");
            qb.push_bind(search);
        }

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);
        let query = qb.build_query_as().fetch_all(pool).await?;
        Ok(query)
    }
}
