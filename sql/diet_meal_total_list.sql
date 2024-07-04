WITH
    meal_total AS (
        SELECT
            t3.id,
            t3.name,
            t3.slug,
            t3.ordering,
            t1.user_id,
            SUM(t1.quantity * t2.energy) AS total_energy,
            SUM(t1.quantity * t2.protein) AS total_protein,
            SUM(t1.quantity * t2.carbohydrate) AS total_carbohydrate,
            SUM(t1.quantity * t2.fat) AS total_fat,
            SUM(t1.quantity * t2.saturates) AS total_saturates,
            SUM(t1.quantity * t2.sugars) AS total_sugars,
            SUM(t1.quantity * t2.fibre) AS total_fibre,
            SUM(t1.quantity * t2.salt) AS total_salt
        FROM
            meal_of_day t3
            LEFT JOIN food_log t1 ON t3.id = t1.meal_of_day_id
            AND t1.date = $2
            LEFT JOIN food t2 ON t2.id = t1.food_id
            LEFT JOIN users_user t4 ON t4.id = t1.user_id
            AND t4.username = $1
        GROUP BY
            t3.id,
            t3.name,
            t3.slug,
            t3.ordering,
            t1.user_id
    )
SELECT
    id,
    name,
    slug,
    ordering,
    user_id,
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
    meal_total
ORDER BY
    ordering;