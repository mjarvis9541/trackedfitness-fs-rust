use std::collections::BTreeMap;

use chrono::prelude::*;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Result;
use crate::exercise::model::{ExerciseQuery, ExerciseQueryWithPrevious};
use crate::set::model::{SetQuery, SetQueryWithPrevious};
use crate::util::datetime::{get_week_end, get_week_start};

use super::model::{
    ExerciseDTO, SetDTO, WorkoutBase, WorkoutDTO, WorkoutDetail, WorkoutQuery,
    WorkoutQueryWithPrevious, WorkoutWeek,
};

impl WorkoutBase {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM workout WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        date: NaiveDate,
        created_by_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO workout (user_id, date, created_by_id) VALUES ($1, $2, $3) RETURNING *",
            user_id,
            date,
            created_by_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        workout_id: Uuid,
        date: NaiveDate,
        updated_by_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self, "UPDATE workout SET date = $1, updated_at = NOW(), updated_by_id = $2 WHERE id = $3 RETURNING *",
            date,
            updated_by_id,
            workout_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM workout WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }
}

impl WorkoutQueryWithPrevious {
    pub async fn get_all_by_username(pool: &PgPool, username: &str) -> Result<Vec<Self>> {
        let query = sqlx::query_as(
            "
            SELECT
                t1.user_id,
                t1.id AS workout_id,
                t1.date AS workout_date,
                t1.created_at AS workout_created_at,
                --
                t4.username,
                --
                COUNT(DISTINCT t2.id) AS workout_exercise_count,
                COUNT(t3.id) AS workout_set_count,
                COALESCE(SUM(t3.reps), 0) AS workout_rep_count
            FROM
                workout t1
                LEFT JOIN exercise t2 ON t1.id = t2.workout_id
                LEFT JOIN tracked_set t3 ON t2.id = t3.exercise_id
                LEFT JOIN users_user t4 ON t4.id = t1.user_id
            WHERE
                t4.username = $1
                GROUP BY
                t1.id,
                t4.id
            ORDER BY
                t1.date DESC
            ",
        )
        .bind(username)
        .fetch_all(pool)
        .await?;
        Ok(query)
    }

    pub async fn get(pool: &PgPool, username: &str, date: NaiveDate) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "
            WITH
                numbered_exercise AS (
                    SELECT
                        w.user_id,
                        w.id AS workout_id,
                        e.id AS exercise_id,
                        e.order AS exercise_order,
                        ROW_NUMBER() OVER (
                            PARTITION BY
                                w.user_id,
                                e.movement_id
                            ORDER BY
                                w.date,
                                w.created_at,
                                e.created_at
                        ) AS exercise_rank,
                        e.movement_id,
                        w.date AS workout_date,
                        w.created_at AS workout_created_at,
                        e.created_at AS exercise_created_at
                    FROM
                        workout w
                        LEFT JOIN exercise e ON e.workout_id = w.id
                ),
                numbered_set AS (
                    SELECT
                        s.exercise_id,
                        s.id AS set_id,
                        s.order AS set_order,
                        s.weight,
                        s.reps,
                        s.rest,
                        ROW_NUMBER() OVER (
                            PARTITION BY
                                s.exercise_id
                            ORDER BY
                                s.order,
                                s.created_at
                        ) AS set_rank
                    FROM
                        tracked_set s
                ),
                workout_aggregates AS (
                    SELECT
                        ne.workout_id,
                        COUNT(DISTINCT ne.exercise_id) AS exercise_count,
                        COUNT(DISTINCT ns.set_id) AS set_count,
                        SUM(ns.reps) AS rep_count
                    FROM
                        numbered_exercise ne
                        JOIN numbered_set ns ON ns.exercise_id = ne.exercise_id
                    GROUP BY
                        ne.workout_id
                ),
                exercise_aggregates AS (
                    SELECT
                        ns.exercise_id,
                        COUNT(*) AS set_count,
                        SUM(ns.reps) AS rep_count
                    FROM
                        numbered_set ns
                    GROUP BY
                        ns.exercise_id
                )
            SELECT
                ne.user_id as user_id,
                uu.username,
                ne.workout_id,
                ne.workout_date,
                ne.workout_created_at,
                ne.exercise_id,
                ne.exercise_created_at,
                ne.exercise_order,
                m.name AS movement_name,
                mg.name AS muscle_group_name,
                --
                ns.set_order,
                ns.set_id,
                ns.weight,
                ns.reps,
                ns.rest,
                --
                prev_ne.workout_id AS previous_workout_id,
                prev_ne.workout_date AS previous_workout_date,
                prev_ne.exercise_id AS previous_exercise_id,
                --
                prev_ns.set_id AS previous_set_id,
                prev_ns.weight AS previous_weight,
                prev_ns.reps AS previous_reps,
                --
                wa.exercise_count AS workout_exercise_count,
                wa.set_count AS workout_set_count,
                wa.rep_count AS workout_rep_count,
                ea.set_count AS exercise_set_count,
                ea.rep_count AS exercise_rep_count
            FROM
                numbered_exercise ne
                LEFT JOIN numbered_set ns ON ne.exercise_id = ns.exercise_id
                LEFT JOIN numbered_exercise prev_ne ON prev_ne.user_id = ne.user_id AND prev_ne.movement_id = ne.movement_id AND prev_ne.exercise_rank = ne.exercise_rank - 1
                LEFT JOIN numbered_set prev_ns ON prev_ns.exercise_id = prev_ne.exercise_id AND prev_ns.set_rank = ns.set_rank LEFT JOIN movement m ON m.id = ne.movement_id
                LEFT JOIN muscle_group mg ON mg.id = m.muscle_group_id
                LEFT JOIN users_user uu ON uu.id = ne.user_id
                LEFT JOIN workout_aggregates wa ON wa.workout_id = ne.workout_id
                LEFT JOIN exercise_aggregates ea ON ea.exercise_id = ne.exercise_id
            WHERE
                uu.username = $1
                AND ne.workout_date = $2
            ORDER BY
                ne.workout_date,
                ne.workout_created_at,
                ne.exercise_order,
                ne.exercise_created_at,
                ns.set_order
            ",
        )
        .bind(username)
        .bind(date)
        .fetch_all(pool)
        .await?;

        let mut workouts = Vec::<WorkoutQueryWithPrevious>::new();

        for row in rows {
            let workout_id: Uuid = row.try_get("workout_id")?;
            let workout_index = workouts
                .iter()
                .position(|workout| workout.workout_id == workout_id)
                .unwrap_or_else(|| {
                    let workout = WorkoutQueryWithPrevious::from_row(&row).unwrap_or_default();
                    workouts.push(workout);
                    workouts.len() - 1
                });

            if let Some(exercise_id) = row.try_get("exercise_id")? {
                let exercise_index = workouts[workout_index]
                    .exercises
                    .iter()
                    .position(|exercise| exercise.exercise_id == exercise_id)
                    .unwrap_or_else(|| {
                        let exercise =
                            ExerciseQueryWithPrevious::from_row(&row).unwrap_or_default();
                        workouts[workout_index].exercises.push(exercise);
                        workouts[workout_index].exercises.len() - 1
                    });

                if let Some(_set_id) = row.get::<Option<Uuid>, _>("set_id") {
                    let set = SetQueryWithPrevious::from_row(&row).unwrap_or_default();
                    workouts[workout_index].exercises[exercise_index]
                        .sets
                        .push(set);
                }
            }
        }
        Ok(workouts)
    }
}

impl WorkoutWeek {
    pub async fn all(pool: &PgPool, username: &str, date: NaiveDate) -> Result<Vec<Self>> {
        let start = get_week_start(date);
        let end = get_week_end(date);

        let rows = sqlx::query(
            "
        SELECT
            t1.id AS workout_id,
            t1.date AS workout_date,
            t1.created_at AS workout_created_at,
            --
            t5.username,
            --
            t2.id as exercise_id,
            t2.order as exercise_order,
            t2.workout_id as exercise_workout_id,
            t2.movement_id as exercise_movement_id,
            t2.created_at as exercise_created_at,
            t2.updated_at as exercise_updated_at,
            t2.created_by_id as exercise_created_by_id,
            t2.updated_by_id as exercise_updated_by_id,
            --
            t3.name as movement_name,
            t3.slug as movement_slug,
            --
            t6.name as muscle_group_name,
            t6.slug as muscle_group_slug,
            --
            t1.date as exercise_date,
            t1.user_id as exercise_user_id,
            --
            t4.id AS set_id,
            t4.order AS set_order,
            t4.weight AS set_weight,
            t4.reps AS set_reps
        FROM
            workout t1
            LEFT JOIN exercise t2 ON t2.workout_id = t1.id
            LEFT JOIN movement t3 ON t3.id = t2.movement_id
            LEFT JOIN tracked_set t4 ON t4.exercise_id = t2.id
            LEFT JOIN users_user t5 ON t5.id = t1.user_id
            LEFT JOIN muscle_group t6 ON t6.id = t3.muscle_group_id
        WHERE
            t5.username = $1
            AND t1.date BETWEEN $2 AND $3
        ORDER BY
            t1.date,
            t2.order,
            t4.order
        ",
        )
        .bind(&username)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        let mut days_in_week: Vec<WorkoutWeek> = (0..7)
            .map(|d| WorkoutWeek {
                username: username.to_string(),
                date: start + chrono::TimeDelta::days(d),
                user_id: Uuid::nil(),
                workouts: Vec::new(),
            })
            .collect();

        for row in rows {
            let date: NaiveDate = row.try_get("workout_date")?;
            let day_in_week_index = days_in_week.iter().position(|ww| ww.date == date).unwrap();

            let workout_id: Uuid = row.get("workout_id");
            let workout_index = days_in_week[day_in_week_index]
                .workouts
                .iter()
                .position(|w| w.workout_id == workout_id)
                .unwrap_or_else(|| {
                    let workout = WorkoutQuery::from_row(&row).unwrap_or_default();
                    days_in_week[day_in_week_index].workouts.push(workout);
                    days_in_week[day_in_week_index].workouts.len() - 1
                });

            if let Some(exercise_id) = row.get::<Option<Uuid>, _>("exercise_id") {
                let exercise_index = days_in_week[day_in_week_index].workouts[workout_index]
                    .exercises
                    .iter()
                    .position(|e| e.id == exercise_id)
                    .unwrap_or_else(|| {
                        let exercise = ExerciseQuery::from_row(&row).unwrap_or_default();

                        days_in_week[day_in_week_index].workouts[workout_index]
                            .exercises
                            .push(exercise);
                        days_in_week[day_in_week_index].workouts[workout_index]
                            .exercises
                            .len()
                            - 1
                    });

                if let Some(set_id) = row.get::<Option<Uuid>, _>("set_id") {
                    let _set_index = days_in_week[day_in_week_index].workouts[workout_index]
                        .exercises[exercise_index]
                        .sets
                        .iter()
                        .position(|s| s.id == set_id)
                        .unwrap_or_else(|| {
                            let set = SetQuery::from_row(&row).unwrap_or_default();

                            days_in_week[day_in_week_index].workouts[workout_index].exercises
                                [exercise_index]
                                .sets
                                .push(set);
                            days_in_week[day_in_week_index].workouts[workout_index].exercises
                                [exercise_index]
                                .sets
                                .len()
                                - 1
                        });
                }
            }
        }
        Ok(days_in_week)
    }
}

impl FromRow<'_, PgRow> for WorkoutQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            workout_id: row.try_get("workout_id")?,
            workout_date: row.try_get("workout_date")?,
            created_at: row.try_get("workout_created_at")?,
            username: row.try_get("username")?,
            exercise_count: row.try_get("workout_exercise_count").unwrap_or(0),
            set_count: row.try_get("workout_set_count").unwrap_or(0),
            rep_count: row.try_get("workout_rep_count").unwrap_or(0),
            exercises: Vec::new(),
        })
    }
}

impl FromRow<'_, PgRow> for WorkoutQueryWithPrevious {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            workout_id: row.try_get("workout_id")?,
            workout_date: row.try_get("workout_date")?,
            created_at: row.try_get("workout_created_at")?,
            username: row.try_get("username")?,
            exercise_count: row.try_get("workout_exercise_count").unwrap_or(0),
            set_count: row.try_get("workout_set_count").unwrap_or(0),
            rep_count: row.try_get("workout_rep_count").unwrap_or(0),
            exercises: Vec::new(),
        })
    }
}

impl WorkoutDetail {
    pub async fn get_all_by_username_date(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Vec<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/workout_day_list.sql", username, date)
            .fetch_all(pool)
            .await?;
        Ok(query)
    }

    pub async fn aggregate_workout_day_data(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<Vec<WorkoutDTO>> {
        let workout_query = WorkoutDetail::get_all_by_username_date(pool, username, date).await?;

        let mut workouts_map: BTreeMap<Uuid, WorkoutDTO> = BTreeMap::new();

        for detail in workout_query {
            let workout_entry =
                workouts_map
                    .entry(detail.workout_id)
                    .or_insert_with(|| WorkoutDTO {
                        workout_id: detail.workout_id,
                        user_id: detail.user_id,
                        username: detail.username.clone(),
                        date: detail.workout_date,
                        exercise_count: detail.workout_exercise_count.unwrap_or(0),
                        set_count: detail.workout_set_count.unwrap_or(0),
                        rep_count: detail.workout_rep_count.unwrap_or(0),
                        exercise_list: vec![],
                    });

            let mut exercise_exists = false;
            for exercise in &mut workout_entry.exercise_list {
                if exercise.exercise_id == detail.exercise_id {
                    exercise_exists = true;

                    if let (Some(set_id), Some(weight), Some(reps), Some(rest), Some(order)) = (
                        detail.set_id,
                        detail.weight,
                        detail.reps,
                        detail.rest,
                        detail.set_order,
                    ) {
                        let set_entry = SetDTO {
                            workout_id: detail.workout_id,
                            exercise_id: detail.exercise_id,
                            set_id,
                            username: detail.username.clone(),
                            date: detail.workout_date,
                            weight,
                            reps,
                            rest,
                            order,
                        };
                        exercise.set_list.push(set_entry);
                    }

                    // Optionally handle previous sets if needed
                    if let (
                        Some(prev_set_id),
                        Some(prev_weight),
                        Some(prev_reps),
                        Some(prev_order),
                    ) = (
                        detail.previous_set_id,
                        detail.previous_weight,
                        detail.previous_reps,
                        detail.set_order,
                    ) {
                        let prev_set_entry = SetDTO {
                            workout_id: detail.previous_workout_id.unwrap_or(detail.workout_id),
                            exercise_id: detail.previous_exercise_id.unwrap_or(detail.exercise_id),
                            set_id: prev_set_id,
                            username: detail.username.clone(),
                            date: detail.previous_workout_date.unwrap_or(detail.workout_date),
                            weight: prev_weight,
                            reps: prev_reps,
                            rest: detail.rest.unwrap_or(0),
                            order: prev_order,
                        };
                        exercise.previous_set_list.push(prev_set_entry);
                    }
                    break;
                }
            }

            if !exercise_exists {
                let mut new_exercise = ExerciseDTO {
                    workout_id: detail.workout_id,
                    exercise_id: detail.exercise_id,
                    username: detail.username.clone(),
                    date: detail.workout_date,
                    movement_name: detail.movement_name.clone(),
                    muscle_group_name: detail.muscle_group_name.clone().unwrap_or_default(),
                    set_list: vec![],
                    previous_set_list: vec![],
                };

                if let (Some(set_id), Some(weight), Some(reps), Some(rest), Some(order)) = (
                    detail.set_id,
                    detail.weight,
                    detail.reps,
                    detail.rest,
                    detail.set_order,
                ) {
                    let set_entry = SetDTO {
                        workout_id: detail.workout_id,
                        exercise_id: detail.exercise_id,
                        set_id,
                        username: detail.username.clone(),
                        date: detail.workout_date,
                        weight,
                        reps,
                        rest,
                        order,
                    };
                    new_exercise.set_list.push(set_entry);
                }

                // Optionally handle previous sets if needed
                if let (Some(prev_set_id), Some(prev_weight), Some(prev_reps), Some(prev_order)) = (
                    detail.previous_set_id,
                    detail.previous_weight,
                    detail.previous_reps,
                    detail.set_order,
                ) {
                    let prev_set_entry = SetDTO {
                        workout_id: detail.previous_workout_id.unwrap_or(detail.workout_id),
                        exercise_id: detail.previous_exercise_id.unwrap_or(detail.exercise_id),
                        set_id: prev_set_id,
                        username: detail.username.clone(),
                        date: detail.previous_workout_date.unwrap_or(detail.workout_date),
                        weight: prev_weight,
                        reps: prev_reps,
                        rest: detail.rest.unwrap_or(0),
                        order: prev_order,
                    };
                    new_exercise.previous_set_list.push(prev_set_entry);
                }

                workout_entry.exercise_list.push(new_exercise);
            }
        }

        let iter = workouts_map
            .into_iter()
            .map(|(_, workout)| workout)
            .collect();

        Ok(iter)
    }
}

// #[derive(Debug, Deserialize, Serialize, Clone, Default)]
// pub struct WorkoutQueryWithPrevious {
//     pub user_id: Uuid,
//     pub workout_id: Uuid,
//     pub workout_date: NaiveDate,
//     pub created_at: DateTime<Utc>,
//     pub username: String,
//     pub exercise_count: i64,
//     pub set_count: i64,
//     pub rep_count: i64,
//     pub exercises: Vec<ExerciseQueryWithPrevious>,
// }
// #[derive(Debug, Deserialize, Serialize, Clone, Default)]
// pub struct ExerciseQueryWithPrevious {
//     pub exercise_id: Uuid,
//     pub movement_name: String,
//     pub muscle_group_name: String,
//     pub order: i32,
//     pub set_count: i64,
//     pub rep_count: i64,
//     pub sets: Vec<SetQueryWithPrevious>,
// }

// #[derive(Debug, Deserialize, Serialize, Clone, Default)]
// pub struct SetQueryWithPrevious {
//     pub set_id: Uuid,
//     pub order: i32,
//     pub weight: Decimal,
//     pub reps: i32,
//     pub rest: i32,
//     pub previous_workout_id: Option<Uuid>,
//     pub previous_workout_date: Option<NaiveDate>,
//     pub previous_exercise_id: Option<Uuid>,
//     pub previous_weight: Option<Decimal>,
//     pub previous_reps: Option<i32>,
// }
