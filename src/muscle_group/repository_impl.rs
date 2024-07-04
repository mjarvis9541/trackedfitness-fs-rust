use super::model::{MuscleGroup, MuscleGroupBase};
use crate::component::select::{SelectSlugName, SelectUuidName};
use crate::error::{handle_sqlx_contraint_error, Result};
use crate::util::database::Filter;
use crate::util::server::{normalize_whitespace, slugify};
use sqlx::PgPool;
use uuid::Uuid;

impl MuscleGroupBase {
    const BASE_NAME: &'static str = "Muscle group";

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM muscle_group WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(pool: &PgPool, name: &str, created_by_id: Uuid) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO muscle_group (name, slug, created_by_id)
            VALUES ($1, $2, $3) RETURNING *",
            normalized_name,
            slug,
            created_by_id
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(err, Self::BASE_NAME, "name", &["slug_key", "name_key"])
        })?;
        Ok(query)
    }

    pub async fn update(pool: &PgPool, id: Uuid, name: &str, updated_by_id: Uuid) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "UPDATE muscle_group SET name = COALESCE($1, name), slug = COALESCE($2, slug),
            updated_by_id = $3, updated_at = NOW() WHERE id = $4 RETURNING *",
            normalized_name,
            slug,
            updated_by_id,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            handle_sqlx_contraint_error(err, Self::BASE_NAME, "name", &["slug_key", "name_key"])
        })?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM muscle_group WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}

impl MuscleGroup {
    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT
                t1.*,
                t2.username AS created_by,
                t3.username AS "updated_by?",
                COALESCE(t4.count, 0) AS "exercise_count!"
            FROM
                muscle_group t1
                LEFT JOIN users_user t2 ON t1.created_by_id = t2.id
                LEFT JOIN users_user t3 ON t1.updated_by_id = t3.id
                LEFT JOIN (SELECT muscle_group_id, COUNT(*) AS count FROM movement GROUP BY muscle_group_id) AS t4 ON t4.muscle_group_id = t1.id
            WHERE
                t1.slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new("SELECT COUNT(t1.*) FROM muscle_group t1 WHERE TRUE");
        qb.filter("t1.name", "ilike", search);
        let query = qb.build_query_scalar().fetch_one(pool).await?;
        Ok(query)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"
            SELECT
                t1.*,
                t2.username AS created_by,
                t3.username AS updated_by,
                COALESCE(t4.count, 0) AS exercise_count
            FROM
                muscle_group t1
                LEFT JOIN users_user t2 ON t1.created_by_id = t2.id
                LEFT JOIN users_user t3 ON t1.updated_by_id = t3.id
                LEFT JOIN (SELECT muscle_group_id, COUNT(*) AS count FROM movement GROUP BY muscle_group_id) AS t4 ON t4.muscle_group_id = t1.id
            WHERE
                TRUE
            "#,
        );
        qb.filter("t1.name", "ilike", search);
        qb.order("t1.name", order);
        qb.paginate(size, page);
        let query = qb.build_query_as().fetch_all(pool).await?;
        Ok(query)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        let query = sqlx::query_as!(
            SelectUuidName,
            r#"
            SELECT t1.id, CONCAT(t1.name, ' (', COUNT(t2.*), ')') AS "name!"
            FROM muscle_group t1 
            LEFT JOIN movement t2 ON t1.id = t2.muscle_group_id
            GROUP BY t1.id 
            ORDER BY t1.name LIMIT 1000
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }

    pub async fn option_list_slug(pool: &PgPool) -> sqlx::Result<Vec<SelectSlugName>> {
        let query = sqlx::query_as!(
            SelectSlugName,
            r#"
            SELECT t1.slug, CONCAT(t1.name, ' (', COUNT(t2.*), ')') AS "name!"
            FROM muscle_group t1 
            LEFT JOIN movement t2 ON t1.id = t2.muscle_group_id
            GROUP BY t1.id 
            ORDER BY t1.name LIMIT 1000
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}
