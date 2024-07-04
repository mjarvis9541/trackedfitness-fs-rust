-- Add up migration script here
ALTER TABLE tracked_set
DROP CONSTRAINT fk_exercise_id;

ALTER TABLE tracked_set
ADD CONSTRAINT fk_exercise_id FOREIGN KEY (exercise_id) REFERENCES exercise (id) ON DELETE CASCADE