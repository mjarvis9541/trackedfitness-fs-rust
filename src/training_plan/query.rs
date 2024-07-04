use leptos::ServerFnError;

use indexmap::IndexMap;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::error::Error;
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::training_plan::model::{TrainingPlan, TrainingPlanInput, TrainingPlanQuery};
use crate::util::database::Filter;
use crate::util::server::slugify;
use crate::workout_plan::model::WorkoutPlanQuery;

impl FromRow<'_, PgRow> for TrainingPlanQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("training_id")?,
            user_id: row.try_get("training_user_id")?,
            name: row.try_get("training_name")?,
            slug: row.try_get("training_slug")?,
            duration_weeks: row.try_get("training_duration_weeks")?,
            description: row.try_get("training_description")?,
            created_at: row.try_get("training_created_at")?,
            updated_at: row.try_get("training_updated_at")?,
            created_by_id: row.try_get("training_created_by_id")?,
            updated_by_id: row.try_get("training_updated_by_id")?,
            workout_count: row.try_get("training_workout_count").unwrap_or(0),
            exercise_count: row.try_get("training_exercise_count").unwrap_or(0),
            set_count: row.try_get("training_set_count").unwrap_or(0),
            rep_count: row.try_get("training_rep_count").unwrap_or(0),
            workout_plans: Vec::new(),
        })
    }
}

impl TrainingPlanQuery {
    pub async fn get_all_with_workout_plans(pool: &PgPool) -> sqlx::Result<Vec<Self>> {
        let rows = sqlx::query(
            "
            WITH
            training_plan_aggregation AS (
                SELECT
                    wtj.training_plan_id,
                    COUNT(DISTINCT wp.id) AS workout_count,
                    COUNT(DISTINCT ep.id) AS exercise_count,
                    SUM(ep.sets) AS set_count,
                    SUM(ep.reps) AS rep_count
                FROM
                    training_plan_workout_plan wtj
                    LEFT JOIN workout_plan wp ON wp.id = wtj.workout_plan_id
                    LEFT JOIN exercise_plan ep ON wp.id = ep.workout_plan_id
                GROUP BY
                    wtj.training_plan_id
            ),
            workout_plan_aggregation AS (
                SELECT
                    wp.id AS workout_plan_id,
                    COUNT(DISTINCT ep.id) AS exercise_count,
                    SUM(ep.sets) AS set_count,
                    SUM(ep.reps) AS rep_count
                FROM
                    workout_plan wp
                    LEFT JOIN exercise_plan ep ON wp.id = ep.workout_plan_id
                GROUP BY
                    wp.id
            )
            SELECT
                tp.id AS training_id,
                tp.user_id AS training_user_id,
                tp.name AS training_name,
                tp.slug AS training_slug,
                tp.duration_weeks AS training_duration_weeks,
                tp.description AS training_description,
                tp.created_at AS training_created_at,
                tp.updated_at AS training_updated_at,
                tp.created_by_id AS training_created_by_id,
                tp.updated_by_id AS training_updated_by_id,
                --
                wp.id AS workout_id,
                wp.user_id AS workout_user_id,
                wtj.sequence AS workout_sequence,
                wtj.weekday AS workout_weekday,
                wp.name AS workout_name,
                wp.slug AS workout_slug,
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
                mv.name AS exercise_movement_name,
                --
                COALESCE(wpa.exercise_count, 0) AS workout_exercise_count,
                COALESCE(wpa.set_count, 0) AS workout_set_count,
                COALESCE(wpa.rep_count, 0) AS workout_rep_count,
                --
                COALESCE(tpa.workout_count, 0) AS training_workout_count,
                COALESCE(tpa.exercise_count, 0) AS training_exercise_count,
                COALESCE(tpa.set_count, 0) AS training_set_count,
                COALESCE(tpa.rep_count, 0) AS training_rep_count
            FROM
                training_plan tp
                LEFT JOIN training_plan_workout_plan wtj ON wtj.training_plan_id = tp.id
                LEFT JOIN workout_plan wp ON wp.id = wtj.workout_plan_id
                LEFT JOIN exercise_plan ep ON ep.workout_plan_id = wp.id
                LEFT JOIN movement mv ON mv.id = ep.movement_id
                LEFT JOIN workout_plan_aggregation wpa ON wpa.workout_plan_id = wp.id
                LEFT JOIN training_plan_aggregation tpa ON tpa.training_plan_id = tp.id
            ORDER BY
                tp.name,
                wtj.weekday,
                wtj.sequence,
                wp.created_at,
                ep.sequence,
                ep.created_at
            ",
        )
        .fetch_all(pool)
        .await?;

        let mut training_plans = Vec::<TrainingPlanQuery>::new();
        for row in rows {
            let training_id = row.get("training_id");
            let training_index = training_plans
                .iter()
                .position(|tp| tp.id == training_id)
                .unwrap_or_else(|| {
                    let training = TrainingPlanQuery::from_row(&row).unwrap_or_default();
                    training_plans.push(training);
                    training_plans.len() - 1
                });

            if let Some(workout_id) = row.get("workout_id") {
                let workout_index = training_plans[training_index]
                    .workout_plans
                    .iter()
                    .position(|wp| wp.id == workout_id)
                    .unwrap_or_else(|| {
                        let workout = WorkoutPlanQuery::from_row(&row).unwrap_or_default();
                        training_plans[training_index].workout_plans.push(workout);
                        training_plans[training_index].workout_plans.len() - 1
                    });

                if let Some(_exercise_id) = row.get::<Option<Uuid>, _>("exercise_id") {
                    let exercise = ExercisePlanQuery::from_row(&row).unwrap_or_default();
                    training_plans[training_index].workout_plans[workout_index]
                        .exercise_plans
                        .push(exercise);
                }
            }
        }
        Ok(training_plans)
    }

    pub async fn get_with_workout_plans(pool: &PgPool, slug: &str) -> sqlx::Result<Option<Self>> {
        let rows = sqlx::query(
            "
            WITH
            training_plan_aggregation AS (
                SELECT
                    wtj.training_plan_id,
                    COUNT(DISTINCT wp.id) AS workout_count,
                    COUNT(DISTINCT ep.id) AS exercise_count,
                    SUM(ep.sets) AS set_count,
                    SUM(ep.reps) AS rep_count
                FROM
                    training_plan_workout_plan wtj
                    LEFT JOIN workout_plan wp ON wp.id = wtj.workout_plan_id
                    LEFT JOIN exercise_plan ep ON wp.id = ep.workout_plan_id
                GROUP BY
                    wtj.training_plan_id
            ),
            workout_plan_aggregation AS (
                SELECT
                    wp.id AS workout_plan_id,
                    COUNT(DISTINCT ep.id) AS exercise_count,
                    SUM(ep.sets) AS set_count,
                    SUM(ep.reps) AS rep_count
                FROM
                    workout_plan wp
                    LEFT JOIN exercise_plan ep ON wp.id = ep.workout_plan_id
                GROUP BY
                    wp.id
            )
            SELECT
                tp.id AS training_id,
                tp.user_id AS training_user_id,
                tp.name AS training_name,
                tp.slug AS training_slug,
                tp.duration_weeks AS training_duration_weeks,
                tp.description AS training_description,
                tp.created_at AS training_created_at,
                tp.updated_at AS training_updated_at,
                tp.created_by_id AS training_created_by_id,
                tp.updated_by_id AS training_updated_by_id,
                --
                wp.id AS workout_id,
                wp.user_id AS workout_user_id,
                wtj.sequence AS workout_sequence,
                wtj.weekday AS workout_weekday,
                wp.name AS workout_name,
                wp.slug AS workout_slug,
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
                mv.name AS exercise_movement_name,
                --
                COALESCE(wpa.exercise_count, 0) AS workout_exercise_count,
                COALESCE(wpa.set_count, 0) AS workout_set_count,
                COALESCE(wpa.rep_count, 0) AS workout_rep_count,
                --
                COALESCE(tpa.workout_count, 0) AS training_workout_count,
                COALESCE(tpa.exercise_count, 0) AS training_exercise_count,
                COALESCE(tpa.set_count, 0) AS training_set_count,
                COALESCE(tpa.rep_count, 0) AS training_rep_count
            FROM
                training_plan tp
                LEFT JOIN training_plan_workout_plan wtj ON wtj.training_plan_id = tp.id
                LEFT JOIN workout_plan wp ON wp.id = wtj.workout_plan_id
                LEFT JOIN exercise_plan ep ON ep.workout_plan_id = wp.id
                LEFT JOIN movement mv ON mv.id = ep.movement_id
                LEFT JOIN workout_plan_aggregation wpa ON wpa.workout_plan_id = wp.id
                LEFT JOIN training_plan_aggregation tpa ON tpa.training_plan_id = tp.id
            WHERE
                tp.slug = $1
            ORDER BY
                wtj.sequence,
                ep.sequence;
            ",
        )
        .bind(slug)
        .fetch_all(pool)
        .await?;

        let mut training_plan: Option<TrainingPlanQuery> = None;
        let mut workouts = IndexMap::new();

        for row in &rows {
            if training_plan.is_none() {
                training_plan = Some(TrainingPlanQuery::from_row(&row)?);
            }

            if let Some(workout_id) = row.get::<Option<Uuid>, _>("workout_id") {
                let workout = WorkoutPlanQuery::from_row(&row).unwrap();
                let workout = workouts.entry(workout_id).or_insert_with(|| workout);

                if let Some(_exercise_id) = row.get::<Option<Uuid>, _>("exercise_id") {
                    let exercise = ExercisePlanQuery::from_row(&row).unwrap_or_default();
                    workout.exercise_plans.push(exercise);
                }
            }
        }
        if let Some(ref mut plan) = training_plan {
            plan.workout_plans = workouts.into_values().collect();
        }
        Ok(training_plan)
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
            WITH
            training_plan_aggregation AS (
                SELECT
                    wtj.training_plan_id,
                    COUNT(DISTINCT wp.id) AS workout_count,
                    COUNT(DISTINCT ep.id) AS exercise_count,
                    SUM(ep.sets) AS set_count,
                    SUM(ep.reps) AS rep_count
                FROM
                    training_plan_workout_plan wtj
                    LEFT JOIN workout_plan wp ON wp.id = wtj.workout_plan_id
                    LEFT JOIN exercise_plan ep ON wp.id = ep.workout_plan_id
                GROUP BY
                    wtj.training_plan_id
            )
            SELECT
                t1.id as training_id,
                t1.user_id as training_user_id,
                t1.name as training_name,
                t1.slug as training_slug,
                t1.duration_weeks as training_duration_weeks,
                t1.description as training_description,
                t1.created_at as training_created_at,
                t1.updated_at as training_updated_at,
                t1.created_by_id as training_created_by_id,
                t1.updated_by_id as training_updated_by_id,
                COALESCE(t2.workout_count, 0) as training_workout_count,
                COALESCE(t2.exercise_count, 0) as training_exercise_count,
                COALESCE(t2.set_count, 0) as training_set_count,
                COALESCE(t2.rep_count, 0) as training_rep_count
            FROM 
                training_plan t1
                LEFT JOIN training_plan_aggregation t2 ON t2.training_plan_id = t1.id
            WHERE 
                TRUE
            ",
        );
        qb.filter("t1.name", "ilike", search);
        qb.order("t1.name", order);
        qb.paginate(size, page);
        let results = qb.build_query_as().fetch_all(pool).await?;
        Ok(results)
    }

    pub async fn count(pool: &PgPool, search: &str) -> sqlx::Result<i64> {
        let mut qb = sqlx::QueryBuilder::new("SELECT COUNT(t1.*) FROM training_plan t1 WHERE TRUE");
        qb.filter("t1.name", "ilike", search);
        let count = qb.build_query_scalar().fetch_one(pool).await?;
        Ok(count)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>, sqlx::Error> {
        sqlx::query_as!(
            SelectUuidName,
            "SELECT id, name FROM training_plan ORDER BY name LIMIT 1000"
        )
        .fetch_all(pool)
        .await
    }
}

impl TrainingPlan {
    pub async fn try_get(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM training_plan WHERE id = $1", id)
            .fetch_optional(pool)
            .await
    }
    pub async fn get_object_or_404(pool: &PgPool, id: Uuid) -> Result<Self, ServerFnError> {
        let query = Self::try_get(pool, id).await?.ok_or(Error::NotFound)?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        data: &TrainingPlanInput,
        request_user_id: Uuid,
    ) -> sqlx::Result<Self> {
        let slug = slugify(&data.name);
        sqlx::query_as!(
            Self,
            "INSERT INTO
                training_plan (
                    user_id,
                    name,
                    slug,
                    duration_weeks,
                    description,
                    created_by_id
                )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *",
            user_id,
            data.name,
            slug,
            data.duration_weeks,
            data.description,
            request_user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        data: &TrainingPlanInput,
        request_user_id: Uuid,
    ) -> sqlx::Result<Self> {
        let slug = slugify(&data.name);
        sqlx::query_as!(
            Self,
            "
            UPDATE training_plan
            SET
                name = $2,
                slug = $3,
                duration_weeks = $4,
                description = $5,
                updated_by_id = $6,
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            ",
            id,
            data.name,
            slug,
            data.duration_weeks,
            data.description,
            request_user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "DELETE FROM training_plan WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
    }
}
