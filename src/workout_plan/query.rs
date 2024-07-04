use indexmap::IndexMap;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::training_plan::weekday::WeekDay;
use crate::util::database::Filter;
use crate::util::server::slugify;

use super::model::{WorkoutPlan, WorkoutPlanInput, WorkoutPlanQuery};

impl FromRow<'_, PgRow> for WorkoutPlanQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let through_weekday: Option<i32> = row.try_get("workout_weekday")?;
        let weekday = through_weekday.map(|opt| WeekDay::from(opt));
        Ok(Self {
            id: row.try_get("workout_id")?,
            user_id: row.try_get("workout_user_id")?,
            name: row.try_get("workout_name")?,
            slug: row.try_get("workout_slug")?,
            created_at: row.try_get("workout_created_at")?,
            updated_at: row.try_get("workout_updated_at")?,
            created_by_id: row.try_get("workout_created_by_id")?,
            updated_by_id: row.try_get("workout_updated_by_id")?,
            //
            sequence: row.try_get("workout_sequence")?,
            weekday,
            training_plan_id: row.try_get("training_id")?,
            training_plan_name: row.try_get("training_name")?,
            training_plan_slug: row.try_get("training_slug")?,
            exercise_count: row.try_get("workout_exercise_count").unwrap_or(0),
            set_count: row.try_get("workout_set_count").unwrap_or(0),
            rep_count: row.try_get("workout_rep_count").unwrap_or(0),
            exercise_plans: Vec::new(),
        })
    }
}

impl WorkoutPlan {
    pub async fn try_get(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM workout_plan WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }
    pub async fn get_object_or_404(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = Self::try_get(pool, id).await?.ok_or(Error::NotFound)?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        data: &WorkoutPlanInput,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let slug = slugify(&data.name);
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO workout_plan (
                    user_id,
                    name,
                    slug,
                    created_by_id
                )
            VALUES ($1, $2, $3, $4)
            RETURNING *",
            data.user_id,
            data.name,
            slug,
            request_user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        data: &WorkoutPlanInput,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let slug = slugify(&data.name);
        let query = sqlx::query_as!(
            Self,
            "UPDATE workout_plan
            SET
                name = $2,
                slug = $3,
                updated_by_id = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            ",
            id,
            data.name,
            slug,
            request_user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM workout_plan WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}

impl WorkoutPlanQuery {
    pub async fn get_with_exercise_plans(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let rows = sqlx::query(
            "
            WITH
            workout_plan_aggregation AS (
                SELECT
                    workout_plan_id, 
                    COUNT(DISTINCT id) AS exercise_count,
                    SUM(sets) AS set_count,
                    SUM(reps) AS rep_count
                FROM
                    exercise_plan
                GROUP BY 
                    workout_plan_id
            )
            SELECT
                wp.id AS workout_id,
                wp.user_id AS workout_user_id,
                tpw.training_plan_id AS training_id,
                wp.name AS workout_name,
                wp.slug AS workout_slug,
                tpw.sequence AS workout_sequence,
                tpw.weekday AS workout_weekday,
                wp.created_at AS workout_created_at,
                wp.updated_at AS workout_updated_at,
                wp.created_by_id AS workout_created_by_id,
                wp.updated_by_id AS workout_updated_by_id,
                --
                ep.id AS exercise_id,
                ep.workout_plan_id AS exercise_workout_plan_id,
                ep.movement_id AS exercise_movement_id,
                ep.sequence AS exercise_sequence,
                ep.weight AS exercise_weight,
                ep.sets AS exercise_sets,
                ep.reps AS exercise_reps,
                ep.rest AS exercise_rest,
                ep.created_at AS exercise_created_at,
                ep.updated_at AS exercise_updated_at,
                ep.created_by_id AS exercise_created_by_id,
                ep.updated_by_id AS exercise_updated_by_id,
                --
                m.name AS exercise_movement_name,
                --
                COALESCE(wpa.exercise_count, 0) AS workout_exercise_count,
                COALESCE(wpa.set_count, 0) AS workout_set_count,
                COALESCE(wpa.rep_count, 0) AS workout_rep_count,
                --
                tp.name AS training_name,
                tp.slug AS training_slug
            FROM
                workout_plan wp
                LEFT JOIN training_plan_workout_plan tpw ON tpw.workout_plan_id = wp.id
                LEFT JOIN training_plan tp ON tp.id = tpw.training_plan_id
                LEFT JOIN exercise_plan ep ON ep.workout_plan_id = wp.id
                LEFT JOIN movement m ON m.id = ep.movement_id
                LEFT JOIN workout_plan_aggregation wpa ON wpa.workout_plan_id = wp.id
            WHERE
                wp.slug = $1
            ORDER BY
                tpw.sequence, ep.sequence;
            ",
        )
        .bind(slug)
        .fetch_all(pool)
        .await?;

        let mut workout_plan: Option<WorkoutPlanQuery> = None;
        for row in &rows {
            if workout_plan.is_none() {
                workout_plan = Some(WorkoutPlanQuery::from_row(row)?);
            }

            if let Some(_exercise_id) = row.get::<Option<Uuid>, _>("exercise_id") {
                let workout = workout_plan.as_mut().unwrap();
                workout
                    .exercise_plans
                    .push(ExercisePlanQuery::from_row(row).unwrap_or_default());
            }
        }

        Ok(workout_plan)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            WITH
            workout_plan_aggregation AS (
                SELECT
                    workout_plan_id, 
                    COUNT(DISTINCT id) AS exercise_count,
                    SUM(sets) AS set_count,
                    SUM(reps) AS rep_count
                FROM
                    exercise_plan
                GROUP BY 
                    workout_plan_id
            )
            SELECT
                wp.id AS workout_id,
                wp.user_id AS workout_user_id,
                tpw.training_plan_id AS training_id,
                wp.name AS workout_name,
                wp.slug AS workout_slug,
                tpw.sequence AS workout_sequence,
                tpw.weekday AS workout_weekday,
                wp.created_at AS workout_created_at,
                wp.updated_at AS workout_updated_at,
                wp.created_by_id AS workout_created_by_id,
                wp.updated_by_id AS workout_updated_by_id,
                --
                COALESCE(wpa.exercise_count, 0) AS workout_exercise_count,
                COALESCE(wpa.set_count, 0) AS workout_set_count,
                COALESCE(wpa.rep_count, 0) AS workout_rep_count,
                --
                tp.name AS training_name,
                tp.slug AS training_slug
            FROM
                workout_plan wp
                LEFT JOIN training_plan_workout_plan tpw ON tpw.workout_plan_id = wp.id
                LEFT JOIN training_plan tp ON tp.id = tpw.training_plan_id
                LEFT JOIN workout_plan_aggregation wpa ON wpa.workout_plan_id = wp.id
            WHERE
                TRUE
            ",
        );
        qb.filter("wp.name", "ilike", search);
        qb.order("wp.name", order);
        qb.paginate(size, page);
        Ok(qb.build_query_as().fetch_all(pool).await?)
    }

    pub async fn count(pool: &PgPool, search: &str) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new("SELECT COUNT(t1.*) FROM workout_plan t1 WHERE TRUE");
        qb.filter("t1.name", "ilike", search);
        Ok(qb.build_query_scalar().fetch_one(pool).await?)
    }

    pub async fn get_all_with_exercise_plans(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            WITH
            workout_plan_aggregation AS (
                SELECT
                    workout_plan_id, 
                    COUNT(DISTINCT id) AS exercise_count,
                    SUM(sets) AS set_count,
                    SUM(reps) AS rep_count
                FROM
                    exercise_plan
                GROUP BY 
                    workout_plan_id
            )
            SELECT
                wp.id AS workout_id,
                wp.user_id AS workout_user_id,
                tpw.training_plan_id AS training_id,
                wp.name AS workout_name,
                wp.slug AS workout_slug,
                tpw.sequence AS workout_sequence,
                tpw.weekday AS workout_weekday,
                wp.created_at AS workout_created_at,
                wp.updated_at AS workout_updated_at,
                wp.created_by_id AS workout_created_by_id,
                wp.updated_by_id AS workout_updated_by_id,
                --
                ep.id AS exercise_id,
                ep.workout_plan_id AS exercise_workout_plan_id,
                ep.movement_id AS exercise_movement_id,
                ep.sequence AS exercise_sequence,
                ep.weight AS exercise_weight,
                ep.sets AS exercise_sets,
                ep.reps AS exercise_reps,
                ep.rest AS exercise_rest,
                ep.created_at AS exercise_created_at,
                ep.updated_at AS exercise_updated_at,
                ep.created_by_id AS exercise_created_by_id,
                ep.updated_by_id AS exercise_updated_by_id,
                --
                m.name AS exercise_movement_name,
                --
                COALESCE(wpa.exercise_count, 0) AS workout_exercise_count,
                COALESCE(wpa.set_count, 0) AS workout_set_count,
                COALESCE(wpa.rep_count, 0) AS workout_rep_count,
                --
                tp.name AS training_name,
                tp.slug AS training_slug
            FROM
                workout_plan wp
                LEFT JOIN training_plan_workout_plan tpw ON tpw.workout_plan_id = wp.id
                LEFT JOIN training_plan tp ON tp.id = tpw.training_plan_id
                LEFT JOIN exercise_plan ep ON ep.workout_plan_id = wp.id
                LEFT JOIN movement m ON m.id = ep.movement_id
                LEFT JOIN workout_plan_aggregation wpa ON wpa.workout_plan_id = wp.id
            WHERE
                TRUE
            ",
        );
        qb.filter("wp.name", "ilike", search);
        qb.push(
            "
            ORDER BY
                wp.name,
                tpw.sequence,
                ep.sequence
            ",
        );
        let rows = qb.build().fetch_all(pool).await?;

        let mut workout_plans_map: IndexMap<Uuid, WorkoutPlanQuery> = IndexMap::new();
        for row in rows {
            let workout_id: Uuid = row.get("workout_id");
            let workout_plan_entry = workout_plans_map
                .entry(workout_id)
                .or_insert_with(|| WorkoutPlanQuery::from_row(&row).unwrap());

            if let Some(_exercise_id) = row.get::<Option<Uuid>, _>("exercise_id") {
                let exercise_plan = ExercisePlanQuery::from_row(&row).unwrap();
                workout_plan_entry.exercise_plans.push(exercise_plan);
            }
        }

        let workout_plans = workout_plans_map
            .into_values()
            .collect::<Vec<WorkoutPlanQuery>>();
        Ok(workout_plans)
    }
}
