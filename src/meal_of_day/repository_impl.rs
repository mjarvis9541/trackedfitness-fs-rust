use sqlx::PgPool;
use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::error::Result;
use crate::util::database::Filter;
use crate::util::server::{normalize_whitespace, slugify};

use super::model::MealOfDay;

impl MealOfDay {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM meal_of_day ORDER BY ordering")
            .fetch_all(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM meal_of_day WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM meal_of_day WHERE slug = $1", slug)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        ordering: i32,
        created_by_id: Uuid,
    ) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO meal_of_day (name, ordering, slug, created_by_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *",
            normalized_name,
            ordering,
            slug,
            created_by_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: &str,
        ordering: i32,
        updated_by_id: Uuid,
    ) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "UPDATE meal_of_day
            SET
                name = $1,
                ordering = $2,
                slug = $3,
                updated_at = NOW(),
                updated_by_id = $4
            WHERE
                id = $5
            RETURNING *",
            normalized_name,
            ordering,
            slug,
            updated_by_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Self> {
        let query = sqlx::query_as!(
            Self,
            "DELETE FROM meal_of_day WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str) -> Result<i64> {
        let mut qbc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM meal_of_day t1 WHERE TRUE");
        qbc.filter("t1.name", "ilike", &search);
        let query = qbc.build_query_scalar().fetch_one(pool).await?;
        Ok(query)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let order_by_column = match order {
            "name" => "t1.name",
            "-name" => "t1.name DESC",
            "ordering" => "t1.ordering",
            "-ordering" => "t1.ordering DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            "updated_at" => "t1.updated_at",
            "-updated_at" => "t1.updated_at DESC",
            _ => "t1.ordering",
        };

        let mut qb = sqlx::QueryBuilder::new("SELECT t1.* FROM meal_of_day t1 WHERE TRUE");
        qb.filter("t1.name", "ilike", search);

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);
        let query = qb.build_query_as().fetch_all(pool).await?;
        Ok(query)
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        let query = sqlx::query_as!(
            SelectUuidName,
            "SELECT id, name FROM meal_of_day ORDER BY ordering LIMIT 1000"
        )
        .fetch_all(pool)
        .await?;
        Ok(query)
    }
}
