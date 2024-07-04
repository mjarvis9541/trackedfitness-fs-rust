WITH
    day_total AS (
        SELECT
            t1.user_id,
            t1.date,
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
            LEFT JOIN food t2 ON t2.id = t1.food_id
            LEFT JOIN users_user t3 ON t3.id = t1.user_id
        WHERE
            t3.username = $1
            AND t1.date = $2
        GROUP BY
            t1.user_id,
            t1.date
    )
SELECT
    user_id,
    date,
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
    day_total
LIMIT
    1