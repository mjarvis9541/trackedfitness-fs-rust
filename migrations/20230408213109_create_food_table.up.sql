-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS food (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(255) UNIQUE NOT NULL,
        slug VARCHAR(255) UNIQUE NOT NULL,
        brand_id UUID NOT NULL,
        data_value INTEGER NOT NULL,
        data_measurement VARCHAR(3) NOT NULL,
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
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_brand FOREIGN KEY (brand_id) REFERENCES food_brand (id)
    )