-- Add up migration script here
CREATE TABLE IF NOT EXISTS
    food_log (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        date DATE NOT NULL,
        user_id UUID NOT NULL,
        food_id UUID NOT NULL,
        meal_of_day_id UUID NOT NULL,
        quantity NUMERIC(5, 2) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_food_id FOREIGN KEY (food_id) REFERENCES food (id),
        CONSTRAINT fk_meal_of_day_id FOREIGN KEY (meal_of_day_id) REFERENCES meal_of_day (id)
    )