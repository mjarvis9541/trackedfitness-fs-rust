use indexmap::IndexMap;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Result;
use crate::exercise::model::{ExerciseBase, ExerciseQuery};
use crate::set::model::SetQuery;

impl ExerciseBase {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM exercise WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn try_get_previous_exercise_id(
        pool: &PgPool,
        exercise_id: Uuid,
    ) -> Result<Option<Uuid>> {
        let query = sqlx::query_scalar(
            "
            SELECT
                previous_exercise_id
            FROM
                (
                    SELECT
                        e.id,
                        e.workout_id,
                        e.movement_id,
                        w.user_id,
                        w.date,
                        LAG(e.id) OVER (
                            PARTITION BY
                                w.user_id,
                                e.movement_id
                            ORDER BY
                                w.date,
                                w.created_at,
                                e.created_at
                        ) AS previous_exercise_id
                    FROM
                        exercise e
                        JOIN workout w ON e.workout_id = w.id
                ) AS current_exercise
            WHERE
                current_exercise.id = $1
            ",
        )
        .bind(exercise_id)
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        workout_id: Uuid,
        movement_id: Uuid,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "
            INSERT INTO
                exercise (workout_id, movement_id, created_by_id)
            VALUES
                ($1, $2, $3)
            RETURNING
                *
            ",
            workout_id,
            movement_id,
            request_user_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        workout_id: Uuid,
        movement_id: Uuid,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "
            UPDATE exercise
            SET
                workout_id = $1,
                movement_id = $2,
                updated_at = NOW(),
                updated_by_id = $3
            WHERE
                id = $4
            RETURNING
                *
            ",
            workout_id,
            movement_id,
            request_user_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM exercise WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }

    // pub async fn bulk_create_from_exercise_plan(
    //     pool: &PgPool,
    //     workout_id: Uuid,
    //     exercises: &[ExercisePlan],
    //     request_user_id: Uuid,
    // ) -> Result<Vec<Self>> {
    //     let movement_ids: Vec<Uuid> = exercises.iter().map(|e| e.movement_id).collect();
    //     let sequences: Vec<i32> = exercises.iter().map(|e| e.sequence).collect();
    //     let query = sqlx::query_as!(
    //         Self,
    //         r#"
    //         INSERT INTO exercise (workout_id, movement_id, "order", created_by_id)
    //         SELECT $1, UNNEST($2::UUID[]), UNNEST($3::INTEGER[]), $4
    //         RETURNING *
    //         "#,
    //         workout_id,
    //         &movement_ids,
    //         &sequences,
    //         request_user_id,
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //     Ok(query)
    // }
}

impl ExerciseQuery {
    pub async fn get_all_with_sets_for_user_by_movement(
        pool: &PgPool,
        username: &str,
        movement_slug: &str,
    ) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "
            SELECT
                exercise.id as exercise_id,
                exercise.order as exercise_order,
                exercise.workout_id as exercise_workout_id,
                exercise.movement_id as exercise_movement_id,
                exercise.created_at as exercise_created_at,
                exercise.updated_at as exercise_updated_at,
                exercise.created_by_id as exercise_created_by_id,
                exercise.updated_by_id as exercise_updated_by_id,
                movement.name as movement_name,
                movement.slug as movement_slug,
                muscle_group.name as muscle_group_name,
                muscle_group.slug as muscle_group_slug,
                users_user.username as username,
                workout.date as exercise_date,
                workout.user_id as exercise_user_id,
                tracked_set.id as set_id,
                tracked_set.order as set_order,
                tracked_set.weight as set_weight,
                tracked_set.reps as set_reps
            FROM
                exercise
                LEFT JOIN movement ON movement.id = exercise.movement_id
                LEFT JOIN muscle_group ON muscle_group.id = movement.muscle_group_id
                LEFT JOIN workout ON workout.id = exercise.workout_id
                LEFT JOIN tracked_set ON tracked_set.exercise_id = exercise.id
                LEFT JOIN users_user ON users_user.id = workout.user_id
            WHERE
                movement.slug = $1
                AND users_user.username = $2
            ORDER BY
                workout.date,
                workout.created_at,
                exercise.order,
                exercise.created_at,
                tracked_set.order,
                tracked_set.created_at
            ",
        )
        .bind(movement_slug)
        .bind(username)
        .fetch_all(pool)
        .await?;

        let mut exercise_map: IndexMap<Uuid, ExerciseQuery> = IndexMap::new();

        for row in rows {
            let exercise_id: Uuid = row.get("exercise_id");

            let exercise_map_entry = exercise_map
                .entry(exercise_id)
                .or_insert_with(|| ExerciseQuery::from_row(&row).unwrap());

            if let Some(_set_id) = row.get::<Option<Uuid>, _>("set_id") {
                let set_data = SetQuery::from_row(&row)?;
                exercise_map_entry.sets.push(set_data)
            };
        }

        let results = exercise_map.into_values().collect::<Vec<ExerciseQuery>>();

        Ok(results)
    }

    pub async fn try_get_with_sets(pool: &PgPool, exercise_id: Uuid) -> Result<Option<Self>> {
        let rows = sqlx::query(
            "
            SELECT
                t1.id as exercise_id,
                t1.order as exercise_order,
                t1.workout_id as exercise_workout_id,
                t1.movement_id as exercise_movement_id,
                t1.created_at as exercise_created_at,
                t1.updated_at as exercise_updated_at,
                t1.created_by_id as exercise_created_by_id,
                t1.updated_by_id as exercise_updated_by_id,
                --
                t2.name as movement_name,
                t2.slug as movement_slug,
                --
                t3.name as muscle_group_name,
                t3.slug as muscle_group_slug,
                --
                t6.username as username,
                --
                t4.date as exercise_date,
                t4.user_id as exercise_user_id,
                --
                t5.id as set_id,
                t5.order as set_order,
                t5.weight as set_weight,
                t5.reps as set_reps
                --
            FROM
                exercise t1
                LEFT JOIN movement t2 ON t2.id = t1.movement_id
                LEFT JOIN muscle_group t3 ON t3.id = t2.muscle_group_id
                LEFT JOIN workout t4 ON t4.id = t1.workout_id
                LEFT JOIN tracked_set t5 ON t5.exercise_id = t1.id
                LEFT JOIN users_user t6 ON t6.id = t4.user_id
            WHERE
                t1.id = $1
            ORDER BY
                t5.order,
                t5.created_at
            ",
        )
        .bind(exercise_id)
        .fetch_all(pool)
        .await?;

        let mut exercise: Option<ExerciseQuery> = None;
        for row in rows {
            if exercise.is_none() {
                exercise = Some(ExerciseQuery::from_row(&row).unwrap_or_default());
            }

            if let Some(_set_id) = row.get::<Option<Uuid>, _>("set_id") {
                exercise
                    .as_mut()
                    .unwrap()
                    .sets
                    .push(SetQuery::from_row(&row).unwrap_or_default())
            };
        }

        Ok(exercise)
    }
}

impl FromRow<'_, PgRow> for ExerciseQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("exercise_id")?,
            user_id: row.try_get("exercise_user_id")?,
            workout_id: row.try_get("exercise_workout_id")?,
            order: row.try_get("exercise_order")?,
            created_at: row.try_get("exercise_created_at")?,
            updated_at: row.try_get("exercise_updated_at")?,
            created_by_id: row.try_get("exercise_created_by_id")?,
            updated_by_id: row.try_get("exercise_updated_by_id")?,
            date: row.try_get("exercise_date")?,
            username: row.try_get("username")?,
            movement_name: row.try_get("movement_name")?,
            movement_slug: row.try_get("movement_slug")?,
            muscle_group_name: row.try_get("muscle_group_name")?,
            muscle_group_slug: row.try_get("muscle_group_slug")?,
            set_count: row.try_get("exercise_set_count").unwrap_or(0),
            rep_count: row.try_get("exercise_rep_count").unwrap_or(0),
            sets: Vec::new(),
        })
    }
}
