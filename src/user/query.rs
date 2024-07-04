use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::{Error, Result};

use super::model::{MonthSummary, WeekSummary, WeekSummaryItem};
use crate::auth::privacy_level::PrivacyLevel;
use crate::follower::status::FollowerStatus;
use crate::food::model::Nutrition;
use crate::user::model::UserQuery;
use crate::user_block::model::UserBlockStatus;
use crate::util::datetime::{
    get_month_end_comprehensive, get_month_start_comprehensive, get_week_end, get_week_start,
};

impl FromRow<'_, PgRow> for UserQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let privacy_level: i32 = row.try_get("privacy_level").unwrap_or(-1);
        let follower_status: i32 = row.try_get("follower_status").unwrap_or(-1);
        let blocked_status: i32 = row.try_get("blocked_status").unwrap_or(-1);
        Ok(Self {
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            is_self: row.try_get("is_self")?,
            privacy_level: PrivacyLevel::from(privacy_level),
            blocked_status: UserBlockStatus::from(blocked_status),
            follower_status: FollowerStatus::from(follower_status),
            follower_count: row.try_get("follower_count")?,
            following_count: row.try_get("following_count")?,
            can_view: false,
        })
    }
}

impl UserQuery {
    pub async fn get(pool: &PgPool, username: &str, request_user_id: Uuid) -> Result<Self> {
        let mut query: UserQuery = sqlx::query_as(
            r#"
            SELECT
                false as can_view,
                t1.name,
                t1.username,
                CASE WHEN t1.id = $2 THEN true ELSE false END as is_self,
                t1.privacy_level,
                t2.status AS follower_status,
                t3.blocked_status AS blocked_status,
                (SELECT COUNT(*) FROM user_follower WHERE user_id = t1.id AND status = 1) as follower_count,
                (SELECT COUNT(*) FROM user_follower WHERE follower_id = t1.id AND status = 1) as following_count
            FROM
                users_user t1
                LEFT JOIN user_follower t2 ON t2.user_id = t1.id AND t2.follower_id = $2
                LEFT JOIN user_block t3 ON t3.blocker_id = t1.id AND t3.blocked_id = $2
            WHERE
                t1.username = $1
            "#,)
            .bind(username,)
            .bind(request_user_id).fetch_one(pool).await?;

        if query.blocked_status == UserBlockStatus::Blocked {
            return Err(Error::Forbidden);
        }
        if query.is_self || query.follower_status == FollowerStatus::Accepted {
            query.can_view = true;
        }
        Ok(query)
    }
}

impl WeekSummary {
    pub async fn try_get(pool: &PgPool, username: &str, date: NaiveDate) -> Result<Self> {
        let start = get_week_start(date);
        let end = get_week_end(date);
        let rows = sqlx::query(
            "
            WITH
                week_series AS (
                    SELECT
                        generate_series($2::TIMESTAMP, $3::TIMESTAMP, interval '1 day')::DATE AS date
                ),
                user_week_series_with_latest_weight AS (
                    SELECT
                        t2.date,
                        t1.id as user_id,
                        t1.username,
                        t3.weight_kg as latest_weight,
                        t3.date as latest_weight_date
                    FROM
                        users_user t1
                        CROSS JOIN week_series t2
                        LEFT JOIN progress t3 ON t3.user_id = t1.id
                        AND t3.date = (
                            SELECT
                                MAX(date)
                            FROM
                                progress
                            WHERE
                                user_id = t1.id
                                AND weight_kg IS NOT NULL
                                AND date <= t2.date
                        )
                ),
                week_series_diet as (
                    SELECT
                        t1.user_id,
                        t1.date,
                        SUM(t1.quantity * t2.energy) AS energy,
                        SUM(t1.quantity * t2.protein) AS protein,
                        SUM(t1.quantity * t2.carbohydrate) AS carbohydrate,
                        SUM(t1.quantity * t2.fat) fat,
                        SUM(t1.quantity * t2.saturates) AS saturates,
                        SUM(t1.quantity * t2.sugars) AS sugars,
                        SUM(t1.quantity * t2.fibre) AS fibre,
                        SUM(t1.quantity * t2.salt) AS salt,
                        COALESCE(SUM(t1.quantity * t2.protein * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS protein_pct,
                        COALESCE(SUM(t1.quantity * t2.carbohydrate * 4) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS carbohydrate_pct,
                        COALESCE(SUM(t1.quantity * t2.fat * 9) / NULLIF(SUM(t1.quantity * t2.energy), 0) * 100, 0) AS fat_pct
                    FROM
                        food_log t1
                        LEFT JOIN food t2 ON t2.id = t1.food_id
                    GROUP BY
                        t1.user_id,
                        t1.date
                ),
                week_series_diet_avg as (
                    SELECT
                        t1.user_id,
                        DATE_TRUNC('week', t1.date) AS date,
                        AVG(t1.energy) AS week_avg_energy,
                        AVG(t1.protein) AS week_avg_protein,
                        AVG(t1.carbohydrate) AS week_avg_carbohydrate,
                        AVG(t1.fat) AS week_avg_fat,
                        AVG(t1.saturates) AS week_avg_saturates,
                        AVG(t1.sugars) AS week_avg_sugars,
                        AVG(t1.fibre) AS week_avg_fibre,
                        AVG(t1.salt) AS week_avg_salt,
                        COALESCE(AVG(t1.protein) * 4 / NULLIF(AVG(t1.energy) * 100, 0), 0) AS week_avg_protein_pct,
                        COALESCE(AVG(t1.carbohydrate) * 4 / NULLIF(AVG(t1.energy) * 100, 0), 0) AS week_avg_carbohydrate_pct,
                        COALESCE(AVG(t1.fat) * 9 / NULLIF(AVG(t1.energy) * 100, 0), 0) AS week_avg_fat_pct
                    FROM
                        week_series_diet t1
                    GROUP BY
                        t1.user_id,
                        DATE_TRUNC('week', t1.date)
                )
            SELECT
                t1.username,
                t1.date,
                t1.latest_weight,
                t1.latest_weight_date,
                t3.energy,
                t3.protein,
                t3.carbohydrate,
                t3.fat,
                t3.saturates,
                t3.sugars,
                t3.fibre,
                t3.salt,
                t3.protein_pct,
                t3.carbohydrate_pct,
                t3.fat_pct,
                COALESCE(t3.energy / NULLIF(t1.latest_weight, 0), 0) as energy_per_kg,
                COALESCE(t3.protein / NULLIF(t1.latest_weight, 0), 0) as protein_per_kg,
                COALESCE(t3.carbohydrate / NULLIF(t1.latest_weight, 0), 0) as carbohydrate_per_kg,
                COALESCE(t3.fat / NULLIF(t1.latest_weight, 0), 0) as fat_per_kg,           
                t4.week_avg_energy,
                t4.week_avg_protein,
                t4.week_avg_carbohydrate,
                t4.week_avg_fat,
                t4.week_avg_saturates,
                t4.week_avg_sugars,
                t4.week_avg_fibre,
                t4.week_avg_salt,
                t4.week_avg_protein_pct,
                t4.week_avg_carbohydrate_pct,
                t4.week_avg_fat_pct
            FROM
                user_week_series_with_latest_weight t1
                LEFT JOIN week_series_diet t3 ON t3.user_id = t1.user_id AND t3.date = t1.date
                LEFT JOIN week_series_diet_avg t4 ON t4.user_id = t1.user_id AND t4.date = DATE_TRUNC('week', t1.date)
            WHERE
                t1.username = $1
            ",
        )
        .bind(username)
        .bind(start)
        .bind(end)
        .fetch_all(pool).await?;

        let mut rows_with_data = 0;
        let mut total_energy_per_kg = Decimal::from(0);
        let mut total_protein_per_kg = Decimal::from(0);
        let mut total_carbohydrate_per_kg = Decimal::from(0);
        let mut total_fat_per_kg = Decimal::from(0);

        let mut first_processed = false;
        let mut week = WeekSummary::default();
        let mut stream: Vec<WeekSummaryItem> = Vec::new();

        for row in rows {
            if !first_processed {
                week.username = row.try_get("username").unwrap_or_default();
                week.date = row.try_get("date").unwrap_or_default();
                week.nutrition =
                    Nutrition::from_row_with_prefix(&row, "week_avg").unwrap_or_default();
                first_processed = true;
            }

            let row_energy: Option<Decimal> = row.try_get("energy")?;
            let row_energy_per_kg: Decimal = row.try_get("energy_per_kg").unwrap_or_default();
            let row_protein_per_kg: Decimal = row.try_get("protein_per_kg").unwrap_or_default();
            let row_carbohydrate_per_kg: Decimal =
                row.try_get("carbohydrate_per_kg").unwrap_or_default();
            let row_fat_per_kg: Decimal = row.try_get("fat_per_kg").unwrap_or_default();

            if row_energy.is_some() {
                rows_with_data += 1;
                total_energy_per_kg += row_energy_per_kg;
                total_protein_per_kg += row_protein_per_kg;
                total_carbohydrate_per_kg += row_carbohydrate_per_kg;
                total_fat_per_kg += row_fat_per_kg;
            }
            stream.push(WeekSummaryItem::from_row(&row).unwrap_or_default());
        }
        if rows_with_data >= 1 {
            let rows_with_data_dec = Decimal::from(rows_with_data);
            week.energy_per_kg = total_energy_per_kg / rows_with_data_dec;
            week.protein_per_kg = total_protein_per_kg / rows_with_data_dec;
            week.carbohydrate_per_kg = total_carbohydrate_per_kg / rows_with_data_dec;
            week.fat_per_kg = total_fat_per_kg / rows_with_data_dec;
        }
        week.week_item = stream;
        Ok(week)
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

impl FromRow<'_, PgRow> for WeekSummaryItem {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            username: row.try_get("username")?,
            date: row.try_get("date")?,
            nutrition: Nutrition::from_row(&row).unwrap_or_default(),
            energy_per_kg: row.try_get("energy_per_kg").unwrap_or_default(),
            protein_per_kg: row.try_get("protein_per_kg").unwrap_or_default(),
            carbohydrate_per_kg: row.try_get("carbohydrate_per_kg").unwrap_or_default(),
            fat_per_kg: row.try_get("fat_per_kg").unwrap_or_default(),
            latest_weight: row.try_get("latest_weight")?,
            latest_weight_date: row.try_get("latest_weight_date")?,
        })
    }
}
