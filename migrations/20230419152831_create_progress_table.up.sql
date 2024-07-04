-- Add up migration script here
CREATE TABLE IF NOT EXISTS
    progress (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL,
        "date" DATE NOT NULL,
        weight_kg NUMERIC(5, 2),
        energy_burnt INTEGER,
        notes VARCHAR(250),
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        CONSTRAINT progress_unique_user_id_date UNIQUE (user_id, "date")
    )