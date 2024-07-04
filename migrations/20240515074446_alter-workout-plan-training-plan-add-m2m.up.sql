-- Add up migration script here
-- Step 1: Create the join table
CREATE TABLE IF NOT EXISTS
    training_plan_workout_plan (
        training_plan_id uuid NOT NULL,
        workout_plan_id uuid NOT NULL,
        sequence INTEGER NOT NULL DEFAULT 1 CHECK (
            sequence >= 1
            AND sequence <= 100
        ),
        weekday INTEGER NOT NULL DEFAULT 0 CHECK (
            weekday >= 0
            AND weekday <= 7
        ),
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_workout_plan FOREIGN KEY (workout_plan_id) REFERENCES workout_plan (id) ON DELETE CASCADE,
        CONSTRAINT fk_training_plan FOREIGN KEY (training_plan_id) REFERENCES training_plan (id) ON DELETE CASCADE,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        PRIMARY KEY (training_plan_id, workout_plan_id)
    );

-- Step 2: Insert existing data into the join table
INSERT INTO
    training_plan_workout_plan (
        workout_plan_id,
        training_plan_id,
        sequence,
        weekday,
        created_at,
        updated_at,
        created_by_id,
        updated_by_id
    )
SELECT
    id,
    training_plan_id,
    sequence,
    weekday,
    created_at,
    updated_at,
    created_by_id,
    updated_by_id
FROM
    workout_plan
WHERE
    training_plan_id IS NOT NULL;