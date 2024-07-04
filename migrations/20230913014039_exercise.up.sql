-- Add up migration script here
CREATE TABLE IF NOT EXISTS
    muscle_group (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "name" VARCHAR(255) NOT NULL,
        slug VARCHAR(255) UNIQUE NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    );

CREATE TABLE IF NOT EXISTS
    movement (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        muscle_group_id UUID NOT NULL,
        "name" VARCHAR(255) NOT NULL,
        slug VARCHAR(255) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_muscle_group_id FOREIGN KEY (muscle_group_id) REFERENCES muscle_group (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    );

CREATE TABLE IF NOT EXISTS
    workout (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL,
        "date" DATE NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    );

CREATE TABLE IF NOT EXISTS
    exercise (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        workout_id UUID NOT NULL,
        movement_id UUID NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_workout_id FOREIGN KEY (workout_id) REFERENCES workout (id),
        CONSTRAINT fk_movement_id FOREIGN KEY (movement_id) REFERENCES movement (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    );

CREATE TABLE IF NOT EXISTS
    tracked_set (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        exercise_id UUID NOT NULL,
        weight NUMERIC(8, 2) NOT NULL,
        reps INTEGER NOT NULL,
        rest INTEGER NOT NULL,
        notes VARCHAR(255),
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_exercise_id FOREIGN KEY (exercise_id) REFERENCES exercise (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    );