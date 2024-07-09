WITH
    date_series AS (
        SELECT
            generate_series($2::date, $3::date, '1 day'::interval) AS date
    ),
    user_stats AS (
        SELECT
            w.user_id,
            w.date::date AS date,
            COUNT(DISTINCT w.id) AS total_workouts,
            COUNT(DISTINCT e.id) AS total_exercises,
            COUNT(DISTINCT ts.id) AS total_sets,
            SUM(ts.reps) AS total_reps
        FROM
            workout w
            LEFT JOIN exercise e ON w.id = e.workout_id
            LEFT JOIN tracked_set ts ON e.id = ts.exercise_id
        GROUP BY
            w.user_id,
            w.date::date
    ),
    user_filtered AS (
        SELECT
            u.id AS user_id,
            u.username
        FROM
            users_user u
        WHERE
            u.username = $1
    )
SELECT
    uf.username,
    ds.date::date AS "date!",
    COALESCE(us.total_workouts, 0) AS "total_workouts!",
    COALESCE(us.total_exercises, 0) AS "total_exercises!",
    COALESCE(us.total_sets, 0) AS "total_sets!",
    COALESCE(us.total_reps, 0) AS "total_reps!"
FROM
    date_series ds
    CROSS JOIN user_filtered uf
    LEFT JOIN user_stats us ON ds.date = us.date
    AND uf.user_id = us.user_id
ORDER BY
    uf.username,
    ds.date;