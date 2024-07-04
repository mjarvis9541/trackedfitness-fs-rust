use chrono::NaiveDate;
use sqlx::PgPool;

use crate::error::Result;

use super::model::UserDaySummary;

impl UserDaySummary {
    pub async fn get_target_range(
        pool: &PgPool,
        username: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<UserDaySummary>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.user_id,
                t1.date,
                t1.weight,
                t1.energy::DECIMAL as "energy!",
                t1.fat,
                t1.saturates,
                t1.carbohydrate,
                t1.sugars,
                t1.fibre,
                t1.protein,
                t1.salt,
                t2.username,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "protein_pct!",
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "carbohydrate_pct!",
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS "fat_pct!",
                COALESCE(t1.energy / NULLIF(t1.weight, 0), 0) AS "energy_per_kg!",
                COALESCE(t1.protein / NULLIF(t1.weight, 0), 0) AS "protein_per_kg!",
                COALESCE(t1.carbohydrate / NULLIF(t1.weight, 0), 0) AS "carbohydrate_per_kg!",
                COALESCE(t1.fat / NULLIF(t1.weight, 0), 0) AS "fat_per_kg!",
                true as "actual!"
            FROM
                diet_target t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
            WHERE
                t2.username = $1
                AND t1.date >= $2
                AND t1.date <= $3
            ORDER BY
                t1.date
            "#,
            username,
            start,
            end,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }

    pub async fn get_target_latest(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<UserDaySummary>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.user_id,
                t1.date,
                t1.weight,
                t1.energy::DECIMAL as "energy!",
                t1.fat,
                t1.saturates,
                t1.carbohydrate,
                t1.sugars,
                t1.fibre,
                t1.protein,
                t1.salt,
                t2.username,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "protein_pct!",
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS "carbohydrate_pct!",
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS "fat_pct!",
                COALESCE(t1.energy / NULLIF(t1.weight, 0), 0) AS "energy_per_kg!",
                COALESCE(t1.protein / NULLIF(t1.weight, 0), 0) AS "protein_per_kg!",
                COALESCE(t1.carbohydrate / NULLIF(t1.weight, 0), 0) AS "carbohydrate_per_kg!",
                COALESCE(t1.fat / NULLIF(t1.weight, 0), 0) AS "fat_per_kg!",
                true as "actual!"
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

    pub async fn get_diet_range(
        pool: &PgPool,
        username: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<UserDaySummary>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.user_id,
                t1.date,
                t4.username,
                t3.weight_kg as "weight!",
                SUM(t1.quantity * t2.energy) AS "energy!",
                SUM(t1.quantity * t2.protein) AS "protein!",
                SUM(t1.quantity * t2.carbohydrate) AS "carbohydrate!",
                SUM(t1.quantity * t2.fat) AS "fat!",
                SUM(t1.quantity * t2.saturates) AS "saturates!",
                SUM(t1.quantity * t2.sugars) AS "sugars!",
                SUM(t1.quantity * t2.fibre) AS "fibre!",
                SUM(t1.quantity * t2.salt) AS "salt!",
                COALESCE(SUM(t1.quantity * t2.protein * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS "protein_pct!",
                COALESCE(SUM(t1.quantity * t2.carbohydrate * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS "carbohydrate_pct!",
                COALESCE(SUM(t1.quantity * t2.fat * 9) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS "fat_pct!",
                COALESCE(SUM(t1.quantity * t2.energy) / NULLIF(t3.weight_kg, 0), 0) AS "energy_per_kg!",
                COALESCE(SUM(t1.quantity * t2.protein) / NULLIF(t3.weight_kg, 0), 0) AS "protein_per_kg!",
                COALESCE(SUM(t1.quantity * t2.carbohydrate) / NULLIF(t3.weight_kg, 0), 0) AS "carbohydrate_per_kg!",
                COALESCE(SUM(t1.quantity * t2.fat) / NULLIF(t3.weight_kg, 0), 0) AS "fat_per_kg!",
                true as "actual!"
            FROM
                food_log t1
                LEFT JOIN food t2 ON t2.id = t1.food_id
                LEFT JOIN progress t3 ON t3.user_id = t1.user_id
                AND t3.date = (
                    SELECT
                        MAX(date)
                    FROM
                        progress
                    WHERE
                        user_id = t1.user_id
                        AND date <= t1.date
                        AND weight_kg IS NOT NULL
                )
                LEFT JOIN users_user t4 ON t4.id = t1.user_id
            WHERE
                t4.username = $1
                AND t1.date >= $2
                AND t1.date <= $3
            GROUP BY
                t1.user_id,
                t1.date,
                t3.date,
                t3.weight_kg,
                t4.id
            ORDER BY
                t1.date;
            "#,
            username,
            start,
            end,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}
