use rust_decimal::Decimal;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::Result;
use crate::food::data_measurement::DataMeasurement;
use crate::food::model::Food;
use crate::util::database::Filter;
use crate::util::server::{normalize_whitespace, slugify};

impl FromRow<'_, PgRow> for Food {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let value: String = row.try_get("data_measurement")?;
        let data_measurement = DataMeasurement::from(value);

        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            brand_id: row.try_get("brand_id")?,
            data_value: row.try_get("data_value")?,
            data_measurement,
            energy: row.try_get("energy")?,
            fat: row.try_get("fat")?,
            saturates: row.try_get("saturates")?,
            carbohydrate: row.try_get("carbohydrate")?,
            sugars: row.try_get("sugars")?,
            fibre: row.try_get("fibre")?,
            protein: row.try_get("protein")?,
            salt: row.try_get("salt")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by_id: row.try_get("created_by_id")?,
            updated_by_id: row.try_get("updated_by_id")?,
            protein_pct: row.try_get("protein_pct")?,
            carbohydrate_pct: row.try_get("carbohydrate_pct")?,
            fat_pct: row.try_get("fat_pct")?,
            brand_name: row.try_get("brand_name")?,
            brand_slug: row.try_get("brand_slug")?,
            brand_image_url: row.try_get("brand_image_url")?,
            food_code: row.try_get("food_code")?,
            food_description: row.try_get("food_description")?,
            food_category: row.try_get("food_category")?,
            food_data_source: row.try_get("food_data_source")?,
            data_value_numeric: row.try_get("data_value_numeric")?,
            last_added_quantity: row.try_get("last_added_quantity")?,
            last_added_date: row.try_get("last_added_date")?,
        })
    }
}

impl Food {
    pub fn create_slug(
        name: &str,
        brand_name: &str,
        data_value: i32,
        data_measurement: &str,
    ) -> String {
        slugify(&format!(
            "{}-{}-{}{}",
            name, brand_name, data_value, data_measurement
        ))
    }

    pub async fn create(
        pool: &PgPool,
        name: &String,
        brand_id: Uuid,
        brand_name: &str,
        serving: String,
        energy: i32,
        fat: Decimal,
        saturates: Decimal,
        carbohydrate: Decimal,
        sugars: Decimal,
        fibre: Decimal,
        protein: Decimal,
        salt: Decimal,
        created_by_id: Uuid,
    ) -> Result<String> {
        let data_measurement = DataMeasurement::from(serving);
        let data_value = data_measurement.to_data_value();
        let data_measurement_str = data_measurement.to_string();

        let normalized_name = normalize_whitespace(&name);
        let slug = Self::create_slug(
            &normalized_name,
            brand_name,
            data_value,
            &data_measurement_str,
        );
        let query = sqlx::query_scalar!(
            r#"
            INSERT INTO
                food (
                    name,
                    slug,
                    brand_id,
                    data_value,
                    data_measurement,
                    energy,
                    fat,
                    saturates,
                    carbohydrate,
                    sugars,
                    fibre,
                    protein,
                    salt,
                    created_by_id
                )
            VALUES
                (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10,
                    $11,
                    $12,
                    $13,
                    $14
                )
            RETURNING
                slug
            "#,
            normalized_name,
            slug,
            brand_id,
            data_value,
            data_measurement_str,
            energy,
            fat,
            saturates,
            carbohydrate,
            sugars,
            fibre,
            protein,
            salt,
            created_by_id
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: &String,
        brand_id: Uuid,
        brand_name: &str,
        serving: String,
        energy: i32,
        fat: Decimal,
        saturates: Decimal,
        carbohydrate: Decimal,
        sugars: Decimal,
        fibre: Decimal,
        protein: Decimal,
        salt: Decimal,
        updated_by_id: Uuid,
    ) -> Result<String> {
        let data_measurement = DataMeasurement::from(serving);
        let data_value = data_measurement.to_data_value();
        let data_measurement_str = data_measurement.to_string();

        let normalized_name = normalize_whitespace(&name);
        let slug = Self::create_slug(
            &normalized_name,
            brand_name,
            data_value,
            &data_measurement_str,
        );
        let query = sqlx::query_scalar!(
            r#"
            UPDATE food
            SET
                name = $1,
                slug = $2,
                brand_id = $3,
                data_value = $4,
                data_measurement = $5,
                energy = $6,
                fat = $7,
                saturates = $8,
                carbohydrate = $9,
                sugars = $10,
                fibre = $11,
                protein = $12,
                salt = $13,
                updated_at = NOW(),
                updated_by_id = $14
            WHERE
                id = $15
            RETURNING
                slug
            "#,
            normalized_name,
            slug,
            brand_id,
            data_value,
            data_measurement_str,
            energy,
            fat,
            saturates,
            carbohydrate,
            sugars,
            fibre,
            protein,
            salt,
            updated_by_id,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(query)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64> {
        Ok(sqlx::query!("DELETE FROM food WHERE id = $1", id)
            .execute(pool)
            .await?
            .rows_affected())
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let query = sqlx::query_as(
            r#"
            SELECT
                t1.*,
                t2.name AS brand_name,
                t2.slug AS brand_slug,
                t2.image_url AS brand_image_url,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS protein_pct,
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS carbohydrate_pct,
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS fat_pct,
                null AS last_added_quantity,
                null AS last_added_date
            FROM
                food t1
                LEFT JOIN food_brand t2 ON t1.brand_id = t2.id
            WHERE t1.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let query = sqlx::query_as(
            r#"
            SELECT
                t1.*,
                t2.name AS brand_name,
                t2.slug AS brand_slug,
                t2.image_url AS brand_image_url,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS protein_pct,
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS carbohydrate_pct,
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS fat_pct,
                null AS last_added_quantity,
                null AS last_added_date
            FROM
                food t1
                LEFT JOIN food_brand t2 ON t1.brand_id = t2.id
            WHERE
                t1.slug = $1"#,
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;
        Ok(query)
    }

    pub async fn count(pool: &PgPool, search: &str, brand: &str, serving: &str) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT COUNT(t1.*) FROM food t1 
            LEFT JOIN food_brand t2 ON t2.id = t1.brand_id 
            WHERE TRUE
            ",
        );
        qb.filter("t1.name", "ilike", search);
        qb.filter("t2.slug", "=", brand);
        qb.filter("t1.data_measurement", "=", serving);
        let count = qb.build_query_scalar().fetch_one(pool).await?;
        Ok(count)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        brand: &str,
        serving: &str,
        user_id: Option<Uuid>,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let order_by_column = Self::get_order_by_column(order);
        let init = "
            SELECT
                t1.*,
                t2.name as brand_name,
                t2.slug as brand_slug,
                t2.image_url AS brand_image_url,
                COALESCE(t1.protein * 4 / NULLIF(t1.energy, 0), 0) * 100 AS protein_pct,
                COALESCE(t1.carbohydrate * 4 / NULLIF(t1.energy, 0), 0) * 100 AS carbohydrate_pct,
                COALESCE(t1.fat * 9 / NULLIF(t1.energy, 0), 0) * 100 AS fat_pct,
            ";
        let user_filter = if let Some(user_id) = user_id {
            format!(
                r#"
                {}
                t3.date as last_added_date,
                t3.quantity as last_added_quantity
            FROM 
                food t1
                LEFT JOIN food_brand t2 ON t2.id = t1.brand_id
                LEFT JOIN food_log t3 ON t3.food_id = t1.id
                AND t3.user_id = '{}'
                AND t3.created_at = (
                    SELECT
                        MAX(created_at)
                    FROM
                        food_log
                    WHERE
                        food_id = t1.id
                        AND user_id = '{}'
                )
            WHERE
                TRUE
                "#,
                init, user_id, user_id
            )
        } else {
            format!(
                r#"
                {}
                null as last_added_date,
                null as last_added_quantity
            FROM
                food t1
                    LEFT JOIN food_brand t2 ON t2.id = t1.brand_id
            WHERE
                TRUE
                "#,
                init
            )
        };
        let mut qb = sqlx::QueryBuilder::new(user_filter);
        qb.filter("t1.name", "ilike", search);
        qb.filter("t1.data_measurement", "=", serving);
        qb.filter("t2.slug", "=", brand);

        qb.push(" ORDER BY ");
        qb.push(format!("{} NULLS LAST", order_by_column));
        qb.paginate(size, page);

        Ok(qb.build_query_as().fetch_all(pool).await?)
    }
}

// let energy_numeric = Decimal::from(energy);
// let energy_numeric_with_buffer = energy_numeric + energy_numeric * Decimal::new(1, 1);

// let energy_from_macros =
//     protein * Decimal::from(4) + carbohydrate * Decimal::from(4) + fat * Decimal::from(9);
// let energy_from_macros_with_buffer =
//     energy_from_macros + energy_from_macros * Decimal::new(125, 2);

// if calories calcuated from macros is 10% greater the calories the user has input:
// if energy_from_macros > energy_numeric_with_buffer {
//     non_field_errors.push(format!("Calories provided are too low for the amount of protein, carbs and fat you have entered. Please review your input, if correct set calories to {energy_from_macros} kcal."));
// }
// // if calories the user has input is 25% greater than the calories calculated from macros:
// if energy_numeric > energy_from_macros_with_buffer {
//     non_field_errors.push(format!(
//         "Calories provided are too high for the amount of protein, carbs and fat you have entered. Please review your input, if correct set calories to {energy_from_macros_with_buffer} kcal.",
//     ))
// }
