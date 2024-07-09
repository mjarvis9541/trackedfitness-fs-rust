use sqlx::PgPool;
use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::error::{handle_sqlx_contraint_error, Result};
use crate::util::database::Filter;
use crate::util::server::{normalize_whitespace, slugify};

use super::model::{Movement, MovementQuery, MovementWithLatestWeight};

impl Movement {
    const BASE_NAME: &'static str = "Exercise";

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM movement WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        muscle_group_id: Uuid,
        name: &str,
        created_by_id: Uuid,
    ) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "
            INSERT INTO
                movement (name, slug, muscle_group_id, created_by_id)
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                *
            ",
            normalized_name,
            slug,
            muscle_group_id,
            created_by_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(err, Self::BASE_NAME, "name", &["slug_key", "name_key"])
        })?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: &str,
        muscle_group_id: Uuid,
        updated_by_id: Uuid,
    ) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            r#"
            UPDATE movement
            SET
                name = $1,
                slug = $2,
                muscle_group_id = $3,
                updated_at = NOW(),
                updated_by_id = $4
            WHERE
                id = $5
            RETURNING
                *
            "#,
            normalized_name,
            slug,
            muscle_group_id,
            updated_by_id,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(err, Self::BASE_NAME, "name", &["slug_key", "name_key"])
        })?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(Self, "DELETE FROM movement WHERE id = $1 returning *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }
}

impl MovementQuery {
    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.*,
                t4.name AS muscle_group_name,
                t4.slug AS muscle_group_slug,
                t2.username AS created_by,
                t3.username AS "updated_by?"
            FROM
                movement t1
                LEFT JOIN users_user t2 ON t1.created_by_id = t2.id
                LEFT JOIN users_user t3 ON t1.updated_by_id = t3.id
                LEFT JOIN muscle_group t4 ON t1.muscle_group_id = t4.id
            WHERE 
                t1.slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str, muscle_group_slug: &str) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                COUNT(t1.*)
            FROM
                movement t1
                LEFT JOIN muscle_group t2 ON t2.id = t1.muscle_group_id
            WHERE
                TRUE
            ",
        );
        qb.filter("t1.name", "ilike", search);
        qb.filter("t2.slug", "=", muscle_group_slug);
        Ok(qb.build_query_scalar().fetch_one(pool).await?)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        muscle_group_slug: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let order_by_column = match order {
            "name" => "t1.name",
            "-name" => "t1.name DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            "updated_at" => "t1.updated_at",
            "-updated_at" => "t1.updated_at DESC",
            _ => "t1.name",
        };

        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.*,
                t4.name as muscle_group_name,
                t4.slug as muscle_group_slug,
                t2.username as created_by,
                t3.username as updated_by
            FROM
                movement t1
                LEFT JOIN users_user t2 ON t1.created_by_id = t2.id
                LEFT JOIN users_user t3 ON t1.updated_by_id = t3.id
                LEFT JOIN muscle_group t4 ON t1.muscle_group_id = t4.id
            WHERE
                TRUE
            ",
        );
        qb.filter("t1.name", "ilike", search);
        qb.filter("t4.slug", "=", muscle_group_slug);

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);
        Ok(qb.build_query_as().fetch_all(pool).await?)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        let query = sqlx::query_as!(
            SelectUuidName,
            r#"SELECT id, name FROM movement ORDER BY name LIMIT 1000"#
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}

impl MovementWithLatestWeight {
    pub async fn with_latest_weight(
        pool: &PgPool,
        username: &str,
        search: &str,
        muscle_group_slug: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let order_by_column = match order {
            "movement_name" => "movement_name",
            "-movement_name" => "movement_name DESC",
            "muscle_group_name" => "muscle_group_name",
            "-muscle_group_name" => "muscle_group_name DESC",
            "created_at" => "created_at",
            "-created_at" => "created_at DESC",
            "updated_at" => "updated_at",
            "-updated_at" => "updated_at DESC",
            _ => "movement_name",
        };

        let mut qb = sqlx::QueryBuilder::new(&format!(
            "
            WITH
                user_movements AS (
                    SELECT
                        e.movement_id,
                        ts.weight,
                        ts.reps,
                        w.date AS workout_date,
                        COUNT(ts.id) OVER (
                            PARTITION BY
                                e.movement_id,
                                w.date
                        ) AS number_of_sets,
                        ROW_NUMBER() OVER (
                            PARTITION BY
                                e.movement_id
                            ORDER BY
                                w.date DESC
                        ) AS rn
                    FROM
                        workout w
                        JOIN exercise e ON w.id = e.workout_id
                        JOIN tracked_set ts ON e.id = ts.exercise_id
                        JOIN users_user u ON w.user_id = u.id
                    WHERE
                        u.username = '{username}'
                )
            SELECT
                m.id AS movement_id,
                m.name AS movement_name,
                m.slug AS movement_slug,
                mg.name AS muscle_group_name,
                mg.slug AS muscle_group_slug,
                COALESCE(um.workout_date, '1970-01-01') AS latest_workout_date,
                COALESCE(um.weight, 0)::numeric AS latest_exercise_weight,
                COALESCE(um.number_of_sets, 0)::int8 AS latest_exercise_sets,
                COALESCE(um.reps, 0)::int8 AS latest_exercise_reps
            FROM
                movement m
                LEFT JOIN user_movements um ON m.id = um.movement_id
                AND um.rn = 1
                LEFT JOIN muscle_group mg ON mg.id = m.muscle_group_id
            WHERE
                TRUE 
            ",
        ));
        qb.filter("m.name", "ilike", search);
        qb.filter("mg.slug", "=", muscle_group_slug);

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);

        let results = qb.build_query_as().fetch_all(pool).await?;
        Ok(results)
    }
}
