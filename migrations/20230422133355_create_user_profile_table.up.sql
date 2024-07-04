CREATE TABLE IF NOT EXISTS
    user_profile (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID UNIQUE NOT NULL,
        sex VARCHAR(1) NOT NULL,
        HEIGHT NUMERIC(5, 2) NOT NULL,
        date_of_birth DATE NOT NULL,
        fitness_goal VARCHAR(2) NOT NULL,
        activity_level VARCHAR(2) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    )