-- Add up migration script here
ALTER TABLE workout_plan
DROP COLUMN sequence,
DROP COLUMN weekday;