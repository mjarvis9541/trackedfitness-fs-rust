WITH
    week_total AS (
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
            JOIN food t2 ON t2.id = t1.food_id
            JOIN users_user t5 ON t5.id = t1.user_id
        WHERE
            t5.username = 'michael'
            AND t1.date >= '2024-05-13'::date
            AND t1.date < '2024-05-13'::date + INTERVAL '7 days'
        GROUP BY
            t1.user_id
    ),
    day_average AS (
        SELECT
            user_id,
            AVG(total_energy) AS avg_energy,
            AVG(total_protein) AS avg_protein,
            AVG(total_carbohydrate) AS avg_carbohydrate,
            AVG(total_fat) AS avg_fat,
            AVG(total_saturates) AS avg_saturates,
            AVG(total_sugars) AS avg_sugars,
            AVG(total_fibre) AS avg_fibre,
            AVG(total_salt) AS avg_salt
        FROM
            week_total
        GROUP BY
            user_id
    )
SELECT
    user_id,
    avg_energy,
    avg_protein,
    avg_carbohydrate,
    avg_fat,
    avg_saturates,
    avg_sugars,
    avg_fibre,
    avg_salt
FROM
    day_average;