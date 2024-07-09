use chrono::NaiveDate;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};

use crate::error::Result;
use crate::util::datetime::{get_month_end_comprehensive, get_month_start_comprehensive};

use super::model::{MonthSummary, UserDaySummary};

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

impl MonthSummary {
    pub async fn get(pool: &PgPool, username: &str, date: NaiveDate) -> sqlx::Result<Vec<Self>> {
        let start = get_month_start_comprehensive(date);
        let end = get_month_end_comprehensive(date);
        let rows = sqlx::query(
        "
        WITH
            month_series AS (
                SELECT 
                generate_series($2::TIMESTAMP, $3::TIMESTAMP, interval '1 day')::DATE AS date
            ),
            month_series_diet as (
                SELECT
                    t1.date,
                    t1.user_id,
                    SUM(t1.quantity * t2.energy) AS energy,
                    SUM(t1.quantity * t2.protein) AS protein,
                    SUM(t1.quantity * t2.carbohydrate) AS carbohydrate,
                    SUM(t1.quantity * t2.fat) AS fat
                FROM
                    food_log t1
                    LEFT JOIN food t2 ON t2.id = t1.food_id
                GROUP BY
                    t1.user_id,
                    t1.date
            ),
            month_series_avg_diet as (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date) AS date,
                    AVG(t1.energy) AS week_avg_energy,
                    AVG(t1.protein) AS week_avg_protein,
                    AVG(t1.carbohydrate) AS week_avg_carbohydrate,
                    AVG(t1.fat) AS week_avg_fat
                FROM
                    month_series_diet t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)
            ),
            month_series_progress as (
                SELECT
                    t1.date,
                    t1.user_id,
                    t1.weight_kg as weight,
                    t1.energy_burnt
                FROM
                    progress t1
            ),
            month_series_progress_week_avg as (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date) as date,
                    AVG(t1.weight_kg) AS week_avg_weight,
                    AVG(t1.energy_burnt)::INT8 AS week_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)
            ),
            month_series_workout as (
                SELECT
                    t1.date,
                    t1.user_id,
                    COUNT(DISTINCT t1.id) as workout_count,
                    COUNT(DISTINCT t2.id) as exercise_count,
                    COUNT(t3.id) as set_count,
                    COALESCE(SUM(t3.reps), 0) as rep_count
                FROM
                    workout t1
                    LEFT JOIN exercise t2 on t2.workout_id = t1.id
                    LEFT JOIN tracked_set t3 ON t3.exercise_id = t2.id
                GROUP BY
                    t1.date,
                    t1.user_id
            ),
            month_series_total_workout as (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date) AS date,
                    SUM(t1.workout_count)::INT8 AS week_total_workouts,
                    SUM(t1.exercise_count)::INT8 AS week_total_exercises,
                    SUM(t1.set_count)::INT8 AS week_total_sets,
                    SUM(t1.rep_count)::INT8 AS week_total_reps
                FROM
                    month_series_workout t1
                GROUP BY
                    DATE_TRUNC('week', t1.date),
                    t1.user_id
            )
        SELECT
            t1.*,
            t2.date,
            t3.workout_count,
            t3.exercise_count,
            t3.set_count,
            t3.rep_count,
            -- workout week total
            t3a.week_total_workouts,
            t3a.week_total_exercises,
            t3a.week_total_sets,
            t3a.week_total_reps,
            -- diet
            t4.energy,
            t4.protein,
            t4.carbohydrate,
            t4.fat,
            -- diet avg
            t4a.week_avg_energy,
            t4a.week_avg_protein,
            t4a.week_avg_carbohydrate,
            t4a.week_avg_fat,
            -- progress
            t5.date as progress_date,
            t5.weight,
            t5.energy_burnt,
            -- progress week avg
            t5a.week_avg_weight,
            t5a.week_avg_energy_burnt
        FROM
            users_user t1
            CROSS JOIN month_series t2
            LEFT JOIN month_series_workout t3 ON t3.user_id = t1.id AND t3.date = t2.date
            LEFT JOIN month_series_total_workout t3a ON t3a.user_id = t1.id AND t3a.date = DATE_TRUNC('week', t2.date)
            LEFT JOIN month_series_diet t4 ON t4.user_id = t1.id AND t4.date = t2.date
            LEFT JOIN month_series_avg_diet t4a ON t4a.user_id = t1.id AND t4a.date = DATE_TRUNC('week', t2.date)
            LEFT JOIN month_series_progress t5 ON t5.user_id = t1.id AND t5.date = t2.date
            LEFT JOIN month_series_progress_week_avg t5a ON t5a.user_id = t1.id AND t5a.date = DATE_TRUNC('week', t2.date)
        WHERE
            t1.username = $1
        ",
        )
        .bind(username)
        .bind(start)
        .bind(end)
        .fetch_all(pool).await?;

        let mut stream = Vec::new();
        for row in rows {
            stream.push(MonthSummary::from_row(&row).unwrap());
        }

        Ok(stream)
    }
}

impl FromRow<'_, PgRow> for MonthSummary {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            username: row.try_get("username")?,
            date: row.try_get("date")?,
            energy: row.try_get("energy").unwrap_or_default(),
            protein: row.try_get("protein").unwrap_or_default(),
            fat: row.try_get("fat").unwrap_or_default(),
            carbohydrate: row.try_get("carbohydrate").unwrap_or_default(),
            week_avg_energy: row.try_get("week_avg_energy").unwrap_or_default(),
            week_avg_protein: row.try_get("week_avg_protein").unwrap_or_default(),
            week_avg_carbohydrate: row.try_get("week_avg_carbohydrate").unwrap_or_default(),
            week_avg_fat: row.try_get("week_avg_fat").unwrap_or_default(),
            workout_count: row.try_get("workout_count").unwrap_or_default(),
            exercise_count: row.try_get("exercise_count").unwrap_or_default(),
            set_count: row.try_get("set_count").unwrap_or_default(),
            rep_count: row.try_get("rep_count").unwrap_or_default(),
            week_total_workouts: row.try_get("week_total_workouts").unwrap_or_default(),
            week_total_exercises: row.try_get("week_total_exercises").unwrap_or_default(),
            week_total_sets: row.try_get("week_total_sets").unwrap_or_default(),
            week_total_reps: row.try_get("week_total_reps").unwrap_or_default(),
            progress_date: row.try_get("progress_date")?,
            weight: row.try_get("weight")?,
            energy_burnt: row.try_get("energy_burnt")?,
            week_avg_weight: row.try_get("week_avg_weight").unwrap_or_default(),
            week_avg_energy_burnt: row.try_get("week_avg_energy_burnt").unwrap_or_default(),
        })
    }
}
