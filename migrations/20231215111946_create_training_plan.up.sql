/*
training_plan
represents a training plan over a series of weeks
- duration = duration in weeks

workout_plan
represents a workout plan
- can be linked to a training plan
- should contain one or more exercise plans
weekdays
- 0 = n/a
- 1, 2, 3, 4, 5, 6, 7 = mon-sun

exercise_plan
represents an exercise plan
- should be linked to a workout plan
- should contain one or more set plans

set_plan
represents a set plan
- should be linked to an exercise plan
- should contain one or more set plans
 */
CREATE TABLE IF NOT EXISTS
    training_plan (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL,
        "name" VARCHAR(255) UNIQUE NOT NULL,
        slug VARCHAR(255) UNIQUE NOT NULL,
        duration INTEGER NOT NULL DEFAULT 1,
        "description" VARCHAR(255),
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id) ON DELETE CASCADE,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id) ON DELETE CASCADE,
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id) ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS
    workout_plan (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        training_plan_id UUID,
        "order" INTEGER NOT NULL DEFAULT 1,
        "weekday" INTEGER NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_training_plan_id FOREIGN KEY (training_plan_id) REFERENCES training_plan (id) ON DELETE CASCADE,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id) ON DELETE CASCADE,
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id) ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS
    exercise_plan (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        workout_plan_id UUID NOT NULL,
        movement_id UUID NOT NULL,
        "order" INTEGER NOT NULL DEFAULT 1,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_workout_plan_id FOREIGN KEY (workout_plan_id) REFERENCES workout_plan (id) ON DELETE CASCADE,
        CONSTRAINT fk_movement_id FOREIGN KEY (movement_id) REFERENCES movement (id) ON DELETE CASCADE,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id) ON DELETE CASCADE,
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id) ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS
    set_plan (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        exercise_plan_id UUID NOT NULL,
        "order" INTEGER NOT NULL DEFAULT 1,
        "weight" NUMERIC(8, 2) NOT NULL DEFAULT 0.00,
        reps INTEGER NOT NULL DEFAULT 0,
        rest INTEGER NOT NULL DEFAULT 0,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_exercise_plan_id FOREIGN KEY (exercise_plan_id) REFERENCES exercise_plan (id) ON DELETE CASCADE,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id) ON DELETE CASCADE,
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id) ON DELETE CASCADE
    );