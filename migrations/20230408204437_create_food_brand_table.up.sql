-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS food_brand (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(50) UNIQUE NOT NULL,
        slug VARCHAR(50) UNIQUE NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        created_by_id UUID NOT NULL,
        updated_by_id UUID,
        CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users_user (id),
        CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users_user (id)
    )