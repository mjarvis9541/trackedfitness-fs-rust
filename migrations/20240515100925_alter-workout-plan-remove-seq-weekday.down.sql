-- Add down migration script here
ALTER TABLE workout_plan
ADD COLUMN sequence INTEGER NOT NULL DEFAULT 1 CHECK (
    sequence >= 1
    AND sequence <= 100
),
ADD COLUMN weekday INTEGER NOT NULL DEFAULT 0 CHECK (
    weekday >= 0
    AND weekday <= 7
);