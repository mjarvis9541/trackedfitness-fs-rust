use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::brand::model::{Brand, BrandQuery};
use crate::component::select::{SelectSlugName, SelectUuidName};
use crate::error::{handle_sqlx_contraint_error, Result};
use crate::util::server::{normalize_whitespace, slugify};

impl Brand {
    const BASE_NAME: &'static str = "Brand";

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM food_brand WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM food_brand WHERE slug = $1", slug)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn create(pool: &PgPool, name: &str, created_by_id: Uuid) -> Result<Self> {
        let normalized_name = normalize_whitespace(name);
        let slug = slugify(name);
        let query = sqlx::query_as!(
            Self,
            "INSERT INTO food_brand (name, slug, created_by_id)
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
            "UPDATE food_brand SET name = $1, slug = $2, updated_at = NOW(), updated_by_id = $3
            WHERE id = $4 RETURNING *",
            normalized_name,
            slug,
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
        let query = sqlx::query_as!(Self, "DELETE FROM food_brand WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;
        Ok(query)
    }

    pub async fn update_image_url(
        pool: &PgPool,
        slug: &str,
        file_name: &str,
        request_user_id: Uuid,
    ) -> Result<Self> {
        let image_url = format!("/images/brands/{}", file_name);
        dbg!(&image_url);
        let query = sqlx::query_as!(
            Self,
            "UPDATE food_brand SET image_url = $1, updated_at = NOW(), updated_by_id = $2 WHERE slug = $3 RETURNING *",
            image_url,
            request_user_id,
            slug,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }
}

impl BrandQuery {
    const SELECT_ALL_FIELDS: &'static str = "
        SELECT
            t1.*,
            t2.username AS created_by,
            t3.username as updated_by,
            COALESCE(t4.count, 0) AS food_count
        FROM
            food_brand t1
            LEFT JOIN users_user t2 ON t2.id = t1.created_by_id
            LEFT JOIN users_user t3 ON t3.id = t1.updated_by_id
            LEFT JOIN (
                SELECT
                    brand_id,
                    COUNT(*) AS count
                FROM
                    food
                GROUP BY
                    brand_id
            ) AS t4 ON t4.brand_id = t1.id
        WHERE
    ";

    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as(
            "
        SELECT
            t1.*,
            t2.username AS created_by,
            t3.username as updated_by,
            COALESCE(t4.count, 0) AS food_count
        FROM
            food_brand t1
            LEFT JOIN users_user t2 ON t2.id = t1.created_by_id
            LEFT JOIN users_user t3 ON t3.id = t1.updated_by_id
            LEFT JOIN (
                SELECT
                    brand_id,
                    COUNT(*) AS count
                FROM
                    food
                GROUP BY
                    brand_id
            ) AS t4 ON t4.brand_id = t1.id
        WHERE
            t1.slug = $1
        ",
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str) -> sqlx::Result<i64> {
        if search.is_empty() {
            sqlx::query_scalar(r#"SELECT COUNT(*) FROM food_brand"#)
                .fetch_one(pool)
                .await
        } else {
            sqlx::query_scalar(r#"SELECT COUNT(*) FROM food_brand WHERE name ILIKE $1"#)
                .bind(format!("%{}%", search))
                .fetch_one(pool)
                .await
        }
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order_by: &str,
        size: i64,
        page: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let order_by_column = match order_by {
            "name" => "t1.name",
            "-name" => "t1.name DESC",
            "food_count" => "food_count",
            "-food_count" => "food_count DESC",
            "created_at" => "t1.created_at",
            "-created_at" => "t1.created_at DESC",
            "updated" => "t1.updated_at",
            "-updated" => "t1.updated_at DESC",
            _ => "t1.created_at",
        };
        let query = if search.is_empty() {
            format!(
                "{} TRUE ORDER BY {} LIMIT $1 OFFSET $2",
                Self::SELECT_ALL_FIELDS,
                order_by_column
            )
        } else {
            format!(
                "{} t1.name ILIKE $1 ORDER BY {} LIMIT $2 OFFSET $3",
                Self::SELECT_ALL_FIELDS,
                order_by_column
            )
        };
        if search.is_empty() {
            sqlx::query_as(&query)
                .bind(size)
                .bind(size * (page - 1))
                .fetch_all(pool)
                .await
        } else {
            sqlx::query_as(&query)
                .bind(format!("%{}%", search))
                .bind(size)
                .bind(size * (page - 1))
                .fetch_all(pool)
                .await
        }
    }

    pub async fn option_list_id(pool: &PgPool) -> Result<Vec<SelectUuidName>> {
        Ok(sqlx::query_as!(
            SelectUuidName,
            r#"SELECT id, name FROM food_brand ORDER BY name LIMIT 1000"#
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn option_list_slug(pool: &PgPool) -> Result<Vec<SelectSlugName>> {
        Ok(sqlx::query_as!(
            SelectSlugName,
            r#"
            SELECT t1.slug, CONCAT(t1.name, ' (', COUNT(t2.*), ')') AS "name!"
            FROM food_brand t1
            LEFT JOIN food t2 ON t2.brand_id = t1.id
            GROUP BY t1.id
            ORDER BY t1.name LIMIT 1000
            "#,
        )
        .fetch_all(pool)
        .await?)
    }
}

impl FromRow<'_, PgRow> for BrandQuery {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            image_url: row.try_get("image_url")?,
            food_count: row.try_get("food_count")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by_id: row.try_get("created_by_id")?,
            updated_by_id: row.try_get("updated_by_id")?,
            created_by: row.try_get("created_by")?,
            updated_by: row.try_get("updated_by")?,
        })
    }
}

impl FromRow<'_, PgRow> for Brand {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            image_url: row.try_get("image_url")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by_id: row.try_get("created_by_id")?,
            updated_by_id: row.try_get("updated_by_id")?,
        })
    }
}
