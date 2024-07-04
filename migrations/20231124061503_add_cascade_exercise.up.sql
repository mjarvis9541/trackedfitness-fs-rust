-- Add up migration script here
ALTER TABLE exercise
DROP CONSTRAINT fk_workout_id;

ALTER TABLE exercise
ADD CONSTRAINT fk_workout_id FOREIGN KEY (workout_id) REFERENCES workout (id) ON DELETE CASCADE