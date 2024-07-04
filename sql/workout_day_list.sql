WITH
    numbered_exercise AS (
        SELECT
            w.user_id,
            w.id AS workout_id,
            e.id AS exercise_id,
            e.order AS exercise_order,
            ROW_NUMBER() OVER (
                PARTITION BY
                    w.user_id,
                    e.movement_id
                ORDER BY
                    w.date,
                    w.created_at,
                    e.created_at
            ) AS exercise_rank,
            e.movement_id,
            w.date AS workout_date,
            w.created_at AS workout_created_at,
            e.created_at AS exercise_created_at
        FROM
            workout w
            LEFT JOIN exercise e ON e.workout_id = w.id
    ),
    numbered_set AS (
        SELECT
            s.exercise_id,
            s.id AS set_id,
            s.order AS set_order,
            s.weight,
            s.reps,
            s.rest,
            ROW_NUMBER() OVER (
                PARTITION BY
                    s.exercise_id
                ORDER BY
                    s.order,
                    s.created_at
            ) AS set_rank
        FROM
            tracked_set s
    ),
    workout_aggregates AS (
        SELECT
            ne.workout_id,
            COUNT(DISTINCT ne.exercise_id) AS exercise_count,
            COUNT(DISTINCT ns.set_id) AS set_count,
            SUM(ns.reps) AS rep_count
        FROM
            numbered_exercise ne
            JOIN numbered_set ns ON ns.exercise_id = ne.exercise_id
        GROUP BY
            ne.workout_id
    ),
    exercise_aggregates AS (
        SELECT
            ns.exercise_id,
            COUNT(*) AS set_count,
            SUM(ns.reps) AS rep_count
        FROM
            numbered_set ns
        GROUP BY
            ns.exercise_id
    )
SELECT
    ne.user_id as user_id,
    uu.username,
    ne.workout_id,
    ne.workout_date,
    ne.workout_created_at,
    ne.exercise_id,
    ne.exercise_created_at,
    ne.exercise_order,
    m.name AS movement_name,
    mg.name AS muscle_group_name,
    ns.set_order,
    ns.set_id,
    ns.weight,
    ns.reps,
    ns.rest,
    prev_ne.workout_id AS "previous_workout_id?",
    prev_ne.workout_date AS "previous_workout_date?",
    prev_ne.exercise_id AS "previous_exercise_id?",
    prev_ns.set_id AS "previous_set_id?",
    prev_ns.weight AS "previous_weight?",
    prev_ns.reps AS "previous_reps?",
    wa.exercise_count AS workout_exercise_count,
    wa.set_count AS workout_set_count,
    wa.rep_count AS workout_rep_count,
    ea.set_count AS exercise_set_count,
    ea.rep_count AS exercise_rep_count
FROM
    numbered_exercise ne
    LEFT JOIN numbered_set ns ON ne.exercise_id = ns.exercise_id
    LEFT JOIN numbered_exercise prev_ne ON prev_ne.user_id = ne.user_id
    AND prev_ne.movement_id = ne.movement_id
    AND prev_ne.exercise_rank = ne.exercise_rank - 1
    LEFT JOIN numbered_set prev_ns ON prev_ns.exercise_id = prev_ne.exercise_id
    AND prev_ns.set_rank = ns.set_rank
    LEFT JOIN movement m ON m.id = ne.movement_id
    LEFT JOIN muscle_group mg ON mg.id = m.muscle_group_id
    LEFT JOIN users_user uu ON uu.id = ne.user_id
    LEFT JOIN workout_aggregates wa ON wa.workout_id = ne.workout_id
    LEFT JOIN exercise_aggregates ea ON ea.exercise_id = ne.exercise_id
WHERE
    uu.username = $1
    AND ne.workout_date = $2
ORDER BY
    ne.workout_date,
    ne.workout_created_at,
    ne.exercise_order,
    ne.exercise_created_at,
    ns.set_order