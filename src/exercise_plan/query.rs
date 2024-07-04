use crate::error::Error;
use crate::util::database::Filter;
use leptos::ServerFnError;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use super::model::{ExercisePlan, ExercisePlanInput, ExercisePlanQuery};

impl FromRow<'_, PgRow> for ExercisePlanQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("exercise_id")?,
            workout_plan_id: row.try_get("exercise_workout_plan_id")?,
            movement_id: row.try_get("exercise_movement_id")?,
            sequence: row.try_get("exercise_sequence")?,
            weight: row.try_get("exercise_weight")?,
            sets: row.try_get("exercise_sets")?,
            reps: row.try_get("exercise_reps")?,
            rest: row.try_get("exercise_rest")?,
            created_at: row.try_get("exercise_created_at")?,
            updated_at: row.try_get("exercise_updated_at")?,
            created_by_id: row.try_get("exercise_created_by_id")?,
            updated_by_id: row.try_get("exercise_updated_by_id")?,
            movement_name: row.try_get("exercise_movement_name")?,
            workout_plan_name: row.try_get("workout_name")?,
            workout_plan_slug: row.try_get("workout_slug")?,
        })
    }
}

impl ExercisePlanQuery {
    pub async fn get(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        let query = sqlx::query_as(
            "
            SELECT
                t1.id as exercise_id,
                t1.workout_plan_id as exercise_workout_plan_id,
                t1.movement_id as exercise_movement_id,
                t1.sequence as exercise_sequence,
                t1.weight as exercise_weight,
                t1.sets as exercise_sets,
                t1.reps as exercise_reps,
                t1.rest as exercise_rest,
                t1.created_at as exercise_created_at,
                t1.updated_at as exercise_updated_at,
                t1.created_by_id as exercise_created_by_id,
                t1.updated_by_id as exercise_updated_by_id,
                t2.name as exercise_movement_name,
                t3.name as workout_name,
                t3.slug as workout_slug
            FROM
                exercise_plan t1
                LEFT JOIN movement t2 ON t2.id = t1.movement_id
                LEFT JOIN workout_plan t3 ON t3.id = t1.workout_plan_id
            WHERE
                t1.id = $1
            ",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.id as exercise_id,
                t1.workout_plan_id as exercise_workout_plan_id,
                t1.movement_id as exercise_movement_id,
                t1.sequence as exercise_sequence,
                t1.weight as exercise_weight,
                t1.sets as exercise_sets,
                t1.reps as exercise_reps,
                t1.rest as exercise_rest,
                t1.created_at as exercise_created_at,
                t1.updated_at as exercise_updated_at,
                t1.created_by_id as exercise_created_by_id,
                t1.updated_by_id as exercise_updated_by_id,
                t2.name as exercise_movement_name,
                t3.name as workout_name,
                t3.slug as workout_slug
            FROM
                exercise_plan t1
                LEFT JOIN movement t2 ON t2.id = t1.movement_id
                LEFT JOIN workout_plan t3 ON t3.id = t1.workout_plan_id
            WHERE
                TRUE
            ",
        );
        qb.filter("t2.name", "ilike", search);
        qb.order("t2.name", order);
        qb.paginate(size, page);
        qb.build_query_as().fetch_all(pool).await
    }

    pub async fn count(pool: &PgPool, search: &str) -> sqlx::Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT 
                COUNT(t1.*) 
            FROM 
                exercise_plan t1
                LEFT JOIN movement t2 ON t2.id = t1.movement_id
            WHERE 
                TRUE
            ",
        );
        qb.filter("t2.name", "ilike", search);
        qb.build_query_scalar().fetch_one(pool).await
    }
}

impl ExercisePlan {
    pub async fn try_get(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM exercise_plan WHERE id = $1", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_object_or_404(pool: &PgPool, id: Uuid) -> Result<Self, ServerFnError> {
        let query = Self::try_get(pool, id).await?.ok_or(Error::NotFound)?;
        Ok(query)
    }

    pub async fn get_all_by_workout_plan_ids(
        pool: &PgPool,
        workout_plan_ids: &[Uuid],
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM exercise_plan WHERE workout_plan_id = ANY($1) ORDER BY workout_plan_id, sequence",
            workout_plan_ids,
        ).fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        data: &ExercisePlanInput,
        request_user_id: Uuid,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "
            INSERT INTO
                exercise_plan (
                    workout_plan_id,
                    movement_id,
                    sequence,
                    weight,
                    sets,
                    reps,
                    rest,
                    created_by_id
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                *
            ",
            data.workout_plan_id,
            data.movement_id,
            data.sequence,
            data.weight,
            data.sets,
            data.reps,
            data.rest,
            request_user_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        data: &ExercisePlanInput,
        request_user_id: Uuid,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "
            UPDATE exercise_plan
            SET
                workout_plan_id = $2,
                movement_id = $3,
                sequence = $4,
                weight = $5,
                sets = $6,
                reps = $7,
                rest = $8,
                updated_at = NOW(),
                updated_by_id = $9
            WHERE
                id = $1
            RETURNING
                *
            ",
            id,
            data.workout_plan_id,
            data.movement_id,
            data.sequence,
            data.weight,
            data.sets,
            data.reps,
            data.rest,
            request_user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "DELETE FROM exercise_plan WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
    }
}
