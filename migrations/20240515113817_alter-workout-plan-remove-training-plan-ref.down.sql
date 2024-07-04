-- Add down migration script here
ALTER TABLE workout_plan
ADD COLUMN training_plan_id UUID;

ALTER TABLE workout_plan
ADD CONSTRAINT fk_training_plan_id FOREIGN KEY (training_plan_id) REFERENCES training_plan (id) ON DELETE CASCADE;