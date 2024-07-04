CREATE TABLE IF NOT EXISTS
    diet_target (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL,
        "date" DATE NOT NULL,
        "weight" NUMERIC(5, 2) NOT NULL,
        energy INTEGER NOT NULL,
        fat NUMERIC(4, 1) NOT NULL,
        saturates NUMERIC(4, 1) NOT NULL,
        carbohydrate NUMERIC(4, 1) NOT NULL,
        sugars NUMERIC(4, 1) NOT NULL,
        fibre NUMERIC(4, 1) NOT NULL,
        protein NUMERIC(4, 1) NOT NULL,
        salt NUMERIC(5, 2) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        CONSTRAINT diet_target_unique_user_id_date UNIQUE (user_id, "date")
    )