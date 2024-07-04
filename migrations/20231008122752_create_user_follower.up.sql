CREATE TABLE IF NOT EXISTS
    user_follower (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL,
        follower_id UUID NOT NULL,
        status INTEGER NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        CONSTRAINT unique_user_follower UNIQUE (user_id, follower_id),
        CONSTRAINT cannot_follow_self CHECK (NOT user_id = follower_id),
        CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users_user (id),
        CONSTRAINT fk_follower_id FOREIGN KEY (follower_id) REFERENCES users_user (id)
    )