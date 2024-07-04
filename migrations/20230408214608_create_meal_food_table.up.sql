-- Add up migration script here
CREATE TABLE IF NOT EXISTS
    meal_food (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        quantity NUMERIC(5, 2) NOT NULL,
        food_id UUID NOT NULL,
        meal_id UUID NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_food FOREIGN KEY (food_id) REFERENCES food (id),
        CONSTRAINT fk_meal FOREIGN KEY (meal_id) REFERENCES meal (id) ON DELETE CASCADE
    )