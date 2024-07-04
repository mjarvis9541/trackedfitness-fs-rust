use chrono::prelude::*;
use rust_decimal::Decimal;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Result;
use crate::util::database::Filter;

use super::model::{Progress, ProgressBase};

impl FromRow<'_, PgRow> for Progress {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            date: row.try_get("date")?,
            weight: row.try_get("weight_kg")?,
            week_avg_weight: row.try_get("week_avg_weight")?,
            month_avg_weight: row.try_get("month_avg_weight")?,
            energy_burnt: row.try_get("energy_burnt")?,
            week_avg_energy_burnt: row.try_get("week_avg_energy_burnt")?,
            month_avg_energy_burnt: row.try_get("month_avg_energy_burnt")?,
            notes: row.try_get("notes")?,
            username: row.try_get("username")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl ProgressBase {
    pub async fn get_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT t1.* FROM progress t1
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
        user_id: Uuid,
        date: NaiveDate,
        weight: Option<Decimal>,
        energy_burnt: Option<i32>,
        notes: Option<String>,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO progress (user_id, date, weight_kg, energy_burnt, notes, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
            ",
            user_id,
            date,
            weight,
            energy_burnt,
            notes,
            request_user_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        date: NaiveDate,
        weight: Option<Decimal>,
        energy_burnt: Option<i32>,
        notes: Option<String>,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "UPDATE progress
            SET
                date = $1,
                weight_kg = $2,
                energy_burnt = $3,
                notes = $4,
                updated_at = NOW(),
                updated_by_id = $5
            WHERE
                id = $6
            RETURNING *
            ",
            date,
            weight,
            energy_burnt,
            notes,
            request_user_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM progress WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }
}

impl Progress {
    pub async fn get_latest_weight(pool: &PgPool, user_id: Uuid) -> Result<Option<Decimal>> {
        let query = sqlx::query_scalar!(
            "SELECT weight_kg FROM progress WHERE user_id = $1 ORDER BY date DESC LIMIT 1",
            user_id
        )
        .fetch_one(pool)
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
            WITH
            week_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)::date as date,
                    AVG(t1.weight_kg) AS week_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS week_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)
            ), 
            month_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)::date as date,
                    AVG(t1.weight_kg) AS month_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS month_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)
            )
            SELECT
                t1.id,
                t1.user_id,
                t1.date,
                t1.weight_kg as weight,
                t1.energy_burnt,
                t1.notes,
                t3.week_avg_weight,
                t3.week_avg_energy_burnt,
                t4.month_avg_weight,
                t4.month_avg_energy_burnt,
                t2.username,
                t1.created_at,
                t1.updated_at
            FROM 
                progress t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN week_avg t3 ON t3.user_id = t1.user_id
                AND t3.date = DATE_TRUNC('week', t1.date)::date
                LEFT JOIN month_avg t4 ON t4.user_id = t1.user_id
                AND t4.date = DATE_TRUNC('month', t1.date)::date
            WHERE
                t2.username = $1
                AND t1.date <= $2
            ORDER BY
                t1.date DESC
            LIMIT 1
            "#,
            username,
            date
        )
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
            r#"
            WITH
            week_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)::date as date,
                    AVG(t1.weight_kg) AS week_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS week_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)
            ), 
            month_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)::date as date,
                    AVG(t1.weight_kg) AS month_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS month_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)
            )
            SELECT
                t1.id,
                t1.user_id,
                t1.date,
                t1.weight_kg as weight,
                t1.energy_burnt,
                t1.notes,
                t3.week_avg_weight,
                t3.week_avg_energy_burnt,
                t4.month_avg_weight,
                t4.month_avg_energy_burnt,
                t2.username,
                t1.created_at,
                t1.updated_at
            FROM 
                progress t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN week_avg t3 ON t3.user_id = t1.user_id
                AND t3.date = DATE_TRUNC('week', t1.date)::date
                LEFT JOIN month_avg t4 ON t4.user_id = t1.user_id
                AND t4.date = DATE_TRUNC('month', t1.date)::date
            WHERE
                t2.username = $1
                AND t1.date = $2
            LIMIT 1
            "#,
            username,
            date
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str, username: &str) -> sqlx::Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT COUNT(*) 
            FROM progress t1 
            LEFT JOIN users_user t2 ON t2.id = t1.user_id 
            WHERE t2.username = 
            ",
        );
        qb.push_bind(username);
        if !search.is_empty() && search.len() == 4 {
            let search = search.parse::<i32>().unwrap_or_default();
            qb.push(" AND EXTRACT(YEAR FROM t1.date) = ");
            qb.push_bind(search);
        }
        qb.build_query_scalar().fetch_one(pool).await
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        username: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            WITH
            week_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)::date as date,
                    AVG(t1.weight_kg) AS week_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS week_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('week', t1.date)
            ), 
            month_avg AS (
                SELECT
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)::date as date,
                    AVG(t1.weight_kg) AS month_avg_weight,
                    AVG(t1.energy_burnt)::int4 AS month_avg_energy_burnt
                FROM
                    progress t1
                GROUP BY
                    t1.user_id,
                    DATE_TRUNC('month', t1.date)
            )
            SELECT
                t1.*,
                t1.weight_kg as weight,
                t2.username,
                t3.week_avg_weight,
                t3.week_avg_energy_burnt,
                t4.month_avg_weight,
                t4.month_avg_energy_burnt
            FROM 
                progress t1
                LEFT JOIN users_user t2 ON t2.id = t1.user_id
                LEFT JOIN week_avg t3 ON t3.user_id = t1.user_id
                AND t3.date = DATE_TRUNC('week', t1.date)::date
                LEFT JOIN month_avg t4 ON t4.user_id = t1.user_id
                AND t4.date = DATE_TRUNC('month', t1.date)::date
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
        qb.order("t1.date desc", order);
        qb.paginate(size, page);
        let q = qb.build_query_as().fetch_all(pool).await?;
        Ok(q)
    }
}
