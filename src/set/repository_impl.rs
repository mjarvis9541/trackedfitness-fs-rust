use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::exercise::model::ExerciseBase;
// use crate::exercise_plan::model::ExercisePlan;

use super::model::{MergedSetInputData, SetModel, SetToCreate};

impl SetModel {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM tracked_set WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        exercise_id: Uuid,
        order: i32,
        weight: Decimal,
        reps: i32,
        rest: i32,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            r#"INSERT INTO tracked_set (exercise_id, "order", weight, reps, rest, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
            exercise_id,
            order,
            weight,
            reps,
            rest,
            request_user_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        set_id: Uuid,
        exercise_id: Uuid,
        order: i32,
        weight: Decimal,
        reps: i32,
        rest: i32,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            r#"UPDATE tracked_set SET
                exercise_id = $1,
                "order" = $2,
                weight = $3,
                reps = $4,
                rest = $5,
                updated_at = NOW(),
                updated_by_id = $6
            WHERE id = $7 RETURNING *"#,
            exercise_id,
            order,
            weight,
            reps,
            rest,
            request_user_id,
            set_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM tracked_set WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn bulk_create(
        pool: &PgPool,
        exercise_id: Uuid,
        weight: Decimal,
        reps: i32,
        rest: i32,
        set_count: i32,
        request_user_id: Uuid,
    ) -> Result<Vec<Self>> {
        let order_vec: Vec<i32> = (1..=set_count).collect();
        let query = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO
                tracked_set (
                    exercise_id,
                    "order",
                    weight,
                    reps,
                    rest,
                    created_by_id
                )
            SELECT
                $1,
                UNNEST($2::int[]),
                $3,
                $4,
                $5,
                $6
            RETURNING
                *
            "#,
            exercise_id,
            &order_vec,
            weight,
            reps,
            rest,
            request_user_id,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }

    pub async fn bulk_create_from_set_input_data_vec(
        pool: &PgPool,
        merged_vec: &[MergedSetInputData],
        request_user_id: Uuid,
    ) -> Result<u64> {
        let query = sqlx::query(
            r#"
            INSERT INTO
                tracked_set (
                    exercise_id,
                    "order",
                    weight,
                    reps,
                    rest,
                    created_by_id
                )
            SELECT
                UNNEST($1::uuid[]),
                UNNEST($2::int[]),
                UNNEST($3::decimal[]),
                UNNEST($4::int[]),
                UNNEST($5::int[]),
                $6
                "#,
        )
        .bind(merged_vec.iter().map(|s| s.exercise_id).collect::<Vec<_>>())
        .bind(merged_vec.iter().map(|s| s.set_order).collect::<Vec<_>>())
        .bind(merged_vec.iter().map(|s| s.weight).collect::<Vec<_>>())
        .bind(merged_vec.iter().map(|s| s.reps).collect::<Vec<_>>())
        .bind(merged_vec.iter().map(|s| s.rest).collect::<Vec<_>>())
        .bind(request_user_id)
        .execute(pool)
        .await?
        .rows_affected();
        Ok(query)
    }
}

// impl SetToCreate {
//     pub fn from_exercise_plan(exercise_plans: &[ExercisePlan]) -> Vec<SetToCreate> {
//         exercise_plans
//             .iter()
//             .enumerate()
//             .flat_map(|(index, exercise)| {
//                 let exercise_num = index as i32 + 1;
//                 (0..exercise.sets)
//                     .map(|_| SetToCreate {
//                         exercise_num,
//                         weight: exercise.weight,
//                         reps: exercise.reps,
//                         rest: exercise.rest,
//                     })
//                     .collect::<Vec<_>>()
//             })
//             .collect()
//     }
// }

impl MergedSetInputData {
    pub fn merge_from_exercise_sets(
        exercises: &[ExerciseBase],
        sets: &[SetToCreate],
    ) -> Vec<MergedSetInputData> {
        sets.iter()
            .filter_map(|set| {
                exercises
                    .iter()
                    .find(|&ex| ex.order == set.exercise_num)
                    .map(|ex| MergedSetInputData {
                        exercise_id: ex.id,
                        exercise_num: ex.order,
                        set_order: set.exercise_num,
                        weight: set.weight,
                        reps: set.reps,
                        rest: set.rest,
                    })
            })
            .collect()
    }
}
