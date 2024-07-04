WITH
    food_total AS (
        SELECT
            t1.id,
            t1.user_id,
            t1.date,
            t5.username,
            t2.id AS food_id,
            t2.name AS food_name,
            t2.slug AS food_slug,
            t3.id AS brand_id,
            t3.name AS brand_name,
            t3.slug AS brand_slug,
            t4.id AS meal_of_day_id,
            t4.name AS meal_of_day_name,
            t4.slug AS meal_of_day_slug,
            t4.ordering AS meal_of_day_ordering,
            SUM(t1.quantity * t2.data_value) AS data_value,
            t2.data_measurement AS data_measurement,
            SUM(t1.quantity * t2.energy) AS total_energy,
            SUM(t1.quantity * t2.protein) AS total_protein,
            SUM(t1.quantity * t2.carbohydrate) AS total_carbohydrate,
            SUM(t1.quantity * t2.fat) AS total_fat,
            SUM(t1.quantity * t2.saturates) AS total_saturates,
            SUM(t1.quantity * t2.sugars) AS total_sugars,
            SUM(t1.quantity * t2.fibre) AS total_fibre,
            SUM(t1.quantity * t2.salt) AS total_salt
        FROM
            food_log t1
            JOIN food t2 ON t2.id = t1.food_id
            JOIN food_brand t3 ON t3.id = t2.brand_id
            JOIN meal_of_day t4 ON t4.id = t1.meal_of_day_id
            JOIN users_user t5 ON t5.id = t1.user_id
        WHERE
            t5.username = $1
            AND t1.date = $2
        GROUP BY
            t1.id,
            t1.user_id,
            t5.username,
            t1.date,
            t2.id,
            t2.name,
            t2.slug,
            t2.data_value,
            t2.data_measurement,
            t3.id,
            t3.name,
            t3.slug,
            t4.id,
            t4.name,
            t4.slug,
            t4.ordering
    )
SELECT
    id,
    user_id,
    username,
    date,
    food_id,
    food_name,
    food_slug,
    brand_id,
    brand_name,
    brand_slug,
    meal_of_day_id,
    meal_of_day_name,
    meal_of_day_slug,
    meal_of_day_ordering,
    data_value AS "data_value!",
    data_measurement,
    COALESCE(total_energy, 0) AS "energy!",
    COALESCE(total_protein, 0) AS "protein!",
    COALESCE(total_carbohydrate, 0) AS "carbohydrate!",
    COALESCE(total_fat, 0) AS "fat!",
    COALESCE(total_saturates, 0) AS "saturates!",
    COALESCE(total_sugars, 0) AS "sugars!",
    COALESCE(total_fibre, 0) AS "fibre!",
    COALESCE(total_salt, 0) AS "salt!",
    COALESCE(
        (total_protein * 4) / NULLIF(total_energy, 0) * 100,
        0
    ) AS "protein_pct!",
    COALESCE(
        (total_carbohydrate * 4) / NULLIF(total_energy, 0) * 100,
        0
    ) AS "carbohydrate_pct!",
    COALESCE(
        (total_fat * 9) / NULLIF(total_energy, 0) * 100,
        0
    ) AS "fat_pct!"
FROM
    food_total
ORDER BY
    meal_of_day_ordering;